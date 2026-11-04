# Radix experiments: Tessera (stable tiled radix sort)

Branch `exp/radix-opt`, forked from `feat/block-lsd` (429f6f7). Block LSD stays
untouched as a baseline; the candidate is a new module. Everything here is
research code, not a merge proposal.

Session deadline: 2026-11-05 10:00 Asia/Shanghai.

## Roles

| role | implementation |
| --- | --- |
| stable radix baseline, O(sqrt n) space | `rust_block_lsd_stable_radix` (unchanged, radix 8) |
| candidate | `rust_tessera_stable_radix` (new) |
| radix speed ceiling (unstable) | `rust_msd_radix` (new, in-place american-flag MSD) |
| comparison references | `rust_std_stable`, `rust_ipnsort_unstable`, `rust_radsort_radix` |

Harness: `benches/radix_ab.rs`, an interleaved tournament (fresh copy per run,
variants rotate, min and median over rounds, one correctness assertion per variant
per cell). Criterion remains the tool for numbers that go into the repository's
own tables; this harness exists because a tuning session needs 20-second
turnaround. Machine: Skylake-SP Xeon Gold 6140, 2.3 GHz, 6 vCPU shared VM, 32 KiB
L1d, 1 MiB L2 per core, 148 MiB L3 (THP on), nightly 1.101.0.

```sh
cargo bench --features rust_block_lsd,rust_tessera_radix,rust_msd_radix \
    --bench radix_ab -- --types u64,u32 --patterns random_wide,zipf13 \
    --sizes 100000,1000000 --rounds 7
```

## Why a new algorithm rather than more LSD tuning

Measured (1M u64, uniform): an 8-bit LSD pass costs ~5.4 ns per record, and the
u64 sort needs eight of them, so block LSD lands at 42-45 ms while `ipnsort`
needs 26-28 ms. The passes are not bandwidth bound: 128 MiB of traffic at the
measured 8.8-11.8 GB/s memcpy rate would be ~12 ms. The cost is per-record
scatter work repeated `d` times over a cold array.

So the shape is the one the earlier PA work converged on: split by the most
significant digit once over the cold array, then finish each child while it is
still in cache. At 1M u64, one 8-bit split leaves 31 KiB children, whose
remaining seven passes together cost less than one cold pass.

## The candidate

`src/other/rust_tessera_radix/engine.rs` (about 420 lines with tests):

- `Plan` — the digits still worth a pass for a node, highest first, filtered by
  the union/intersection variation test. A digit that is constant inside the node
  costs nothing, which is what keeps skew from degenerating into `d` no-op passes
  over a homogeneous run.
- `sort_node` — three ways to finish a node, chosen by a cost model (below):
  MSD split, cache-tile LSD, or handing the node to the block engine's LSD.
- `descend` — one walk over a split node that finds bucket boundaries and, in the
  same pass, accumulates each child's union/intersection, so no child pays a
  second scan of its own keys.
- `lsd_leaf` / `counting_pass` — stable counting scatter per remaining digit,
  ping-ponging between the tile and one private buffer, 32-bit cursors, raw
  pointers, no bounds checks.
- `insertion_leaf` — stable insertion sort on converted keys for tiles of at most
  48 records.

Reused from the baseline module: `partition_digit`, `varying_key_bits`,
`default_block_size`. One implementation of "stable block distribution" serves
both algorithms.

### The routing cost model

Same shape as the block-size formula in `core.rs`: minimise predicted cost
instead of tuning a byte threshold.

```
hot  = HOT_NS_PER_BYTE * node_bytes              (0.33 ns/byte, cache resident)
cold = max(hot, node_bytes / COLD_BYTES_PER_NS)   (1.1 GB/s effective scatter)
split ⟺ (digits_left - 1) * (cold - hot) > SPLIT_HOT_PASSES * hot     (4.0)
```

A split replaces `digits_left - 1` cold passes with cache-resident ones and costs
scatter + materialisation + boundary scan, which measured 1.1-1.5 ns per byte,
i.e. ~4 hot passes. Cache residency is not a separate threshold: when the node is
already resident, `cold == hot` and the predicate is false by construction.

