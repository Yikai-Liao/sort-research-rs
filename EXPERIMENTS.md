# Radix experiments: Tessera (stable tiled radix sort)

Branch `exp/radix-opt`, forked from `feat/block-lsd` (429f6f7). Block LSD is
untouched and stays the baseline; the candidate is a new module. This is
research code, not a merge proposal.

Session deadline: 2026-11-05 10:00 Asia/Shanghai.

## Roles

| role | implementation |
| --- | --- |
| stable radix baseline, O(sqrt n) space | `rust_block_lsd_stable_radix` (unchanged) |
| candidate | `rust_tessera_stable_radix` (new) |
| radix speed ceiling, unstable | `rust_msd_radix` (new, in-place american-flag MSD) |
| comparison references | `rust_std_stable`, `rust_ipnsort_unstable`, `rust_radsort_radix` |

Harness: `benches/radix_ab.rs`, an interleaved tournament (fresh copy per run,
variants rotate, min and median over rounds, one correctness assertion per
variant per cell). Criterion stays the tool for numbers that go into the
repository's own tables; the tournament exists because a tuning session needs a
20-second turnaround.

Machine: Skylake-SP Xeon Gold 6140, 2.3 GHz, 6 vCPU shared VM, 32 KiB L1d, 1 MiB
L2 per core, 148 MiB L3 (THP on), nightly 1.101.0 (2026-10-08).

```sh
cargo bench --features rust_block_lsd,rust_tessera_radix,rust_msd_radix \
    --bench radix_ab -- --types u64,u32 --patterns random_wide,zipf13 \
    --sizes 100000,1000000 --rounds 7
RADIX_TRACE=1 ...            # print every routing decision
RADIX_AB_LEAVES=1 ...        # add the tile/leaf sweep variants
```

## Why a new algorithm rather than more LSD tuning

Measured (1M u64, uniform): an 8-bit LSD pass costs ~5.4 ns per record, and the
u64 sort needs eight of them, so block LSD lands at 42-45 ms while `ipnsort`
needs 26-28 ms. The passes are not bandwidth bound: 128 MiB of traffic at the
measured 8.8-11.8 GB/s memcpy rate would be ~12 ms. The cost is per-record
scatter work repeated `d` times over a cold array.

So the shape is the one the earlier PA work converged on: split by the most
significant varying bits once over the cold array, then finish each child while
it is still in cache. At 1M u64 one 8-bit split leaves 31 KiB children whose
remaining passes together cost less than one cold pass.

## The candidate

`src/other/rust_tessera_radix/engine.rs` (about 600 lines plus tests):

- `Variation` — the union/intersection of the node's keys. One scan of the node
  produces it, and it is what tells every later decision which bits can still
  differ. Both routes consume it, so the fallback does not rescan the node.
- `plan_digits` — turns a variation into the list of passes the node will really
  run: digits of a chosen width, highest first, each narrowed to the span that
  actually varies. Constant bits are dropped, which is what keeps duplicate
  heavy, narrow-range and skewed inputs from paying no-op passes.
- `sort_node` — three ways to finish a node, chosen by the cost model below:
  MSD split, cache-tile leaves, or the block engine's LSD for the whole node.
- `descend` — one walk over a split node: bucket boundaries, and each child's
  union/intersection accumulated while the boundary is still open.
- `lsd_leaf` / `counting_pass` — stable counting scatter per remaining digit,
  ping-ponging between the tile and one private buffer; 32-bit cursors, raw
  pointers, no bounds checks.
- `merge_leaf` / `insertion_leaf` — stable merge and insertion sorts for tiles
  small enough that a bucket table per pass is the dominant cost.

Reused from the baseline module: `partition_digit`, `varying_key_bits`,
`default_block_size`, `Sorter`. One implementation of "stable block
distribution" serves both algorithms; Tessera adds `sort_with_variation` so a
caller that already scanned the node can skip the second scan.

### Cost model

Same shape as the block-size formula in `core.rs`: compare predicted totals
instead of tuning thresholds. Two totals, in ns per record:

