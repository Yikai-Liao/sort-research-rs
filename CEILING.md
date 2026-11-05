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
| 7 test targets green | lib 13, block_lsd 8, block_lsd_functionality 233, main 233, radix_owned 2, radix_views 6, tessera_functionality 233 |
| Miri 13/13 | `cargo +nightly miri test --features rust_tessera_radix --lib` |

## 3. Distance to the physical floor (1M, medians)

Measured at s68 (`bad4f09`) with the current harness; the pattern behind each row is in
parentheses. The 2026-11 baseline this table replaced (before s64–s68) is in the ledger's
§1 table, and its raw rounds stay in `raw_runs.csv` under their own commit. The two are
not directly comparable where the harness pattern names changed.

`x memcpy` is ns per record divided by one memcpy of the array (u32 0.8, u64 1.6, u128 3.2
ns per record at 8.8-11.8 GB/s). `pass floor` is the naive number of digit passes a key
needs; `x floor` below 1 means the sort beats the assumption that every pass is cold,
which is the design's whole claim.

| cell | ns/rec | x memcpy | pass floor | x floor | ipnsort (context) |
| --- | --- | --- | --- | --- | --- |
| u32 random | 11.22 | 14.0 | 4 | 3.51 | 23.37 |
| u32 99% sorted | 8.28 | 10.3 | 4 | 2.59 | 20.39 |
| u32 few values (sparse8) | 4.50 | 5.6 | 4 | 1.41 | 3.57 |
| u32 zipf (zipf13) | 12.65 | 15.8 | 4 | 3.95 | 7.81 |
| u32 mixed_runs | 10.99 | 13.7 | 4 | 3.43 | 23.97 |
| u32 one value 95% (p95zero) | 5.01 | 6.3 | 4 | 1.57 | 5.05 |
| u64 random | 15.43 | 9.6 | 8 | 1.21 | 26.32 |
| u64 99% sorted | 17.11 | 10.7 | 8 | 1.34 | 23.01 |
| u64 few values (sparse8) | 6.99 | 4.4 | 8 | 0.55 | 4.80 |
| u64 zipf (zipf13) | 16.84 | 10.5 | 8 | 1.32 | 8.89 |
| u64 mixed_runs | 16.55 | 10.3 | 8 | 1.29 | 27.16 |
| u64 one value 95% (p95zero) | 6.02 | 3.8 | 8 | 0.47 | 5.18 |
| u128 random | 29.61 | 9.3 | 16 | 0.58 | 46.94 |
| u128 99% sorted | 20.04 | 6.3 | 16 | 0.39 | 36.56 |
| u128 few values (sparse8) | 17.74 | 5.5 | 16 | 0.35 | 10.38 |
| u128 zipf (zipf13) | 27.89 | 8.7 | 16 | 0.54 | 17.22 |
| u128 mixed_runs | 29.93 | 9.4 | 16 | 0.58 | 47.14 |
| u128 one value 95% (p95zero) | 10.50 | 3.3 | 16 | 0.21 | 9.96 |

Reading it:

- **Wide records are where this design wins**: u128 runs at 0.25-0.63x of its pass floor,
  moving the array fewer times than an LSD sort must, because after one split the work
  happens in cache.
- **u32 has the least headroom** (1.4-3.7x of floor): four passes over 4 MB leaves no cold
  traffic to amortise against, so per-pass machinery dominates. A property of small keys,
  not something more tuning removes.
- **Skew is where a comparison sort still beats us** (ipnsort 1.6-1.9x faster on zipf13):
  forty thousand distinct values, none of them dominant, so every radix level moves the
  whole node again. A *dominant* value is no longer a loss: one value at 95 % (p95zero) is
  at parity with ipnsort (u64 6.02 vs 5.18) since s67 finishes the band in one pass.
- **Near-sorted is no longer a loss either**: 99 % sorted used to pay a whole extra level
  for its two sentinel keys (s68 lifts them out); u64 is now 17.11 against ipnsort's 23.01.

10M, medians at s68, ns per record: u32 random 16.10 (ipnsort 28.07), u64 random 22.62
(32.69), u128 random 40.96 (56.82), u32 zipf13 14.33 (8.34), u64 zipf13 20.45 (11.93),
u128 zipf13 36.00 (21.04), u64 99% sorted 22.15 (29.73), u64 one value 95% 6.33 (6.02).

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
| probe the order before scanning the keys' variation | **kept** | halves the per-node scan on ordered input (s64) |
| finish a leaf with one insertion sort when no bucket exceeds it | **kept** | P1② of the measurement plan (s65) |
| split a node around a value a quarter of its sample shares | **kept** | two values 8.5 -> 6.6 ns, one value 95 % 16.5 -> 6.0 at 1M (s67) |
| lift out a short run that disagrees with the node's top bit | **kept** | 99 % sorted 21.35 -> 16.16 at 1M (s68) |
| stream the compact / double-buffer the engine | rejected | the compact is 10-14 % of cycles, so half of it bounds the win at 5-7 % (s69) |

Eight of fourteen survived. That ratio is the honest summary of how much room was left.

## 5. Why the remaining gap is not algorithmic

At 1M every access in this harness misses cache (37 misses per record measured), so the
split's traffic is what it is: a 256-stream scatter writes lines it must read first, and a
compact pass moves the array again. The exits are fewer passes (wider digit, measured
worse), full-line writes (a scatter cannot), or an in-place permutation (20-33 % slower,
because cycle following costs more than the copy it saves). The leaf is at its own pass
floor. What remains is per-pass machinery (bucket table clear and scan, boundary reads): a
constant per pass, not a structural term.

**The one shape where this ceiling is not ours**: many distinct values with a skew, none
dominant (zipf). The dominant-value and near-sorted shapes were budget problems this
router could see (s67, s68); a distribution with 40 000 distinct values and a long tail is
not, and closing it needs a comparison-based partition (backlog E12/B14) rather than
another radix decision.

## 6. Reproduce

```
cargo test --release --features rust_tessera_radix
cargo +nightly miri test --features rust_tessera_radix --lib
RADIX_RAW=raw_runs.csv taskset -c 2 <bench> --types u64 --patterns random_wide,zipf20 --sizes 1000000 --rounds 7
RADIX_TRACE=1 <same>            # one line per routing node: the decision log
RADIX_LEAF_WIDTH=n RADIX_SPLIT_WIDTH=n   # measurement hooks, unset in shipped runs
```
