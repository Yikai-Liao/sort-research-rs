Implemented this on a branch: https://github.com/Yikai-Liao/sort-research-rs/tree/feat/block-lsd
(compare: https://github.com/Voultapher/sort-research-rs/compare/main...Yikai-Liao:sort-research-rs:feat/block-lsd)

Two decisions changed relative to the issue text. Both are worth settling
before I open a PR.

## What is in the branch

- src/other/rust_block_lsd/core.rs — the port. src/other/rust_block_lsd.rs is
  the adapter, feature rust_block_lsd, sort name
  rust_block_lsd_stable_radix.
- src/other/radix_support.rs — the key contract (Key, Unsigned) and the
  adapter macro shared by the three radix sorts.
- tests/block_lsd_functionality.rs instantiates the standard suite (233
  tests). tests/block_lsd.rs adds block tails, wide records, integer widths
  and the rejection cases. tests/radix_owned.rs and tests/radix_views.rs
  cover non-Copy records, drop counts and a key extractor that panics.
- benches/extra_memory.rs reports peak extra heap so the O(sqrt(n)) claim is
  measured rather than asserted.

## Deviation 1: the block size

The issue proposes the reference's fixed 512-record block. The port derives it
from n and the record size instead:

    b = max(ceil(4 * sqrt(9n / (2 * R * s))), floor(1024 / s), 1)

with R = 2^radix buckets and s = sizeof::<T>(). This keeps scratch blocks and
the two permutation tables at O(sqrt(n)) together instead of pinning scratch to
512 * 512 records regardless of the input. Ablation over the benchmark types
found it faster than a fixed block at every length below ~4M records and equal
above it. If you prefer the reference value it is one call:
core::sort_with_block_size(arr, 8, 512).

## Deviation 2: two declarations in sort_test_tools::Sort

The standard suite assumes every sort accepts any Ord type and implements
sort_by. This adapter rejects both: no Key impl, no comparator. Instead of
keeping a hand-written subset of checks, I added supports_type::<T>() and
supports_sort_by() to Sort, both defaulting to true so no existing sort
changes, and the affected tests return early when the answer is false. The
adapter declares them from RadixDispatch so the two cannot disagree.

That change is independent of this sort and can be a separate PR if you prefer.

## Results

Cold benchmarks, warm-up 2 s / measurement 4 s, the same settings
util/run_benchmarks.py uses. Baseline file and graphs are attached.

Times in ms, cold benchmarks (caches trashed before each call, which the
harness describes as "as part of a larger program"):

| type | n | std stable | ipnsort | radsort | radsort keyed | block LSD | block / radsort | block / std |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| f64 | 100000 | 11.25 | – | – | 3.23 | 2.73 | 0.85x | 0.24x |
| f64 | 1000000 | 142.39 | – | – | 74.10 | 54.20 | 0.73x | 0.38x |
| u128 | 100000 | 6.74 | – | 7.48 | – | 4.58 | 0.61x | 0.68x |
| u128 | 1000000 | 78.66 | – | 111.22 | – | 86.03 | 0.77x | 1.09x |
| u64 | 100000 | 3.36 | 2.15 | 2.62 | – | 2.49 | 0.95x | 0.74x |
| u64 | 1000000 | 43.09 | 27.91 | 69.63 | – | 42.79 | 0.61x | 0.99x |
| u64 | 10000000 | 630.22 | 364.53 | 786.48 | – | 583.46 | 0.74x | 0.93x |
| u64_128b | 10000 | 2.52 | – | – | 2.66 | 1.42 | 0.53x | 0.56x |
| u64_128b | 100000 | 34.75 | – | – | 75.58 | 33.95 | 0.45x | 0.98x |
| u64_128b | 1000000 | 672.24 | – | – | 812.15 | 544.94 | 0.67x | 0.81x |

Hot benchmarks (the same buffers reused, caches warm):

| type | n | std stable | ipnsort | radsort | radsort keyed | block LSD | block / radsort | block / std |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| f64 | 10000 | 0.92 | 0.84 | – | 0.32 | 0.34 | 1.05x | 0.37x |
| f64 | 100000 | 11.05 | 9.50 | – | 3.07 | 2.67 | 0.87x | 0.24x |
| f64 | 1000000 | 145.64 | 117.65 | – | 71.77 | 53.94 | 0.75x | 0.37x |
| u128 | 10000 | 0.41 | 0.31 | 0.34 | – | 0.37 | 1.10x | 0.91x |
| u128 | 100000 | 7.00 | 3.70 | 6.34 | – | 4.16 | 0.66x | 0.59x |
| u128 | 1000000 | 82.03 | 47.38 | 125.39 | – | 102.45 | 0.82x | 1.25x |
| u64 | 10000 | 0.32 | 0.18 | 0.18 | – | 0.30 | 1.65x | 0.92x |
| u64 | 100000 | 3.33 | 2.24 | 2.34 | – | 2.35 | 1.01x | 0.71x |
| u64 | 1000000 | 44.85 | 27.60 | 81.38 | – | 47.17 | 0.58x | 1.05x |
| u64 | 10000000 | 662.81 | 368.68 | 777.62 | – | 687.97 | 0.88x | 1.04x |
| u64_128b | 10000 | 2.48 | – | – | 2.41 | 1.38 | 0.57x | 0.56x |
| u64_128b | 100000 | 34.21 | – | – | 70.47 | 34.57 | 0.49x | 1.01x |
| u64_128b | 1000000 | 606.30 | – | – | 806.55 | 557.69 | 0.69x | 0.92x |

Other patterns (hot, u64, ms):

| pattern | n | std stable | ipnsort | radsort | block LSD | block / radsort | block / std |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| ascending | 100000 | 0.09 | 0.05 | 4.70 | 3.45 | 0.73x | 40.47x |
| ascending | 1000000 | 1.37 | 1.21 | 112.59 | 41.15 | 0.37x | 30.09x |
| descending | 100000 | 0.11 | 0.11 | 4.02 | 3.45 | 0.86x | 30.66x |
| descending | 1000000 | 2.06 | 1.89 | 107.40 | 41.40 | 0.39x | 20.08x |
| random_p5 | 100000 | 1.17 | 0.29 | 2.82 | 2.71 | 0.96x | 2.31x |
| random_p5 | 1000000 | 17.89 | 4.69 | 41.17 | 27.24 | 0.66x | 1.52x |

Auxiliary heap:

| 1M records | byte/record | std stable | ipnsort | radsort | block LSD |
| --- | ---: | ---: | ---: | ---: | ---: |
| u64 | 8 | 8.0 MB | 0 | 8.0 MB | 0.83 MB |
| f128 | 16 | 8.0 MB | 0 | – | 1.17 MB |
| record32 | 32 | 16.0 MB | 0 | 32.0 MB | 1.65 MB |
| record128 | 128 | 64.0 MB | 0 | 128.0 MB | 3.29 MB |
| owned (72 B) | 72 | 36.0 MB | 0 | 72.0 MB | 2.48 MB |
| 1000 x u64 | 8 | 8 KB | 0 | 8 KB | 0.54 MB |

Peak extra heap per sort, measured by benches/extra_memory.rs (requested bytes,
global allocator wrapper, retained heap is 0 for every row).

## Questions before a PR

1. Panic contract. Key::key() is user code and may panic mid-pass. The port
   then restores the slice to a permutation of the original records, which
   matters for types with destructors: without it a half-moved slice holds the
   same owner twice and gets dropped twice. That costs 3% of the hot loop, so
   the bookkeeping is gated on needs_drop::<T>() and disappears for Copy
   records, where a duplicate is inert bits. Types without destructors
   therefore get memory safety but unspecified contents after a panic, while
   types with destructors are restored exactly. Fine with you, or do you want
   exactness for every type at the cost?

2. Vendored or dependency. The port is ~700 lines adapted from the reference
   (BSD-2-Clause, notices kept, deviations commented). Vendoring keeps the
   harness dependency free, matching the other src/other implementations.

3. Keep benches/extra_memory.rs? It is a new bench target; it measures
   requested heap through a global allocator wrapper, and it is what backs the
   auxiliary-space claim in the table above.

No C-reference performance numbers are included: the comparison above is
against the other sorts in this repository only.