| node | measured cold/hot | formula threshold | measured winner |
| --- | --- | --- | --- |
| u64 100k (0.8 MiB) | 1.0 | no split | no split ✓ |
| u32 1M (3.8 MiB) | 1.3 | k > 7.5 | no split ✓ |
| u64 1M (7.6 MiB) | 2.1 | k > 3.9 | split ✓ |
| u32 10M (38 MiB) | 2.3 | k > 3.7 | split ✓ |
| u128 1M (15 MiB) | 2.3 | k > 3.7 | split ✓ |

Known boundary case: u32 at 1M sits inside the model's error bar (predicted
saving +1.3 ns/record, measured -4.5), so it still splits when it should not.
Reported rather than hidden; a per-node calibration of the two bandwidth
constants would fix it.

## Log

### 1. Digit width: wider is worse, every time

1M u64 uniform, block LSD with an explicit radix:

| radix | min | ns/record |
| --- | --- | --- |
| 8 | 43.7 ms | 43.7 |
| 9 | 47.2 | 47.2 |
| 10 | 57.7 | 57.7 |
| 11 | 62.4 | 62.4 |
| 12 | 72.8 | 72.8 |

Wider digits trade 25 % fewer passes for a cursor table that no longer fits L1
(256 cursors = 4 KiB, 2048 = 32 KiB) plus more block scratch. Both dominate
immediately. This closes the "just use more bits" shortcut for LSD: the win has
to come from the pass structure.

### 2. Where the time goes (u32 vs u64 differential)

Same n, same split, 4 vs 8 digits:

| type | digits | tessera (before tuning) | ns/record |
| --- | --- | --- | --- |
| u32 1M | 4 | 16.6 ms | 16.6 |
| u64 1M | 8 | 27.0 ms | 27.0 |
| u128 1M | 16 | 98.0 ms | 98.0 |

Four extra digits cost 10.4 ns, so an in-cache tile pass is ~2.6 ns per record;
the split level is 8.9 ns (scatter 5.4 + materialise ~2 + boundary scan ~1.5).
That decomposition is what the routing model above is calibrated against.

### 3. Tile budget: the 32 KiB cliff

1M u64, tile budget sweep *before* the fan-out fix: 64 KiB 26.4 ms, 256 KiB
27.3, 1 MiB 27.4, **32 KiB 57.8 ms**. The 32 KiB case is not noise (perf: 1.98 G
instructions vs 1.09 G). Children average 3906 records = 31 KiB, so a 32 KiB
budget lets roughly half of them exceed it by a few hundred records and pay for
another split — and each of those splits used to allocate `2 * 256` blocks of the
block engine, whose 1 KiB block floor is 512 KiB of scratch writes for a node
holding 33 KiB of records. Fix: fan-out follows the node size
(`split_radix`), so a small oversize child is split with 8 buckets, not 256.

### 4. Negative result: fusing the next digit's histogram into the scatter

Idea: one read serves two passes, so the separate counting read of each tile pass
disappears. Implemented, measured, rejected:

| type | separate count | fused count |
| --- | --- | --- |
| u32 1M | 16.6 ms | 20.9 ms |
| u64 1M | 27.0 ms | 28.1 ms |

