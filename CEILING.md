# Ceiling report: what Tessera costs, and what was measured to get there

Branch `exp/radix-opt`. Every number is from this machine (Xeon Gold 6140 KVM, 2.3 GHz,
L1d 32 KiB / L2 1 MiB / L3 24.8 MiB shared, effectively DRAM-bound at 1M), the `radix_ab`
harness, **medians** of interleaved rounds, raw rounds archived in `raw_runs.csv`.

## 1. The design in one paragraph

One stable MSD level partitions the array by the digit the sampler says divides it; a
child that fits a ~31 KiB tile is finished in cache by one MSD level over the tile (or by
flat counting passes, chosen by two counted quantities); constant key bits are dropped
everywhere; the block-permutation engine from `rust_block_lsd` keeps auxiliary space at
O(sqrt n). Nothing in the router or the leaf is a magic threshold: each decision reads one
or two counted facts (passes needed, distinct buckets sampled, tile bytes).

## 2. Gates that must hold (all verified at HEAD)

| gate | evidence |
| --- | --- |
| O(n) time | one permutation per level, levels bounded by key width / digit width |
| stable | `tessera_functionality` (233 cases), `radix_owned`, `radix_views` |
| O(sqrt n) extra space | block engine + one tile buffer; no n-sized buffer exists |
| 7 test targets green | lib 10, block_lsd 8, block_lsd_functionality 233, main 233, radix_owned 2, radix_views 6, tessera_functionality 233 |
| Miri 7/7 | `cargo +nightly miri test --features rust_tessera_radix --lib other::rust_tessera_radix` |

## 3. Distance to the physical floor (1M, medians)

`x memcpy` is ns per record divided by one memcpy of the array (u32 0.8, u64 1.6, u128 3.2
ns per record at 8.8-11.8 GB/s). `pass floor` is the naive number of digit passes a key
needs; `x floor` below 1 means the sort beats the assumption that every pass is cold,
which is the design's whole claim.

| cell | ns/rec | x memcpy | pass floor | x floor | ipnsort (context) |
| --- | --- | --- | --- | --- | --- |
| u32 random | 11.30 | 14.1 | 4 | 3.53 | 23.90 |
| u32 99% sorted | 7.83 | 9.8 | 4 | 2.45 | 21.18 |
| u32 few values | 4.55 | 5.7 | 4 | 1.42 | 3.82 |
| u32 zipf | 8.53 | 10.7 | 4 | 2.66 | 3.50 |
| u32 mixed_runs | 11.93 | 14.9 | 4 | 3.73 | 27.25 |
| u32 one value 99% | 4.60 | 5.8 | 4 | 1.44 | 1.96 |
| u64 random | 17.89 | 11.2 | 8 | 1.40 | 27.55 |
| u64 99% sorted | 24.04 | 15.0 | 8 | 1.88 | 24.08 |
| u64 few values | 7.60 | 4.7 | 8 | 0.59 | 5.57 |
| u64 zipf | 11.34 | 7.1 | 8 | 0.89 | 4.74 |
| u64 mixed_runs | 19.02 | 11.9 | 8 | 1.49 | 27.93 |
| u64 one value 99% | 6.44 | 4.0 | 8 | 0.50 | 3.28 |
| u128 random | 32.28 | 10.1 | 16 | 0.63 | 49.99 |
| u128 99% sorted | 22.06 | 6.9 | 16 | 0.43 | 39.03 |
| u128 few values | 17.91 | 5.6 | 16 | 0.35 | 11.89 |
| u128 zipf | 25.46 | 8.0 | 16 | 0.50 | 9.69 |
| u128 mixed_runs | 30.67 | 9.6 | 16 | 0.60 | 48.75 |
| u128 one value 99% | 12.86 | 4.0 | 16 | 0.25 | 6.50 |

Reading it:

- **Wide records are where this design wins**: u128 runs at 0.25-0.63x of its pass floor,
  moving the array fewer times than an LSD sort must, because after one split the work
  happens in cache.
- **u32 has the least headroom** (1.4-3.7x of floor): four passes over 4 MB leaves no cold
  traffic to amortise against, so per-pass machinery dominates. A property of small keys,
  not something more tuning removes.
- **Skew is where a comparison sort beats us** (ipnsort 2.4-2.7x faster on zipf): equal
  keys fall out of a partition in one pass, while a radix level still moves every record.

10M, medians, ns per record: u32 random 16.3 (ipnsort 27.9), u64 random 26.5 (33.2),
u128 random 47.8 (59.6), u64 zipf 15.1 (6.6), u128 zipf 29.4 (10.8).

## 4. What was tried and what it said

Every row is one mechanism, with the metric that decided it.

| mechanism | verdict | number |
| --- | --- | --- |
| in-place permutation of a level | rejected | 20-33 % slower (s36) |
| wider split digit | rejected | worse at 1M and 10M (s21) |
| cold/hot ratio in the router | rejected | -53 % on few values but +8-14 % instructions elsewhere; replaced (s40) |
| structural rule: split while more than one pass remains | **kept** | few values -58 % (s41) |
| leaf width re-sweep | rejected | best 3.5 %, sign unstable (s48) |
| router compares span instead of running the planner | **kept** | 4770 nodes, 0 decision changes, instructions flat (s51) |
| fold the descent into the permutation | rejected | +10-16 % instructions, +35-41 % wall (s50) |
| skew-aware split width | **kept** | zipf -17 % instructions (s49) |
| leaf cost model replaced by two counts | **kept** | decision-neutral in instructions, -23 code lines (s53) |

Four of ten survived. That ratio is the honest summary of how much room was left.

## 5. Why the remaining gap is not algorithmic

At 1M every access in this harness misses cache (37 misses per record measured), so the
split's traffic is what it is: a 256-stream scatter writes lines it must read first, and a
compact pass moves the array again. The exits are fewer passes (wider digit, measured
worse), full-line writes (a scatter cannot), or an in-place permutation (20-33 % slower,
because cycle following costs more than the copy it saves). The leaf is at its own pass
floor. What remains is per-pass machinery (bucket table clear and scan, boundary reads): a
constant per pass, not a structural term.

**The one shape where this ceiling is not ours**: heavy duplicates (zipf, one value at
99 %). There the answer is a comparison-based partition, i.e. a different sort rather than
an optimisation of this one.

## 6. Reproduce

```
cargo test --release --features rust_tessera_radix
cargo +nightly miri test --features rust_tessera_radix --lib other::rust_tessera_radix
RADIX_RAW=raw_runs.csv taskset -c 2 <bench> --types u64 --patterns random_wide,zipf20 --sizes 1000000 --rounds 7
RADIX_TRACE=1 <same>            # one line per routing node: the decision log
RADIX_LEAF_WIDTH=n RADIX_SPLIT_WIDTH=n   # measurement hooks, unset in shipped runs
```
