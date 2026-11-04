#!/usr/bin/env python3
"""Refresh saved research artifacts without building or running any sort.

Usage: python scripts/refresh.py [--plots]
Only --plots needs requirements-plots.txt. All paths are relative to this file.
"""
import argparse
import csv
import hashlib
import html
import io
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "results/block_lsd.json"
CORRECTED = "22463e9ef3e59a917afe8fcd1ed1a52ca123d9b7"
WARNING = (
    "Historical baseline; corrected candidate 22463e9 unmeasured. "
    "Pattern rows: saved baseline only, original measurement log unavailable."
)
LABELS = [
    ("rust_std_stable", "std stable"),
    ("rust_ipnsort_unstable", "ipnsort"),
    ("rust_radsort_radix", "radsort"),
    ("rust_radsort_keyed", "radsort keyed"),
    ("rust_block_lsd_stable_radix", "block LSD"),
]
NAME = re.compile(r"^(rust_[a-z_0-9]+)-(hot|cold)-([a-z0-9_]+)-([a-z_0-9]+)-(\d+)$")


def median(entry):
    return entry["criterion_estimates_v1"]["median"]["point_estimate"]


def random_table(benchmarks, temp):
    rows = {}
    for name, value in benchmarks.items():
        sort, state, ty, pattern, n = NAME.fullmatch(name).groups()
        if state == temp and pattern == "random":
            rows.setdefault((ty, int(n)), {})[sort] = median(value) / 1e6
    lines = [
        "| type | n | " + " | ".join(label for _, label in LABELS) + " | block / radsort | block / std |",
        "| --- | ---: | " + " | ".join("---:" for _ in LABELS) + " | ---: | ---: |",
    ]
    for (ty, n), values in sorted(rows.items()):
        block = values["rust_block_lsd_stable_radix"]
        rad = values.get("rust_radsort_radix", values.get("rust_radsort_keyed"))
        std = values.get("rust_std_stable")
        cells = [f"{values[key]:.2f}" if key in values else "–" for key, _ in LABELS]
        cells += [f"{block / ref:.2f}x" if ref else "–" for ref in (rad, std)]
        lines.append(f"| {ty} | {n} | " + " | ".join(cells) + " |")
    return "\n".join(lines)


def memory_table():
    text = (ROOT / "logs/extra-memory.log").read_text()
    start = text.index("algorithm,type,pattern,")
    rows = list(csv.DictReader(io.StringIO(text[start:])))
    rows = [row for row in rows if row.get("algorithm", "").startswith("rust_")]
    assert rows and all(int(row["retained_extra_heap_bytes"]) == 0 for row in rows)
    lines = [
        "| type | n | bytes / record | std stable | ipnsort | radsort | block LSD |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for ty, n in [("u64", 1000000), ("f128", 1000000), ("record32", 1000000),
                  ("record128", 1000000), ("owned", 1000000), ("u64", 1000)]:
        selected = {row["algorithm"]: row for row in rows if row["type"] == ty and int(row["n"]) == n}
        cells = []
        for alternatives in [("rust_std_stable",), ("rust_ipnsort_unstable",),
                             ("rust_radsort_radix", "rust_radsort_by_key"),
                             ("rust_block_lsd_stable_radix",)]:
            row = next((selected[key] for key in alternatives if key in selected), None)
            cells.append(str(row["peak_extra_heap_bytes"]) if row else "–")
        size = next(iter(selected.values()))["record_bytes"]
        lines.append(f"| {ty} | {n} | {size} | " + " | ".join(cells) + " |")
    return "\n".join(lines)


