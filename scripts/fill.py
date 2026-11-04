
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

def table(temp):
    out = ["| type | n | " + " | ".join(l for _, l in lab) + " | block / radsort | block / std |",
           "| --- | ---: | " + " | ".join("---:" for _ in lab) + " | ---: | ---: |"]
    for (t, ty, n) in sorted(rows, key=lambda k: (k[1], k[2])):
        if t != temp: continue
        s = rows[(t, ty, n)]
        cells = [f"{s[key]:.2f}" if key in s else "–" for key, _ in lab]
        rad = s.get("rust_radsort_radix") or s.get("rust_radsort_keyed")
        std = s.get("rust_std_stable")
        b = s["rust_block_lsd_stable_radix"]
        out.append(f"| {ty} | {n} | " + " | ".join(cells) +
                   f" | {b/rad:.2f}x | {b/std:.2f}x |" if (rad and std) else
                   f"| {ty} | {n} | " + " | ".join(cells) +
                   f" | {(format(b/rad, '.2f') + 'x') if rad else '–'} | {(format(b/std, '.2f') + 'x') if std else '–'} |")
    return "\n".join(out)

pat_rows = collections.defaultdict(dict)
for name, v in d.items():
    m = re.match(r"^(rust_[a-z_0-9]+)-hot-u64-(ascending|descending|random_p5)-(\d+)$", name)
    if not m: continue
    sort, pat, n = m.groups()
    pat_rows[(pat, int(n))][sort] = v["criterion_estimates_v1"]["median"]["point_estimate"] / 1e6

def pattern_table():
    out = ["| pattern | n | std stable | ipnsort | radsort | block LSD | block / radsort | block / std |",
           "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for (pat, n) in sorted(pat_rows):
        s2 = pat_rows[(pat, n)]
        cells = [f"{s2[k]:.2f}" if k in s2 else "–" for k in
                 ("rust_std_stable", "rust_ipnsort_unstable", "rust_radsort_radix", "rust_block_lsd_stable_radix")]
        r, st, b = s2.get("rust_radsort_radix"), s2.get("rust_std_stable"), s2["rust_block_lsd_stable_radix"]
        out.append(f"| {pat} | {n} | " + " | ".join(cells) +
                   f" | {(format(b/r, '.2f') + 'x') if r else '–'} | {(format(b/st, '.2f') + 'x') if st else '–'} |")
    return "\n".join(out)

mem = """| 1M records | byte/record | std stable | ipnsort | radsort | block LSD |
| --- | ---: | ---: | ---: | ---: | ---: |
| u64 | 8 | 8.0 MB | 0 | 8.0 MB | 0.83 MB |
| f128 | 16 | 8.0 MB | 0 | – | 1.17 MB |
| record32 | 32 | 16.0 MB | 0 | 32.0 MB | 1.65 MB |
| record128 | 128 | 64.0 MB | 0 | 128.0 MB | 3.29 MB |
| owned (72 B) | 72 | 36.0 MB | 0 | 72.0 MB | 2.48 MB |
| 1000 x u64 | 8 | 8 KB | 0 | 8 KB | 0.54 MB |

Peak extra heap per sort, measured by benches/extra_memory.rs (requested bytes,
global allocator wrapper, retained heap is 0 for every row)."""

block = ("Times in ms, cold benchmarks (caches trashed before each call, which the\n"
         "harness describes as \"as part of a larger program\"):\n\n" + table("cold") +
         "\n\nHot benchmarks (the same buffers reused, caches warm):\n\n" + table("hot") +
         "\n\nOther patterns (hot, u64, ms):\n\n" + pattern_table() +
         "\n\nAuxiliary heap:\n\n" + mem)

for path, tail in (("/root/code/sort-research-rs-pr/issue-reply.md", "\n\n## Questions before a PR"),
                   ("/root/code/sort-research-rs-pr/pr-body.md", "\n\nAttached: results JSON")):
    s = open(path).read()
    start = s.index("Times in ms, cold benchmarks")
    end = s.index(tail)
    s = s[:start] + block + s[end:]
    open(path, "w").write(s)
    print(path, "updated")
