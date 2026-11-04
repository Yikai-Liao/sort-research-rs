# Where every number in the drafts comes from

Each table row in drafts/pr-body.md and drafts/issue-reply.md is a criterion
median for the benchmark named below, taken from results/block_lsd.json and
reproducible from the raw criterion output in logs/. Ratios are computed from
those medians: block / radsort and block / std, so a value below 1.0 means
block LSD is faster.

Cold and hot differ only in the harness setup: cold additionally trashes CPU
prediction state before each call (benches/modules/util.rs); it does not evict
caches. logs/contaminated/ holds one discarded batch that was measured while
other work competed for the cores, kept only to explain older numbers.

## cold rows

| type | pattern | n | sort | median ms | benchmark id | raw log |
| --- | --- | ---: | --- | ---: | --- | --- |
| f64 | random | 100000 | rust_block_lsd_stable_radix | 2.73 | rust_block_lsd_stable_radix-cold-f64-random-100000 | logs/bench3.log |
| f64 | random | 100000 | rust_radsort_keyed | 3.23 | rust_radsort_keyed-cold-f64-random-100000 | logs/bench3.log |
| f64 | random | 100000 | rust_std_stable | 11.25 | rust_std_stable-cold-f64-random-100000 | logs/bench3.log |
| f64 | random | 1000000 | rust_block_lsd_stable_radix | 54.20 | rust_block_lsd_stable_radix-cold-f64-random-1000000 | logs/bench3.log |
| f64 | random | 1000000 | rust_radsort_keyed | 74.10 | rust_radsort_keyed-cold-f64-random-1000000 | logs/bench3.log |
| f64 | random | 1000000 | rust_std_stable | 142.39 | rust_std_stable-cold-f64-random-1000000 | logs/bench3.log |
| u128 | random | 100000 | rust_block_lsd_stable_radix | 4.58 | rust_block_lsd_stable_radix-cold-u128-random-100000 | logs/bench3.log |
| u128 | random | 100000 | rust_radsort_radix | 7.48 | rust_radsort_radix-cold-u128-random-100000 | logs/bench3.log |
| u128 | random | 100000 | rust_std_stable | 6.74 | rust_std_stable-cold-u128-random-100000 | logs/bench3.log |
| u128 | random | 1000000 | rust_block_lsd_stable_radix | 86.03 | rust_block_lsd_stable_radix-cold-u128-random-1000000 | logs/bench3.log |
| u128 | random | 1000000 | rust_radsort_radix | 111.22 | rust_radsort_radix-cold-u128-random-1000000 | logs/bench3.log |
| u128 | random | 1000000 | rust_std_stable | 78.66 | rust_std_stable-cold-u128-random-1000000 | logs/bench3.log |
| u64 | random | 100000 | rust_block_lsd_stable_radix | 2.49 | rust_block_lsd_stable_radix-cold-u64-random-100000 | logs/bench3.log |
| u64 | random | 100000 | rust_ipnsort_unstable | 2.15 | rust_ipnsort_unstable-cold-u64-random-100000 | logs/bench3.log |
| u64 | random | 100000 | rust_radsort_radix | 2.62 | rust_radsort_radix-cold-u64-random-100000 | logs/bench3.log |
| u64 | random | 100000 | rust_std_stable | 3.36 | rust_std_stable-cold-u64-random-100000 | logs/bench3.log |
| u64 | random | 1000000 | rust_block_lsd_stable_radix | 42.79 | rust_block_lsd_stable_radix-cold-u64-random-1000000 | logs/bench3.log |
| u64 | random | 1000000 | rust_ipnsort_unstable | 27.91 | rust_ipnsort_unstable-cold-u64-random-1000000 | logs/bench3.log |
| u64 | random | 1000000 | rust_radsort_radix | 69.63 | rust_radsort_radix-cold-u64-random-1000000 | logs/bench3.log |
| u64 | random | 1000000 | rust_std_stable | 43.09 | rust_std_stable-cold-u64-random-1000000 | logs/bench3.log |
| u64 | random | 10000000 | rust_block_lsd_stable_radix | 583.46 | rust_block_lsd_stable_radix-cold-u64-random-10000000 | logs/bench4.log |
| u64 | random | 10000000 | rust_ipnsort_unstable | 364.53 | rust_ipnsort_unstable-cold-u64-random-10000000 | logs/bench4.log |
| u64 | random | 10000000 | rust_radsort_radix | 786.48 | rust_radsort_radix-cold-u64-random-10000000 | logs/bench4.log |
| u64 | random | 10000000 | rust_std_stable | 630.22 | rust_std_stable-cold-u64-random-10000000 | logs/bench4.log |
| u64_128b | random | 10000 | rust_block_lsd_stable_radix | 1.42 | rust_block_lsd_stable_radix-cold-u64_128b-random-10000 | logs/bench3.log |
| u64_128b | random | 10000 | rust_radsort_keyed | 2.66 | rust_radsort_keyed-cold-u64_128b-random-10000 | logs/bench3.log |
| u64_128b | random | 10000 | rust_std_stable | 2.52 | rust_std_stable-cold-u64_128b-random-10000 | logs/bench3.log |
| u64_128b | random | 100000 | rust_block_lsd_stable_radix | 33.95 | rust_block_lsd_stable_radix-cold-u64_128b-random-100000 | logs/bench3.log |
| u64_128b | random | 100000 | rust_radsort_keyed | 75.58 | rust_radsort_keyed-cold-u64_128b-random-100000 | logs/bench3.log |
| u64_128b | random | 100000 | rust_std_stable | 34.75 | rust_std_stable-cold-u64_128b-random-100000 | logs/bench3.log |
| u64_128b | random | 1000000 | rust_block_lsd_stable_radix | 544.94 | rust_block_lsd_stable_radix-cold-u64_128b-random-1000000 | logs/bench3.log |
| u64_128b | random | 1000000 | rust_radsort_keyed | 812.15 | rust_radsort_keyed-cold-u64_128b-random-1000000 | logs/bench3.log |
| u64_128b | random | 1000000 | rust_std_stable | 672.24 | rust_std_stable-cold-u64_128b-random-1000000 | logs/bench3.log |

