
import json, re, collections
d = json.load(open("/root/code/sort-research-rs-pr/results/block_lsd.json"))["benchmarks"]
rows = collections.defaultdict(dict)
for name, v in d.items():
    m = re.match(r"^(rust_[a-z_0-9]+?)-(hot|cold)-(u64|u128|f64|u64_128b)-random-(\d+)$", name)
    if not m: continue
    sort, temp, ty, n = m.groups()
    rows[(temp, ty, int(n))][sort] = v["criterion_estimates_v1"]["median"]["point_estimate"] / 1e6

lab = [("rust_std_stable","std stable"),("rust_ipnsort_unstable","ipnsort"),
       ("rust_radsort_radix","radsort"),("rust_radsort_keyed","radsort keyed"),
       ("rust_block_lsd_stable_radix","block LSD")]

for temp in ("cold","hot"):
    print(f"\n### {temp}\n")
    print("| type | n | " + " | ".join(l for _, l in lab) + " | block / radsort | block / std |")
    print("| --- | ---: | " + " | ".join("---:" for _ in lab) + " | ---: | ---: |")
    for (t, ty, n) in sorted(rows, key=lambda k: (k[1], k[2])):
        if t != temp: continue
        s = rows[(t, ty, n)]
        cells = []
        for key, _ in lab:
            cells.append(f"{s[key]:.2f}" if key in s else "–")
        rad = s.get("rust_radsort_radix") or s.get("rust_radsort_keyed")
        std = s.get("rust_std_stable")
        b = s["rust_block_lsd_stable_radix"]
        print(f"| {ty} | {n} | " + " | ".join(cells) + f" | {(b/rad if rad else float("nan")):.2f}× | {(b/std if std else float("nan")):.2f}× |")
