from pathlib import Path
import hashlib
import json
import re
import subprocess

out = Path(__file__).resolve().parent
root = out.parent
sources = {
    'window-chain': ('api-compat-bench', 'api-compat-source.json', 'api-compat-source'),
    'unchecked-chain': ('unchecked-probe-bench', 'unchecked-probe-source.json', 'unchecked-probe-source'),
    'window-fold': ('fold-window-bench', 'fold-window-source.json', 'fold-window-source'),
    'unchecked-fold': ('fold-probe-bench', 'fold-probe-source.json', 'fold-probe-source'),
}
specs = [
    ('probe', 'window-chain', 0x56be0, 0x56c1e),
    ('probe', 'unchecked-chain', 0x56a10, 0x56a41),
    ('probe', 'window-fold', 0x56ba0, 0x56bde),
    ('probe', 'unchecked-fold', 0x56910, 0x56941),
    ('confirm-descending', 'window-chain', 0x572b0, 0x572c6),
    ('confirm-descending', 'unchecked-chain', 0x89550, 0x89564),
    ('confirm-descending', 'window-fold', 0x57290, 0x572a9),
    ('confirm-descending', 'unchecked-fold', 0x56f15, 0x56f2e),
]
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
meta = {
    'tool': subprocess.check_output(['llvm-mca-19', '--version'], text=True).splitlines()[0],
    'model_cpu': 'skylake-avx512', 'iterations': 1000, 'analysis_cpu': 0,
    'conditions': {
        'probe': 'steady-state descending sample pair: ascending is false, descending is true; terminal and unsorted exits are not taken',
        'confirm-descending': 'steady-state descending adjacent-key check after first unequal key; remaining length is nonzero; unsorted and terminal exits are not taken',
    },
    'units': 'all traces process one sample pair or one next record per iteration',
    'scope': 'fixed linked-ELF instruction traces; branch direction and warm L1 are assumptions; real branch prediction, address alignment, instruction cache, OS scheduling, and complete-sort time are not modeled',
    'sources': {}, 'paths': [],
}

for label, (binary_name, source_json, source_dir) in sources.items():
    binary = root / binary_name
    frozen = json.loads((root / source_json).read_text())
    for name, expected in frozen.items():
        assert sha(root / source_dir / name) == expected, (label, name)
    symbols = subprocess.check_output(['nm', '-S', '-n', '-C', str(binary)], text=True)
    (out / (label + '-symbols.txt')).write_text(symbols)
    root_symbol = next(line for line in symbols.splitlines() if line.endswith('engine::sort_with_tile_bytes::<u64>'))
    start, length = [int(x, 16) for x in root_symbol.split()[:2]]
    raw = subprocess.check_output(['objdump', '-d', '-C', '--no-show-raw-insn', f'--start-address={start}', f'--stop-address={start + length}', str(binary)], text=True)
    (out / (label + '-root.asm')).write_text(raw)
    chain_symbols = [line for line in symbols.splitlines() if 'Chain<' in line and '::is_sorted' in line]
    if label.endswith('-fold'):
        assert not chain_symbols
        assert not any('Chain<' in line and 'call' in line for line in raw.splitlines())
    meta['sources'][label] = {
        'binary': binary_name, 'binary_sha256': sha(binary),
        'source_manifest': source_json, 'source_manifest_sha256': sha(root / source_json),
        'source_snapshot': source_dir, 'source_sha256': frozen,
        'u64_root': {'start': hex(start), 'length': hex(length), 'symbol': root_symbol},
        'chain_is_sorted_symbols': chain_symbols,
        'root_calls': [line.strip() for line in raw.splitlines() if '\tcall' in line],
    }

for region, label, start, end in specs:
    binary = root / sources[label][0]
    stem = region + '-' + label
    raw = subprocess.check_output(['objdump', '-d', '--no-show-raw-insn', f'--start-address={start}', f'--stop-address={end}', str(binary)], text=True)
    (out / (stem + '.asm')).write_text(raw)
    instructions = []
    for line in raw.splitlines():
        match = re.match(r'\s*([0-9a-f]+):\s+(.+)', line)
        if match:
            address = int(match[1], 16)
            op = match[2].split(' #')[0]
            assert '(bad)' not in op and '.byte' not in op and 'nop' not in op, (stem, op)
            instructions.append((address, op))
    assert instructions[0][0] == start
    addresses = {address for address, op in instructions}
    lines = ['.text']
    for address, op in instructions:
        lines.append(f'.L{address:x}:')
        branch = re.match(r'(j[a-z]+)\s+([0-9a-f]+)', op)
        if branch:
            target = int(branch[2], 16)
            op = branch[1] + (' .L' + format(target, 'x') if target in addresses else ' .Lexit')
        lines.append('  ' + op)
    lines.append('.Lexit:')
    asm = '\n'.join(lines) + '\n'
    (out / (stem + '.s')).write_text(asm)
    command = ['taskset', '-c', '0', 'llvm-mca-19', '-mcpu=skylake-avx512', '-iterations=1000', '-noalias=true', '-bottleneck-analysis', '-all-stats', '-timeline', '-timeline-max-iterations=2', '-timeline-max-cycles=160']
    proc = subprocess.run(command, input=asm, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    (out / (stem + '.mca.txt')).write_text(proc.stdout)
    (out / (stem + '.stderr')).write_text(proc.stderr)
    assert proc.returncode == 0, (stem, proc.stderr)
    metrics = {}
    for key in ['Instructions', 'Total Cycles', 'Total uOps', 'IPC', 'Block RThroughput']:
        match = re.search(r'^' + re.escape(key) + r':\s+([0-9.]+)', proc.stdout, re.M)
        assert match, (stem, key)
        metrics[key] = float(match[1])
    metrics['cycles_per_iteration'] = metrics['Total Cycles'] / 1000
    metrics['uops_per_iteration'] = metrics['Total uOps'] / 1000
    metrics['static_instructions'] = len(instructions)
    meta['paths'].append({'region': region, 'label': label, 'binary': sources[label][0], 'binary_sha256': sha(binary), 'start': hex(start), 'end_exclusive': hex(end), 'selected_addresses': [hex(address) for address, op in instructions], 'records_or_sample_pairs_per_iteration': 1, 'command': command, 'metrics': metrics})
    print(stem, metrics, flush=True)

meta['artifact_sha256'] = {p.name: sha(p) for p in out.iterdir() if p.is_file() and p.name != 'analysis.json'}
(out / 'analysis.json').write_text(json.dumps(meta, indent=2) + '\n')
