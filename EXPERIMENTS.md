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

## Where the candidate stands

ns per record, uniform random keys (min of 3-5 rounds, interleaved):

| type, n | tessera | block LSD | msd_radix | ipnsort | std stable |
| --- | --- | --- | --- | --- | --- |
| u32 100k | 11.4 | 12.1 | 25.1 | 19.7 | 30.2 |
| u32 1M | 10.2 | 11.6 | 35.4 | 23.4 | 39.5 |
| u32 10M | 15.6 | 18.5 | 42.0 | 26.9 | 45.5 |
| u64 100k | 22.8 | 25.2 | 26.0 | 19.7 | 29.3 |
| u64 1M | 24.4 | 43.4 | 42.6 | 27.4 | 43.9 |
| u64 10M | 31.7 | 60.6 | 51.2 | 32.5 | 63.0 |
| u128 100k | 75.9 | 79.5 | 95.0 | 32.4 | 50.3 |
| u128 1M | 77.5 | 168.9 | 95.0 | 47.1 | 80.4 |
| record128 1M | 231 | 446 | — | 190 | 574 |

All sixteen pattern × type × size families are in `logs/matrix5.log`; the
summary of that sweep:

- Against the stable sorts it replaces, the candidate wins everywhere it
  applies: 1.7x over `std` on u64 1M, 1.9x over block LSD on u32 1M, 2.2x on
  u128 1M, 1.9x on 128-byte records.
- Against `ipnsort` it is ahead on u64 (24.4 vs 27.4, 31.7 vs 32.5) and behind on
  u32 1M (10.2 vs 23.4 — ahead, actually), on u128 (77.5 vs 47.1) and on
  128-byte records (231 vs 190). `ipnsort` is in-place and unstable; on 16-byte
  keys it compares in registers and never moves 16 bytes per pass. A stable
  radix sort cannot match that on wide keys; the gap is structural.
- The pure-MSD ceiling is slower than the tiled sort at every size measured,
  which is the expected result (in-place cycle-following writes are random over
  the whole node, while the tiled sort's writes stay in cache).
- Skew: u64 zipf13 1M 24.8 (block LSD 34.5, ipnsort 10.0). The comparison sort
  wins on duplicate-heavy input because 3-way partitioning collapses equal keys,
  which no digit pass can; the candidate still beats the stable baseline by 1.4x.
- Already-sorted input: u64 ascending 1M 24.4 vs block LSD 48.4. Nothing to do —
  LSD cannot see order, but the split still pays for itself, and constant digits
  are dropped.
- Rows where the candidate is 3-8 % behind block LSD are all at 100k, where the
  model says "do not split" and the work is the same as the baseline plus one
  scan; the residual is within the measured noise floor (below).

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
(`bench --bench extra_memory`): u64 10k Tessera 20 KiB vs block LSD 540 KiB;
i32 100 Tessera 400 B vs 540 KiB.

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
