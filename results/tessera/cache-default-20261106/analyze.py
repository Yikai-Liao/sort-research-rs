"""Summarize paired rounds from radix_ab RADIX_RAW files.

Per round: AA = A2/A1; X/A = X / mean(A1, A2) for the copy and cached arms;
cached/copy compares the two separately placed engine copies. Each statistic
is the median over rounds within a cycle, then the median over cycles.
"""
import csv, statistics, sys
from collections import defaultdict
from pathlib import Path

raw = Path(sys.argv[1])
rows = defaultdict(dict)  # (cycle, label, pattern, n, round) -> {variant: secs}
for f in sorted(raw.glob("cycle*.csv")):
    cycle = f.stem
    for sha, label, pattern, n, variant, rnd, secs in csv.reader(f.open()):
        rows[(cycle, label, pattern, int(n), int(rnd))][variant] = float(secs)

per_cycle = defaultdict(lambda: defaultdict(list))  # (case, cycle) -> metric -> [per-round]
for (cycle, label, pattern, n, rnd), v in rows.items():
    a1, a2, cp, ca = v["tessera"], v["tessera_a2"], v["tessera_copy"], v["tessera_cached"]
    a = (a1 + a2) / 2
    m = per_cycle[((label, pattern, n), cycle)]
    m["aa"].append(a2 / a1); m["copy"].append(cp / a); m["cached"].append(ca / a)
    m["cached_copy"].append(ca / cp); m["ns_a"].append(a * 1e9 / n)

cases = defaultdict(lambda: defaultdict(list))  # case -> metric -> [per-cycle medians]
for (case, cycle), m in per_cycle.items():
    for k, xs in m.items():
        cases[case][k].append(statistics.median(xs))

order = {"u32": 0, "u64": 1, "u128": 2, "record16": 3, "record128": 4}
print("| type | pattern | n | ns/elem (A) | A2/A1 | copy/A | cached/A | cached/copy | cycles cached>A |")
print("|---|---|---:|---:|---:|---:|---:|---:|---:|")
summary = []
for case in sorted(cases, key=lambda c: (order[c[0]], c[1], c[2])):
    m = cases[case]
    med = {k: statistics.median(v) for k, v in m.items()}
    slower = sum(x > 1 for x in m["cached"])
    summary.append((case, med, m))
    print(f"| {case[0]} | {case[1]} | {case[2]} | {med['ns_a']:.2f} | {med['aa']:.3f} | {med['copy']:.3f} | "
          f"{med['cached']:.3f} | {med['cached_copy']:.3f} | {slower}/{len(m['cached'])} |")

# A cached result counts as a real difference only if it lies outside the
# spread of both placement controls (A2/A1 and copy/A) for that case, and at
# least 4 of 5 cycles agree on its direction.
print()
for case, med, m in summary:
    lo = min(min(m["aa"]), min(m["copy"])); hi = max(max(m["aa"]), max(m["copy"]))
    c = med["cached"]
    agree = max(sum(x > 1 for x in m["cached"]), sum(x < 1 for x in m["cached"]))
    if (c > hi or c < lo) and agree >= 4:
        print(f"outside controls: {case} cached/A={c:.3f} controls=[{lo:.3f}, {hi:.3f}] agree={agree}/5")