```
streamed = cold_passes  * cold_ns_per_record(node)
split    = SPLIT_HOT_PASSES * hot_ns_per_record(record) + hot_passes * hot_ns_per_record(record)
```

- `hot_ns_per_record` = max(2.5, bytes/3.0) — measured 2.3 ns per 4-byte and
  2.6 ns per 8-byte record on cache-resident tiles.
- `cold_ns_per_record` = max(3.0, bpr / bandwidth(node)) with
  `bandwidth = 3.4 * (node_bytes / 2 MiB)^-0.30` — fitted to measured block LSD
  passes: 3.4 GB/s at 1.6 MiB, 2.1 at 8 MiB, 1.7 at 16 MiB, 1.05 at 80 MiB.
- `cold_passes`/`hot_passes` are the real pass counts this node would run, at
  the block engine's 8-bit digits and at the leaf's own width. A 16-byte key
  with sixteen digits and a narrow one with two are therefore compared on their
  own work.

Verification of the decisions against measurement (`tessera` vs `block LSD`,
ratio < 1 means the split route won):

| node | model | measured |
| --- | --- | --- |
| u64 100k (0.8 MiB) | no split | equal (partition noise) ✓ |
| u32 1M (4 MiB) | no split | 0.9-1.0 ✓ |
| u64 1M (8 MiB) | split | 0.52 ✓ |
| u32 10M (40 MiB) | no split | 1.0-1.1 ✗ (split is 12 % faster; see open items) |
| u128 100k (1.6 MiB) | no split | 0.95 ✓ |
| u128 1M (16 MiB) | split | 0.46 ✓ |
| record128 100k (12.8 MiB) | split | 0.48 ✓ |
| record128 1M (128 MiB) | split | 0.52 ✓ |

Leaf policy is placed by measurement too: `leaf_radix(records)` widens the digit
from 8 to 11 bits as the tile grows (fewer passes, table still in L1), tiles of
at most 48 records use an insertion sort, up to 384 a merge sort.

## Log of experiments

### 1. Digit width, block LSD: wider is worse, every time

1M u64 uniform, block LSD with an explicit radix:

| radix | min | ns/record |
| --- | --- | --- |
| 8 | 43.7 ms | 43.7 |
| 9 | 47.2 | 47.2 |
| 10 | 57.7 | 57.7 |
| 11 | 62.4 | 62.4 |
| 12 | 72.8 | 72.8 |

Wider digits trade 25 % fewer passes for a cursor table that leaves L1 (256
cursors = 4 KiB, 2048 = 32 KiB) plus more block scratch. This closes the "just
use more bits" shortcut for LSD; the win has to come from the pass structure.
The same measurement is why the MSD split stays at 8 bits even though the engine
accepts 12: an 11-bit split pass costs ~19 ns per record against 5.4 for an
8-bit one, which is more than the split level it would save.

### 2. Where the time goes (u32 vs u64 differential)

Same n, same split, 4 vs 8 digits, before tuning: u32 1M 16.6 ms, u64 1M 27.0,
u128 1M 98.0. Four extra digits cost 10.4 ns, so an in-cache tile pass is ~2.6 ns
per record; the split level is 8.9 ns (scatter 5.4 + materialise ~2 + boundary
scan ~1.5). That decomposition is what the cost model is calibrated against.

### 3. Tile budget: the 32 KiB cliff, and why 64 KiB stays

1M u64, tile sweep via `RADIX_AB_LEAVES=1`, ms: 16 KiB 42.9, 24 KiB 43.5,
32 KiB 28.3, 48 KiB 24.2, 64 KiB (default) 24.4, 96 KiB 25.7.

The 16-24 KiB cases are not noise: with a smaller budget the split target shrinks
below what one 8-bit level can produce, so the children need a second split level
and pay another ~9 ns per record. 48-64 KiB is the flat optimum; the 32 KiB case
sits on the boundary (perf: 1.98 G instructions vs 1.09 G — the extra level is
visible in the instruction count).

### 4. Negative result: fusing the next digit's histogram into the scatter

Idea: one read serves two passes. Implemented, measured, rejected:

