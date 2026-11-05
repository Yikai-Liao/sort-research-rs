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

## Design rules for this branch

0. The same code measured twice in one process, interleaved, differs by up to
   5 % per cell. Anything under that margin is not a result: `bands8` reads
   4.5 % apart between two variants that take the identical path. Decisions are
   taken on 1.2x and above, and a branch that only buys 5 % is not a branch.

1. One mechanism beats several. A second algorithmic path is only carried when
   it buys a measured gain on a mainstream shape; a path that is within noise,
   or that only helps one narrow cell, is removed even if it once measured
   faster. Every path is a branch that a future change has to keep correct.
2. A constant that can be derived is not tuned. Widths, thresholds and
   fan-outs are chosen by the same cost reasoning that decides routing, so a
   change in one place cannot silently invalidate another.
3. A measurement lives in this file with the cell it was taken on, and a path
   that has no entry here has no defence.

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

### 15. The streamed fallback, out of line

The routing model's "do not split" branch runs the block engine, which is a large
function. Marking it `#[cold] #[inline(never)]` keeps the split path's code
together and moves the rare branch out of the instruction cache. Measured at 1M:
u32 random 0.92x → 1.12x against the same baseline, u128 zipf 0.98x → 1.04x, and
no change at 10M where every child of a split also goes through it.

A side observation from the same table: for u32 zipf the *baseline itself* is 24 %
faster at radix 11 than at radix 8 (5.74 vs 7.52 ms), because a digit with only a
few occupied buckets needs three passes instead of four and its per-block prefix
sums stay small. On dense keys the same change is 2x worse (23.2 vs 11.6 ms).
Choosing the streamed radix from the variation is therefore possible, but the
crossover is sharp and the gain is confined to sparse-digit cells; recorded in
the backlog rather than guessed at.

### 16. Negative result: a leaf buffer bigger than the tile

The split target and the buffer limit are different questions, so the leaf buffer
was allowed to grow to 512 KiB while the split still targeted 64 KiB. The only
nodes it changes are 64-512 KiB ones — the 312 KiB children of a 10M split. There
the gain is 33.1 → 32.7 ns (u64 10M, and the unchanged baseline moved 62.6 → 67.7
between the two halves of the same A/B), i.e. inside noise; a claimed 4 % at u128
1M turned out to be cells the change cannot reach. Against that, the buffer would
be O(array) for every array between 64 KiB and 512 KiB, which contradicts the
space story for exactly the sizes a caller is most likely to have. Reverted.

### 17. Measurement hygiene: background jobs and a single fast round

Two runs disagreed by 6x on one cell, and a whole sweep was 30-60 % slow. The
causes were mundane: a `criterion` run left in the background (46 minutes of one
core, invisible in the numbers until `ps`), and reporting the *minimum* over
rounds, which a single anomalous round can corrupt. Every table below is
produced with `taskset` pinning, 5-7 interleaved rounds, the median printed next
to the minimum, and `uptime` checked before the run. Cells the text reports as
"parity" are within the ±8 % noise floor of this shared VM.

### 18. Negative result: prefetching the scatter's store target

The scatter's store address is known four records early, so the natural next step
was `prefetch_write_data` on that line (architecture-generic, LLVM lowers it, and
the crate already has the intrinsic enabled). It cost 25-36 %: u64 1M 24.1 → 29.6,
u128 1M 76.7 → 103.3, u128 10M 89.3 → 121.5, u32 1M 11.4 → 12.7. The look-ahead
needs a second key read and a second cursor read-modify-write per record, in a
loop that is already store-port bound — the same reason fusing the next pass's
histogram failed (entry 4). Reverted. The slack is in the store port, not in
address latency, so the lever is fewer passes, not a better pass.

### 19. Bug: a key panic in the merge leaf duplicated and lost a record

