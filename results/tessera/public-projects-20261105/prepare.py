#!/usr/bin/env python3
"""Freeze upstream kernels and local Tessera; build without database dependencies."""
import hashlib
import json
from pathlib import Path
import subprocess
import urllib.request

ROOT = Path(__file__).resolve().parent
REPO = ROOT.parents[2]
SOURCES = {
    "starrocks_radix_sort.h": "https://raw.githubusercontent.com/StarRocks/starrocks/890b8fffcd639c2cd70b1acbab799794a71d4c62/be/src/column/sorting/radix_sort.h",
    "clickhouse_radix_sort.h": "https://raw.githubusercontent.com/ClickHouse/ClickHouse/c8c7411269eb045ef70cf6e90dbc997d60dfa76f/src/Common/RadixSort.h",
    "sr_bench.cpp": "https://raw.githubusercontent.com/StarRocks/starrocks/890b8fffcd639c2cd70b1acbab799794a71d4c62/be/src/bench/radix_sort_bench.cpp",
    "sr_raw_container.h": "https://raw.githubusercontent.com/StarRocks/starrocks/890b8fffcd639c2cd70b1acbab799794a71d4c62/be/src/base/container/raw_container.h",
    "sr_pdqsort.h": "https://raw.githubusercontent.com/StarRocks/starrocks/890b8fffcd639c2cd70b1acbab799794a71d4c62/be/src/base/orlp/pdqsort.h",
    "sr_comparator.h": "https://raw.githubusercontent.com/StarRocks/starrocks/890b8fffcd639c2cd70b1acbab799794a71d4c62/be/src/column/column_sorter_comparator.h",
    "sr_simd.h": "https://raw.githubusercontent.com/StarRocks/starrocks/890b8fffcd639c2cd70b1acbab799794a71d4c62/be/src/base/simd/simd.h",
}

manifest_path=ROOT/'manifest.json'
manifest=json.loads(manifest_path.read_text()) if manifest_path.exists() else {"upstream": {}, "local": {}}
manifest['adaptation']="Upstream StarRocks benchmark functions, generators and registrations unchanged. Project-specific includes replaced by sr_shim.h; original comparator, SIMD, raw-vector resize and radix bodies retained. Added Tessera registrations and untimed validation."
for name, url in SOURCES.items():
    dest=ROOT/'vendor'/name
    data=dest.read_bytes() if dest.exists() else urllib.request.urlopen(url).read()
    if name in manifest['upstream']:
        assert hashlib.sha256(data).hexdigest()==manifest['upstream'][name]['sha256'],name
    dest.write_bytes(data)
    manifest["upstream"][name] = {"url": url, "sha256": hashlib.sha256(data).hexdigest()}
    lines = data.decode().splitlines(keepends=True)
    adapted = ''.join(l for l in lines if not (l.startswith('#include "') or l.startswith('#include <base/') or l.startswith('#include <Common/')))
    (ROOT / "vendor" / ("adapted_" + name)).write_text(adapted)

bench=(ROOT/'vendor'/'adapted_sr_bench.cpp').read_text()
marker='} // namespace\n} // namespace starrocks'
assert bench.count(marker)==1
bench='#include "sr_shim.h"\n#include <iostream>\n#include <stdexcept>\n'+bench.replace(marker,'#include "sr_additions.inc"\n\n'+marker)
(ROOT/'starrocks_bench.cpp').write_text(bench)

for src, dest in [
    ("src/other/radix_support.rs", "radix_support.rs"),
    ("src/other/rust_block_lsd/core.rs", "block_core.rs"),
    ("src/other/rust_tessera_radix/engine.rs", "tessera.rs"),
    ("src/cpp/thirdparty/pdqsort/pdqsort.h", "pdqsort.h"),
]:
    snapshot=ROOT/'vendor'/dest
    data=snapshot.read_bytes() if snapshot.exists() else (REPO/src).read_bytes()
    if src in manifest['local']:
        assert hashlib.sha256(data).hexdigest()==manifest['local'][src],src
    snapshot.write_bytes(data)
    manifest["local"][src] = hashlib.sha256(data).hexdigest()

manifest.setdefault("local_commit",subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO, text=True).strip())
manifest.setdefault("rustc",subprocess.check_output(["rustc", "-Vv"], text=True))
manifest.setdefault("clang",subprocess.check_output(["clang++", "--version"], text=True))
manifest["cpp_flags"] = ["-std=c++23", "-O3", "-march=native", "-DNDEBUG"]
manifest["rust_flags"] = "-C target-cpu=native; opt-level=3, thin LTO"
manifest.setdefault("lscpu",subprocess.check_output(["lscpu"], text=True))
(ROOT / "manifest.json").write_text(json.dumps(manifest, indent=2) + '\n')
print("Frozen kernels and Tessera at", ROOT)
