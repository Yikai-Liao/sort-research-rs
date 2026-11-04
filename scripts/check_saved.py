#!/usr/bin/env python3
"""Check saved graph payloads against JSON medians; executes no sorting code."""
import json
import math
import re

from refresh import ROOT, SOURCE, WARNING, median


def walk(node):
    if isinstance(node, dict):
        yield node
        for value in node.values():
            yield from walk(value)
    elif isinstance(node, list):
        for value in node:
            yield from walk(value)


def check():
    benchmarks = json.loads(SOURCE.read_text())["benchmarks"]
    coverage = {"single_size": set(), "scaling": set()}
    counts = {"single_size": 0, "scaling": 0, "direct_versus": 0}
    files = list((ROOT / "analysis_block_lsd").rglob("*.html"))
    for path in files:
        text = path.read_text()
        assert WARNING in text, path
        match = re.search(r'<script type="application/json"[^>]*>(.*?)</script>', text, re.S)
        payload = json.loads(match[1])
        sources = [dict(node["attributes"]["data"]["entries"]) for node in walk(payload)
                   if node.get("name") == "ColumnDataSource"]
        kind = path.parent.name
        if kind == "single_size":
            state, ty, n = path.stem.split("-")
            labels = [node.get("attributes", {}).get("axis_label", "") for node in walk(payload)]
            label = next(label for label in labels if label.startswith("Time ("))
            unit = re.search(r"Time \((\w+)\)", label)[1]
            scale = {"ns": 1, "us": 1000, "ms": 1e6, "s": 1e9}[unit]
            for source in sources:
                for (pattern, sort), time in zip(source["y"], source["bench_times"], strict=True):
                    name = f"{sort}-{state}-{ty}-{pattern}-{n}"
                    assert math.isclose(time * scale, median(benchmarks[name]), rel_tol=1e-12), name
                    coverage[kind].add(name)
                    counts[kind] += 1
        elif kind == "scaling":
            state, ty, _, pattern = path.stem.split("-")
            for source in sources:
                for n, throughput, sort in zip(source["x"], source["y"], source["name"], strict=True):
                    name = f"{sort}-{state}-{ty}-{pattern}-{n}"
                    assert math.isclose(throughput, n * 1000 / median(benchmarks[name]), rel_tol=1e-12), name
                    coverage[kind].add(name)
                    counts[kind] += 1
        elif kind == "direct_versus":
            sort_a, sort_b, state, ty = re.fullmatch(
                r"(.+)-vs-(.+)-(hot|cold)-([a-z0-9_]+)(?:-clipped)?", path.stem
            ).groups()
            for source in sources:
                if "name" not in source:  # zero reference line
                    continue
                for n, speedup, adjusted, pattern in zip(
                    source["x"], source["y"], source["y_adjusted"], source["name"], strict=True
                ):
                    a = median(benchmarks[f"{sort_a}-{state}-{ty}-{pattern}-{n}"])
                    b = median(benchmarks[f"{sort_b}-{state}-{ty}-{pattern}-{n}"])
                    expected = b / a - 1 if a <= b else -(a / b - 1)
                    assert math.isclose(speedup, expected, rel_tol=1e-12, abs_tol=1e-12), path
                    assert adjusted == speedup + (1 if speedup >= 0 else -1), path
                    counts[kind] += 1
        else:
            raise AssertionError(path)
    assert all(names == set(benchmarks) for names in coverage.values())
    print(f"Checked {len(files)} HTML files against saved JSON: {counts} data points.")


if __name__ == "__main__":
    check()