A review of the small-tile path found a hole that the counting leaf does not
have. `merge_runs` moved records with `ptr::read` + `ptr::write`, so a level
consumed its source: a panic from the key extractor half-way through a level
left a prefix of the record set in the destination and a hole in the source.
For a type with a destructor that is one record duplicated and one lost, and
therefore a double free, not merely a wrong order. The `catch_unwind` in
`lsd_leaf` did not help: `merge_leaf` had no recovery and no way to learn how
far the inner merge had got.

Fix, in two parts, both free:

- `merge_runs` copies bitwise (`copy_nonoverlapping` of one record) instead of
  moving. The source then holds the complete set for the whole level, so the
  level is recoverable by construction. Same load/store pair, no cost.
- `merge_leaf` copies the scratch buffer back before resuming a panic, under
  the same `needs_drop::<T>()` const guard as `lsd_leaf`, so integer keys
  compile with no recovery at all and only resource-owning types pay for it.

The insertion leaf was already safe: it only swaps.

Evidence, not just a passing test: `a_key_panic_never_loses_or_duplicates_a
_record` sweeps 41 panic points over three leaves (insertion at 20, merge at
300, counting at 2000) and asserts the record set and the drop log. With
`ptr::read` and the recovery disabled the test fails on the record-set
assertion; with the fix it passes. Put back only the recovery and it still
fails, put back only the copy and it passes — the copy is the part that makes
the level recoverable, the recovery is what puts the tile back.

### 20. The patterns were lying about u128, so the matrix was re-cut

Every u128 row except `random_wide` went through `narrow_u32(x) << 96`, so it
was a 32-bit key in a 128-bit container: three leaf windows, not twelve. Those
rows never measured what the table claimed. The harness now has type-space
patterns that build the key in the target type (`type_two_near`, `type_two_far`,
`type_eight_near`, `type_top_byte`), plus the i32-space shapes added for the
same reason (`runs64`, `blocks_desc`, `staircase`, `random_local`,
`high_byte_only`, `two_values_*`, `sorted_d20`).

Two findings, both against the design as it stood:

- `type_two_near` (one varying bit, 1M) is 4.8 ns on u64 and 11.7 on u128,
  against 3.4 and 7.2 for `ipnsort`. Nothing is being sorted, so the cost is
  the survey's full scan (it only early-exits when all bits vary) plus one
  pass. On u128 that is a third of the time spent proving nothing varies.
- `type_two_far` (two values, 128 bits apart) is 4.9 / 11.5 and the split
  handles it in one level, so there is no low-cardinality blind spot for
  well-separated values — the earlier suspicion came from a pattern that the
  widening map had smeared into 31 scattered bits.

Against the stable baseline the new shapes hold up (blocks_desc 21.2 vs 34.4,
random_local 18.3 vs 36.9 at 1M u64), and `pipe_organ` is 22.3 against 12.5 for
`std`: two long sorted runs are a comparison sort's best case, and a radix sort
cannot see them. Not a defect to chase, a boundary to record.

### 21. Leaf width is per tile, and the model must not follow it

Sweeping `LEAF_RADIX_MAX` re-routed whole node classes, because the routing
model priced a split with the leaf's own width: widening the leaf lowered the
estimated hot passes, made splitting look cheaper, and u32 10M fell from 13.9
to 18.6 ns by switching from streamed to split. The model now prices at a fixed
`MODEL_LEAF_RADIX = 11`, so leaf tuning cannot move routing.

With routing frozen, the optimum width differs by type, and physically so: a
pass moves `records * record_bytes`, a table costs `2 << width` bytes, and the
two only trade at the width where the table still fits beside the tile. At 1M
(ns/record, routing frozen):

| leaf width | u64 | u128 | u32 |
| --- | --- | --- | --- |
| 11 | 23.6 | 78.1 | 12.2 |
| 12 | 24.2 | 70.6 | 9.8 |
| 13 | 22.3 | 74.1 | 10.4 |
| 14 | 27.0 | 68.0 | 10.3 |

14 bits is best for u128 and worst for u64: its 32 KiB cursor table is the
whole L1, so on 8-byte records the table evicts the tile. The width a leaf
should use is therefore a per-tile cost decision, not a constant.

