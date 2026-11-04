#!/usr/bin/env python3
"""PNG figures for the PR/issue from the critcmp export.

The repository's util/graph_bench_result produces interactive HTML; issue and
PR comments need static images, so this reads the same JSON and draws the three
views that matter for the discussion: scaling on u64, wide records, and the
relative speedup against the two reference sorts.
"""
import json
import re
import sys
import collections
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

PATH = sys.argv[1] if len(sys.argv) > 1 else "results/block_lsd.json"
OUT = sys.argv[2] if len(sys.argv) > 2 else "figures"

SORT_LABEL = {
    "rust_std_stable": "std stable",
    "rust_ipnsort_unstable": "ipnsort",
    "rust_radsort_radix": "radsort",
    "rust_radsort_keyed": "radsort (keyed)",
    "rust_block_lsd_stable_radix": "block LSD",
}
COLOR = {
    "rust_std_stable": "#4d4d4d",
    "rust_ipnsort_unstable": "#999999",
    "rust_radsort_radix": "#e69f00",
    "rust_radsort_keyed": "#e69f00",
    "rust_block_lsd_stable_radix": "#0072b2",
}

data = json.load(open(PATH))["benchmarks"]
rows = collections.defaultdict(dict)
for name, value in data.items():
    m = re.match(r"^(rust_[a-z_0-9]+?)-(hot|cold)-(u64|u128|f64|u64_128b)-random-(\d+)$", name)
    if not m:
        continue
    sort, temp, ty, n = m.groups()
    rows[(temp, ty)][(sort, int(n))] = value["criterion_estimates_v1"]["median"]["point_estimate"]

def series(temp, ty, sort):
    pts = [(n, t / 1e6) for (s, n), t in rows[(temp, ty)].items() if s == sort]
    return sorted(pts)

def panel(ax, temp, ty, sorts, title, ylabel="time in ms"):
    for sort in sorts:
        pts = series(temp, ty, sort)
        if not pts:
            continue
        xs, ys = zip(*pts)
        ax.plot(xs, ys, marker="o", ms=3, lw=1.2, color=COLOR[sort], label=SORT_LABEL[sort])
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("n")
    ax.set_ylabel(ylabel)
    ax.set_title(title, fontsize=9)
    ax.grid(alpha=0.25, which="both", lw=0.4)
    ax.legend(fontsize=7, frameon=False)

def speedup(ax, temp, ty, base, title):
    pts_b = dict(series(temp, ty, base))
    pts_c = dict(series(temp, ty, "rust_block_lsd_stable_radix"))
    ns = sorted(set(pts_b) & set(pts_c))
    ratios = [pts_b[n] / pts_c[n] for n in ns]
    ax.axhline(1.0, color="#888888", lw=0.8, ls="--")
    ax.plot(ns, ratios, marker="o", ms=3, lw=1.2, color="#0072b2")
    ax.set_xscale("log")
    ax.set_xlabel("n")
    ax.set_ylabel(f"{SORT_LABEL[base]} / block LSD")
    ax.set_title(title, fontsize=9)
    ax.grid(alpha=0.25, which="both", lw=0.4)
    ax.annotate("block LSD faster", xy=(0.02, 0.94), xycoords="axes fraction", fontsize=6, color="#0072b2")
    ax.annotate("block LSD slower", xy=(0.02, 0.06), xycoords="axes fraction", fontsize=6, color="#b00020")

for temp in ("cold", "hot"):
    fig, axes = plt.subplots(1, 3, figsize=(11, 3.1))
    panel(axes[0], temp, "u64", ["rust_std_stable", "rust_ipnsort_unstable", "rust_radsort_radix",
                                 "rust_block_lsd_stable_radix"], f"u64 random ({temp})")
    panel(axes[1], temp, "u64_128b", ["rust_std_stable", "rust_radsort_keyed",
                                      "rust_block_lsd_stable_radix"], f"128 byte records ({temp})")
    speedup(axes[2], temp, "u64", "rust_radsort_radix", f"block LSD vs radsort, u64 ({temp})")
    fig.suptitle("Historical baseline; corrected candidate 22463e9 unmeasured", fontsize=9)
    fig.tight_layout(rect=(0, 0, 1, 0.92))
    fig.savefig(f"{OUT}/scaling-{temp}.png", dpi=180)
    plt.close(fig)

# Relative speedup against both reference sorts over every type we measured.
fig, axes = plt.subplots(1, 2, figsize=(9, 3.2))
for ax, base in zip(axes, ["rust_radsort_radix", "rust_std_stable"]):
    for ty, marker in [("u64", "o"), ("u128", "s"), ("u64_128b", "^")]:
        pts_b = dict(series("cold", ty, base)) or dict(series("cold", ty, "rust_radsort_keyed"))
        pts_c = dict(series("cold", ty, "rust_block_lsd_stable_radix"))
        ns = sorted(set(pts_b) & set(pts_c))
        if not ns:
            continue
        ax.plot(ns, [pts_b[n] / pts_c[n] for n in ns], marker=marker, ms=3, lw=1.1, label=ty)
    ax.axhline(1.0, color="#888888", lw=0.8, ls="--")
    ax.set_xscale("log")
    ax.set_xlabel("n")
    ax.set_ylabel(f"{SORT_LABEL[base]} / block LSD")
    ax.set_title(f"block LSD relative to {SORT_LABEL[base]} (cold)", fontsize=9)
    ax.grid(alpha=0.25, which="both", lw=0.4)
    ax.legend(fontsize=7, frameon=False)
fig.suptitle("Historical baseline; corrected candidate 22463e9 unmeasured", fontsize=9)
fig.tight_layout(rect=(0, 0, 1, 0.92))
fig.savefig(f"{OUT}/relative-{ty}.png".replace(f"-{ty}", ""), dpi=180)
print("wrote figures to", OUT)
