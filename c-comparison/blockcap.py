
import subprocess, statistics, os
B = "/root/code/sort-research-rs-c-compare"
CUR = f"{B}/target/release/rust-compare"
CAP = f"{B}/target/release/abl-blockcap"
def one(exe, ty, n):
    out = subprocess.run(["taskset","-c","2",exe,ty,str(n),"1"],capture_output=True,text=True,check=True).stdout.strip()
    return int(out.split(",")[2])/1e6
print(f"{'type':7} {'n':>7} {'current ms':>11} {'capped ms':>10} {'cap/cur':>8}  (alt, 9 rounds, median)")
for ty in ("pair","u64","rec128"):
    for n in (1000, 3000, 10000, 30000, 100000):
        a=[];b=[]
        for _ in range(9):
            a.append(one(CUR,ty,n)); b.append(one(CAP,ty,n))
        r=sorted(y/x for x,y in zip(a,b))
        print(f"{ty:7} {n:>7} {statistics.median(a):11.3f} {statistics.median(b):10.3f} {statistics.median(r):8.3f}")