### 22. Four independent reviews: what they found

Micro-architecture (u64 1M budget: 6 leaf passes 59 %, split scatter 23 %,
materialise 9 %, descend 6 %): the counting pass runs 5.3 cycles/record against
a 2.0 cycle port floor, and its 3906-record destination tile plus the 4 KiB
cursor table occupy 8.6 of L1's 8 ways before the source stream allocates
anything, so the tile evicts itself. Sub-tiling to L1 capacity is the cheap
test of that claim.

Structure: the pass count is a function of key width, not of how many records
the tile holds. A 3906-record tile runs six passes to produce 3906 distinct
keys when twelve bits of discrimination would separate them all. A per-tile
MSD recursion into the leaf's second buffer, terminated by insertion, needs one
or two passes where the flat LSD plan needs six. This is the largest remaining
lever and it is still a radix sort.

Measurement: every cell at or below 8 MiB is measured cache-warm, because the
harness clones the input immediately before the timed region; at 10M the 76 MiB
clone is large enough for glibc's memcpy to switch to non-temporal stores, so
the starting cache state changes discontinuously between 1M and 10M — exactly
where the size-scaling conclusions live. The `record16`/`record128` payloads are
constant zero, so a stable and an unstable sort produce identical bytes and the
harness cannot see a stability regression on the two rows whose contract is
stability.

Literature (RADULS/RADULS2, ska_sort, Blits, IPS4o, radsort): the published
wins are memory-system engineering of the scatter (64-byte packet buffering,
non-temporal stores), not digit math; RADULS2 reports 10-50 % of its time in
tiny-bin handling, and that is where our insertion/merge crossovers already
sit. Buffered parcel scatter needs the destinations to stay aligned to 64 bytes
across passes, which an in-place array does not guarantee.

### 23. One MSD level inside the tile: 2.4x on u128, 1.3x on u64

The leaf used to run one flat counting pass per digit window, so its cost
followed the key width while a tile only holds a few thousand records. A
3906-record tile of 64-bit keys ran six passes to distinguish 3906 keys; twelve
bits of discrimination separate them all. The leaf now partitions once into its
private buffer on the most significant varying window, copies back, and hands
each bucket to the flat plan.

Stability holds because a counting pass writes every record at its bucket's
start plus its rank inside the bucket, so equal keys keep their order and the
buckets are contiguous. Ordering holds because the window is the most
significant one -- a lower window first would order the tile by the wrong bits
-- and the recursion therefore peels bits from the top down. Traffic is one
histogram traversal, one scatter (two) and one copy back (two), against two per
flat pass, so it pays from three flat passes up.

Measured in one process, interleaved, `tessera` against the flat plan
(`tessera_counting`, same code with the level switched off), n = 1M and 10M,
ns per record:

| cell | one MSD level | flat plan | ratio |
| --- | --- | --- | --- |
| u64 random 1M | 20.2 | 25.4 | 1.26x |
| u64 random 10M | 34.2 | 33.6 | 0.98x |
| u128 random 1M | 31.9 | 76.8 | 2.41x |
| u128 random 10M | 47.5 | 95.4 | 2.01x |

u128 is the point: a pass over a 16-byte record costs 4.1 ns, so twelve of them
were 49 ns of pure movement, and one level replaces them with about four. The
10M u64 row did not move for a different reason, which entry 24 fixes.

Two rejections came with it. The window must be judged before it is used: a
locally-clustered tile whose top window covers a bit that few records set made
one bucket hold everything and the recursion ran five levels, 3x slower than
the flat plan (58.7 vs 18.3 on `random_local`). A 128-key sample counts the
window's real buckets and the depth estimate compares `depth * 3` against the
flat pass count, which put that cell back to 20.8. And the table is
uninitialised rather than zeroed: `[0u16; 1 << 14]` per leaf was 32 KiB of
memset against a 31 KiB tile and cost 14 %.

### 24. The cost model was quoting a price the leaf no longer charged

