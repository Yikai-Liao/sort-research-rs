Thanks for welcoming a PR. I have prepared a vendored implementation on
[feat/block-lsd](https://github.com/Yikai-Liao/sort-research-rs/tree/feat/block-lsd),
following your preference for vendoring. The current candidate is
`22463e9ef3e59a917afe8fcd1ed1a52ca123d9b7`.

The implementation reuses consumed input blocks and keeps permutation metadata,
with O(sqrt(n)) auxiliary space for fixed radix and record size. Compared with
Clausecker's C reference, it adds the adaptive block-size policy described in
this issue, generic converted keys through 128 bits, constant-digit skipping,
grouped scatter for small records, and Rust ownership/panic handling. The
BSD-2-Clause source notice and reference revision are retained.

The adapter uses the `rust_block_lsd` feature and the benchmark name
`rust_block_lsd_stable_radix`. It accepts supported fixed converted keys and
rejects unsupported types and `sort_by`. I added `supports_type` and
`supports_sort_by` declarations to the standard test interface, defaulting to
true so existing implementations keep their behavior. For this adapter,
233 tests register, 79 enter test bodies, and 154 return early because of these
capabilities or existing skip rules. The capability declarations and the separate
extra-memory benchmark can be split if that helps review. The nightly ipnsort
`abort` compatibility fix is already a separate commit, `70557f3`.

During closeout I corrected a safety bug in my implementation: `needs_drop` is
not a valid boundary for panic recovery, because a type without a destructor
can contain exclusive references. Every moved record now updates the recovery
offset, and recovery restores the complete original record set before resuming
the key panic. The regression uses records containing `&mut u64`, with no Drop
implementation. I also corrected a local panic test that had checked a fresh
clone instead of the recovered slice; it now checks the recovered IDs/payloads
and one drop per original ID. These were bugs in this contribution and its
added tests, not demonstrated defects in the upstream suite.

[Historical artifacts](https://github.com/Yikai-Liao/sort-research-rs/tree/block-lsd-artifacts)
show why the memory/runtime trade-off is useful to study. In the old random
runs, most medium/large cases beat the existing radsort adapter, but hot u64 at
100k was tied (2.35 vs 2.34 ms). Comparison sorts still won several cases;
128-byte records at 1M took approximately 0.81x/0.92x std's cold/hot time.
The historical peak-extra-heap run used 832,767 bytes at 1M u64 records, versus
8,000,000 for radsort and std stable. These are pre-closeout measurements, not
measurements of the corrected candidate. Individual run commit IDs were not
recorded, so the numbers must be refreshed before presenting final results.

The archive preserves the original 106-entry JSON and regenerates every HTML,
PNG and text summary from it. Eighty-two random entries match the clean logs'
aggregate estimates. The other 24 pattern entries are saved-baseline-only;
`pattern-bench.log` is a later run, not their original measurement log.

This closeout was static only: no compilation, test/Miri/ASan execution or new
benchmarks. Earlier passing tests missed the no-destructor case. Before posting
this reply and opening the PR, I still need release/ownership validation and a
small representative remeasurement on the exact corrected commit. No broader
performance search is planned.