## hot rows

| type | pattern | n | sort | median ms | benchmark id | raw log |
| --- | --- | ---: | --- | ---: | --- | --- |
| f64 | random | 10000 | rust_block_lsd_stable_radix | 0.34 | rust_block_lsd_stable_radix-hot-f64-random-10000 | logs/hot-redo.log |
| f64 | random | 10000 | rust_ipnsort_unstable | 0.84 | rust_ipnsort_unstable-hot-f64-random-10000 | logs/hot-redo.log |
| f64 | random | 10000 | rust_radsort_keyed | 0.32 | rust_radsort_keyed-hot-f64-random-10000 | logs/hot-redo.log |
| f64 | random | 10000 | rust_std_stable | 0.92 | rust_std_stable-hot-f64-random-10000 | logs/hot-redo.log |
| f64 | random | 100000 | rust_block_lsd_stable_radix | 2.67 | rust_block_lsd_stable_radix-hot-f64-random-100000 | logs/hot-redo.log |
| f64 | random | 100000 | rust_ipnsort_unstable | 9.50 | rust_ipnsort_unstable-hot-f64-random-100000 | logs/hot-redo.log |
| f64 | random | 100000 | rust_radsort_keyed | 3.07 | rust_radsort_keyed-hot-f64-random-100000 | logs/hot-redo.log |
| f64 | random | 100000 | rust_std_stable | 11.05 | rust_std_stable-hot-f64-random-100000 | logs/hot-redo.log |
| f64 | random | 1000000 | rust_block_lsd_stable_radix | 53.94 | rust_block_lsd_stable_radix-hot-f64-random-1000000 | logs/hot-redo.log |
| f64 | random | 1000000 | rust_ipnsort_unstable | 117.65 | rust_ipnsort_unstable-hot-f64-random-1000000 | logs/hot-redo.log |
| f64 | random | 1000000 | rust_radsort_keyed | 71.77 | rust_radsort_keyed-hot-f64-random-1000000 | logs/hot-redo.log |
| f64 | random | 1000000 | rust_std_stable | 145.64 | rust_std_stable-hot-f64-random-1000000 | logs/hot-redo.log |
| u128 | random | 10000 | rust_block_lsd_stable_radix | 0.37 | rust_block_lsd_stable_radix-hot-u128-random-10000 | logs/hot-redo.log |
| u128 | random | 10000 | rust_ipnsort_unstable | 0.31 | rust_ipnsort_unstable-hot-u128-random-10000 | logs/hot-redo.log |
| u128 | random | 10000 | rust_radsort_radix | 0.34 | rust_radsort_radix-hot-u128-random-10000 | logs/hot-redo.log |
| u128 | random | 10000 | rust_std_stable | 0.41 | rust_std_stable-hot-u128-random-10000 | logs/hot-redo.log |
| u128 | random | 100000 | rust_block_lsd_stable_radix | 4.16 | rust_block_lsd_stable_radix-hot-u128-random-100000 | logs/hot-redo.log |
| u128 | random | 100000 | rust_ipnsort_unstable | 3.70 | rust_ipnsort_unstable-hot-u128-random-100000 | logs/hot-redo.log |
| u128 | random | 100000 | rust_radsort_radix | 6.34 | rust_radsort_radix-hot-u128-random-100000 | logs/hot-redo.log |
| u128 | random | 100000 | rust_std_stable | 7.00 | rust_std_stable-hot-u128-random-100000 | logs/hot-redo.log |
| u128 | random | 1000000 | rust_block_lsd_stable_radix | 102.45 | rust_block_lsd_stable_radix-hot-u128-random-1000000 | logs/hot-redo.log |
| u128 | random | 1000000 | rust_ipnsort_unstable | 47.38 | rust_ipnsort_unstable-hot-u128-random-1000000 | logs/hot-redo.log |
| u128 | random | 1000000 | rust_radsort_radix | 125.39 | rust_radsort_radix-hot-u128-random-1000000 | logs/hot-redo.log |
| u128 | random | 1000000 | rust_std_stable | 82.03 | rust_std_stable-hot-u128-random-1000000 | logs/hot-redo.log |
| u64 | ascending | 100000 | rust_block_lsd_stable_radix | 3.45 | rust_block_lsd_stable_radix-hot-u64-ascending-100000 | logs/pattern-bench.log |
| u64 | ascending | 100000 | rust_ipnsort_unstable | 0.05 | rust_ipnsort_unstable-hot-u64-ascending-100000 | logs/pattern-bench.log |
| u64 | ascending | 100000 | rust_radsort_radix | 4.70 | rust_radsort_radix-hot-u64-ascending-100000 | logs/pattern-bench.log |
| u64 | ascending | 100000 | rust_std_stable | 0.09 | rust_std_stable-hot-u64-ascending-100000 | logs/pattern-bench.log |
| u64 | ascending | 1000000 | rust_block_lsd_stable_radix | 41.15 | rust_block_lsd_stable_radix-hot-u64-ascending-1000000 | logs/pattern-bench.log |
| u64 | ascending | 1000000 | rust_ipnsort_unstable | 1.21 | rust_ipnsort_unstable-hot-u64-ascending-1000000 | logs/pattern-bench.log |
| u64 | ascending | 1000000 | rust_radsort_radix | 112.59 | rust_radsort_radix-hot-u64-ascending-1000000 | logs/pattern-bench.log |
| u64 | ascending | 1000000 | rust_std_stable | 1.37 | rust_std_stable-hot-u64-ascending-1000000 | logs/pattern-bench.log |
| u64 | descending | 100000 | rust_block_lsd_stable_radix | 3.45 | rust_block_lsd_stable_radix-hot-u64-descending-100000 | logs/pattern-bench.log |
| u64 | descending | 100000 | rust_ipnsort_unstable | 0.11 | rust_ipnsort_unstable-hot-u64-descending-100000 | logs/pattern-bench.log |
| u64 | descending | 100000 | rust_radsort_radix | 4.02 | rust_radsort_radix-hot-u64-descending-100000 | logs/pattern-bench.log |
| u64 | descending | 100000 | rust_std_stable | 0.11 | rust_std_stable-hot-u64-descending-100000 | logs/pattern-bench.log |
| u64 | descending | 1000000 | rust_block_lsd_stable_radix | 41.40 | rust_block_lsd_stable_radix-hot-u64-descending-1000000 | logs/pattern-bench.log |
| u64 | descending | 1000000 | rust_ipnsort_unstable | 1.89 | rust_ipnsort_unstable-hot-u64-descending-1000000 | logs/pattern-bench.log |
| u64 | descending | 1000000 | rust_radsort_radix | 107.40 | rust_radsort_radix-hot-u64-descending-1000000 | logs/pattern-bench.log |
| u64 | descending | 1000000 | rust_std_stable | 2.06 | rust_std_stable-hot-u64-descending-1000000 | logs/pattern-bench.log |
| u64 | random | 10000 | rust_block_lsd_stable_radix | 0.30 | rust_block_lsd_stable_radix-hot-u64-random-10000 | logs/hot-redo.log |
| u64 | random | 10000 | rust_ipnsort_unstable | 0.18 | rust_ipnsort_unstable-hot-u64-random-10000 | logs/hot-redo.log |
| u64 | random | 10000 | rust_radsort_radix | 0.18 | rust_radsort_radix-hot-u64-random-10000 | logs/hot-redo.log |
| u64 | random | 10000 | rust_std_stable | 0.32 | rust_std_stable-hot-u64-random-10000 | logs/hot-redo.log |
| u64 | random | 100000 | rust_block_lsd_stable_radix | 2.35 | rust_block_lsd_stable_radix-hot-u64-random-100000 | logs/hot-redo.log |
| u64 | random | 100000 | rust_ipnsort_unstable | 2.24 | rust_ipnsort_unstable-hot-u64-random-100000 | logs/hot-redo.log |
| u64 | random | 100000 | rust_radsort_radix | 2.34 | rust_radsort_radix-hot-u64-random-100000 | logs/hot-redo.log |
| u64 | random | 100000 | rust_std_stable | 3.33 | rust_std_stable-hot-u64-random-100000 | logs/hot-redo.log |
| u64 | random | 1000000 | rust_block_lsd_stable_radix | 47.17 | rust_block_lsd_stable_radix-hot-u64-random-1000000 | logs/hot-redo.log |
| u64 | random | 1000000 | rust_ipnsort_unstable | 27.60 | rust_ipnsort_unstable-hot-u64-random-1000000 | logs/hot-redo.log |
| u64 | random | 1000000 | rust_radsort_radix | 81.38 | rust_radsort_radix-hot-u64-random-1000000 | logs/hot-redo.log |
| u64 | random | 1000000 | rust_std_stable | 44.85 | rust_std_stable-hot-u64-random-1000000 | logs/hot-redo.log |
| u64 | random | 10000000 | rust_block_lsd_stable_radix | 687.97 | rust_block_lsd_stable_radix-hot-u64-random-10000000 | logs/hot-redo.log |
| u64 | random | 10000000 | rust_ipnsort_unstable | 368.68 | rust_ipnsort_unstable-hot-u64-random-10000000 | logs/hot-redo.log |
| u64 | random | 10000000 | rust_radsort_radix | 777.62 | rust_radsort_radix-hot-u64-random-10000000 | logs/hot-redo.log |
| u64 | random | 10000000 | rust_std_stable | 662.81 | rust_std_stable-hot-u64-random-10000000 | logs/hot-redo.log |
| u64 | random_p5 | 100000 | rust_block_lsd_stable_radix | 2.71 | rust_block_lsd_stable_radix-hot-u64-random_p5-100000 | logs/pattern-bench.log |
| u64 | random_p5 | 100000 | rust_ipnsort_unstable | 0.29 | rust_ipnsort_unstable-hot-u64-random_p5-100000 | logs/pattern-bench.log |
| u64 | random_p5 | 100000 | rust_radsort_radix | 2.82 | rust_radsort_radix-hot-u64-random_p5-100000 | logs/pattern-bench.log |
| u64 | random_p5 | 100000 | rust_std_stable | 1.17 | rust_std_stable-hot-u64-random_p5-100000 | logs/pattern-bench.log |
| u64 | random_p5 | 1000000 | rust_block_lsd_stable_radix | 27.24 | rust_block_lsd_stable_radix-hot-u64-random_p5-1000000 | logs/pattern-bench.log |
| u64 | random_p5 | 1000000 | rust_ipnsort_unstable | 4.69 | rust_ipnsort_unstable-hot-u64-random_p5-1000000 | logs/pattern-bench.log |
| u64 | random_p5 | 1000000 | rust_radsort_radix | 41.17 | rust_radsort_radix-hot-u64-random_p5-1000000 | logs/pattern-bench.log |
| u64 | random_p5 | 1000000 | rust_std_stable | 17.89 | rust_std_stable-hot-u64-random_p5-1000000 | logs/pattern-bench.log |
| u64_128b | random | 10000 | rust_block_lsd_stable_radix | 1.38 | rust_block_lsd_stable_radix-hot-u64_128b-random-10000 | logs/hot-redo.log |
| u64_128b | random | 10000 | rust_radsort_keyed | 2.41 | rust_radsort_keyed-hot-u64_128b-random-10000 | logs/hot-redo.log |
| u64_128b | random | 10000 | rust_std_stable | 2.48 | rust_std_stable-hot-u64_128b-random-10000 | logs/hot-redo.log |
| u64_128b | random | 100000 | rust_block_lsd_stable_radix | 34.57 | rust_block_lsd_stable_radix-hot-u64_128b-random-100000 | logs/hot-redo.log |
| u64_128b | random | 100000 | rust_radsort_keyed | 70.47 | rust_radsort_keyed-hot-u64_128b-random-100000 | logs/hot-redo.log |
| u64_128b | random | 100000 | rust_std_stable | 34.21 | rust_std_stable-hot-u64_128b-random-100000 | logs/hot-redo.log |
| u64_128b | random | 1000000 | rust_block_lsd_stable_radix | 557.69 | rust_block_lsd_stable_radix-hot-u64_128b-random-1000000 | logs/hot-redo.log |
| u64_128b | random | 1000000 | rust_radsort_keyed | 806.55 | rust_radsort_keyed-hot-u64_128b-random-1000000 | logs/hot-redo.log |
| u64_128b | random | 1000000 | rust_std_stable | 606.30 | rust_std_stable-hot-u64_128b-random-1000000 | logs/hot-redo.log |

## Auxiliary heap rows

Source: logs/extra-memory.log, produced by benches/extra_memory.rs with:

    cargo bench --bench extra_memory --features rust_block_lsd,rust_radsort
      -- --sizes 1000,1000000 --types u64,f128,record32,record128,owned
      --patterns random

The column quoted in the drafts is peak_extra_heap_bytes. retained_extra_heap_bytes
is 0 for every row, which is the check that a sort gives everything back.

## Not from this data

- ipnsort being unstable and in-place: ipnsort/src/lib.rs, its own documentation.
- The block size formula: src/other/rust_block_lsd/core.rs, fn sort.
- Anything about the C reference or the C++ template comparison: that lives in
  c-comparison/ and is deliberately absent from both drafts.
- The claim that the pre-scan cuts 8 passes to 3 on ordered u64: from the number
  of varying digits in an ascending u64 sequence, and visible in the ascending
  rows above (41 ms instead of ~110 ms).
