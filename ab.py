#!/usr/bin/env python3
"""Interleaved A/B comparison of the templated C++ reference port and the Rust
backend, on the record shapes both harnesses generate.

Alternates the two binaries within every round so VM noise hits both equally,
then reports the median of the per-round ratios.
"""
import subprocess, statistics, sys, json

B = "/root/code/sort-research-rs-c-compare"
CPP = f"{B}/cxx/driver_gcc"
RS = __import__("os").environ.get("RUST_BIN", f"{B}/target/release/rust-compare")
CPU = "2"

def one(exe, ty, n):
    out = subprocess.run(["taskset", "-c", CPU, exe, ty, str(n), "1"],
                         capture_output=True, text=True, check=True).stdout.strip()
    parts = out.split(",")
    return int(parts[2]) / 1e6  # milliseconds

def compare(ty, n, rounds):
    cpp, rs = [], []
    for _ in range(rounds):
        rs.append(one(RS, ty, n))
        cpp.append(one(CPP, ty, n))
    ratios = sorted(r / c for r, c in zip(rs, cpp))
    return statistics.median(cpp), statistics.median(rs), statistics.median(ratios), ratios

def main():
    rounds = int(sys.argv[1]) if len(sys.argv) > 1 else 9
    import os
    types = os.environ.get("AB_TYPES", "pair,u64,u128,rec32,rec128").split(",")
    sizes = [int(v) for v in os.environ.get("AB_SIZES", "100000,1000000").split(",")]
    print(f"rounds={rounds}, altCPP/Rust per round, median reported")
    print(f"{'type':7} {'n':>9} {'C++ ms':>9} {'Rust ms':>9} {'ratio':>7} {'p10':>7} {'p90':>7}")
    for ty in types:
        for n in sizes:
            c, r, med, ratios = compare(ty, n, rounds)
            p10 = ratios[int(0.1 * (len(ratios) - 1))]
            p90 = ratios[int(0.9 * (len(ratios) - 1))]
            print(f"{ty:7} {n:>9} {c:9.2f} {r:9.2f} {med:7.3f} {p10:7.3f} {p90:7.3f}")

main()