| type | separate count | fused count |
| --- | --- | --- |
| u32 1M | 16.6 ms | 20.9 ms |
| u64 1M | 27.0 ms | 28.1 ms |

`llvm-mca` explains it: the scatter loop is store-port limited (8 stores per
4 records, port 4 saturated at 8.00/iteration), so adding a cursor
read-modify-store per record buys nothing and costs a store.

### 5. Small tiles: insertion, then merge

A counting pass pays a full bucket table before it moves a record. Tiles of at
most 48 records finish with a stable insertion sort; tiles up to 384 with a
stable bottom-up merge, which needs no table at all. Measured at 100k (where the
tile's remaining digit count is small): u64 random 25.7 → 21.8 ns, u32 random
15.3 → 11.5, u64 zipf13 19.5 → 17.3. A leaf-policy sweep at larger sizes showed
the crossovers matter only for thin tiles (≤ 400 records), which is exactly the
u32 10M corner (152-record children): 13.0 ns with merge against 14.3 with
counting.

### 6. Kernel: 32-bit cursors and raw pointers

`counting_pass` on raw pointers with `u32` cursors (a tile is far below 2^32
records) removed bounds checks and halved counter traffic: u64 1M 27.5 → 25.4,
u64 10M 34.2 → 31.6, u32 10M 16.7 → 15.6. The rare rollover branch in the block
engine also got a `branches`-style `#[cold]` hint (block LSD 42.8 → 36-40 at 1M,
within noise of each other, kept because it is free).

### 7. Leaf digit width: 8 → 11 bits

The leaf used to inherit the plan's 8-bit digits, so a 128-bit key paid sixteen
passes over the tile. The leaf now picks its own width from the tile size
(`leaf_radix`): u128 1M 95.3 → 77.5 (−19 %), u128 10M 118 → 89.7, u64 1M
25.2 → 24.4 (6 instead of 8 passes, partly offset by the bigger table). 12 bits
measured 1-2 % worse (16 KiB table) and was reverted.

### 8. One variation scan instead of two

A node that ends up streamed cold was paying two scans: one in Tessera to decide,
one inside `block::sort_with_block_size`. `sort_with_variation` lets the front
end hand its scan over. Small nodes stopped losing 3-8 % against the baseline;
the 100k rows that used to read 1.1-1.25 are now at parity.

### 9. Ordered input: probe instead of passes

An ordered array needs no digit passes at all. A monotone probe that stops at the
first inversion costs two comparisons on random data, one read on an ascending
array, and on a descending array a reversal plus a repair of the equal-key runs
(reversal alone would move equal keys past each other).

The probe is kept *out* of the variation scan. Fusing the two cost 5 % on
clustered inputs: the order flags defeat the vectorisation of a scan that has to
run to the end, and on that data the flags never pay. A strided gate answers
first — at most 4096 spread pairs, both directions — so "sorted, then a few
appends" (the common shape, and what `random_sorted(99)` builds) rejects in a
handful of comparisons instead of reading the array once more. The gate can only
reject, never accept; acceptance always comes from the exact pass.

| u64 1M | before | after |
| --- | --- | --- |
| ascending | 46.3 | 2.9 |
| descending | 42.6 | 5.5 |
| 99 % sorted | 51.1 | 25.0 (passes, no fast path) |

### 10. Splitting only divides if the digit does

The model assumed a split turns one cold pass into a hot one. On skewed input
that is false: if the top varying digit puts 99 % of the records in one bucket,
the recursion pays a pass and the large child still streams. Sampling the top
digit's occupancy (1024 keys in 32-record runs, so a repeating input pattern
cannot alias with the sample) gave the model the missing term: the split is
charged for the passes on its *largest* child, at that child's working set.

The split's own passes were also charged 2x too much: they read sequentially and
write nothing, which on a wide node moves bytes about twice as fast as a scatter
pass. With the read-bandwidth term (`6 GB/s`, `1.5 ns/record` floor) the model
picked the right route at every measured size, including the two cases it had
wrong before: u128 zipf at 10M (was 37.2, now 24.2, baseline 24.5) and u64
random at 1M (was 43.4, now 24.9).

