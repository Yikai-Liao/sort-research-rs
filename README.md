# Block LSD radix sort: templated C++ port of the reference implementation

Measurement scaffolding, not a library. It exists to answer one question about
the Rust port in sort-research-rs: does the port reproduce the reference
implementation's speed, on the reference's own data type and on other record
shapes?

## Provenance

c-reference/ holds radixsort_permuted.c and radixsort.h from
https://github.com/clausecker/radsort @ f69e816, BSD-2-Clause, notices kept.
That file is the reference the Rust port was adapted from:

    Robert Clausecker, Florian Schintke
    "Parallel O(sqrt(n)) Overhead LSD Radix Sort"

The reference sorts uint32_t[2] records with a compile-time block of 512 and a
compile-time record type. Two things are parameterised here, nothing else:

1. cxx/prefix_radix.hpp — the same algorithm as a template over the record type,
   so u64, u128, 32 and 128 byte records can be measured with the same code.
   Algorithm, permutation bookkeeping, partial-block tracking and the compact
   step are unchanged; std::memcpy replaces the C memcpy on trivially copyable
   records.
2. c-reference/radixsort_permuted_dyn.c — a copy of the reference with the block
   size made a runtime value and the Rust backend's policy added, so the two
   sides can be compared under the same block size instead of "fixed 512 versus
   something else". The block policy is:

       b = max(ceil(4 * sqrt(9n / (2 * R * s))), floor(1024 / s), 1)
       R = 1 << radix, s = sizeof(record)

   identical to src/other/rust_block_lsd/core.rs::sort in sort-research-rs.

Both drivers generate the same input as the Rust harness (xorshift64, see
driver.cpp fill) and print an order independent checksum, which is how the two
sides are proved to sort the same data.

## Build

    make            # g++ and cc
    make clang      # clang++ and clang

Produces driver_cxx, driver_ref (pristine, block 512) and driver_policy
(runtime block size, policy applied).

## Run

    ./driver_cxx <pair|u64|u128|rec32|rec128> <n> <reps>
    ./driver_policy <n> <reps> policy
    ./driver_ref <n> <reps>

Each prints one CSV line: type, n, median ns, ns per element, checksum. The
driver asserts the output is sorted and that the checksum is unchanged; a
duplicated or lost record fails the run.

## Rust side

rust-side/ is the counterpart harness. It includes
src/other/rust_block_lsd/core.rs from sort-research-rs verbatim, so the two
binaries are built from the same algorithm at the same commit. Both the Cargo
dependency and the #[path] attribute in rust-side/main.rs expect a sort-research-rs
checkout in a sibling directory (../../../sort-research-rs from the source file);
adjust them to your layout.

    cd rust-side && cargo build --release
    ./target/release/rust-compare <type> <n> <reps>

## Comparing

ab.py alternates the two binaries within every round (so VM noise hits both
equally), then reports the median of the per-round ratios:

    AB_TYPES=pair,u64,u128,rec32,rec128 AB_SIZES=100000,1000000 \
      RUST_BIN=.../rust-compare python3 ab.py 9

logs/ holds the runs behind the numbers quoted in the parent project:

| log | what |
| --- | --- |
| ab.log | A/B against the reference at 1e5 and 1e6, five record shapes |
| ab-nostore.log | same, Rust with the panic-recovery store removed by hand |
| ab-final.log | same, after the needs_drop gate landed in the port |
| ab-small.log | A/B at 1e3, 1e4, 1e5 |
| typesweep.log | min of three medians per shape, first coarse pass |
| blockcap.log | block size capped at n/radix, i.e. scratch proportional to n |
| cmp5.log | gcc versus clang versus Rust instruction counts |

## What the numbers said

- With the same block policy, the Rust port and this C++ template are within
  noise of each other on all five record shapes at 1e5 and 1e6 (0.96x - 1.06x,
  ab-final.log). The port reproduces the reference; there is no structural
  slowdown to chase.
- Before the needs_drop gate, the Rust side was 4-7% slower and the difference
  vanished when the panic-recovery store was removed (ab-nostore.log). That store
  exists because a half-moved slice would otherwise drop the same owner twice;
  it is skipped for records without destructors.
- Capping the block size at n/radix (scratch proportional to n instead of a
  fixed 512 blocks) is 2.5-4x faster at 1e3-1e4 records and neutral above
  (blockcap.log). It is not in the port: small inputs are dominated by the block
  tables either way and the tuned policy was kept.

## Caveats

- One machine, shared VM: run-to-run variation is a few percent, so the interleaved
  A/B runner is used rather than comparing separate runs.
- The C reference's own benchmark framework (benchmark.c, generic/) is not part
  of this tree; the drivers here exist to share the input generator with the Rust
  harness.
- No tests beyond the sortedness and checksum assertions in the drivers.
