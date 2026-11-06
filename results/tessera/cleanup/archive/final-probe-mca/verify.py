from pathlib import Path
import hashlib
import json
import re
import struct
import subprocess

out = Path(__file__).resolve().parent
root = out.parent
sha = lambda data: hashlib.sha256(data).hexdigest()
old_binary = root / 'fold-probe-bench'
new_binary = root / 'final-bench'
old = old_binary.read_bytes()
new = new_binary.read_bytes()
assert sha(old) == '764f6b424e644914e79287ad94475ffb79c9b7de701caafb293af61964b1f535'
assert sha(new) == 'c4196d5168747f1995c10c501729044a1cc13873647366a4cdb7bd3676ab6e1e'
frozen = json.loads((root / 'frozen-source.json').read_text())
for name, expected in frozen.items():
    assert sha((root / 'final-source' / name).read_bytes()) == expected
    assert sha((root.parent.parent / name).read_bytes()) == expected
assert frozen['src/other/rust_tessera_radix/engine.rs'] == 'e81e08fba9c49830a2a56d84567bb57add328575bbcc5330619537045ef58de4'
assert frozen['src/other/rust_block_lsd/core.rs'] == 'f2fafad8d1a7b0e859fa53c207e9068da8154c60e83f38c07df2ee0cfbd83b96'

def section(data, wanted):
    assert data[:6] == b'\x7fELF\x02\x01'
    offset = struct.unpack_from('<Q', data, 0x28)[0]
    size, count, names_index = struct.unpack_from('<HHH', data, 0x3a)
    headers = [struct.unpack_from('<IIQQQQIIQQ', data, offset + i * size) for i in range(count)]
    names_header = headers[names_index]
    names = data[names_header[4]:names_header[4] + names_header[5]]
    for header in headers:
        start = header[0]
        name = names[start:names.index(b'\0', start)].decode()
        if name == wanted:
            return header[3], data[header[4]:header[4] + header[5]]
    raise AssertionError(wanted)

old_start, old_text = section(old, '.text')
new_start, new_text = section(new, '.text')
meta = {
    'status': 'passed', 'analysis_cpu': 0,
    'binary_sha256': {'previous': sha(old), 'final': sha(new)},
    'source_manifest': 'frozen-source.json', 'source_sha256': frozen,
    'text_section': {'previous_start': hex(old_start), 'final_start': hex(new_start),
                     'previous_sha256': sha(old_text), 'final_sha256': sha(new_text),
                     'all_bytes_equal': old_text == new_text},
    'paths': [],
    'model_reuse': 'exact same addresses, instruction bytes, and MCA input as the verified fold-probe ELF; no new MCA execution is necessary',
    'model_scope': 'fixed descending instruction traces; branch directions, warm L1 and noalias are assumptions; no whole-sort performance inference',
}
root_address = 0x56870
root_length = 0xc53
old_root = old_text[root_address - old_start:root_address - old_start + root_length]
new_root = new_text[root_address - new_start:root_address - new_start + root_length]
assert len(old_root) == len(new_root) == root_length and old_root == new_root
symbols = subprocess.check_output(['nm', '-S', '-n', '-C', str(new_binary)], text=True)
root_symbol = next(line for line in symbols.splitlines() if line.endswith('engine::sort_with_tile_bytes::<u64>'))
assert [int(value, 16) for value in root_symbol.split()[:2]] == [root_address, root_length]
assert not any('Chain<' in line and '::is_sorted' in line for line in symbols.splitlines())
meta['u64_root'] = {'start': hex(root_address), 'length': hex(root_length), 'symbol': root_symbol,
                    'previous_sha256': sha(old_root), 'final_sha256': sha(new_root),
                    'all_bytes_equal': True, 'chain_is_sorted_symbols_present': False}
raw = subprocess.check_output(['objdump', '-d', '-C', '--no-show-raw-insn', f'--start-address={root_address}', f'--stop-address={root_address + root_length}', str(new_binary)], text=True)
(out / 'final-u64-root.asm').write_text(raw)
prior = json.loads((root / 'fold-probe-mca/analysis.json').read_text())
for region in ['probe', 'confirm-descending']:
    path = next(p for p in prior['paths'] if p['region'] == region and p['label'] == 'unchecked-fold')
    start, end = int(path['start'], 16), int(path['end_exclusive'], 16)
    old_slice = old_text[start - old_start:end - old_start]
    new_slice = new_text[start - new_start:end - new_start]
    assert old_slice == new_slice
    stem = region + '-unchecked-fold'
    old_input = (root / 'fold-probe-mca' / (stem + '.s')).read_bytes()
    assert sha(old_input) == prior['artifact_sha256'][stem + '.s']
    fragment = subprocess.check_output(['objdump', '-d', '--no-show-raw-insn', f'--start-address={start}', f'--stop-address={end}', str(new_binary)], text=True)
    (out / (region + '.asm')).write_text(fragment)
    addresses = {int(a, 16) for a in path['selected_addresses']}
    instructions = []
    for line in fragment.splitlines():
        match = re.match(r'\s*([0-9a-f]+):\s+(.+)', line)
        if match:
            instructions.append((int(match[1], 16), match[2].split(' #')[0]))
    assert [hex(a) for a, op in instructions] == path['selected_addresses']
    lines = ['.text']
    for address, op in instructions:
        lines.append(f'.L{address:x}:')
        branch = re.match(r'(j[a-z]+)\s+([0-9a-f]+)', op)
        if branch:
            target = int(branch[2], 16)
            op = branch[1] + (' .L' + format(target, 'x') if target in addresses else ' .Lexit')
        lines.append('  ' + op)
    lines.append('.Lexit:')
    regenerated = ('\n'.join(lines) + '\n').encode()
    assert regenerated == old_input
    (out / (region + '.s')).write_bytes(regenerated)
    report = (root / 'fold-probe-mca' / (stem + '.mca.txt')).read_bytes()
    assert sha(report) == prior['artifact_sha256'][stem + '.mca.txt']
    meta['paths'].append({'region': region, 'start': hex(start), 'end_exclusive': hex(end),
                          'machine_bytes_sha256': sha(new_slice), 'machine_bytes_equal_to_764': True,
                          'mca_input_sha256': sha(regenerated), 'mca_input_equal_to_764': True,
                          'prior_analysis': '../fold-probe-mca/analysis.json',
                          'prior_report': '../fold-probe-mca/' + stem + '.mca.txt',
                          'prior_report_sha256': sha(report), 'command': path['command'],
                          'metrics': path['metrics'], 'units': 'per sample pair' if region == 'probe' else 'per next record'})
    print(region, path['metrics'])
meta['artifact_sha256'] = {p.name: sha(p.read_bytes()) for p in out.iterdir() if p.is_file() and p.name != 'analysis.json'}
(out / 'analysis.json').write_text(json.dumps(meta, indent=2) + '\n')
print('PASS u64 root all 3155 machine-code bytes identical; both actual traces and MCA inputs identical; all 12 current/frozen source hashes match')
print('whole text identical:', old_text == new_text)