### 11. 16-bit cursors

A tile is at most `MAX_TILE_RECORDS = 65535` records (a constant, 1 MiB even for
16-byte records, so the space bound is untouched), so the bucket cursors fit in
`u16`. The cursor table is what stays hot in a tile pass; halving it is a few per
cent on wide keys, and it removes 2048 bytes of the leaf's stack frame.

### 12. Measuring: pin, interleave, and distrust a single fast round

Two of the runs here disagreed by 6x on one cell. The cause was a single
anomalous round: the harness reports the minimum, and one round had measured
something else. Since then every table is produced with `taskset` pinning, 5-7
rounds, interleaved variant order, and the median next to the minimum; a cell
that moves more than 10 % against its neighbours is re-run before it is believed.
The `ascending= descending= distinct=` fields on each cell line are there to
catch a harness pattern that is not the shape its name claims (u128 "ascending"
was a hashed random array until the type mapping was fixed).

### 13. Windows follow the varying bits, not a grid

A digit plan used to be a fixed `width`-bit grid over the key, with each cell
narrowed to the varying bits inside it. That split a cluster of varying bits
straddling a cell boundary into two passes, and gave one pass to every cell
holding any varying bit: a key with 31 varying bits scattered over 62 needed six
passes where three windows suffice.

The leaf now packs windows: each window starts at the highest varying bit not yet
consumed, reaches down at most `width` bits, and is then narrowed to the cluster
it covers. The plan comes out most-significant first (the leaf walks it
backwards, which is what LSD needs); spans are disjoint, so key order is
preserved. A window is also never wider than its cluster, so a sparse plan also
allocates fewer buckets per pass.

Measured at 1M against the fixed grid: u128 8 narrow bands 37.0 → 31.3 ns, u128
8 distinct values 18.8 → 16.0, u128 99 % sorted 26.1 → 24.4, u64 zipf 12.4 →
11.9; dense random keys are unchanged, as they must be (every window is full).

The cold-pass count for the model deliberately still uses the fixed grid: that is
what the block engine really runs when a node is streamed, and pricing a route by
a plan it will not execute would bias every decision.

### 14. Negative result: a wider split fan-out

Letting the split reach the leaf maximum (11 bits, tile-sized children) instead
of stopping at 8 bits (256 buckets) produced exactly the children the model
wanted and was still slower: u64 10M 35.4 → 37.9 (9 bits) → 41.4 (10 bits), u128
10M 100 → 94 (9 bits) → 97 (10 bits). With 2048 open streams, every write in the
split's last pass lands in a different 40 KiB region; the page and TLB pressure
costs more than the L1-resident leaves save. Reverted, with the numbers in the
comment so it is not retried blind.

### 16. Negative result: a leaf buffer bigger than the tile

The split target and the buffer limit are different questions, so the leaf buffer
was allowed to grow to 512 KiB while the split still targeted 64 KiB. The only
nodes it changes are 64-512 KiB ones — the 312 KiB children of a 10M split. There
the gain is 33.1 → 32.7 ns (u64 10M, and the unchanged baseline moved 62.6 → 67.7
between the two halves of the same A/B), i.e. inside noise; a claimed 4 % at u128
1M turned out to be cells the change cannot reach. Against that, the buffer would
be O(array) for every array between 64 KiB and 512 KiB, which contradicts the
space story for exactly the sizes a caller is most likely to have. Reverted.

### 15. Measurement hygiene: background jobs and a single fast round

Two runs disagreed by 6x on one cell, and a whole sweep was 30-60 % slow. The
causes were mundane: a `criterion` run left in the background (46 minutes of one
core, invisible in the numbers until `ps`), and reporting the *minimum* over
rounds, which a single anomalous round can corrupt. Every table below is
produced with `taskset` pinning, 5-7 interleaved rounds, the median printed next
to the minimum, and `uptime` checked before the run. Cells the text reports as
"parity" are within the ±8 % noise floor of this shared VM.

## Where the candidate stands

ns per record, one root each, pinned, min of 5 interleaved rounds.
Distribution matrix at n = 1M (`logs/calib6.log`):