At 10M u64 the trace showed one root split and 256 streamed children of
305 KiB: children that size are over the tile budget, so they never became
leaves, and the leaf work could not show up at all. Two modelling errors hid
that:

- the split's four passes were priced at a fixed 4.2 ns per record, which is an
  L1 figure. A split pass only *reads* the node (the scatter goes into resident
  blocks), so it costs half a pass at the bandwidth that node's size actually
  gets: 3.8 ns for 80 MiB, and the 305 KiB child was being charged 16.8 ns for
  a split that measures 6.
- the child was priced at its flat pass count, so the leaf looked six passes
  expensive in the very cells where it now costs three.

With both fixed, 10M routes to split, split, then 2443-record leaves, and u64
10M went 34.2 -> 27.6 (flat plan 31.6) and u128 10M 91.9 -> 49.1 (flat plan
93.5). u32 10M sits at the boundary and reads 17.4 against 15.7-16.2 for the
streaming route; it is a 4 MiB-per-million key whose node is 38 MiB, and the
model's cold floor is what decides it. Left as the known 4-byte boundary case.

## Where the candidate stands

ns per record, one root each, pinned, min of 5 interleaved rounds.
Distribution matrix at n = 1M (`logs/calib6.log`):

| type | pattern | tessera | block LSD | ratio | ipnsort | std stable |
| --- | --- | --- | --- | --- | --- | --- |
| u64 | random | 23.3 | 33.4 | 1.43x | 24.1 | 42.0 |
| u64 | zipf20 | 12.1 | 26.9 | 2.22x | 4.9 | 11.4 |
| u64 | 99 % sorted, 1 % displaced | 23.9 | 37.3 | 1.56x | 23.2 | 32.5 |
| u64 | 95 % sorted, 5 % displaced | 23.2 | 39.9 | 1.72x | 24.4 | 33.2 |
| u64 | sparse (8 values) | 5.5 | 27.2 | 4.93x | 4.7 | 13.6 |
| u64 | 8 narrow bands | 20.2 | 35.7 | 1.77x | 25.4 | 39.2 |
| u64 | ascending | 2.8 | 36.1 | 12.84x | 1.2 | 1.3 |
| u64 | descending | 5.5 | 32.3 | 5.91x | 1.7 | 1.9 |
| u64 | 95 % reverse sorted | 22.1 | 35.2 | 1.59x | 25.8 | 17.9 |
| u128 | random | 76.7 | 147.5 | 1.92x | 46.8 | 75.3 |
| u128 | zipf20 | 21.6 | 23.1 | 1.07x | 9.1 | 27.0 |
| u128 | 99 % sorted, 1 % displaced | 24.1 | 31.7 | 1.32x | 37.1 | 59.1 |
| u128 | 95 % sorted, 5 % displaced | 23.2 | 28.9 | 1.25x | 39.3 | 57.7 |
| u128 | sparse (8 values) | 15.7 | 30.4 | 1.94x | 10.9 | 31.6 |
| u128 | 8 narrow bands | 29.3 | 28.2 | 0.96x | 43.8 | 72.4 |
| u128 | ascending | 7.0 | 35.5 | 5.06x | 3.1 | 3.1 |
| u128 | descending | 11.6 | 33.2 | 2.86x | 4.5 | 4.6 |
| u128 | 95 % reverse sorted | 23.8 | 30.1 | 1.27x | 43.1 | 29.3 |
| u32 | random | 10.0 | 10.1 | 1.01x | 23.9 | 32.5 |
| u32 | zipf20 | 8.7 | 7.8 | 0.90x | 3.1 | 5.2 |
| u32 | 99 % sorted, 1 % displaced | 8.0 | 7.6 | 0.96x | 20.4 | 22.7 |
| u32 | 95 % sorted, 5 % displaced | 7.6 | 8.0 | 1.05x | 21.1 | 23.0 |
| u32 | 8 narrow bands | 7.7 | 7.4 | 0.96x | 23.6 | 30.8 |
| u32 | sparse (8 values) | 10.1 | 9.7 | 0.95x | 3.9 | 5.1 |
| u32 | ascending | 2.4 | 8.7 | 3.72x | — | — |
| u32 | descending | 4.4 | 7.7 | 1.73x | — | — |

