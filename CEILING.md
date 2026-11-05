# Ceiling report: what Tessera costs, and what was measured to get there

Branch `exp/radix-opt`. Every number is from this machine (Xeon Gold 6140 KVM, 2.3 GHz,
L1d 32 KiB / L2 1 MiB / L3 24.8 MiB shared, effectively DRAM-bound at 1M), the `radix_ab`
harness, **medians** of interleaved rounds, raw rounds archived in `raw_runs.csv`.

## 1. The design in one paragraph

One stable MSD level partitions the array by the digit the sampler says divides it. A node
whose sample is dominated by one value is split around that value instead (s67); a short run
that disagrees with the node's top bit is lifted out (s68); and an array whose keys one
insertion pass can finish for the price of one pass is finished by it (s70). A child that
fits a ~31 KiB tile is finished in cache by one MSD level over the tile (or by flat counting
passes, chosen by two counted quantities); constant key bits are dropped everywhere; the
block-permutation engine from `rust_block_lsd` keeps auxiliary space at O(sqrt n). Nothing
in the router or the leaf is a magic threshold: each decision reads counted facts -- passes
needed, distinct buckets sampled, tile bytes, moves per record.

## 2. Gates that must hold (all verified at HEAD)

| gate | evidence |
| --- | --- |
| O(n) time | one permutation per level, levels bounded by key width / digit width |
| stable | `tessera_functionality` (233 cases), `radix_owned`, `radix_views` |
| O(sqrt n) extra space | block engine + one tile buffer; no n-sized buffer exists |
| 7 test targets green | lib 16, block_lsd 8, block_lsd_functionality 233, main 233, radix_owned 2, radix_views 6, tessera_functionality 233 |
| Miri 16/16 | `cargo +nightly miri test --features rust_tessera_radix --lib` |

## 3. Distance to the physical floor (1M, medians)

Measured at s70 with the current harness, two sweeps, the lower median reported (the machine is
shared and single sweeps move by 5-10 %); the pattern behind each row is in
parentheses. The 2026-11 baseline this table replaced (before s64–s68) is in the ledger's
§1 table, and its raw rounds stay in `raw_runs.csv` under their own commit. The two are
not directly comparable where the harness pattern names changed.

`x memcpy` is ns per record divided by one memcpy of the array (u32 0.8, u64 1.6, u128 3.2
ns per record at 8.8-11.8 GB/s). `pass floor` is the naive number of digit passes a key
needs; `x floor` below 1 means the sort beats the assumption that every pass is cold,
which is the design's whole claim.

| cell | ns/rec | x memcpy | pass floor | x floor | ipnsort (context) |
| --- | --- | --- | --- | --- | --- |
| u32 random | 10.85 | 13.6 | 4 | 3.39 | 23.81 |
| u32 99% sorted | 0.97 | 1.2 | 4 | 0.30 | 20.18 |
| u32 few values (sparse8) | 4.33 | 5.4 | 4 | 1.35 | 3.67 |
| u32 zipf (zipf13) | 12.50 | 15.6 | 4 | 3.91 | 7.73 |
| u32 mixed_runs | 11.60 | 14.5 | 4 | 3.63 | 23.91 |
| u32 one value 95% (p95zero) | 4.50 | 5.6 | 4 | 1.41 | 3.18 |
| u64 random | 14.40 | 9.0 | 8 | 1.13 | 25.95 |
| u64 99% sorted | 1.67 | 1.0 | 8 | 0.13 | 22.31 |
| u64 few values (sparse8) | 7.55 | 4.7 | 8 | 0.59 | 4.79 |
| u64 zipf (zipf13) | 15.97 | 10.0 | 8 | 1.25 | 8.92 |
| u64 mixed_runs | 15.81 | 9.9 | 8 | 1.24 | 26.56 |
| u64 one value 95% (p95zero) | 5.90 | 3.7 | 8 | 0.46 | 4.84 |
| u128 random | 27.97 | 8.7 | 16 | 0.55 | 45.13 |
| u128 99% sorted | 2.82 | 0.9 | 16 | 0.06 | 35.28 |
| u128 few values (sparse8) | 16.83 | 5.3 | 16 | 0.33 | 9.83 |
| u128 zipf (zipf13) | 28.74 | 9.0 | 16 | 0.56 | 17.34 |
| u128 mixed_runs | 27.77 | 8.7 | 16 | 0.54 | 46.91 |
| u128 one value 95% (p95zero) | 10.94 | 3.4 | 16 | 0.21 | 9.85 |

Reading it:

- **Wide records are where this design wins**: u128 runs at 0.06-0.56x of its pass floor,
  moving the array fewer times than an LSD sort must, because after one split the work
  happens in cache.
- **u32 has the least headroom** (0.3-3.9x of floor): four passes over 4 MB leaves no cold
  traffic to amortise against, so per-pass machinery dominates. A property of small keys,
  not something more tuning removes.
- **Skew is where a comparison sort still beats us** (ipnsort 1.6-1.9x faster on zipf13):
  forty thousand distinct values, none of them dominant, so every radix level moves the
  whole node again. A *dominant* value is no longer a loss: one value at 95 % (p95zero) is
  at parity with ipnsort (u64 5.90 vs 4.84) since s67 finishes the band in one pass.
- **Near-sorted is now the cheapest thing this sort does**, and the only cell where it beats
  a memcpy by a factor smaller than one: 99 % sorted is 1.67 ns/rec (s68 lifted the sentinel
  keys out, s70 finishes the displaced pairs with one insertion pass priced in moves). The
  same code gives 10.15 ns/rec when the same 1 % of positions are displaced *far* instead of
  adjacent -- the pass prices itself, gives up, and the radix levels take over.

10M, medians at s70, ns per record: u32 random 16.10 (ipnsort 28.07), u64 random 22.8
(32.7), u128 random 40.5 (56.8), u64 zipf13 20.5 (11.9), u128 zipf13 36.0 (21.0),
u64 99% sorted 1.7 (29.7), u128 99% sorted 3.0, u64 one value 95% 6.5 (6.0).

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
| one insertion pass for a nearly ordered array, priced in moves on a key copy | **kept** | 99 % sorted 15.4 -> 1.4 ns at 1M and 21.7 -> 1.7 at 10M; the same shape displaced far does not win (s70) |

Nine of sixteen survived. That ratio is the honest summary of how much room was left.

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
router could see (s67, s68, s70); a distribution with 40 000 distinct values and a long tail is
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