| type | pattern | tessera | block LSD | ratio | ipnsort | std stable |
| --- | --- | --- | --- | --- | --- | --- |
| u64 | random | 23.7 | 47.4 | 2.00x | 26.8 | 45.1 |
| u64 | zipf20 | 13.8 | 31.9 | 2.30x | 4.9 | 11.4 |
| u64 | 99 % sorted, 1 % displaced | 24.9 | 40.8 | 1.64x | 25.0 | 36.9 |
| u64 | 95 % sorted, 5 % displaced | 26.7 | 45.5 | 1.71x | 24.8 | 36.4 |
| u64 | sparse (8 values) | 6.6 | 34.2 | 5.17x | 6.3 | 12.6 |
| u64 | 8 narrow bands | 23.5 | 43.7 | 1.86x | 27.5 | 44.4 |
| u64 | ascending | 2.9 | 37.2 | 12.73x | 1.1 | — |
| u64 | descending | 6.0 | 40.3 | 6.71x | 1.8 | 1.7 |
| u64 | 95 % reverse sorted | 25.9 | 42.8 | 1.65x | 27.1 | 18.3 |
| u128 | random | 78.8 | 165.0 | 2.09x | 47.0 | 75.8 |
| u128 | zipf20 | 23.5 (2.4x) | 23.1 | 1.00x | 9.7 | 32.8 |
| u128 | 99 % sorted, 1 % displaced | 24.4 | 33.5 | 1.37x | 39.0 | 61.0 |
| u128 | 95 % sorted, 5 % displaced | 24.4 | 32.1 | 1.32x | 40.0 | 58.5 |
| u128 | sparse (8 values) | 16.0 | 31.4 | 1.96x | 12.1 | 30.2 |
| u128 | 8 narrow bands | 31.3 | 31.4 | 1.00x | 47.0 | 75.8 |
| u128 | ascending | 6.5 | 30.9 | 4.75x | 2.8 | 2.7 |
| u128 | descending | 13.2 | 36.9 | 2.79x | 4.8 | 4.6 |
| u128 | 95 % reverse sorted | 24.6 | 34.7 | 1.41x | 48.7 | 30.5 |
| u32 | random | 13.7 | 12.7 | 0.92x | 24.0 | 38.9 |
| u32 | zipf20 | 8.4 | 9.0 | 1.07x | 3.4 | 5.6 |
| u32 | 99 % sorted, 1 % displaced | 10.2 | 9.8 | 0.96x | 21.2 | 29.9 |
| u32 | 95 % sorted, 5 % displaced | 8.5 | 9.0 | 1.06x | 21.3 | 28.7 |
| u32 | 8 narrow bands | 10.2 | 11.7 | 1.14x | 25.5 | 40.5 |
| u32 | sparse (8 values) | 11.0 | 10.8 | 0.98x | 4.0 | 6.2 |
| u32 | ascending | 2.0 | 9.7 | 4.80x | — | 1.0 |
| u32 | descending | 4.3 | 9.1 | 2.10x | 1.1 | — |

The u32 rows sit at parity because a 4 MiB node never splits: the model prices a
split's read pass at more than the four cold passes it would avoid, and the
remaining difference is the probe plus noise (the same cell reads 9.7-13.7
between runs on this VM). The u128 zipf and narrow-band rows are likewise
parity: both routes are the same three passing block engine, and the model
correctly refuses to split a digit that puts 99 % of the records in one bucket.

Scaling with size, uniform random (§logs/matrix5.log§, §logs/matrix4.log§):

