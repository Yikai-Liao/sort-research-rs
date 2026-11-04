
import subprocess, re
B="/root/code/sort-research-rs-c-compare"
cases=[("rust (with store)", f"{B}/target/release/rust-compare"),
       ("no-store variant ", f"{B}/target/release/abl-no-store")]
def inst(exe, n=1_000_000, reps=7, cpu="4"):
    best=None
    for _ in range(5):
        p=subprocess.run(["perf","stat","-e","instructions","taskset","-c",cpu,exe,"pair",str(n),str(reps)],
                         capture_output=True,text=True)
        m=re.search(r"^\s*([\d,]+)\s+instructions", p.stderr, re.M)
        if not m: raise SystemExit(p.stderr[-300:])
        v=int(m.group(1).replace(",",""))/reps/1e6
        best=v if best is None else min(best,v)
    return best
res={}
for name,exe in cases:
    res[name]=inst(exe)
    print(f"{name} {res[name]:7.2f}M instr/sort")
a=list(res.values())
print(f"delta: {(a[1]/a[0]-1)*100:+.2f}%")
