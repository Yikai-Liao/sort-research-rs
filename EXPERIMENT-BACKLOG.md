# Experiment backlog

Ideas not yet measured, and what happened to the ones that were. Keep this file
current: an idea that is dropped must say why, and an idea that moves into the
code must move out of here with its measured result in EXPERIMENTS.md.

Status legend: **open** (not tried), **running** (code exists, measuring),
**done** (decided, result recorded), **dropped** (rejected, reason given).

## Open

| # | idea | expected | notes |
| --- | --- | --- | --- |
| 1 | Fuse the split's materialisation pass with the first leaf pass: read the block scratch in bucket order and scatter straight into the tile | −2 ns/record per split level (8 % at 1M, 13 % at 10M) | Still the largest structural item. Needs the block engine to hand out per-bucket block spans; the first leaf pass would read 1 KiB blocks instead of a contiguous run. Packed windows (EXPERIMENTS.md 13) removed the *other* half of what this was meant to buy (passes for scattered varying bits), so measure again before building it. |
| 2 | Accumulate each bucket's union/intersection inside the split's scatter loop, so the child scan disappears | −1.4 ns/record per split (4-7 %) | Costs 2 register ops per record in a memory-bound loop. Design cost: the block engine would learn a caller-specific statistic. Compare with keeping the scan and shrinking it. |
| 4 | 12-bit leaf digits *with* 16-bit cursors | u128 12 → 11 passes | 16-bit cursors landed (EXPERIMENTS.md 11); plain 12-bit digits measured 1-2 % worse, so retry only with the table kept at 8 KiB. |
| 5 | Prefetch the destination line for the next record in the scatter | unknown | `core::intrinsics::prefetch_write_data` is architecture-generic (LLVM lowers it), so it does not violate the "no x86 intrinsics" rule. MCA put the loop at ~6 cycles/record against a 2-cycle store bound. |
| 6 | Let a leaf take a node up to N× the tile budget, growing its buffer, instead of streaming it cold | ~3 % at 10M | **Tried, reverted** (EXPERIMENTS.md 16): 33.1 → 32.7 ns at u64 10M, inside noise, while the buffer becomes O(array) for arrays of 64-512 KiB. Revisit only together with a raised split target. |
| 8 | Second-level split for the 300 KiB children at 10M | ~1 ns/record | The model streams them; measured both routes are within 1 ns. Revisit with idea 1, which changes the split's cost. |
| 9 | Natural-run detection in the merge leaf (galloping / skip already-ordered pairs) | helps 95 %-sorted tiles | `merge_runs` already skips a pair whose runs meet in order; a full run stack would be a different algorithm inside the leaf. Only matters for tiles ≤ 384 records. |
| 10 | Sorting-network leaf below the insertion crossover | tiny | The repository has `sort4_stable_orson`; insertion at ≤ 48 records is 0.5-1 % of a 1M sort. |
| 11 | ~~`#[cold]` on the block-LSD fallback call~~ | **done** | Out of line: u32 random 1M 0.92x → 1.12x, u128 zipf 0.98x → 1.04x, nothing at 10M. (EXPERIMENTS.md 17) |
| 13 | Choose the *streamed* radix from the variation: 11 bits when a digit's buckets are sparse, 8 bits when they are dense | +24 % on u32 zipf, 0 elsewhere | **Tried, reverted**: the gate ("varying bits ≤ radix") almost never fires on real inputs — u32 zipf has 11-12 varying bits, just over the edge — so nothing moved (u32 zipf 1M 7.37 vs 7.59, u64 10M random unchanged) and the code would have carried an untested block-engine configuration. The baseline's own 24 % win at radix 11 on that cell stands; reaching it would need a finer gate than bit count. |
| 12 | Detect "few distinct values" and sort their *equivalence classes* instead of their bits | large on 8-distinct rows | 8 distinct u128 values need 3 windows because they have 31 varying bits; a comparison sort needs 8 comparisons. Detecting the class count in one pass costs a table; the payoff is real only when the count is tiny, and the detection is exactly the hard part. |

## Done

| # | idea | result |
| --- | --- | --- |
| A | One MSD split then cache-resident LSD leaves (the Tessera shape) | 1.8-2.2x over the pre-session baseline at 1M-10M. |
| B | Per-node variation filter (drop constant digits) | Skew and duplicate-heavy inputs stop paying no-op passes; u128 100k 168 → 76 at 1M. |
| C | Narrow each digit to the span that varies | Fixed the skew regressions at 100k (1.25 → 0.85 on u32 zipf); 2-4 bucket splits instead of 256. |
| D | Cost-model routing (split vs stream) in ns per record | Replaced the byte threshold; every boundary case measured since then matches. |
| E | Leaf-chosen digit width, 8 → 11 bits | u128 1M 95 → 77 (−19 %); u64 1M 25.2 → 24.4. |
| F | Single variation scan shared with the block engine (`sort_with_variation`) | Removed half of the small-n regressions against the baseline. |
| G | 32-bit cursors and raw pointers in `counting_pass` | u64 1M 27.5 → 25.4, u32 10M 16.7 → 15.6. |
| H | Fuse the next digit's histogram into the scatter | **Dropped**: 16.6 → 20.9 (u32 1M). Store-port bound loop; `llvm-mca` shows port 4 saturated. |
| I | Insertion leaf ≤ 48 records, merge leaf ≤ 384 | Skew tails and thin buckets stop paying a 256-entry table. |
| J | Tile budget sweep 16 KiB … 512 KiB | **64 KiB stays**: 16-24 KiB pays an extra split level (1.8x worse), 128-512 KiB loses 3-10 % at 1M and only ties at 10M. |
| K | 12-bit leaf digits (32-bit cursors) | **Dropped**: 1-2 % worse on u128, no change on u64. |
| L | Sorted / reverse-sorted detection | **Done, but not in the variation scan**: fusing the flags into that scan cost 5 % on clustered input (it defeats vectorisation). A separate strided-gated probe costs two comparisons on random data and one read on ascending. Ascending returns at once, descending is reversed and its equal-key runs repaired. u64 ascending 46 → 2.9 ns. |
| M | `u16` bucket cursors (tiles are ≤ 65535 records) | Few per cent on wide keys, and 2048 bytes less leaf stack. Tile budget now capped at `MAX_TILE_RECORDS` to keep the invariant. |
| N | Price a split by the child it creates, sampled in runs of 32 | Fixed both u128 zipf regressions (10M 37.2 → 24.2) and the u64 1M "no split by a hair" (43.4 → 24.9). |
| O | Packed digit windows instead of a fixed grid (`plan_digits_packed`) | u128 8 narrow bands 37.0 → 31.3, u128 8-distinct 18.8 → 16.0, u128 99 % sorted 26.1 → 24.4; dense random unchanged by construction. Supersedes most of the expected payoff of idea 7. |
| P | Wider split fan-out (9-11 bits) so children land on the tile | **Dropped**: u64 10M 35.4 → 37.9 → 41.4 as the fan-out widens; 2048 open streams cost more than L1-resident leaves save (EXPERIMENTS.md 14). |
