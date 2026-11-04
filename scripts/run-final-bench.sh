#!/bin/bash
set -u
export PATH=$HOME/.cargo/bin:$PATH
export CARGO_TARGET_DIR=/root/code/sort-research-rs-target
export CPU_MAX_FREQ_GHZ=2.3
cd /root/code/sort-research-rs
rm -rf "$CARGO_TARGET_DIR/criterion"

# Repo convention: util/run_benchmarks.py uses warm-up 2 s / measurement 4 s,
# --noplot and a named baseline, then exports that baseline with critcmp.
export BENCH_REGEX='^(rust_std_stable|rust_ipnsort_unstable|rust_radsort_radix|rust_block_lsd_stable_radix)-hot-u64-random-(1000|10000|100000|1000000|10000000)$|^(rust_std_stable|rust_radsort_keyed|rust_block_lsd_stable_radix)-hot-u64_128b-random-(10000|100000|1000000)$'
FEATURES=rust_block_lsd,rust_radsort,cold_benchmarks,bench_type_u128,bench_type_f64,bench_type_u64_16b,bench_type_u64_128b

cargo bench --features "$FEATURES" --bench bench -- \
  --warm-up-time 2 --measurement-time 4 --noplot --save-baseline block_lsd
echo BENCHDONE
mkdir -p /root/code/sort-research-rs-pr/results
critcmp --export block_lsd > /root/code/sort-research-rs-pr/results/block_lsd.json
echo EXPORTDONE
