
import subprocess
B="/root/code/sort-research-rs-c-compare"
cases = [
 ("gcc",   f"{B}/c-compare/driver_dyn", "policy", True),
 ("clang", f"{B}/c-compare/driver_dyn_clang", "policy", True),
 ("rust", f"{B}/target/release/rust-compare", "auto", False),
 ("rust-plain", f"{B}/target/release/rust-compare-plain", "auto", False),
 ("abl-baseline", f"{B}/target/release/abl-baseline", "auto", False),
 ("abl-no_offset_store", f"{B}/target/release/abl-no_offset_store", "auto", False),
 ("abl-inline_scatter", f"{B}/target/release/abl-inline_scatter", "auto", False),
]
import re
def meas(exe, arg):
    best = None
    for _ in range(3):
        p = subprocess.run(["perf","stat","-e","instructions","taskset","-c","2",exe,"1000000","7",arg],
                           capture_output=True,text=True)
        m = re.search(r"^\s*([\d,]+)\s+instructions", p.stderr, re.M)
        v = int(m.group(1).replace(",",""))
        best = v if best is None else min(best, v)
    return best/7/1e6
base = None
for name, exe, arg, _ in cases:
    v = meas(exe, arg)
    if name == "gcc": base = v
    print(f"{name:22s} {v:7.1f}M  x{v/base:5.3f}")
