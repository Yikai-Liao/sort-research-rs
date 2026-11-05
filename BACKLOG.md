# Brainstorm backlog: status of all 16 ideas

Source: the external brainstorm for `948d486`. Every row is either done (with the
section that proves it), measured negative (with the number that killed it), or open
with the reason it has not been taken. Nothing here is "planned" without a verdict.

| idea | status | evidence |
| --- | --- | --- |
| 01 single-digit terminal | **done**, decision-neutral | s56: trace identical on 10 cells, instructions +-0.6%, deletes stream_node and sort_with_digits. Form B (fan-out = span) rejected: the block engine sizes its working set from the fan-out. |
| 02-A node passes varying mask only | open | mechanical but touches 6 sites; Variation::varying() is its only consumer, so this is readability, not work -- and it cannot move a number, which is the bar for every remaining change. |
| 02-B integer cost condition for a hybrid leaf | **moot** | the hybrid cost model is gone (s53): the leaf decides from passes > MSD_LEVEL_PASSES and distinct > 1. |
| 03 sampling that cannot pass | **done** | s55: widths >= 10 skipped; trace identical on 8 cells, instructions +-0.03%. |
| 04 export bucket boundaries | measured at the bar | r6 puts it at ~1.5 ns/record; its variant that folds the aggregates into the permutation was measured slower (s50, +10-16% instructions), and the cursor table this notebook did measure was 48% faster only inside a tile. Open, expected < 5%. |
| 05 reuse one sample across widths, estimate max share | open | the estimate it would feed no longer exists (s53). Only worth it if a share-based decision comes back. |
| 06 reuse the block workspace across nodes | open | plausible for many mid-sized children; the split allocates per node today. Not measured. |
| 07 block size / layout sweep | **done**, negative | s57: RADIX_BLOCK_SCALE over 0.5x-4x spans 1.0-2.1% of instructions, no step near the bar. Fan-out sweeps (s21) and leaf width (s48) were already done. |
| 08 real MSD-only ablation | **done** | s53: MSD-only is within noise of auto on 14/15 cells and catastrophically better than flat-only on wide records (u128 random 82.1 against 31.2). |
| 09 indirect tile sort for wide records | open | a specialised path; the tile is the only place this design touches wide payloads, so it is the one place a new structure could pay. |
| 10 block-boundary mask instead of end pointers | open | hot-loop metadata; unmeasured. |
| 11 dense bucket numbering for sparse digits | open | pays only when many buckets are empty, which the widening sampler now avoids by construction. |
| 12 compact the varying bits | open | equivalent to renumbering keys; would change the digit plan for every node. |
| 13 fuse compact with leaf consumption | rejected | s50 is its cheapest form and it lost. |
| 14 ordered equal-bucket path for few wide values | open | zipf and few-value cells are where a comparison sort beats us; a separate prototype, not a routing change. |
| 15 insertion with cached key + one rotation | open | small experiment, no measurement yet. |
| 16 probe order before the variation scan | **already the case** | probe_order runs first; the variation scan follows it, so the cheap detector already runs before the expensive one. |

## What the open rows have in common

Every one of them changes *how* work is organised, not *how much* work there is: they
delete a few instructions per record (10, 02-A), change an allocation policy (06, 07), or
add a second structure (09, 14). The measured ledger is 4 kept and 5 rejected out of 9
attempted mechanisms, and the two independent reviews put the remaining headroom at or
below the 5% bar (r5, r6). The ceiling report says what that means: on this machine the
design is at its floor for random, sorted and few-value data, and its real gap is heavy
duplicates, where the answer is a comparison-based partition rather than another radix
refinement.
