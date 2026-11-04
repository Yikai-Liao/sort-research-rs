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
`supports_type`. If key extraction panics the panic resumes and the order is
unspecified. The sorter restores the slice to a permutation of the original
records for every type, including those without destructors. Such records can
contain exclusive references and must not be duplicated.

## Tests

```sh
cargo test --release --no-default-features --features rust_block_lsd
```

- `tests/block_lsd_functionality.rs`: the standard `instantiate_sort_tests!`
  suite: 233 registered tests, of which 79 enter the test body and 154 return
  early because of capability declarations or existing skip rules.
- `tests/block_lsd.rs`: stability with payloads, block tails, all integer widths,
  float total order, unsupported-type behavior; also runs the backend's own unit
  tests over configurable digit widths and explicit block sizes (8 tests).
- `tests/radix_owned.rs`, `tests/radix_views.rs`: records that are neither `Copy`
  nor `Clone`, per-ID drop counts, and key panics during different phases
  (2 + 7 tests). The no-destructor regression owns `&mut u64` references and
  checks the original panic payload, record IDs, and writes through each reference.

The standard suite assumes a sort accepts every `Ord` type and implements
`sort_by`. This one takes a fixed key and rejects everything else, so
`sort_test_tools::Sort` gained two declarations:

```rust
fn supports_type<T>() -> bool where T: Ord { true }
fn supports_sort_by() -> bool { true }
```

`radix_adapter!` reports `<T as Key>::SUPPORTED` and `false` respectively, and the
affected tests return early instead of failing on the deliberate rejection. The
types without a key conversion, and the comparison based tests, are therefore
skipped rather than unchecked-by-accident; the explicit rejection itself is
tested in `tests/block_lsd.rs`.

### ASan

```sh
CARGO_TARGET_DIR=target-asan RUSTFLAGS=-Zsanitizer=address \
  cargo test --target x86_64-unknown-linux-gnu \
  --no-default-features --features rust_block_lsd
```

The earlier recorded ASan run predates the recovery correction and the new
regression. It is historical evidence, not validation of this candidate. Of the
233 registered Block LSD standard tests, 79 exercise test bodies; 154 return
early. The current candidate has not been compiled or run during closeout.

### Miri

```sh
cargo miri test --no-default-features --features rust_block_lsd \
  --test block_lsd --test radix_owned --test radix_views
```

Earlier logs record 8 + 2 + 6 passing tests for these binaries, before the
no-destructor regression was added. That coverage missed records containing
exclusive references without a destructor. The recovery correction and the new
regression have not been run under Miri during closeout; the current expected
binary counts are 8 + 2 + 7.

Miri interprets every instruction and every 8-bit pass writes
`2 * 256 * block_size` bucket slots, so the shapes are reduced, not the code
paths:

- Sizes stop at one block plus one instead of 16 385 records; the payload test
  runs the 112 byte payload only, which is the one below the ~1 KiB block floor.
- `all_integer_widths` sorts only `u128`, the widest key; smaller records get a
  larger block from the ~1 KiB floor, which multiplies the bucket-slot work.
- The float total-order test uses an explicit small block size instead of the
  automatic one.
- Each panic target is a separate full sort. `radix_views` keeps three of the ten
  targets and drops the skewed key layout.

The 79 functionality tests are not run under Miri: at these block sizes a single
sort takes minutes there. They belong in the release and ASan runs above; rerunning those configurations
on the corrected candidate remains pending.

Timing comparisons against `rust_std` and `rust_radsort` use the normal
`cargo bench` harness. Peak extra heap is measured separately by
`cargo bench --bench extra_memory`, which needs `rust_block_lsd` and
`rust_radsort`.
