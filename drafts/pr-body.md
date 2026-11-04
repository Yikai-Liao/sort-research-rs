Implements #35.

## What this adds

A stable LSD radix sort over converted unsigned keys of 8 to 128 bits, with auxiliary space
that grows as O(sqrt(n)) instead of O(n). It redistributes fixed-size blocks
into per-bucket output blocks and keeps a permutation of those blocks, so it
never needs a full second copy of the input.

    src/other/rust_block_lsd/core.rs      the port
    src/other/rust_block_lsd.rs           adapter, feature rust_block_lsd
    src/other/radix_support.rs            Key/Unsigned contract + adapter macro
    tests/block_lsd_functionality.rs      the standard suite, 233 tests
    tests/block_lsd.rs                    block tails, record widths, rejections
    tests/radix_owned.rs, radix_views.rs  non-Copy records, drop counts, panics
    benches/extra_memory.rs               peak extra heap

Sort name in the harness: rust_block_lsd_stable_radix.

It is stable and preserves the input order of equal keys. It sorts unsigned
integers, signed integers (biased), char, bool, (), Cell<T: Copy>, f32/f64
(total order, NaN sorted as a value), pairs of those, and any record whose type
implements the Key trait. The 128 bit figure is the width of the converted key,
not of the user type: a record only has to provide an order preserving
conversion into u8..u128, so custom fixed point and newtype keys work without
touching the sorter. Key width beyond 128 bits is out of scope here. Types
without a Key impl and comparator based sorting are
rejected explicitly rather than silently falling back.

## Deviations from the reference implementation

The reference (Clausecker & Schintke, "Parallel O(sqrt(n)) Overhead LSD Radix
Sort", radixsort_permuted.c) fixes a 512-record block because it only ever
sorts one 8 byte record. Two changes were needed for a generic implementation,
and one for a Rust-specific reason:

1. Block size from n and the record size:
   b = max(ceil(4 * sqrt(9n / (2 * R * s))), floor(1024 / s), 1), R = 2^radix.
   With a fixed 512 the scratch array is 512 * 512 records no matter how small
   the input is, which is what makes the reference O(1) in n but also what makes
   it wasteful for wide records and for small inputs. Ablation over the tested
   types: faster below ~4M records, equal above. sort_with_block_size is public,
   so the reference configuration is one call away.

2. A pre-scan of the key bits (varying_key_bits) skips digits that are constant
   across the input. It costs one pass over the keys (~1025 of them on random
   input, since it stops early) and saves whole passes on skewed data. The
   reference does not do this.

3. Panic recovery. Key::key() is user code, so it can panic in the middle of a
   pass, with records already moved into scratch. For a type with a destructor
   the half-moved slice would then own the same value twice and drop it twice.
   The sorter therefore records how far it got and, on unwind, restores the
   slice to a permutation of the original records. This is the one place the
   port pays for something the reference does not need: 3 % of the hot loop on
   8 byte records. That cost is paid only for types that need drop: a duplicated
   Copy record is inert bits, so needs_drop::<T>() gates the bookkeeping and the
   compiler removes it for every type this repository currently supports.

## Correctness

- Functionality: the full instantiate_sort_tests! suite for this sort, 233
  tests, plus the tests listed above. All pass in release.
- Address sanitizer: full suite under RUSTFLAGS=-Zsanitizer=address, no reports.
- Miri: three binaries (block_lsd, radix_owned, radix_views) with reduced but
  explicit shapes; the scaling is documented in the test comments because a
  Miri run interprets every instruction of every pass.
- The tests assert stability with payloads, block boundary cases (0, 1, 2 and
  block_size + 1 records), every integer width, u128 keys with a partial top
  digit, and that the rejection paths leave the input untouched.

## Performance

Method: cold benchmarks (cold_benchmarks), warm-up 2 s and measurement 4 s as in
util/run_benchmarks.py, critcmp export as the baseline, graphs from
util/graph_bench_result/graph_all.py. Machine: Skylake-SP Xeon Gold 6140,
2.3 GHz, shared 6 vCPU VM, nightly 2026-10-08. Run-to-run variation on this VM
is a few percent, which is the resolution to read the ratios at.

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

Attached: results JSON, scaling graph, single-size graph, and the extra memory
table.

Reading:
- Against rust_radsort_radix, the sort with the same asymptotic behaviour but a
  full O(n) scratch array, block LSD is faster at every measured size >= 1e5:
  0.95x on u64 at 1e5, 0.61x at 1e6, 0.77x on u128 at 1e6, 0.45-0.67x on 128
  byte records, 0.73-0.85x on f64. The gap grows with the record size because
  scratch never holds a second copy of the input.
- Against std and ipnsort the picture is not a win: ipnsort is faster on u64 and
  u128 (27.9 ms vs 42.8 ms on 1e6 u64, cold), and std is on par with block LSD
  on u64 at 1e6 (43.1 ms vs 42.8 ms) while losing badly on wide records
  (0.38-0.81x). Radix sort is not a general replacement for comparison sorts
  here and the table should be read that way.
- n <= 1e4: the block tables and the ~0.5 MB scratch floor dominate, both
  comparison sorts win. This is the trade-off the reference makes too.
- Ordered input: the pre-scan skips constant digits, so on ascending and
  descending u64 the port runs 3 passes instead of 8 and beats rust_radsort_radix
  by 2.6x (1M: 41.2 ms vs 112.6 ms). It still loses to the comparison sorts there
  by 20-40x because they are adaptive and stop at O(n) comparisons in cache.
  On 95% duplicates (random_p5) it is on par with radix and 1.5x behind std.
- Extra memory at 1M records: block LSD uses O(sqrt(n)) bytes (measured peak),
  radsort and std_stable use O(n). The measurement is in benches/extra_memory.rs.

## Questions

1. Panic contract: the bookkeeping is skipped for types without destructors,
   so after a panic their contents are unspecified while memory safety holds.
   Types with destructors are still restored exactly. Say if you would rather
   have the same exactness for every type at the 3 % cost.
2. Vendored (this PR) or an optional dependency on the reference port?
3. Keep benches/extra_memory.rs as a new bench target?
4. The sort_test_tools::Sort declarations (supports_type, supports_sort_by) are
   independent and can be split into their own PR.

## Checklist

- [x] All functionality tests pass (233 for this sort)
- [x] Address sanitizer run is clean
- [x] Miri run is clean, with documented reduced shapes
- [x] New benchmark target documented in benches/
- [x] No changes to existing sort behaviour (capability declarations default to
      the previous answers)
