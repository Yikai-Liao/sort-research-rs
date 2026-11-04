# Experiment backlog

Ideas not yet measured, and what happened to the ones that were. Keep this file
current: an idea that is dropped must say why, and an idea that moves into the
code must move out of here with its measured result in EXPERIMENTS.md.

Status legend: **open** (not tried), **running** (code exists, measuring),
**done** (decided, result recorded), **dropped** (rejected, reason given).

## Open

| # | idea | expected | notes |
| --- | --- | --- | --- |
| 1 | Fuse the split's materialisation pass with the first leaf pass: read the block scratch in bucket order and scatter straight into the tile | −2 ns/record per split level (8 % at 1M, 13 % at 10M) | Needs the block engine to hand out per-bucket block spans. The first leaf pass would read from 1 KiB blocks instead of a contiguous run, which is fine for the prefetcher. Highest structural payoff left. |
| 2 | Accumulate each bucket's union/intersection inside the split's scatter loop, so `descend`'s key scan disappears | −1.4 ns/record per split (4-7 %) | Costs 2 register ops per record in a memory-bound loop, so probably net positive. Design cost: the block engine would learn a caller-specific statistic. Compare with keeping `descend` and shrinking it. |
| 3 | `u16` cursors in `counting_pass` (tiles are far below 2^16 records) | few % from halved cursor traffic; also frees room for 12-bit digits at the same 8 KiB table | Untried. Watch for the prefix sum needing a wider accumulator. |
| 4 | 12-bit leaf digits *with* 16-bit cursors | u128 12 → 11 passes | Plain 12-bit with 32-bit cursors measured 1-2 % worse; this variant keeps the table at 8 KiB. |
| 5 | Prefetch the destination line for the next record in the scatter | unknown | `core::intrinsics::prefetch_write_data` is architecture-generic (LLVM lowers it), so it does not violate the "no x86 intrinsics" rule. MCA says the loop is memory bound at ~6 cycles/record against a 2-cycle store bound, so there is something to hide. |
| 6 | Let a leaf take a node up to N× the tile budget, growing its buffer, instead of streaming it cold | ~3 % at 10M | Measured: at 10M the 300 KiB children are streamed cold (21.6 ns) while an L2-resident leaf costs 17-21. Needs the model to separate "buffer limit" from "split target". The 512 KiB tile experiment already brackets this. |
| 7 | Choose each pass's digit *offset* to maximise varying-bit density instead of laying digits on a fixed grid from the top | one pass less on sparse keys | Relevant to "few values, far apart" inputs: digits on a fixed grid can straddle two clusters and pay two passes for what one could cover. Untried. |
| 8 | Second-level split for the 300 KiB children at 10M | ~1 ns/record | The model currently streams them; measured both routes are within 1 ns, so this is a coin flip. Revisit with idea 1 in place, which changes the split's cost. |
| 9 | Natural-run detection in the merge leaf (galloping / skip already-ordered pairs) | helps 95 %-sorted tiles | `merge_runs` already skips a pair whose runs meet in order; a full TimSort-style run stack would be a different algorithm in the leaf. Only matters for tiles ≤ 384 records. |
| 10 | Sorting-network leaf below the insertion crossover | tiny | The repository has `sort4_stable_orson`; insertion at ≤ 48 records is 0.5-1 % of a 1M sort, so the ceiling is small. |
| 11 | `#[cold]` on the block-LSD fallback call | tiny | Free; do it if touching that code. |

## Done

| # | idea | result |
| --- | --- | --- |
| A | One MSD split then cache-resident LSD leaves (the Tessera shape) | 1.8-2.2x over the pre-session baseline at 1M-10M. |
| B | Per-node variation filter (drop constant digits) | Skew and duplicate-heavy inputs stop paying no-op passes; u128 100k 168 → 76 at 1M. |
| C | Narrow each digit to the span that varies (so a split's fan-out follows the bits that differ) | Fixed the skew regressions at 100k (1.25 → 0.85 on u32 zipf); 2-4 bucket splits instead of 256. |
| D | Cost-model routing (split vs stream) in ns per record | Replaced the byte threshold; 8 of 9 measured boundary cases match. |
| E | Leaf-chosen digit width, 8 → 11 bits | u128 1M 95 → 77 (−19 %); u64 1M 25.2 → 24.4. |
| F | Single variation scan shared with the block engine (`sort_with_variation`) | Removed half of the small-n regressions against the baseline. |
| G | 32-bit cursors and raw pointers in `counting_pass` | u64 1M 27.5 → 25.4, u32 10M 16.7 → 15.6. |
| H | Fuse the next digit's histogram into the scatter | **Dropped**: 16.6 → 20.9 (u32 1M). Store-port bound loop; `llvm-mca` shows port 4 saturated. |
| I | Insertion leaf ≤ 48 records, merge leaf ≤ 384 | Skew tails and thin buckets stop paying a 256-entry table; value confirmed at 100k (u64 21.8 vs 25.7) and in the u32 10M corner. |
| J | Tile budget sweep 16 KiB … 512 KiB | **64 KiB stays**: 16-24 KiB pays an extra split level (1.8x worse), 128-512 KiB loses 3-10 % at 1M and only ties at 10M. |
| K | 12-bit leaf digits (32-bit cursors) | **Dropped**: 1-2 % worse on u128, no change on u64. |
| L | Sorted / reverse-sorted detection in the variation scan | **Done**: the scan already walks every key, so the two order flags ride along for free. Ascending returns immediately; descending reverses and repairs equal-key groups. See EXPERIMENTS.md. |
