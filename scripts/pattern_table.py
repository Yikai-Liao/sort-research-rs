"""Historical saved-baseline pattern estimates; original log unavailable."""

import json, re, collections
from pathlib import Path
d = json.load(open(Path(__file__).resolve().parents[1] / "results/block_lsd.json"))["benchmarks"]
rows = collections.defaultdict(dict)
for name, v in d.items():
    m = re.match(r"^(rust_[a-z_0-9]+)-(hot|cold)-(u64)-(ascending|descending|all_equal|random_p5)-(\d+)$", name)
    if not m: continue
    sort, temp, ty, pat, n = m.groups()
    rows[(pat, int(n))][sort] = v["criterion_estimates_v1"]["median"]["point_estimate"]/1e6
lab = [("rust_std_stable","std"),("rust_ipnsort_unstable","ipn"),("rust_radsort_radix","rad"),("rust_block_lsd_stable_radix","block")]
print("Saved baseline only; original measurement log unavailable. Corrected candidate unmeasured.")
print(f"{'pattern':11} {'n':>8} " + " ".join(f"{l:>8}" for _,l in lab) + "   block/rad  block/std")
for (pat,n), s in sorted(rows.items()):
    cells = [f"{s[k]:8.2f}" if k in s else f"{'-':>8}" for k,_ in lab]
    r, st, b = s.get("rust_radsort_radix"), s.get("rust_std_stable"), s["rust_block_lsd_stable_radix"]
    print(f"{pat:11} {n:>8} " + " ".join(cells) + f"   {b/r:9.2f} {b/st:10.2f}")
