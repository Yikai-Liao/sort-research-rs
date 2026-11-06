#!/usr/bin/env python3
"""Summarize the upstream Google Benchmark JSON without deleting outliers."""
import csv,json,statistics,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parent
name=sys.argv[1] if len(sys.argv)>1 else 'starrocks-full'
doc=json.loads((ROOT/(name+'.json')).read_text())
groups={}
for r in doc['benchmarks']:
    if r.get('run_type')!='iteration':continue
    groups.setdefault(r['run_name'],[]).append(r)

def median(name):return statistics.median(r['cpu_time'] for r in groups[name])
rows=[]
for key,samples in groups.items():
    if not key.startswith('BM_Tessera_'):continue
    suffix=key.removeprefix('BM_Tessera_')
    shape,n=suffix.split('/')
    typ,distribution=shape.split('_',1)
    a='BM_RadixSort_'+suffix
    aa='BM_RadixSortAA_'+suffix
    cold='BM_RadixSortColdScratch_'+suffix
    pdq='BM_PdqSort_'+suffix
    if a not in groups:continue
    row={'type':typ,'distribution':distribution,'n':int(n),'tessera_cpu_ns':median(key),'sr_cpu_ns':median(a),'tessera_over_sr':median(key)/median(a),'repetitions':len(samples)}
    if aa in groups:row['AA_over_A']=median(aa)/median(a)
    if cold in groups:
        row['sr_cold_cpu_ns']=median(cold)
        row['tessera_over_sr_cold']=median(key)/median(cold)
    if pdq in groups:
        row['pdq_cpu_ns']=median(pdq)
        row['tessera_over_pdq']=median(key)/median(pdq)
    rows.append(row)
rows.sort(key=lambda r:(r['type'],r['distribution'],r['n']))
fields=list(dict.fromkeys(k for r in rows for k in r))
with (ROOT/(name+'-summary.csv')).open('w') as f:
    w=csv.DictWriter(f,fields);w.writeheader();w.writerows(rows)
for typ in sorted({r['type'] for r in rows}):
    rs=[r for r in rows if r['type']==typ]
    print(typ,'wins',sum(r['tessera_over_sr']<1 for r in rs),'/',len(rs),'median ratio',round(statistics.median(r['tessera_over_sr'] for r in rs),3))
print('UniformRandom:')
for r in rows:
    if r['distribution']=='UniformRandom':print(r)
print('Slowest Tessera/SR:')
for r in sorted(rows,key=lambda r:r['tessera_over_sr'],reverse=True)[:12]:print(r)
print('AA min/max:',min((r.get('AA_over_A',1) for r in rows),default=0),max((r.get('AA_over_A',1) for r in rows),default=0))
stable=[]
for key in groups:
    if key.startswith('BM_TesseraStable_'):
        suffix=key.removeprefix('BM_TesseraStable_')
        stable.append({'case':suffix,'tessera_cpu_ns':median(key),'sr_cpu_ns':median('BM_RadixSortStable_'+suffix),'pdq_cpu_ns':median('BM_PdqSortWithStabilization_'+suffix)})
if stable:
    with (ROOT/(name+'-stable.csv')).open('w') as f:
        w=csv.DictWriter(f,list(stable[0]));w.writeheader();w.writerows(stable)
    print('Stable LowCard:')
    for r in stable:print(r)
