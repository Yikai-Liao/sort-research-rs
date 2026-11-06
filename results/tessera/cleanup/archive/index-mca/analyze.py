from pathlib import Path
import hashlib
import json
import re
import subprocess

out = Path(__file__).resolve().parent
root = out.parent
specs = [
    ('probe', 'window-baseline', 'final-bench', 0x56be0, 0x56c1e, None, 1),
    ('probe', 'checked', 'range-probe-bench', 0x56b50, 0x56b8e, None, 1),
    ('probe', 'unchecked', 'unchecked-probe-bench', 0x56a10, 0x56a41, None, 1),
    ('confirm', 'checked', 'range-probe-bench', 0x56d40, 0x56d56, None, 1),
    ('confirm', 'unchecked', 'unchecked-probe-bench', 0x89550, 0x89564, None, 1),
    ('reverse', 'checked', 'range-probe-bench', 0x57180, 0x571cd, None, 8),
    ('reverse', 'unchecked', 'unchecked-probe-bench', 0x57333, 0x57380, None, 8),
    ('repair', 'checked', 'range-probe-bench', 0x57230, 0x57287,
     [0x57230, 0x57233, 0x57239, 0x5723c, 0x57240, 0x57243, 0x57246, 0x5724a, 0x5724e, 0x57251, 0x57253, 0x57260, 0x57264, 0x57270, 0x57273, 0x57276, 0x57279, 0x5727f, 0x57282, 0x57285], 1),
    ('repair', 'unchecked', 'unchecked-probe-bench', 0x573e0, 0x57437,
     [0x573e0, 0x573e3, 0x573e9, 0x573ec, 0x573f0, 0x573f3, 0x573f6, 0x573fa, 0x573fe, 0x57401, 0x57403, 0x57410, 0x57414, 0x57420, 0x57423, 0x57426, 0x57429, 0x5742f, 0x57432, 0x57435], 1),
    ('scatter', 'checked', 'final-bench', 0x3f0d0, 0x3f1bf, None, 4),
    ('scatter', 'unchecked', 'scatter-unchecked-bench', 0x3c500, 0x3c5df, None, 4),
]
meta = {
    'tool': subprocess.check_output(['llvm-mca-19', '--version'], text=True).splitlines()[0],
    'model_cpu': 'skylake-avx512', 'iterations': 1000,
    'paths': [],
    'conditions': {
        'probe': 'steady-state descending sample, no out-of-order/terminal branch taken; same sample points',
        'confirm': 'steady-state descending adjacent comparison, no terminal/out-of-order branch taken',
        'reverse': 'vectorized swap of four pairs, 8 u64 records processed per iteration',
        'repair': 'strictly distinct keys after global reversal, one single-record equal run per iteration; unexecuted equal-run and nontrivial reverse paths omitted',
        'scatter': 'four u64 digit scatters, no output-block rollover; L1/no-alias model omits actual repeated-bucket memory dependencies',
    },
    'model_scope': 'fixed instruction traces from linked ELF; branch directions are trace assumptions, real prediction/front-end address alignment/cache/OS scheduling are not modeled',
}
for region, label, name, start, end, selected, records in specs:
    binary = root / name
    raw = subprocess.check_output(['objdump', '-d', '--no-show-raw-insn', f'--start-address={start}', f'--stop-address={end}', str(binary)], text=True)
    stem = f'{region}-{label}'
    (out / (stem + '.asm')).write_text(raw)
    instructions = []
    for line in raw.splitlines():
        match = re.match(r'\s*([0-9a-f]+):\s+(.+)', line)
        if not match:
            continue
        address = int(match[1], 16)
        op = match[2].split(' #')[0]
        if selected is not None and address not in selected:
            continue
        if op.startswith(('nop', 'data16', 'cs ')):
            continue
        assert '(bad)' not in op and '.byte' not in op, (stem, op)
        instructions.append((address, op))
    if selected is not None:
        assert [a for a, _ in instructions] == selected
    addresses = {a for a, _ in instructions}
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
    command = ['taskset', '-c', '5', 'llvm-mca-19', '-mcpu=skylake-avx512', '-iterations=1000', '-noalias=true', '-bottleneck-analysis', '-all-stats', '-timeline', '-timeline-max-iterations=2', '-timeline-max-cycles=160']
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
    metrics['cycles_per_record_or_sample'] = metrics['cycles_per_iteration'] / records
    metrics['uops_per_iteration'] = metrics['Total uOps'] / 1000
    metrics['static_instructions'] = len(instructions)
    meta['paths'].append({'region': region, 'label': label, 'binary': name, 'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'start': hex(start), 'end': hex(end), 'selected_addresses': [hex(a) for a, _ in instructions], 'records_per_iteration': records, 'command': command, 'metrics': metrics})
    print(stem, metrics, flush=True)
meta['artifact_sha256'] = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in out.iterdir() if p.is_file() and p.name != 'analysis.json'}
(out / 'analysis.json').write_text(json.dumps(meta, indent=2) + '\n')