def refresh(plots=False):
    benchmarks = json.loads(SOURCE.read_text())["benchmarks"]
    assert len(benchmarks) == 106
    old_numbers = (ROOT / "NUMBERS.md").read_text()
    mapping = {}
    saved_provenance = ROOT / "results/provenance.json"
    if saved_provenance.exists():
        mapping.update({name: entry["log"] for name, entry in
                        json.loads(saved_provenance.read_text())["benchmarks"].items()})
    for line in old_numbers.splitlines():
        if line.startswith("| "):
            cells = [part.strip() for part in line.split("|")[1:-1]]
            if len(cells) == 7 and cells[5] in benchmarks:
                mapping[cells[5]] = cells[6]
    # Correct the two source paths; the other two cold 10M entries are in bench4.
    for sort in ("rust_block_lsd_stable_radix", "rust_radsort_radix"):
        mapping[f"{sort}-cold-u64-random-10000000"] = "logs/bench3.log"
    records = {}
    numbers = [
        "# Historical benchmark provenance", "",
        f"Corrected code candidate: `{CORRECTED}`. It has not been compiled, tested,",
        "or benchmarked during this closeout. None of the timings below measure it.", "",
        "The 106 entries are preserved from the pre-closeout `block_lsd` export.",
        "Individual measurement commit IDs were not recorded. The previous code head",
        "was 429f6f7; that does not establish the revision of each measurement.", "",
        "All times below and in the graphs are JSON median point estimates. Ratios use",
        "those medians. The text summary also reports the mean separately.", "",
        "For the 82 random entries, the JSON mean/slope and confidence interval match",
        "the rounded Criterion output in the listed logs. These aggregate logs do not",
        "contain the raw samples needed to recompute medians; the median source is JSON.", "",
        "The 24 ascending/descending/random_p5 entries are saved baseline estimates only.",
        "Their original measurement log has not been recovered. `logs/pattern-bench.log`",
        "contains a later run against a saved baseline, not the measurements exported here.",
        "These rows are retained for archival completeness and excluded from reply/PR tables.", "",
        "Both hot and cold generate inputs via batch setup. Cold additionally disturbs CPU",
        "prediction state; the harness deliberately avoids testing main-memory loading.",
        "`logs/contaminated/` is archival and is never used as an input to regeneration.", "",
        "| state | type | pattern | n | sort | median ms | benchmark id | provenance |",
        "| --- | --- | --- | ---: | --- | ---: | --- | --- |",
    ]
    summary = [WARNING, "Source: results/block_lsd.json; times in ms.",
               "benchmark\tmedian_ms\tmean_ms\tmean_std_dev_ms\tprovenance"]
    for name, value in sorted(benchmarks.items()):
        sort, state, ty, pattern, n = NAME.fullmatch(name).groups()
        estimates = value["criterion_estimates_v1"]
        source_log = mapping.get(name) if pattern == "random" else None
        if source_log:
            log = (ROOT / source_log).read_text()
            match = re.search(re.escape(name) + r"\s+time:\s+\[([0-9.]+) (\S+) ([0-9.]+) (\S+) ([0-9.]+) (\S+)\]", log)
            assert match, (name, source_log)
            units = {"ns": 1, "µs": 1000, "us": 1000, "ms": 1e6, "s": 1e9}
            logged = [float(match[i]) * units[match[i + 1]] for i in (1, 3, 5)]
            estimate = estimates.get("slope") or estimates["mean"]
            expected = [estimate["confidence_interval"]["lower_bound"], estimate["point_estimate"],
                        estimate["confidence_interval"]["upper_bound"]]
            assert all(abs(a - b) <= max(a, b) * 0.0001 for a, b in zip(logged, expected)), name
        else:
            assert pattern != "random", f"missing random provenance: {name}"
        provenance = source_log or "saved baseline only; original log unavailable"
        records[name] = {"median_ns": median(value), "log": source_log,
                         "status": "matching_aggregate_log" if source_log else "saved_baseline_only"}
        numbers.append(f"| {state} | {ty} | {pattern} | {n} | {sort} | {median(value) / 1e6:.2f} | {name} | {provenance} |")
        summary.append(f"{name}\t{median(value) / 1e6:.6f}\t{estimates['mean']['point_estimate'] / 1e6:.6f}\t"
                       f"{estimates['std_dev']['point_estimate'] / 1e6:.6f}\t{provenance}")
    assert sum(r["log"] is not None for r in records.values()) == 82
    numbers += ["", "## Historical peak extra heap", "",
                "Source: `logs/extra-memory.log`, requested bytes, excluding the input allocation.",
                "Every retained-extra-heap value is zero. These measurements also predate closeout.", "",
                memory_table(), "",
                "The reference's fixed blocks bound scratch only; block metadata grows with n.",
                "The adaptive policy makes both terms O(sqrt(n)) for fixed radix and record size.", "",
                "The benchmark converts ascending i32 values to u64 as `(i + 2^31) * (2^31 - 1)`.",
                "At 100k and 1M all eight byte digits vary: no 8-to-3-pass explanation applies.",
                "The converted u64 inputs have constant high bits, so the 1025-key early-stop",
                "condition is not satisfied and the pre-scan reads the entire input."]
    (ROOT / "NUMBERS.md").write_text("\n".join(numbers) + "\n")
    (ROOT / "results/block_lsd_all.txt").write_text("\n".join(summary) + "\n")
    tables = "\n\n".join(["## Historical random timings", WARNING,
                            "Median milliseconds; ratios below 1 favor block LSD.",
                            "### Cold (prediction state disturbed)", random_table(benchmarks, "cold"),
                            "### Hot (batch-generated inputs)", random_table(benchmarks, "hot"),
                            "## Historical peak extra heap (bytes)", memory_table()]) + "\n"
    (ROOT / "results/TABLES.md").write_text(tables)
    manifest = {"source_sha256": hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
                "corrected_candidate": CORRECTED, "measurement_commit": None,
                "candidate_validation": "not run; static closeout only", "benchmarks": records}
    (ROOT / "results/provenance.json").write_text(json.dumps(manifest, indent=2) + "\n")
    if plots:
        subprocess.run([sys.executable, str(ROOT / "scripts/graph_bench_result/graph_all.py"), str(SOURCE)],
                       cwd=ROOT, check=True)
        for path in (ROOT / "analysis_block_lsd").rglob("*.html"):
            banner = ('<p style="font: 14px sans-serif; margin: 12px; max-width: 900px">'
                      + html.escape(WARNING)
                      + ' <a href="../../NUMBERS.md">Provenance</a></p>')
            content = path.read_text().replace("<body>", "<body>\n" + banner, 1)
            path.write_text("\n".join(line.rstrip() for line in content.splitlines()) + "\n")
        subprocess.run([sys.executable, str(ROOT / "scripts/plot.py"), str(SOURCE), str(ROOT / "figures")],
                       cwd=ROOT, check=True)
    print("Refreshed 106 historical entries: 82 matching aggregate logs, 24 saved-baseline-only.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--plots", action="store_true")
    refresh(parser.parse_args().plots)