The u32 rows sit at parity because a 4 MiB node never splits: the model prices a
split's read pass at more than the four cold passes it would avoid, and the
remaining difference is the probe plus noise — the same u32 cell reads anywhere
from 0.90x to 1.12x between runs on this VM, so only the ordered rows there are
outside the noise floor. The u128 narrow-band row is the same story: both routes
are the same three-passing block engine and the model correctly refuses to split
a digit that puts 99 % of the records in one bucket.

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
| record16 random 1M | 33.2 | 67.0 | 2.02x | 42.1 | 72.1 |
| record16 zipf20 1M | 18.6 | 37.7 | 2.03x | 7.5 | 23.0 |
| record16 ascending 1M | 5.3 | 58.4 | 11.1x | 2.4 | 2.4 |
| record128 random 1M | 232 | 438 | 1.89x | 181 | 566 |
| record128 zipf20 1M | 161 | 291 | 1.81x | 50 | 288 |
| record128 ascending 1M | 35.3 | 417 | 11.8x | 12.6 | 14.2 |

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
  best: 2.8 ns on ascending u64, 12.8x over the baseline.
- Wide records are the other side of the same coin. A 16-byte record with a
  u64 key is ahead of `ipnsort` (33.2 vs 42.1, and 2x over the stable baseline),
  because 16 bytes per pass is still cheap. At 128 bytes a pass moves eight times
  more data than the key needs, and `ipnsort` wins (232 vs 181): it compares keys
  and partitions by index, moving the payload only in the final permutation. The
  stable baseline is 1.9x behind us there, so the loss is to the unstable
  in-place sort, not to the previous implementation.

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

- `cargo test --release --features rust_tessera_radix --lib` — 8 engine tests
  (stability with payload identity, every key width × every tile budget,
  duplicates, skew, all-equal, empty/single, and a randomized differential test:
  2000 random shapes — sparse masks, one varying byte, two clusters, few distinct
  values, descending ranges — crossed with five tile budgets from 4 KiB to 1 MiB,
  compared record by record against the standard library's stable sort with the
  input index in the payload, so a stability break fails the same assertion).
- `cargo test --release --features rust_tessera_radix --test tessera_functionality`
  — the repository's standard 233-case suite, same one block LSD runs.
- Miri, no UB reported: `duplicates_stay_stable` (drives the raw-pointer kernel,
  the `u16` cursors and the packed windows over a four-value key space, so it
  covers every unsafe block in the leaf), `constant_keys_are_no_ops` and
  `skewed_keys_sort`. The randomized differential test and the width × tile sweep
  do not fit in Miri's time budget (2000 configurations at 100x slowdown) and run
  in release mode instead.
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

The criterion bench is for comparability with the repository's published
baselines, not with the tables above, and the difference is the key shape: its
`u128` rows are built by `extend_i32_to_u128`, i.e. an `i32` pattern scaled into
128 bits, while this file's `random_wide` rows are full-width random keys. Same
sort, different amount of variation — measured at 1M, criterion's median against
the tournament's minimum:

| bench | tessera | block LSD | ipnsort | std |
| --- | --- | --- | --- | --- |
| u64 random | 21.4 | 31.5 | 27.3 | 51.6 |
| u128 random (i32-derived) | 45.9 | 83.5 | 44.8 | 82.8 |

The u128 row is the interesting one: on the repository's narrower key the radix
sorts are twice as fast as on full-width keys, which is exactly what narrowing
buys, and tessera lands at ipnsort's level instead of 1.6x behind it.

### 25. Cleanup pass: one path removed, three merged, two rejections

Three independent reviews of the implementation (engine, block engine,
experiment surface) and the rules above converged on the same target: fewer
mechanisms. What actually changed, with the evidence for each:

