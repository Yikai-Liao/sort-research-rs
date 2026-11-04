# Stable block LSD radix sort

Single-threaded stable LSD radix sort that reuses consumed input blocks and tracks
their logical order with permutation metadata. It preserves stability without a
full-size output array. Proposed in
[issue #35](https://github.com/Voultapher/sort-research-rs/issues/35).

Feature: `rust_block_lsd`. Benchmark name: `rust_block_lsd_stable_radix`.

## Source

`core.rs` is a Rust adaptation of Robert Clausecker's
[`radixsort_permuted.c`, revision f69e816c3cd79d312cd67aea5b9cf1c338c1b371](https://github.com/clausecker/radsort/blob/f69e816c3cd79d312cd67aea5b9cf1c338c1b371/radixsort_permuted.c)
(BSD-2-Clause, notice retained in `core.rs`), described in
[Parallel O(sqrt(n)) Overhead LSD Radix Sort](https://arxiv.org/abs/2607.05302)
by Robert Clausecker and Florian Schintke. This is unrelated to the `radsort`
crate behind the existing `rust_radsort` adapter.

## Differences from the reference

- Block size is chosen from input length `n`, record size `s` and `R = 2^radix`
  buckets: `max(1, ceil(4 * sqrt(9*n / (2*R*s))), floor(1024/s))` records. The
  factor 4 and the ~1 KiB floor are experimentally tuned for 8-bit digits; they are
  not from the paper and not proven optimal. Auxiliary space is O(sqrt(n)) for
  fixed radix and record size.
- Keys of any width up to 128 bits, 1 to 8 bits per digit, partial final digit.
  The harness adapter fixes 8 bits.
- Digits that cannot change the order are skipped (union/intersection pre-scan).
  A constant input returns without scattering.
- Small records are scattered in groups of up to four, larger ones one at a time.
- Scratch is `MaybeUninit<T>` with explicit initialized-prefix tracking, so no
  `Copy`, `Clone`, `Default` or zero-initialization is required.

There is no comparison-sort or conventional LSD fallback: every nonconstant input
uses the block backend.

## Keys and unsupported operations

`radix_support::Key` maps a record to a fixed unsigned key (up to 128 bits) that
must agree with `Ord`, including equality. Equal keys keep their input order.
Implemented for integers, `bool`, `char`, `()`, `F128`, `FFIOneKibiByte`,
`Cell<T>` and pairs whose combined key fits in 128 bits. Benchmark wrappers for
floats and padded `u64` records implement it in `benches/bench.rs`.

Unsupported, and explicit about it (no hidden fallback): variable-length keys such
as `String`, and `sort_by`. Benchmark registration skips unsupported types via
`supports_type`. If key extraction panics, all records are restored and the panic
resumes; the resulting order is unspecified.

## Tests

```sh
cargo test --release --no-default-features --features rust_block_lsd
```

- `tests/block_lsd_functionality.rs`: the upstream functionality tests that apply
  to fixed-width keys.
- `tests/block_lsd.rs`: stability with payloads, block tails, all integer widths,
  unsupported-type behavior; also runs the backend's own unit tests.
- `tests/radix_owned.rs`, `tests/radix_views.rs`: records that are neither `Copy`
  nor `Clone`, drop counts, and key panics during different phases.

Miri, ASan and benchmark results are not included yet.
