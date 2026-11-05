# Brainstorm round 2 (N1-N7): status against the current tree

Source: the second external brainstorm, written against `449cf36`. Same rule as the
first backlog: every item gets a verdict, and the verdict cites a measurement or names
the measurement that is missing.

| item | status | why |
| --- | --- | --- |
| N1 leaf only needs to know whether a second digit appears | **withdrawn** | the premise expired with s58: the depth guard consumes the full distinct count (largest /= distinct, level_cost = MSD_LEVEL_PASSES * depth), so a boolean cannot preserve the decision. The earlier justification in this file -- crediting the router's 0.0 % planner count (s51) to the leaf's sampler -- also compared the wrong object. Withdrawn; no measurement needed. |
| N2-A single-digit exit before width planning and sampling | **partly done** | s56 simplified the terminal itself (one permutation, no plan array, no second engine) and deleted stream_node/sort_with_digits, but the exit still sits after the width choice and the sample in sort_node. Moving it earlier is a control-flow tidy, not a performance item. |
| N2-B stop reading the experiment switch per node | **real, but small** | this file was wrong: RADIX_BLOCK_SCALE *is* read from the environment on every block-size computation (default_block_size). trace() is the flag that is a OnceLock. Hook cleanup, no performance claim. |
| N2-C locate each bucket's tail block in the prefix loop | open | block-engine change; would remove a pass over active blocks in finish_pass. Unmeasured, and r6 puts the whole descent at the 5 % bar. |
| N3 flat leaf builds every pass's histogram in one scan | open, but see the precedent | the flat plan currently reads the tile once per pass to count, then once more to scatter. One scan that fills histograms for all passes would cut the count half, which is ~25 % of a flat tile's traffic. Space: one histogram per pass at 2 bytes per bucket (a 13-bit window and 3 passes is 48 KiB) against a 31 KiB tile -- over the O(sqrt n) budget as written, so it needs narrower windows or per-pass u8 counts to fit. That constraint is the experiment. **Precedent against it**: s50 fused a second per-record update into the very same scatter loop to save a later pass, and it cost 10-16 % of instructions for a pass saved. N3 asks for exactly that trade again (two extra ops per record to save one read pass), so its expected value is low unless the histogram update is cheaper than s50's table RMW -- which is the first thing to measure. |
| N4 small batches: compute the digit, then stable scatter | open, same precedent | aimed at the scatter's per-record key extraction. s50 measured that adding even two read-modify-writes inside that loop costs 10-16 % of instructions, so the room here is small and the risk is the same shape. |
| N5 widening should buy discrimination | open | the split's widening (s49) stops at a quarter-occupied bar; N5 argues it should stop when the added width stops separating *this* node's keys. Cheap to measure with the existing hooks: compare the current bar against a rule that requires the next two bits to raise the bucket count by a factor. |
| N6 tile MSD output alternates between buffers to avoid the copy back | open | msd_leaf copies the scatter back into the tile (one traversal per level). Alternating would save it at the cost of a second tile buffer, i.e. doubling the constant in the O(sqrt n) budget -- allowed (it is still O(1) in n), but the tile is the only scratch the design has and this doubles it. Unmeasured. |
| N7 skip the whole permutation when the digit is already ordered | open | the ordering probe exists only at the top of the sort (s: probe_order). Doing it per node/digit needs a monotonicity test on the digit stream, which is one extra read per record to save a permutation -- it pays only on the ordered cells, which are already the cheapest. Low priority. |

## What round 2 agrees with, and where it is already answered

Its own section 1 says the same thing this notebook's s46-s47 measured: the split is at
the scatter floor and the leaf is at its pass floor, so the remaining items are about
removing traversals (N3, N6) or about making a cheap gate cheaper (N1, N2-A). Of the
seven, one is done, one was already true, and five need a measurement this machine
cannot currently resolve at the 5 % level (s56, s59, s60 all show 10-20 % noise on
identical work). The ones that would survive that noise are the traversal removals,
because they change bytes moved rather than instructions -- N3 is the strongest of them
and its blocker is the histogram memory, which is a design decision rather than a
measurement.


## Corrections from the round-2 revision (2026-11-05)

The revised brainstorm re-read the tree at \`cd406f8\` and corrected this file in three
places, all of which are accepted above:

1. **N1 is withdrawn.** Its premise was "the leaf only needs a boolean", which s58
   invalidated by making the guard depend on the full distinct count. The
   justification I wrote for it borrowed s51's router measurement, which is 0.0 % for a
   different call site; that comparison was wrong.
2. **N2-B is real, not absent.** \`RADIX_BLOCK_SCALE\` is read per block-size computation.
   The OnceLock I checked is \`trace()\`.
3. **N2-A is partial, not done.** The terminal is simplified; the exit is still after the
   width choice and the sample.

Two further points accepted without change:

- s58's two comparisons (27.10 vs 23.46, then 24.30 vs 24.28) must be quoted as two
  separate paired measurements, not combined into one precise cross-build percentage.
- The retraction of N3 as a high priority is consistent with s61's conclusion: at these
  sizes, mechanisms that add working-set pressure per record do not pay regardless of
  their instruction count.
