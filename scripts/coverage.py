
import json, re, collections
d = json.load(open("/root/code/sort-research-rs-pr/results/block_lsd.json"))["benchmarks"]
sorts, types, temps, pats, sizes = set(), set(), set(), set(), set()
for name in d:
    m = re.match(r"^(rust_[a-z_0-9]+)-(hot|cold)-([a-z0-9_]+)-([a-z_0-9]+)-(\d+)$", name)
    if not m: print("unparsed", name); continue
    s,t,ty,p,n = m.groups()
    sorts.add(s); temps.add(t); types.add(ty); pats.add(p); sizes.add(int(n))
print("benchmarks:", len(d))
print("sorts :", sorted(sorts))
print("temps :", sorted(temps))
print("types :", sorted(types))
print("pats  :", sorted(pats))
print("sizes :", sorted(sizes))
cov = collections.defaultdict(set)
for name in d:
    m = re.match(r"^(rust_[a-z_0-9]+)-(hot|cold)-([a-z0-9_]+)-([a-z_0-9]+)-(\d+)$", name)
    s,t,ty,p,n = m.groups()
    cov[(ty,t)].add(int(n))
for k in sorted(cov): print(k, sorted(cov[k]))
