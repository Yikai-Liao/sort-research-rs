#!/usr/bin/env python3
import csv
from pathlib import Path
ROOT=Path(__file__).resolve().parent
sr=list(csv.DictReader((ROOT/'starrocks-confirm-summary.csv').open()))
stable=list(csv.DictReader((ROOT/'starrocks-confirm-stable.csv').open()))
ch=list(csv.DictReader((ROOT/'clickhouse-summary.csv').open()))
lines=['# 外部 benchmark 结果表','',
       'StarRocks 取 9 次长测的 CPU 耗时中位数之比；ClickHouse 取 9 个 block 中 Tessera / 两次 LSD 平均耗时的比值中位数。两表中均为 **Tessera / 对照耗时，小于 1 更快**。','',
       '## StarRocks：均匀随机键与行号','',
       '| 类型 | 4096 行 | 65536 行 | 100 万行 |','| --- | ---: | ---: | ---: |']
for typ in ['Int32','Int64','Float','Double']:
    cells=[next(r for r in sr if r['type']==typ and r['distribution']=='UniformRandom' and int(r['n'])==n) for n in [4096,65536,1000000]]
    lines.append('| '+typ+' | '+' | '.join(f"{float(r['tessera_over_sr']):.3f}" for r in cells)+' |')
lines+=['','对照为 PR 的预分配 scratch 路径，双方耗时包含原 benchmark 的输入复制和 tie 工作。','',
        '| 类型 | 行数 | PR warm（ms） | Tessera（ms） | A/A |','| --- | ---: | ---: | ---: | ---: |']
for r in sorted((r for r in sr if r['distribution']=='UniformRandom'),key=lambda r:(r['type'],int(r['n']))):
    lines.append(f"| {r['type']} | {r['n']} | {float(r['sr_cpu_ns'])/1e6:.4f} | {float(r['tessera_cpu_ns'])/1e6:.4f} | {float(r['AA_over_A']):.3f} |")
lines+=['','## StarRocks：低基数稳定排序并输出行号','',
        '| 类型 | 行数 | PR（ms） | Tessera（ms） | pdqsort + 稳定化（ms） | Tessera / PR |',
        '| --- | ---: | ---: | ---: | ---: | ---: |']
for r in sorted(stable,key=lambda r:(r['case'].split('/')[0],int(r['case'].split('/')[1]))):
    typ,n=r['case'].split('/')
    t,a,p=(float(r[x])/1e6 for x in ['tessera_cpu_ns','sr_cpu_ns','pdq_cpu_ns'])
    lines.append(f'| {typ} | {n} | {a:.4f} | {t:.4f} | {p:.4f} | {t/a:.3f} |')
lines+=['','## ClickHouse：UInt64 原示例程序','',
        '| 行数 | LSD A1 中位数（ms） | Tessera 中位数（ms） | 配对耗时比 | A/A 中位数 | 更快的 block |',
        '| --- | ---: | ---: | ---: | ---: | ---: |']
for r in ch:
    lines.append(f"| {r['n']} | {float(r['lsd_A1_median_seconds'])*1000:.3f} | {float(r['tessera_median_seconds'])*1000:.3f} | {float(r['median_paired_tessera_over_lsd']):.3f} | {float(r['median_AA']):.3f} | {r['faster_blocks']}/{r['blocks']} |")
lines+=['','4096 行的逐 block 耗时比为 0.740–1.630，A/A 为 0.825–1.678，两者接近且波动较大。其余三个尺寸的 9 个 block 中 Tessera 均更快；100 万与 1000 万行的 A/A 范围分别为 0.946–1.120、0.954–1.050。','',
        'ClickHouse 的完整逐轮顺序、种子和校验输出见 [clickhouse-rounds.csv](clickhouse-rounds.csv) 与 [clickhouse-outputs.jsonl](clickhouse-outputs.jsonl)。','']
(ROOT/'tables.md').write_text('\n'.join(lines))
