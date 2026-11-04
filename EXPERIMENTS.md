# Radix experiments: Tessera (stable tiled radix sort)

Branch `exp/radix-opt`, forked from `feat/block-lsd` (429f6f7). Everything here is
experiment scaffolding plus a candidate implementation; block LSD stays untouched
as the stable baseline it already is.

Deadline for this session: 2026-11-05 10:00 Asia/Shanghai.

## What is being measured, and against what

| role | implementation |
| --- | --- |
| stable radix baseline, O(sqrt n) space | `rust_block_lsd_stable_radix` (unchanged, radix 8) |
| candidate | `rust_tessera_stable_radix` (new, this branch) |
| radix speed ceiling | pure MSD radix, ordinary/unstable (added in this branch) |
| comparison-sort reference | `rust_std_stable`, `rust_ipnsort_unstable`, `rust_radsort` |

Harness: `benches/radix_ab.rs`, an interleaved tournament (min and median over
rounds, variants rotate, input cloned outside the timed region, one correctness
assertion per variant per cell). Criterion stays the tool for the numbers that go
into the repository's own tables; this harness exists because a tuning session
needs a 20-second turnaround instead of a 20-minute one.

```sh
cargo bench --features rust_block_lsd,rust_tessera_radix --bench radix_ab -- \
    --types u64,u32 --patterns random,zipf13 --sizes 100000,1000000 --rounds 7
```

Machine: Skylake-SP Xeon Gold 6140, 2.3 GHz, 6 vCPU shared VM, 6 MiB L2 per core,
148 MiB L3, nightly 1.101.0 (2026-10-08). Absolute milliseconds are noisy on this
host; read ratios.

## Why a new algorithm instead of more LSD tuning

Measured (1M u64, uniform keys, this branch): an 8-bit LSD pass costs about
**5.5 ns per record**, and a full sort needs eight of them, so block LSD lands at
42 ms while `ipnsort` needs 26 ms. The passes are not bandwidth bound: 16 passes
of 8 MiB is 128 MiB of traffic, which this machine can move in far less than 42 ms.
The cost is the per-record scatter work repeated `d` times over a cold array.

The fix is the shape the earlier PA work converged on: split by the most
significant digit once over the cold array, then finish each child while it is
still in cache. After one 8-bit MSD split of 1M records the children are ~31 KiB,
so their remaining seven passes together cost less than one cold pass.

## Design of the candidate

`src/other/rust_tessera_radix/`:

- `engine.rs`: the algorithm.
  - `Plan`: the digits still worth a pass for a node, highest first, filtered by
    the union/intersection variation test. A digit that is constant inside the
    node costs nothing. This is what keeps skew (a 95 %-zero head plus a thin
    tail) from degenerating into `d` no-op passes over a homogeneous run.
  - `sort_node`: MSD split with `block::partition_digit` while the node exceeds
    the tile budget; otherwise finish with `lsd_leaf`.
  - `descend`: one walk over the split node that finds bucket boundaries and,
    in the same pass, accumulates each child's union/intersection, so no child
    pays a second scan of its own keys.
  - `lsd_leaf`: stable counting scatter per remaining digit, ping-ponging between
    the tile and one private buffer. The buffer holds a complete copy of the
    record set before it becomes the source, so a panic in a caller's key
    extractor can always restore the tile; types without destructors skip that
    bookkeeping (same split as block LSD).
- Reuses the block engine's `partition_digit`, `varying_key_bits` and
  `default_block_size`. One implementation of "stable block distribution" serves
  both algorithms; block LSD keeps its own block-size policy and digit width.

Space: the block split keeps block LSD's O(sqrt n) scratch, and the leaf buffer is
bounded by the tile budget, i.e. a constant. Peak = max(split scratch, tile).

## Log

### 1. Baseline, and digit width does not help LSD

`1M u64 uniform`, `--rounds 5`, this branch, before any Tessera work:

| variant | min | ns/record |
| --- | --- | --- |
| block_lsd r8 | 43.7 ms | 43.7 |
| block_lsd r9 | 47.2 ms | 47.2 |
| block_lsd r10 | 57.7 ms | 57.7 |
| block_lsd r11 | 62.4 ms | 62.4 |
| block_lsd r12 | 72.8 ms | 72.8 |
| std stable | 42.7 ms | 42.7 |
| ipnsort | 26.5 ms | 26.5 |

Wider digits trade 25 % fewer passes for a bucket-cursor table that no longer
fits L1 (256 cursors = 4 KiB, 2048 = 32 KiB, 4096 = 64 KiB) and for more block
scratch. Both costs dominate immediately, so the digit width stays at 8. This
also closes the "just use more bits" shortcut for LSD; the win has to come from
the pass structure, not from fewer passes over a cold array.

### 2. First Tessera run: 1.6x over block LSD, parity with ipnsort

`1M u64 uniform`, `--rounds 5`:

| variant | min | ratio |
| --- | --- | --- |
| tessera (64 KiB tile) | 26.4 ms | 1.00 |
| tessera 256 KiB tile | 27.3 ms | 1.04 |
| tessera 1 MiB tile | 27.4 ms | 1.04 |
| ipnsort | 26.4 ms | 1.00 |
| std stable | 40.4 ms | 1.53 |
| block_lsd r8 | 42.2 ms | 1.60 |
| **tessera 32 KiB tile** | **57.8 ms** | **2.19** |

The 32 KiB case is not noise (perf: 1.98 G instructions vs 1.09 G for 256 KiB).
Diagnosis: with 1M/256 = 3906 records per child, a 32 KiB tile is just below the
child size, so roughly half the children exceed the budget by a few hundred
records and pay for another split. Each such split allocates `2 * 256` blocks of
the block engine, whose 1 KiB block floor is 512 KiB of scratch writes for a node
that holds 33 KiB of records. One order of magnitude of wasted traffic, 128 times.

That is the first real fix to make: the split fan-out has to adapt to the node
size, so a small oversize child is split with a small digit, not with 256 buckets.
