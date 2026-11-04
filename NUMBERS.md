# Historical benchmark provenance

Corrected code candidate: `22463e9ef3e59a917afe8fcd1ed1a52ca123d9b7`. It has not been compiled, tested,
or benchmarked during this closeout. None of the timings below measure it.

The 106 entries are preserved from the pre-closeout `block_lsd` export.
Individual measurement commit IDs were not recorded. The previous code head
was 429f6f7; that does not establish the revision of each measurement.

All times below and in the graphs are JSON median point estimates. Ratios use
those medians. The text summary also reports the mean separately.

For the 82 random entries, the JSON mean/slope and confidence interval match
the rounded Criterion output in the listed logs. These aggregate logs do not
contain the raw samples needed to recompute medians; the median source is JSON.

The 24 ascending/descending/random_p5 entries are saved baseline estimates only.
Their original measurement log has not been recovered. `logs/pattern-bench.log`
contains a later run against a saved baseline, not the measurements exported here.
These rows are retained for archival completeness and excluded from reply/PR tables.

Both hot and cold generate inputs via batch setup. Cold additionally disturbs CPU
prediction state; the harness deliberately avoids testing main-memory loading.
`logs/contaminated/` is archival and is never used as an input to regeneration.

| state | type | pattern | n | sort | median ms | benchmark id | provenance |
| --- | --- | --- | ---: | --- | ---: | --- | --- |
| cold | f64 | random | 100000 | rust_block_lsd_stable_radix | 2.73 | rust_block_lsd_stable_radix-cold-f64-random-100000 | logs/bench3.log |
| cold | f64 | random | 1000000 | rust_block_lsd_stable_radix | 54.20 | rust_block_lsd_stable_radix-cold-f64-random-1000000 | logs/bench3.log |
| cold | u128 | random | 100000 | rust_block_lsd_stable_radix | 4.58 | rust_block_lsd_stable_radix-cold-u128-random-100000 | logs/bench3.log |
| cold | u128 | random | 1000000 | rust_block_lsd_stable_radix | 86.03 | rust_block_lsd_stable_radix-cold-u128-random-1000000 | logs/bench3.log |
| cold | u64 | random | 100000 | rust_block_lsd_stable_radix | 2.49 | rust_block_lsd_stable_radix-cold-u64-random-100000 | logs/bench3.log |
| cold | u64 | random | 1000000 | rust_block_lsd_stable_radix | 42.79 | rust_block_lsd_stable_radix-cold-u64-random-1000000 | logs/bench3.log |
| cold | u64 | random | 10000000 | rust_block_lsd_stable_radix | 583.46 | rust_block_lsd_stable_radix-cold-u64-random-10000000 | logs/bench3.log |
| cold | u64_128b | random | 10000 | rust_block_lsd_stable_radix | 1.42 | rust_block_lsd_stable_radix-cold-u64_128b-random-10000 | logs/bench3.log |
| cold | u64_128b | random | 100000 | rust_block_lsd_stable_radix | 33.95 | rust_block_lsd_stable_radix-cold-u64_128b-random-100000 | logs/bench3.log |
| cold | u64_128b | random | 1000000 | rust_block_lsd_stable_radix | 544.94 | rust_block_lsd_stable_radix-cold-u64_128b-random-1000000 | logs/bench3.log |
| hot | f64 | random | 10000 | rust_block_lsd_stable_radix | 0.34 | rust_block_lsd_stable_radix-hot-f64-random-10000 | logs/hot-redo.log |
| hot | f64 | random | 100000 | rust_block_lsd_stable_radix | 2.67 | rust_block_lsd_stable_radix-hot-f64-random-100000 | logs/hot-redo.log |
| hot | f64 | random | 1000000 | rust_block_lsd_stable_radix | 53.94 | rust_block_lsd_stable_radix-hot-f64-random-1000000 | logs/hot-redo.log |
| hot | u128 | random | 10000 | rust_block_lsd_stable_radix | 0.37 | rust_block_lsd_stable_radix-hot-u128-random-10000 | logs/hot-redo.log |
| hot | u128 | random | 100000 | rust_block_lsd_stable_radix | 4.16 | rust_block_lsd_stable_radix-hot-u128-random-100000 | logs/hot-redo.log |
| hot | u128 | random | 1000000 | rust_block_lsd_stable_radix | 102.45 | rust_block_lsd_stable_radix-hot-u128-random-1000000 | logs/hot-redo.log |
| hot | u64 | ascending | 100000 | rust_block_lsd_stable_radix | 3.45 | rust_block_lsd_stable_radix-hot-u64-ascending-100000 | saved baseline only; original log unavailable |
| hot | u64 | ascending | 1000000 | rust_block_lsd_stable_radix | 41.15 | rust_block_lsd_stable_radix-hot-u64-ascending-1000000 | saved baseline only; original log unavailable |
| hot | u64 | descending | 100000 | rust_block_lsd_stable_radix | 3.45 | rust_block_lsd_stable_radix-hot-u64-descending-100000 | saved baseline only; original log unavailable |
| hot | u64 | descending | 1000000 | rust_block_lsd_stable_radix | 41.40 | rust_block_lsd_stable_radix-hot-u64-descending-1000000 | saved baseline only; original log unavailable |
| hot | u64 | random | 10000 | rust_block_lsd_stable_radix | 0.30 | rust_block_lsd_stable_radix-hot-u64-random-10000 | logs/hot-redo.log |
| hot | u64 | random | 100000 | rust_block_lsd_stable_radix | 2.35 | rust_block_lsd_stable_radix-hot-u64-random-100000 | logs/hot-redo.log |
| hot | u64 | random | 1000000 | rust_block_lsd_stable_radix | 47.17 | rust_block_lsd_stable_radix-hot-u64-random-1000000 | logs/hot-redo.log |
| hot | u64 | random | 10000000 | rust_block_lsd_stable_radix | 687.97 | rust_block_lsd_stable_radix-hot-u64-random-10000000 | logs/hot-redo.log |
| hot | u64 | random_p5 | 100000 | rust_block_lsd_stable_radix | 2.71 | rust_block_lsd_stable_radix-hot-u64-random_p5-100000 | saved baseline only; original log unavailable |
| hot | u64 | random_p5 | 1000000 | rust_block_lsd_stable_radix | 27.24 | rust_block_lsd_stable_radix-hot-u64-random_p5-1000000 | saved baseline only; original log unavailable |
| hot | u64_128b | random | 10000 | rust_block_lsd_stable_radix | 1.38 | rust_block_lsd_stable_radix-hot-u64_128b-random-10000 | logs/hot-redo.log |
| hot | u64_128b | random | 100000 | rust_block_lsd_stable_radix | 34.57 | rust_block_lsd_stable_radix-hot-u64_128b-random-100000 | logs/hot-redo.log |
| hot | u64_128b | random | 1000000 | rust_block_lsd_stable_radix | 557.69 | rust_block_lsd_stable_radix-hot-u64_128b-random-1000000 | logs/hot-redo.log |
| cold | u64 | random | 100000 | rust_ipnsort_unstable | 2.15 | rust_ipnsort_unstable-cold-u64-random-100000 | logs/bench3.log |
| cold | u64 | random | 1000000 | rust_ipnsort_unstable | 27.91 | rust_ipnsort_unstable-cold-u64-random-1000000 | logs/bench3.log |
| cold | u64 | random | 10000000 | rust_ipnsort_unstable | 364.53 | rust_ipnsort_unstable-cold-u64-random-10000000 | logs/bench4.log |
| hot | f64 | random | 10000 | rust_ipnsort_unstable | 0.84 | rust_ipnsort_unstable-hot-f64-random-10000 | logs/hot-redo.log |
| hot | f64 | random | 100000 | rust_ipnsort_unstable | 9.50 | rust_ipnsort_unstable-hot-f64-random-100000 | logs/hot-redo.log |
| hot | f64 | random | 1000000 | rust_ipnsort_unstable | 117.65 | rust_ipnsort_unstable-hot-f64-random-1000000 | logs/hot-redo.log |
| hot | u128 | random | 10000 | rust_ipnsort_unstable | 0.31 | rust_ipnsort_unstable-hot-u128-random-10000 | logs/hot-redo.log |
| hot | u128 | random | 100000 | rust_ipnsort_unstable | 3.70 | rust_ipnsort_unstable-hot-u128-random-100000 | logs/hot-redo.log |
| hot | u128 | random | 1000000 | rust_ipnsort_unstable | 47.38 | rust_ipnsort_unstable-hot-u128-random-1000000 | logs/hot-redo.log |
| hot | u64 | ascending | 100000 | rust_ipnsort_unstable | 0.05 | rust_ipnsort_unstable-hot-u64-ascending-100000 | saved baseline only; original log unavailable |
| hot | u64 | ascending | 1000000 | rust_ipnsort_unstable | 1.21 | rust_ipnsort_unstable-hot-u64-ascending-1000000 | saved baseline only; original log unavailable |
| hot | u64 | descending | 100000 | rust_ipnsort_unstable | 0.11 | rust_ipnsort_unstable-hot-u64-descending-100000 | saved baseline only; original log unavailable |
| hot | u64 | descending | 1000000 | rust_ipnsort_unstable | 1.89 | rust_ipnsort_unstable-hot-u64-descending-1000000 | saved baseline only; original log unavailable |
| hot | u64 | random | 10000 | rust_ipnsort_unstable | 0.18 | rust_ipnsort_unstable-hot-u64-random-10000 | logs/hot-redo.log |
| hot | u64 | random | 100000 | rust_ipnsort_unstable | 2.24 | rust_ipnsort_unstable-hot-u64-random-100000 | logs/hot-redo.log |
| hot | u64 | random | 1000000 | rust_ipnsort_unstable | 27.60 | rust_ipnsort_unstable-hot-u64-random-1000000 | logs/hot-redo.log |
| hot | u64 | random | 10000000 | rust_ipnsort_unstable | 368.68 | rust_ipnsort_unstable-hot-u64-random-10000000 | logs/hot-redo.log |
| hot | u64 | random_p5 | 100000 | rust_ipnsort_unstable | 0.29 | rust_ipnsort_unstable-hot-u64-random_p5-100000 | saved baseline only; original log unavailable |
| hot | u64 | random_p5 | 1000000 | rust_ipnsort_unstable | 4.69 | rust_ipnsort_unstable-hot-u64-random_p5-1000000 | saved baseline only; original log unavailable |
| cold | f64 | random | 100000 | rust_radsort_keyed | 3.23 | rust_radsort_keyed-cold-f64-random-100000 | logs/bench3.log |
| cold | f64 | random | 1000000 | rust_radsort_keyed | 74.10 | rust_radsort_keyed-cold-f64-random-1000000 | logs/bench3.log |
| cold | u64_128b | random | 10000 | rust_radsort_keyed | 2.66 | rust_radsort_keyed-cold-u64_128b-random-10000 | logs/bench3.log |
| cold | u64_128b | random | 100000 | rust_radsort_keyed | 75.58 | rust_radsort_keyed-cold-u64_128b-random-100000 | logs/bench3.log |
| cold | u64_128b | random | 1000000 | rust_radsort_keyed | 812.15 | rust_radsort_keyed-cold-u64_128b-random-1000000 | logs/bench3.log |
| hot | f64 | random | 10000 | rust_radsort_keyed | 0.32 | rust_radsort_keyed-hot-f64-random-10000 | logs/hot-redo.log |
| hot | f64 | random | 100000 | rust_radsort_keyed | 3.07 | rust_radsort_keyed-hot-f64-random-100000 | logs/hot-redo.log |
| hot | f64 | random | 1000000 | rust_radsort_keyed | 71.77 | rust_radsort_keyed-hot-f64-random-1000000 | logs/hot-redo.log |
| hot | u64_128b | random | 10000 | rust_radsort_keyed | 2.41 | rust_radsort_keyed-hot-u64_128b-random-10000 | logs/hot-redo.log |
| hot | u64_128b | random | 100000 | rust_radsort_keyed | 70.47 | rust_radsort_keyed-hot-u64_128b-random-100000 | logs/hot-redo.log |
| hot | u64_128b | random | 1000000 | rust_radsort_keyed | 806.55 | rust_radsort_keyed-hot-u64_128b-random-1000000 | logs/hot-redo.log |
| cold | u128 | random | 100000 | rust_radsort_radix | 7.48 | rust_radsort_radix-cold-u128-random-100000 | logs/bench3.log |
| cold | u128 | random | 1000000 | rust_radsort_radix | 111.22 | rust_radsort_radix-cold-u128-random-1000000 | logs/bench3.log |
| cold | u64 | random | 100000 | rust_radsort_radix | 2.62 | rust_radsort_radix-cold-u64-random-100000 | logs/bench3.log |
| cold | u64 | random | 1000000 | rust_radsort_radix | 69.63 | rust_radsort_radix-cold-u64-random-1000000 | logs/bench3.log |
| cold | u64 | random | 10000000 | rust_radsort_radix | 786.48 | rust_radsort_radix-cold-u64-random-10000000 | logs/bench3.log |
| hot | u128 | random | 10000 | rust_radsort_radix | 0.34 | rust_radsort_radix-hot-u128-random-10000 | logs/hot-redo.log |
| hot | u128 | random | 100000 | rust_radsort_radix | 6.34 | rust_radsort_radix-hot-u128-random-100000 | logs/hot-redo.log |
| hot | u128 | random | 1000000 | rust_radsort_radix | 125.39 | rust_radsort_radix-hot-u128-random-1000000 | logs/hot-redo.log |
| hot | u64 | ascending | 100000 | rust_radsort_radix | 4.70 | rust_radsort_radix-hot-u64-ascending-100000 | saved baseline only; original log unavailable |
| hot | u64 | ascending | 1000000 | rust_radsort_radix | 112.59 | rust_radsort_radix-hot-u64-ascending-1000000 | saved baseline only; original log unavailable |
| hot | u64 | descending | 100000 | rust_radsort_radix | 4.02 | rust_radsort_radix-hot-u64-descending-100000 | saved baseline only; original log unavailable |
| hot | u64 | descending | 1000000 | rust_radsort_radix | 107.40 | rust_radsort_radix-hot-u64-descending-1000000 | saved baseline only; original log unavailable |
| hot | u64 | random | 10000 | rust_radsort_radix | 0.18 | rust_radsort_radix-hot-u64-random-10000 | logs/hot-redo.log |
| hot | u64 | random | 100000 | rust_radsort_radix | 2.34 | rust_radsort_radix-hot-u64-random-100000 | logs/hot-redo.log |
| hot | u64 | random | 1000000 | rust_radsort_radix | 81.38 | rust_radsort_radix-hot-u64-random-1000000 | logs/hot-redo.log |
| hot | u64 | random | 10000000 | rust_radsort_radix | 777.62 | rust_radsort_radix-hot-u64-random-10000000 | logs/hot-redo.log |
| hot | u64 | random_p5 | 100000 | rust_radsort_radix | 2.82 | rust_radsort_radix-hot-u64-random_p5-100000 | saved baseline only; original log unavailable |
| hot | u64 | random_p5 | 1000000 | rust_radsort_radix | 41.17 | rust_radsort_radix-hot-u64-random_p5-1000000 | saved baseline only; original log unavailable |
| cold | f64 | random | 100000 | rust_std_stable | 11.25 | rust_std_stable-cold-f64-random-100000 | logs/bench3.log |
| cold | f64 | random | 1000000 | rust_std_stable | 142.39 | rust_std_stable-cold-f64-random-1000000 | logs/bench3.log |
| cold | u128 | random | 100000 | rust_std_stable | 6.74 | rust_std_stable-cold-u128-random-100000 | logs/bench3.log |
| cold | u128 | random | 1000000 | rust_std_stable | 78.66 | rust_std_stable-cold-u128-random-1000000 | logs/bench3.log |
| cold | u64 | random | 100000 | rust_std_stable | 3.36 | rust_std_stable-cold-u64-random-100000 | logs/bench3.log |
| cold | u64 | random | 1000000 | rust_std_stable | 43.09 | rust_std_stable-cold-u64-random-1000000 | logs/bench3.log |
| cold | u64 | random | 10000000 | rust_std_stable | 630.22 | rust_std_stable-cold-u64-random-10000000 | logs/bench4.log |
| cold | u64_128b | random | 10000 | rust_std_stable | 2.52 | rust_std_stable-cold-u64_128b-random-10000 | logs/bench3.log |
| cold | u64_128b | random | 100000 | rust_std_stable | 34.75 | rust_std_stable-cold-u64_128b-random-100000 | logs/bench3.log |
| cold | u64_128b | random | 1000000 | rust_std_stable | 672.24 | rust_std_stable-cold-u64_128b-random-1000000 | logs/bench3.log |
| hot | f64 | random | 10000 | rust_std_stable | 0.92 | rust_std_stable-hot-f64-random-10000 | logs/hot-redo.log |
| hot | f64 | random | 100000 | rust_std_stable | 11.05 | rust_std_stable-hot-f64-random-100000 | logs/hot-redo.log |
| hot | f64 | random | 1000000 | rust_std_stable | 145.64 | rust_std_stable-hot-f64-random-1000000 | logs/hot-redo.log |
| hot | u128 | random | 10000 | rust_std_stable | 0.41 | rust_std_stable-hot-u128-random-10000 | logs/hot-redo.log |
| hot | u128 | random | 100000 | rust_std_stable | 7.00 | rust_std_stable-hot-u128-random-100000 | logs/hot-redo.log |
| hot | u128 | random | 1000000 | rust_std_stable | 82.03 | rust_std_stable-hot-u128-random-1000000 | logs/hot-redo.log |
| hot | u64 | ascending | 100000 | rust_std_stable | 0.09 | rust_std_stable-hot-u64-ascending-100000 | saved baseline only; original log unavailable |
| hot | u64 | ascending | 1000000 | rust_std_stable | 1.37 | rust_std_stable-hot-u64-ascending-1000000 | saved baseline only; original log unavailable |
| hot | u64 | descending | 100000 | rust_std_stable | 0.11 | rust_std_stable-hot-u64-descending-100000 | saved baseline only; original log unavailable |
| hot | u64 | descending | 1000000 | rust_std_stable | 2.06 | rust_std_stable-hot-u64-descending-1000000 | saved baseline only; original log unavailable |
| hot | u64 | random | 10000 | rust_std_stable | 0.32 | rust_std_stable-hot-u64-random-10000 | logs/hot-redo.log |
| hot | u64 | random | 100000 | rust_std_stable | 3.33 | rust_std_stable-hot-u64-random-100000 | logs/hot-redo.log |
| hot | u64 | random | 1000000 | rust_std_stable | 44.85 | rust_std_stable-hot-u64-random-1000000 | logs/hot-redo.log |
| hot | u64 | random | 10000000 | rust_std_stable | 662.81 | rust_std_stable-hot-u64-random-10000000 | logs/hot-redo.log |
| hot | u64 | random_p5 | 100000 | rust_std_stable | 1.17 | rust_std_stable-hot-u64-random_p5-100000 | saved baseline only; original log unavailable |
| hot | u64 | random_p5 | 1000000 | rust_std_stable | 17.89 | rust_std_stable-hot-u64-random_p5-1000000 | saved baseline only; original log unavailable |
| hot | u64_128b | random | 10000 | rust_std_stable | 2.48 | rust_std_stable-hot-u64_128b-random-10000 | logs/hot-redo.log |
| hot | u64_128b | random | 100000 | rust_std_stable | 34.21 | rust_std_stable-hot-u64_128b-random-100000 | logs/hot-redo.log |
| hot | u64_128b | random | 1000000 | rust_std_stable | 606.30 | rust_std_stable-hot-u64_128b-random-1000000 | logs/hot-redo.log |

## Historical peak extra heap

Source: `logs/extra-memory.log`, requested bytes, excluding the input allocation.
Every retained-extra-heap value is zero. These measurements also predate closeout.

| type | n | bytes / record | std stable | ipnsort | radsort | block LSD |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| u64 | 1000000 | 8 | 8000000 | 0 | 8000000 | 832767 |
| f128 | 1000000 | 16 | 8000000 | 0 | – | 1172046 |
| record32 | 1000000 | 32 | 16000000 | 0 | 32000000 | 1650686 |
| record128 | 1000000 | 128 | 64000000 | 0 | 128000000 | 3286524 |
| owned | 1000000 | 72 | 36000000 | 0 | 72000000 | 2480137 |
| u64 | 1000 | 8 | 8000 | 0 | 8000 | 539199 |

The reference's fixed blocks bound scratch only; block metadata grows with n.
The adaptive policy makes both terms O(sqrt(n)) for fixed radix and record size.

The benchmark converts ascending i32 values to u64 as `(i + 2^31) * (2^31 - 1)`.
At 100k and 1M all eight byte digits vary: no 8-to-3-pass explanation applies.
The converted u64 inputs have constant high bits, so the 1025-key early-stop
condition is not satisfied and the pre-scan reads the entire input.
