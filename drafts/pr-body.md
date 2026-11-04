Implements #35 with a vendored stable block LSD radix sort.

The sort reuses consumed input blocks and tracks their logical order rather than
allocating a full second copy of the input. Auxiliary space is O(sqrt(n)) for
fixed radix and record size. This evaluates the runtime/memory trade-off of block
reuse against the repository's conventional radix and comparison sorts.

## Implementation and scope

- `src/other/rust_block_lsd/core.rs`: Rust adaptation of Clausecker's
  [reference C implementation at f69e816](https://github.com/clausecker/radsort/blob/f69e816c3cd79d312cd67aea5b9cf1c338c1b371/radixsort_permuted.c),
  with its BSD-2-Clause notice retained, based on Clausecker and Schintke's
  [paper](https://arxiv.org/abs/2607.05302).
- `rust_block_lsd` feature; harness name `rust_block_lsd_stable_radix`.
- Fixed unsigned converted keys through 128 bits. Integer conversions and
  supported harness wrappers preserve their `Ord`; equal keys remain stable.
  Custom payloads need neither `Copy` nor `Clone`. Floating-point benchmark
  wrappers implement their respective ordering conversions.
- Unsupported key types and `sort_by` are rejected explicitly.
  `sort_test_tools::Sort` gains `supports_type` and `supports_sort_by`, both
  defaulting to true for existing implementations. These declarations let this
  adapter enter the standard suite with explicit skips.
- `benches/extra_memory.rs` measures peak extra requested heap separately.
  The ipnsort nightly compatibility fix remains a separate commit, `70557f3`.

The implementation is vendored following the maintainer's preference in
[issue #35](https://github.com/Voultapher/sort-research-rs/issues/35).

## Changes from the reference

The block size is
`max(1, ceil(4 * sqrt(9*n / (2*R*s))), floor(1024/s))`, with `R = 2^radix`
and `s = sizeof(T)`. The factor 4 and approximately 1 KiB floor are local tuning
choices for 8-bit digits, not paper requirements or proven optima. Fixed
512-record blocks bound scratch only: their permutation metadata still grows
with n. The adaptive policy bounds both components by O(sqrt(n)). The internal
backend also accepts an explicit block size for experiments and tests.

A union/intersection pre-scan skips constant digits. It stops after 1025 keys
only when every key bit already varies; otherwise it scans the whole input.
The harness's converted u64 inputs have constant high bits and take the full
scan. Ascending benchmark values are converted as
`(i + 2^31) * (2^31 - 1)`; at 100k and 1M all eight bytes vary, so these cases
still take eight digit passes.

The backend supports 1–8-bit digits and partial final digits, uses grouped
scatter for small records, and tracks live prefixes in `MaybeUninit<T>` scratch.
There is no comparison-sort fallback.

If a key extractor panics, the original panic resumes after all original
records are restored to the slice in unspecified order. Commit `22463e9`
removes an unsound `needs_drop` shortcut: types without destructors can contain
`&mut` references and also require exact recovery. The tests now inspect the
actual recovered IDs/payloads before dropping the slice, count drops per ID,
and include the no-destructor exclusive-borrow regression.

## Evidence and validation status

Current candidate: `22463e9ef3e59a917afe8fcd1ed1a52ca123d9b7`.
This closeout performed static review, Rust formatting and `git diff --check`.
It did **not** compile or execute the corrected candidate, run tests/Miri/ASan,
or collect new timing or memory measurements.

Earlier release/ASan/Miri logs concern the pre-closeout implementation. They
missed the exclusive-borrow case and cannot establish safety of this candidate.
The standard suite registers 233 tests for Block LSD: 79 enter their test
bodies and 154 return early under capability declarations or existing skip rules.
Additional current binaries register 8 + 2 + 7 tests; those counts are not a
claim that the corrected tests have passed.

[The separate artifacts branch](https://github.com/Yikai-Liao/sort-research-rs/tree/block-lsd-artifacts)
preserves 106 historical JSON entries. Individual measurement commit IDs were
not recorded. Its tables and HTML/PNG plots were regenerated from that JSON,
without rerunning benchmarks. Of those entries, 82 random cases match the
aggregate estimates in the clean logs. The 24 pattern entries are saved
baseline estimates whose original measurement logs have not been recovered;
they are excluded from the proposed reply's numeric comparisons.

The historical results motivate the research; they do not measure `22463e9`:

- Most medium/large random cases were faster than the existing radsort adapter.
  Hot u64 at 100k was effectively tied: 2.35 vs 2.34 ms.
- At 1M, 128-byte records took about 0.81x/0.92x std's time in cold/hot runs;
  at 100k they were near parity. The approximately 0.38x ratio belongs to f64.
- At 10k, outcomes depend on type: u64/u128 favored comparison sorts, while
  f64 and wide records provide counterexamples. Timing data below 10k is absent.
- The historical memory run at 1M u64 records requested 832,767 extra bytes for
  block LSD and 8,000,000 for radsort and std stable. At 1000 u64 records,
  block LSD's fixed minimum scratch cost was 539,199 bytes.

Both hot and cold use batch-generated inputs. Cold additionally disturbs CPU
prediction state; the harness deliberately avoids measuring main-memory load
cost. Hot does not mean repeatedly sorting a single reused input buffer.

## Remaining validation before publication

- [ ] Release suite on the corrected candidate, with skips reported separately.
- [ ] Targeted Miri for `panic_preserves_exclusive_borrows_without_drop`, then the
      documented ownership binaries; ASan as an additional safety check.
- [ ] Remeasure the few representative timings used in the reply and record the
      exact candidate commit and environment.

These items were deliberately deferred because this closeout was requested
without compilation or execution. This body remains a draft until they are done.
