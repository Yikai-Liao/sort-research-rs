#!/usr/bin/env python3
"""Run ClickHouse's own example CLI in randomized, paired blocks."""
import csv,json,os,random,re,statistics,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parent
rng=random.Random(0x5EED1234)
arms=[('std',1),('lsd_A1',2),('lsd_A2',2),('pdq',5),('tessera',9)]
rows=[]
with (ROOT/'clickhouse-rounds.csv').open('w') as f, (ROOT/'clickhouse-outputs.jsonl').open('w') as logs:
    fields=['n','block','sequence','arm','method','seed','seconds','process_wall_seconds','returncode']
    writer=csv.DictWriter(f,fields);writer.writeheader()
    for n in [4096,65536,1000000,10000000]:
        for block in range(9):
            seed=0x5EED1234+block
            order=arms.copy();rng.shuffle(order)
            for sequence,(arm,method) in enumerate(order):
                cmd=['taskset','-c','2',str(ROOT/'clickhouse_example'),str(method),str(n)]
                start=time.monotonic()
                p=subprocess.run(cmd,text=True,capture_output=True,env={**os.environ,'BENCH_SEED':str(seed)})
                elapsed=time.monotonic()-start
                match=re.search(r'Sorted in ([0-9.]+)',p.stderr)
                assert p.returncode==0 and match and 'Reference check: OK' in p.stderr,(cmd,p.stderr)
                row=dict(n=n,block=block,sequence=sequence,arm=arm,method=method,seed=seed,seconds=float(match[1]),process_wall_seconds=elapsed,returncode=p.returncode)
                rows.append(row);writer.writerow(row);f.flush()
                logs.write(json.dumps({**row,'cmd':cmd,'stdout':p.stdout,'stderr':p.stderr})+'\n');logs.flush()
        print('Completed ClickHouse upstream example n='+str(n),flush=True)

summary=[]
for n in [4096,65536,1000000,10000000]:
    samples={a:[r['seconds'] for r in rows if r['n']==n and r['arm']==a] for a,_ in arms}
    ratios=[];aa=[]
    for block in range(9):
        t={r['arm']:r['seconds'] for r in rows if r['n']==n and r['block']==block}
        ratios.append(t['tessera']/((t['lsd_A1']+t['lsd_A2'])/2))
        aa.append(t['lsd_A2']/t['lsd_A1'])
    summary.append({'n':n,**{a+'_median_seconds':statistics.median(v) for a,v in samples.items()},'median_paired_tessera_over_lsd':statistics.median(ratios),'ratio_min':min(ratios),'ratio_max':max(ratios),'median_AA':statistics.median(aa),'AA_min':min(aa),'AA_max':max(aa),'faster_blocks':sum(r<1 for r in ratios),'blocks':9})
with (ROOT/'clickhouse-summary.csv').open('w') as f:
    w=csv.DictWriter(f,list(summary[0]));w.writeheader();w.writerows(summary)
print(json.dumps(summary,indent=2))
