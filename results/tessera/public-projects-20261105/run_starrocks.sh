#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
taskset -c 2 ./starrocks_bench --benchmark_min_time=0.015s --benchmark_repetitions=5 --benchmark_enable_random_interleaving=true --benchmark_out=starrocks-full.json --benchmark_out_format=json >starrocks-full.stdout 2>starrocks-full.stderr
python3 summarize.py >starrocks-summary.stdout
taskset -c 2 ./starrocks_bench --benchmark_filter='^BM_(RadixSort|RadixSortAA|RadixSortColdScratch|Tessera)_(Int32|Int64|Float|Double)_(UniformRandom|LowCardinality)/(4096|65536|1000000)$|^BM_(RadixSortStable|TesseraStable|PdqSortWithStabilization)_(Int32|Int64|Float|Double)_LowCard/(4096|65536|1000000)$' --benchmark_min_time=0.06s --benchmark_repetitions=9 --benchmark_enable_random_interleaving=true --benchmark_out=starrocks-confirm.json --benchmark_out_format=json >starrocks-confirm.stdout 2>starrocks-confirm.stderr
python3 summarize.py starrocks-confirm >starrocks-confirm-summary.stdout