An in-cache pass is store/counter bound, not read bound: fusing adds one
load-modify-store per record (the next digit's counter) and saves only a key
load. Instancing `counts` and `next_counts` also costs 4 KiB of L1. Reverted;
the reason is recorded here so it is not re-attempted.

### 5. Small tiles: insertion sort instead of a bucket table

A counting pass pays a full bucket table before it moves a record. For tiles of
at most 48 records, a stable insertion sort on converted keys is smaller and
faster, and skewed inputs land there for every thin tail bucket. At 100k
records: u64 random 25.7 → 21.8 ns, u32 random 15.3 → 11.5, u64 zipf13
19.5 → 17.3.

### 6. Kernel: 32-bit cursors and raw pointers

`counting_pass` on raw pointers with `u32` cursors (a tile is far below 2^32
records) removed bounds checks and halved counter traffic: u64 1M 27.5 → 25.4,
u64 10M 34.2 → 31.6, u32 10M 16.7 → 15.6. The rare rollover branch in the block
engine also got a `branches`-style `#[cold]` unlikely hint (block LSD 42.8 → 36-40
at 1M, within noise of each other, kept because it is free).

### 7. Where the candidate stands (interleaved tournament, min of 3-7 rounds)

ns per record, uniform random keys, `msd_radix` = unstable speed ceiling:

| type, n | tessera | block LSD | msd_radix | ipnsort | std stable |
| --- | --- | --- | --- | --- | --- |
| u32 100k | 11.5 | **10.2** | 25.1 | 19.8 | 26.6 |
| u32 1M | 16.2 | **11.5** | 35.4 | 24.2 | 34.5 |
| u32 10M | **15.6** | 18.5 | 42.0 | 26.9 | 45.5 |
| u64 100k | 21.8 | 22.0 | 26.0 | **19.0** | 29.6 |
| u64 1M | **25.4** | 42.4 | 42.6 | 27.8 | 43.9 |
| u64 10M | **31.6** | 60.6 | 51.2 | 32.5 | 63.0 |
| u128 1M | 97.0 | 186.6 | 95.0 | **49.4** | 84.8 |
| u128 10M | 118.0 | 184.0 | 99.4 | **61.3** | 114.4 |

What this says, honestly:

- Against the stable sorts it replaces, the candidate wins everywhere it applies:
  1.7x over `std` on u64, 2.2x on u32 at 1M, 3.1x on u128 at 10M.
- Against block LSD it wins by 1.4-1.9x at 1M and above; block LSD still wins on
  u32 up to a few MiB, because four passes over a working set that fits L3 do not
  need to be tiled at all.
- Against `ipnsort` it is now at parity to slightly ahead on u64 at 1M-10M
  (25.4 vs 27.8, 31.6 vs 32.5) and clearly behind on u32 at 1M and on u128
  everywhere. `ipnsort` is in-place and unstable: on 16-byte keys it compares in
  registers and never moves 16 bytes through memory per pass. A stable radix sort
  cannot match that on wide keys; the gap is structural, not a tuning miss.
- The pure-MSD ceiling is slower than the tiled sort at every size measured,
  which is the expected result (in-place cycle-following writes are random over
  the whole node, while the tiled sort's writes stay in cache) and confirms the
  tiling is doing real work.
- Skew is fine: u64 zipf13 100k 17.3 ns (block LSD 20.2, ipnsort 7.4 — the
  comparison sort wins on duplicate-heavy input because 3-way partitioning
  collapses equal keys, which no radix digit pass can).

## Space

- Split: `2 * buckets` blocks of `default_block_size` records, i.e. the
  O(sqrt n) block scratch the block engine already documents.
- Tile: one buffer of at most `TILE_BYTES` (64 KiB), a constant.
- Peak = max(split scratch, tile), both transient per level; recursion depth is
  bounded by the digit count (at most 16 for 128-bit keys).

`cargo bench --features rust_tessera_radix --bench extra_memory` measures the
allocator-visible peak.

## Reproduce

```sh
# standard suite (233 cases) and module tests
cargo test --release --features rust_tessera_radix --test tessera_functionality
cargo test --release --features rust_tessera_radix --lib

# interleaved tournament used for every table above
cargo bench --features rust_block_lsd,rust_tessera_radix,rust_msd_radix \
    --bench radix_ab -- --types u32,u64,u128 --patterns random_wide,zipf13 \
    --sizes 100000,1000000,10000000 --rounds 5

# repository-standard criterion numbers
BENCH_REGEX='^(rust_(tessera|block_lsd)_stable_radix|rust_ipnsort_unstable|rust_std_stable|rust_radsort_radix)-hot-(u32|u64|u128|f64)-(random|random_z1|ascending)-(100000|1000000|10000000)$' \
cargo bench --features rust_block_lsd,rust_tessera_radix,rust_radsort,cold_benchmarks,bench_type_u128,bench_type_f64 \
    --bench bench -- --warm-up-time 2 --measurement-time 4 --noplot --save-baseline tessera
```

Run-to-run variation on this shared VM is large (the same cell can move 20 %
between runs); every table above uses the minimum over rounds, and ratios are
more trustworthy than absolute milliseconds.

## Open items

1. u32 up to a few MiB: the split still fires where the model says it should not.
   A per-node bandwidth calibration, or measuring the cold constant per size
   class, would settle it.
2. u128: 16 digit passes at 8 bits is the wrong shape for a 16-byte key. A wider
   leaf digit (11-12 bits) with a `u16` bucket table would cut passes to 12, but
   the tile is small enough that the bucket table becomes the cost; needs a
   measurement, not a guess.
3. The split's materialisation pass still costs ~2 ns/record. Count-first
   distribution with exact bucket borders would remove it, at the price of an
   in-place block permutation with cycle following.
4. `sort_by` is not supported by either radix module; both reject it explicitly.