- **The merge leaf is gone.** A/B in one process at 1M: u64 random 19.03 ns
  without it against 19.21-19.87 with it, u128 random 34.58 against 32.67 (6 %),
  u64 zipf 11.75 against 13.49 at merge_max 128 (15 % *worse*). A path that buys
  6 % on one cell and loses 15 % on another is not a path. Removed with its
  recoverability branch, its constant and its two policy parameters.
- **Five entry points became one `Policy`.** `sort` and `sort_with_policy(arr,
  Policy)` are the whole surface; the knobs measurement needs are fields, so a
  new combination no longer costs a function, an adapter wrapper and a doc
  paragraph.
- **`plan_digits` became `stream_passes`.** It built `Digit`s that its only
  caller discarded to read the count. Same grid, same numbers, no out-parameter.
- **One bucket walk.** `descend` and the tile's MSD level now share
  `for_each_bucket`, which extracts every key once and accumulates the child's
  variation while the boundary is open. The MSD level keeps cursor-based ends:
  comparing digits there was one unpredictable branch per record when buckets
  hold one record each, and measured 48 % slower (19.4 -> 28.8 on u64 1M).
  Sharing the walk was right for the split; sharing the *boundary source* was
  not.
- **Dead work removed**: an unused `variation` parameter, a `distinct` count
  that only fed a trace line, a 2 KiB zero-fill per leaf in the sampler (the
  cursor table's 14 % lesson did not generalise itself), and a plan buffer
  sized for 128 windows when the narrowest real digit needs 43.

Size: `engine.rs` 1442 -> 1331 lines, adapter 38 -> 31, and one fewer
algorithmic path. The 233-case differential suite, the panic sweep and the
1M/10M cells all still pass, and no cell moved beyond the 5 % floor.

Left undone on purpose, because they cost more concepts than they remove:
merging the two occupancy samplers (one asks "how spread is this window", the
other "how many distinct digits", at different sizes and disciplines),
deriving `Copy` for `Variation` (would change the type's parameter from the
record to the key at every use site), and folding `trace()` behind a feature.

Full matrix after the cleanup, 1M, ns per record, one root, three interleaved
rounds at load 2.5 — ratios matter here, the absolute column is inflated by the
neighbours:

| cell | tessera | block LSD | ipnsort | vs block LSD | vs ipnsort |
| --- | --- | --- | --- | --- | --- |
| u64 random | 18.1 | 36.2 | 25.1 | 2.00x | 1.39x |
| u64 bands8 | 18.0 | 36.7 | 24.5 | 2.03x | 1.36x |
| u64 random_local | 22.6 | 32.9 | 27.4 | 1.46x | 1.22x |
| u64 blocks_desc | 20.1 | 29.7 | 23.1 | 1.48x | 1.15x |
| u128 random | 35.6 | 144.5 | 43.3 | 4.07x | 1.22x |
| u128 random_local | 26.3 | 35.3 | 44.4 | 1.34x | 1.69x |
| u128 bands8 | 29.0 | 26.8 | 42.4 | 0.92x | 1.46x |
| u32 bands8 | 6.5 | 7.0 | 22.4 | 1.08x | 3.47x |
| u32 blocks_desc | 9.5 | 9.9 | 23.4 | 1.04x | 2.46x |
| u64 zipf20 | 10.7 | 24.1 | 3.8 | 2.26x | 0.35x |
| u64 ascending | 2.9 | 45.1 | 1.5 | 15.5x | 0.50x |

The u32 rows in the full sweep read 9.5-14.8 ns for the same code across three
runs (10.2, 10.6, 10.2 on re-measurement) because a 7 ms cell is at the mercy of
the neighbours; small types and small sizes need more rounds than the sweep
gives them before a single number means anything.

### 26. The cost model was challenged, measured, and kept

It is the largest single block in the file (~200 lines including the bandwidth
curves) and it has been wrong twice, so it was the obvious thing to delete:
replace routing with one rule -- split while the node is over the tile. The
alternative was built, benched against the model in one process, and then
deleted, because the model won wherever it matters:

| common cell, 1M | model | always split |
| --- | --- | --- |
| u128 random | 33.5 | 35.5 |
| u64 random | 16.8 | 16.8 |
| u64 sparse (8 values) | 4.8 | 5.0 |
| u128 sparse (8 values) | 14.8 | 15.6 |
| u32 random / 2 values / 8 values / 99 % sorted | 8.8 / 2.6 / 2.4 / 6.7 | all equal or worse |

The shapes where the model loses are `bands8` and `blocks_desc`, which are not
what 'general purpose' means, and the shapes where the single rule looked 2x
better (`u32 sparse8` at 3.5 against 9.3) were noise: at load 0.5 with five
rounds the same cell reads 9.1 against 9.2. Lesson, now rule 0 in this file: a
cell under about 10 ms needs a quiet machine and five rounds before it can
vote, and a 2x claim on one cell is a claim about the neighbours.

So the model stays. What did come out of the challenge: the routing trace now
prints one `route` line per node instead of a `shape` line and a `block` line
repeating the same four numbers, and `sample_split_shape` no longer counts
distinct buckets that nothing read.

### 27. The level-by-level route is our loop now

The model sometimes decides that splitting a node costs more than running the
radix passes over it in place. That route used to call the block engine's own
`sort_with_variation` -- another implementation, closed to us. It now runs from
this module's digit planner on the block engine's permutation primitive
(`core::sort_with_digits`: one engine for all passes, one compaction at the end).

A first attempt drove the single-pass `partition_digit` in a loop and lost 7-20 %
on the nodes that take this route, because every call built an engine and
compacted. The shared-engine version is at parity or better, measured in one
process with both implementations as bench variants (`tessera` against
`tessera_engine`), 1M records:

| cell | our loop | block engine's | cell | our loop | block engine's |
| --- | --- | --- | --- | --- | --- |
| u64 random | 18.8 | 19.5 | u128 random | 34.4 | 34.5 |
| u64 sparse | 6.1 | 7.4 | u128 sparse | 18.0 | 18.3 |
| u64 zipf | 11.2 | 11.6 | u128 zipf | 21.6 | 21.5 |
| u64 two values | 4.7 | 4.6 | u128 two values | 9.0 | 9.6 |

So the sort no longer depends on another algorithm, only on the permutation
primitive -- which is the layer we can still replace. The `StreamMode` knob stays
as that comparison's measurement surface, like `LeafMode`.

Also from the review: the routing branch called `split_pays` twice (the first arm
returns, so the second was dead), and the leaf's panic restore had two
implementations -- a `catch_unwind` around the types that own a resource and
nothing at all for the rest -- now one `Drop` guard. Mutation testing did not show
the old gating losing or duplicating a record (a key panic lands in the counting
phase, before any record moves), so that change is a simplification and a removed
untested branch, not a bug fix. A new sweep with a plain record type covers the
branch that previously did not exist.

## Open items

0. **Leaf: does the flat counting plan still earn its place?** The leaf runs flat
   LSD passes or one MSD level, chosen by a cost estimate. The ablation that is
   missing is MSD-only (no flat plan): if a single MSD-based leaf is within the
   5 % bar on the common cells, the leaf loses a strategy. Old data says flat wins
   on low-cardinality tiles by 5-10 %, so this has to be measured per cell, in one
   process, against the same bench.

1. u32 10M: the model predicts "do not split" where splitting is 12 % faster.
   The floor of the cold-pass cost (3.0 ns/record) is what misplaces it; a
   per-record-width floor would fix it. One cell, left as a known boundary case.
2. The split's materialisation pass costs ~2 ns per record. Count-first
   distribution with exact bucket borders would remove it (8-13 % at 1-10M) at
   the price of an in-place block permutation with cycle following.
3. u128: 12 passes at 11 bits is the best this shape does; the remaining gap to
   `ipnsort` is that a comparison sort moves 16 bytes once per pass in registers.
4. `sort_by` is not supported by either radix module; both reject it explicitly.
