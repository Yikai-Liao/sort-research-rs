# Archived full PA adapter

`rust_pa` registers `rust_pa_unified_groups_stable_radix` in the repository's
Criterion harness. It vendors the full `UnifiedGroups` PA policy from local
voracious_sort commit `2b15e461842343721599b38c25883adbc96fabe3`, path
`experiments/block_msd/src/frontier`. This is the original PA used as the baseline
in `experiments/pa_simd_original`, with no explicit SIMD candidate enabled.

`source.json` records original/integration hashes for all eight modules and the
mechanical adaptations: conversion-trait imports, scoped feature names, a test
import, omission of the archived top-level test driver, and rustfmt. A conversion-only
bridge replaces the original Dispatcher-coupled traits; PA policy, classifiers,
scatter, permutation, compaction, and recursion are unchanged. The MIT source
license is retained here and the BSD-2-Clause block notice remains in core/block.rs.
The optional `hakmem = 0.2.0` dependency is retained for archived projection-policy
code; the selected UnifiedGroups policy does not enable a packed projection.

## Policy and storage

- Root block length: `max(1, floor(1024/S), ceil(4*sqrt(9*N/(512*S))))`.
- Radix is 256; recursive nodes retain the root block parameter.
- Node prefixes/PrefixBand labels are distributed stably, then unresolved runs
  recurse. Terminal/equal-key handling, grouped scatter and ordered-input checks
  are retained.
- Nodes of at most 128 records use the policy's original stable `sort_by_key`
  leaf. This is a hybrid full PA measurement, not a pure radix-only baseline.
- Dense admission compares the full scratch/histogram requirement against
  `512 * B * S`. The `16 MiB` local-workspace threshold is a cache-aware policy
  tuning parameter, not permission to allocate a 16 MiB/full-array buffer at
  every node. Dense/block storage is released before recursion into children.

## Supported interface

The adapter is deliberately closed to known nonpanicking numeric conversions:
signed/unsigned 8/16/32/64/128-bit integers, bool, char, unit, and
`sort_test_tools::ffi_types::RadixRecord<N>` (a Copy numeric-key/payload fixture).
It follows the archived PA's Copy requirement. Owning payloads and the existing
non-Copy padded/F128 wrappers are not adapted through a layout cast or a backup
array. They are omitted by `supports_type`; explicit unsupported calls reject
before mutation. Arbitrary sort_by is also rejected.

All wide-record timing and extra-memory controls use exactly the same
`RadixRecord<16>` / `RadixRecord<112>` types, of size 32/128 bytes. The key is u64;
identity and payload bytes are preserved and equal keys retain input order.
This is separate from the previous non-Copy wrapper timing series.

## Validation and diagnostics

`tests/pa.rs` checks primitive extremes and widths, leaf/block boundaries,
full-record stability, seven distributions, larger block-path inputs, capability
checks and rejection without mutation. With `rust_pa_work`, an additional test
confirms actual dense and block routes. The ordinary timed binary has no work
counters. `rust_pa_work` is only for separate diagnostics; extra_memory emits
PA_WORK summaries outside the measurement interval in that build.

```sh
RUSTC_BOOTSTRAP=1 cargo test --no-default-features --features rust_pa_work --test pa
RUSTC_BOOTSTRAP=1 cargo test --release --no-default-features --features rust_pa_work --test pa
```

The repository requires nightly features; the local toolchain uses Arch Rust
1.99.1 with RUSTC_BOOTSTRAP=1. No sanitizer was run for this integration.