| type, pattern, n | tessera | block LSD | ratio | ipnsort | std stable |
| --- | --- | --- | --- | --- | --- |
| u32 random 100k | 9.5 | 10.8 | 1.13x | 18.8 | 29.3 |
| u32 random 1M | 11.1 | 10.2 | 0.92x | 22.4 | 35.7 |
| u32 random 10M | 20.3 | 21.7 | 1.07x | 27.7 | 47.7 |
| u64 random 100k | 20.7 | 24.8 | 1.19x | 20.6 | 31.5 |
| u64 random 1M | 23.6 | 42.1 | 1.79x | 24.4 | 41.2 |
| u64 random 10M | 32.7 | 63.7 | 1.95x | 32.8 | 64.4 |
| u128 random 100k | 65.5 | 73.3 | 1.12x | 32.8 | 49.3 |
| u128 random 1M | 77.5 | 178.9 | 2.31x | 45.1 | 72.2 |
| u128 random 10M | 92.4 | 192.9 | 2.09x | 59.7 | 112.2 |
| u64 zipf20 100k | 17.9 | 17.8 | 0.99x | 3.1 | 5.3 |
| u64 zipf20 1M | 11.2 | 25.7 | 2.30x | 4.4 | 9.7 |
| u64 zipf20 10M | 17.6 | 30.0 | 1.71x | 6.4 | 22.2 |
| u128 zipf20 100k | 11.1 | 9.3 | 0.84x | 5.1 | 13.4 |
| u128 zipf20 1M | 20.3 | 23.8 | 1.17x | 8.2 | 29.4 |
| u128 zipf20 10M | 22.9 | 22.8 | 1.00x | 12.0 | 43.9 |
| record128 1M | 231 | 446 | 1.93x | 190 | 574 |

The gap over the baseline grows with n, which is the point of the design: the
larger the node, the more a cold pass costs and the more the tiled split saves —
1.1-1.2x at 100k, where both routes are already close to cache resident, up to
2.3x at 1M and 2.1x at 10M for 16-byte keys. The rows that do not win are the
cells where the model declines to split (u32 at 1M, u128 zipf, u128 100k) and the
work is the same block-engine pass the baseline runs, so the difference is the
probe plus noise (u128 zipf 100k reads 0.84x here and 1.14x on the 1M cell of the
same shape, i.e. it is the noise floor of a 1.5 ms cell, not a cost).

All sixteen pattern × type × size families are in `logs/matrix5.log`; the
summary of that sweep:

- Against the stable sort it replaces (`std`) it wins on every u64 and u128
  row, 1.3x to 13x, and on u32 by 1.0-1.1x where u32 never splits: a 32-bit node
  at 1M is 4 MiB, and the model prices a split's read pass at more than the four
  cold passes it would avoid.
- Against `ipnsort` it is ahead on u64 random (23.7 vs 26.8; 32.0 vs 32.8 at
  10M), on u64 near-sorted shapes (99 % sorted 24.9 vs 25.0, 95 % sorted 26.7 vs
  24.8 — a tie once noise is allowed), on u64 narrow bands (23.5 vs 27.5), on
  u128 99 % sorted (24.4 vs 39.0), u128 reverse-sorted (24.6 vs 48.7) and u128
  narrow bands (31.3 vs 47.0). It is behind on u32 (a 4 MiB node never splits, so
  u32 gets a slightly better LSD), on u64 zipf (13.8 vs 4.9) and on u128 random
  (78.8 vs 47.0), u128 zipf (23.5 vs 9.7) and u128 8-distinct (16.0 vs 12.1).
- The u128 random gap is structural: 16 bytes per record per pass, ~12 passes at
  cold bandwidth. `ipnsort` compares a 16-byte key in two registers and moves
  nothing until it partitions. Measured from both sides: the cache-resident leaf
  runs 11 passes at 4.1 ns and the split buys 2.1x, because the scatter of 16-byte
  records is bandwidth-bound and there is no way to touch a 16-byte record fewer
  times than the key has digits.
- Skew and few-distinct-values is where comparison sorts are unbeatable: 8
  distinct u128 values have 31 varying bits, so a digit sort needs 3 windows while
  a comparison sort compares a handful of times and collapses the rest. Tessera
  still beats the stable baseline there (16.0 vs 31.4) because narrowing drops the
  97 constant bits.
- Ordered input used to be the worst row (LSD cannot see order) and is now the
  best: 2.9 ns on ascending u64, 12.7x over the baseline.

### Noise floor

Two functionally identical variants (`leaf_count_only` vs `leaf_ins48` at a size
where neither policy is reached) differ by up to 6 % run to run on this shared
VM. Every conclusion above rests on differences larger than that, or on
instruction counts (`perf`) where the difference was small.

