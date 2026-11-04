
import subprocess, statistics
B="/root/code/sort-research-rs-c-compare"
TYPES=["pair","u64","u128","rec32","rec128"]
SIZES=[100_000, 1_000_000]
def run(exe, t, n, reps=7):
    out = subprocess.run(["taskset","-c","2",exe,t,str(n),str(reps)],capture_output=True,text=True)
    if out.returncode: raise SystemExit(out.stderr[-400:])
    p = out.stdout.strip().split(",")
    return float(p[3])
print(f"{'type':8} {'n':>9} {'C++ ns/elem':>12} {'Rust ns/elem':>13} {'rust/c++':>9}")
for t in TYPES:
    for n in SIZES:
        c = min(run(f"{B}/cxx/driver_gcc", t, n) for _ in range(3))
        r = min(run(f"{B}/target/release/rust-compare", t, n) for _ in range(3))
        print(f"{t:8} {n:>9} {c:12.2f} {r:13.2f} {r/c:9.3f}")
