# Stable radix sort adapters

The single-threaded Block LSD implementation is vendored for the research in
[issue #35](https://github.com/Voultapher/sort-research-rs/issues/35).
It reuses consumed input blocks and permutation metadata to preserve stability
without a full-size output array.

## Source and block policy

The backend comes from
[`voracious_sort` revision 4690d80fe6866afa6b1efbaac3c7af715591e433](https://github.com/Yikai-Liao/voracious_sort/blob/4690d80fe6866afa6b1efbaac3c7af715591e433/src/sorts/block_lsd_stable_sort.rs).
That Rust implementation adapts Robert Clausecker's
[`radixsort_permuted.c`, revision f69e816c3cd79d312cd67aea5b9cf1c338c1b371](https://github.com/clausecker/radsort/blob/f69e816c3cd79d312cd67aea5b9cf1c338c1b371/radixsort_permuted.c),
described by Robert Clausecker and Florian Schintke in
[Parallel O(sqrt(n)) Overhead LSD Radix Sort](https://arxiv.org/abs/2607.05302).
The BSD-2-Clause notice is retained in `core.rs`; the source repository's MIT
license is retained in `LICENSE-VORACIOUS`.

The harness adapter fixes the digit width at 8. The backend retains the snapshot's
adaptive block size, varying-digit pre-scan, scatter, block permutation, and
compaction. For `n` records of `s` bytes and `R` buckets, block size is
`max(1, ceil(4 * sqrt(9*n / (2*R*s))), floor(1024/s))`. Auxiliary space is
O(sqrt(n)) for fixed radix and record size. Nonconstant inputs use this backend
at every size; there is no comparison-sort or conventional LSD fallback.

The local backend borrows keys through `radix_support::Key`, moves records through
uninitialized storage, and maintains one logical owner per record. It does not
require `Copy`, `Clone`, or `Debug`. Source attribution and license notices remain
in `core.rs` and `LICENSE-VORACIOUS`.

## Stable comparison pool

| Feature | Benchmark name | Method | Extra storage |
| --- | --- | --- | --- |
| `rust_block_lsd` | `rust_block_lsd_stable_radix` | 8-bit stable block LSD, varying-digit pre-scan | O(sqrt(n)) for fixed record size and radix |
| `rust_basic_lsd` | `rust_basic_lsd_stable_radix` | Plain 8-bit stable counting scatter, skip constant digits | One full record buffer |
| `rust_basic_msd` | `rust_basic_msd_stable_radix` | Plain 8-bit stable recursive counting scatter | One reusable full record buffer plus bounded recursion |
| `rust_lsd11` | `rust_lsd11_stable_radix` | 11-bit stable counting scatter, skip constant digits | One full record buffer plus two bucket tables |
| `rust_radsort_cached` | `rust_radsort_cached_stable_radix` | radsort 0.1.0 cached keys and indices, then permute payload | O(n) keys/indices, independent of payload size |
| `rust_radsort_unopt` | `rust_radsort_cached_unopt_stable_radix` | Same cached-key path with radsort digit-skipping optimization disabled | O(n) keys/indices |

The existing `rust_radsort` adapter remains the scalar direct radsort baseline.
The cached adapter is a distinct method: it extracts each key once and sorts small
key/index records instead of scattering large payloads on every pass. Basic LSD,
MSD and 11-bit LSD are reference implementations without comparison-sort fallbacks.
They are deliberately not presented as optimized competitors.

## Keys, payloads and panic recovery

`radix_support::Key` represents a fixed unsigned key of up to 128 bits. Converted
keys must be deterministic and agree with `Ord`, including equality, when using
the harness's `sort<T: Ord>` entry point. Equal keys retain input order. The
`sort_keyed<T: Key>` entry point also supports records without `Ord`.

Built-in implementations cover integer primitives, bool, char, unit, `F128`,
`FFIOneKibiByte`, `Cell<T: Key + Copy>`, pairs whose combined key fits in 128 bits,
`Box`/`Rc`/`Arc` of keyed values, and the test suite's numeric `dyn DynTrait`.
The optional float and padded-record benchmark wrappers implement the same trait
directly; their `Copy` properties are unchanged. Partial-float wrappers canonicalize
signed zero; total-order wrappers preserve all total_cmp bit distinctions.

A numeric-key record can own arbitrary payloads, including String, Box, Rc of a
dynamic object, and interior-mutable owning state. Tests use records implementing
neither Copy nor Clone, verify pointer identity and mutations, and count drops.
A variable-length string used as the primary key needs a different key algorithm
and is unsupported here. Arbitrary `sort_by` and comparison-count measurement are
also unsupported; rejection occurs before moving records or invoking a comparator.
Benchmark registration omits unsupported types through `supports_type`.

If extraction panics, adapters restore a complete owning permutation and resume
the original panic. Order after panic is unspecified. Block LSD finishes the
interrupted scatter using a fixed recovery bucket without invoking the key again,
then compacts live blocks; a drop guard restores materialization between passes.
LSD guards the whole pass so a panic during histogram construction also restores
live scratch records and their interior mutations. MSD commits only complete
scatters. Cached-key variants extract keys before moving payloads. No recovery
path allocates a full input backup. Changing returned keys during a sort violates
the key contract; this is separate from mutating payload state while returning a
consistent key.

## Validation

The repository uses nightly features. This machine used Arch Rust 1.99.1 with
`RUSTC_BOOTSTRAP=1`, without changing the global toolchain. For example:

```sh
RUSTC_BOOTSTRAP=1 cargo test --release --features \
  rust_block_lsd,rust_basic_lsd,rust_basic_msd,rust_lsd11,rust_radsort_cached,rust_radsort_unopt
```

The selected upstream functionality tests cover supported key types, while focused
checks cover stability, full payload preservation, digit and block boundaries,
float order, owned payloads, and key panics during different phases. The complete
generic comparator suite cannot pass for these radix adapters: variable-length
string keys and arbitrary sort_by are outside their interface. Raw unfiltered
audits remain in `local-validation/adapter-v2`.

No ASan was run for this adaptation. Valgrind could not start the tests because
the system loader executes AVX-512 instructions unsupported by the installed
Valgrind; that attempt is not a passing memory-safety check. Miri was unavailable.
Debug/release ownership and panic tests provide behavioral evidence, not a formal
proof of unsafe-code correctness.

## Peak extra heap measurement

`benches/extra_memory.rs` counts peak **additional requested live heap bytes during
the sort call**, relative to the live baseline immediately before it. Input,
reference output, and deep payload allocations already exist before measurement.
Generation and validation run outside the interval. Record storage bytes are
reported separately. Allocations, reallocations, cumulative requested bytes, and
retained extra bytes are separate columns. Heap accounting excludes stack,
allocator metadata and fragmentation; it does not use RSS.

```sh
OVERRIDE_SEED=20261027 RUSTC_BOOTSTRAP=1 cargo bench --no-default-features \
  --features rust_block_lsd,rust_basic_lsd,rust_basic_msd,rust_lsd11,rust_radsort_cached,rust_radsort_unopt,rust_radsort \
  --bench extra_memory -- --types i32,u64,f128,record32,record128,owned \
  --sizes 10000,1000000 --patterns random,random_d20,ascending,all_equal
```

The separate Criterion benchmark has no allocator counters. Timing uses the
upstream random-seed mode and should not receive OVERRIDE_SEED.