## Space

- Split: `2 * buckets` blocks of `default_block_size`, the same O(sqrt n) block
  scratch the baseline documents, transient per level.
- Leaves: one buffer of at most `TILE_BYTES` (64 KiB), a constant, reused across
  all leaves. Whichever leaf runs — counting, merge or insertion — borrows that
  same buffer and never more.

So the auxiliary space is the baseline's O(sqrt n) block scratch plus O(1). A
constant buffer is O(1) in n and therefore within O(sqrt n); the small-n regime
where the array is smaller than the buffer is bounded by the buffer itself, which
does not grow with n. Measured with the allocator-visible peak
(`bench --bench extra_memory`, requested bytes, one touched page counted as its
allocation):

| input | Tessera peak | block LSD peak |
| --- | --- | --- |
| i32 100 | 400 B | 539 648 B |
| u64 100 | 800 B | 539 648 B |
| u64 10k | 17 644 B | 540 428 B |
| u64 100k | 93 886 B | — |
| u64 1M | 838 598 B | — |
| u64 10M | 2 612 918 B | 2 612 918 B |
| u128 10M | 3 694 090 B | 3 694 090 B |

At 10M records the peak is 2.6 MiB for 80 MiB of records, i.e. the split's block
scratch, which the two implementations share. Below that, Tessera's is bounded by
the array itself: it sizes the tile buffer to `min(tile, len)`, so 100 u64
records cost 800 bytes rather than half a megabyte.

## Validation

- `cargo test --release --features rust_tessera_radix --lib` — 7 engine tests
  (stability with payload identity, every key width × every tile budget,
  duplicates, skew, all-equal, empty/single).
- `cargo test --release --features rust_tessera_radix --test tessera_functionality`
  — the repository's standard 233-case suite, same one block LSD runs.
- `cargo miri test ... --lib engine::tests::duplicates` — no UB reported.
- `RUSTFLAGS=-Zsanitizer=address cargo test -Zbuild-std ...` — see logs/asan.log.

## Reproduce

```sh
# standard suite (233 cases) and engine tests
cargo test --release --features rust_tessera_radix --test tessera_functionality
cargo test --release --features rust_tessera_radix --lib

# interleaved tournament used for every table above
cargo bench --features rust_block_lsd,rust_tessera_radix,rust_msd_radix \
    --bench radix_ab -- --types u32,u64,u128 --patterns random_wide,zipf13 \
    --sizes 100000,1000000,10000000 --rounds 5

# repository-standard criterion numbers
BENCH_REGEX='^(rust_(tessera|block_lsd)_stable_radix|rust_ipnsort_unstable|rust_std_stable)-hot-(u32|u64|u128|f64)-(random|random_z1|ascending)-(100000|1000000|10000000)$' \
cargo bench --features rust_block_lsd,rust_tessera_radix,cold_benchmarks,bench_type_u128,bench_type_f64 \
    --bench bench -- --warm-up-time 2 --measurement-time 4 --noplot --save-baseline tessera

# allocator-visible peak
cargo bench --features rust_block_lsd,rust_tessera_radix,rust_radsort --bench extra_memory
```

Run-to-run variation on this shared VM is large (the same cell can move 20 %
between runs); every table uses the minimum over rounds, and ratios are more
trustworthy than absolute milliseconds.

## Open items

1. u32 10M: the model predicts "do not split" where splitting is 12 % faster.
   The floor of the cold-pass cost (3.0 ns/record) is what misplaces it; a
   per-record-width floor would fix it. One cell, left as a known boundary case.
2. The split's materialisation pass costs ~2 ns per record. Count-first
   distribution with exact bucket borders would remove it (8-13 % at 1-10M) at
   the price of an in-place block permutation with cycle following.
3. u128: 12 passes at 11 bits is the best this shape does; the remaining gap to
   `ipnsort` is that a comparison sort moves 16 bytes once per pass in registers.
4. `sort_by` is not supported by either radix module; both reject it explicitly.
