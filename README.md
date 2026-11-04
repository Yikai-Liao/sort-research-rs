# block LSD radix sort — artifacts

Orphan branch: measurement data, comparison harnesses and the issue/PR drafts for
the feat/block-lsd branch. Nothing here is meant to be merged; it exists so the
numbers in the issue reply and PR body can be reproduced and audited.

Code under review: branch feat/block-lsd, commit 429f6f7.

## Contents

| path | what |
| --- | --- |
| drafts/issue-reply.md | draft comment for issue #35 |
| drafts/pr-body.md | draft PR body |
| drafts/TODO.md | checklist that produced this state |
| results/block_lsd.json | critcmp export, baseline block_lsd, 106 benchmarks |
| results/block_lsd_all.txt | the same data as critcmp table output |
| analysis_block_lsd/ | graphs from util/graph_bench_result/graph_all.py (HTML) |
| figures/ | static PNG versions of the scaling and relative plots |
| logs/ | raw criterion output of every benchmark run |
| scripts/ | the exact commands and post-processing used |
| c-comparison/ | local comparison against the reference implementation |

## How the benchmark data was produced

    cd sort-research-rs
    cargo bench --features \
        rust_block_lsd,rust_radsort,cold_benchmarks,bench_type_u128,\
bench_type_f64,bench_type_u64_16b,bench_type_u64_128b \
      --bench bench -- --warm-up-time 2 --measurement-time 4 --noplot \
      --save-baseline block_lsd
    critcmp --export block_lsd > block_lsd.json

Same warm-up and measurement times as util/run_benchmarks.py; the regexes that
selected each batch are in scripts/. Machine: Skylake-SP Xeon Gold 6140, 2.3 GHz,
shared 6 vCPU VM, nightly 2026-10-08.

Caveat worth stating: one earlier hot-cache batch ran while other work competed
for the same cores and came out about 2x slow. It was discarded and re-run on an
idle machine; logs/ holds only the clean runs.

## Local comparison against the C reference

Not part of the PR: the issue reply and PR body compare against the sorts in this
repository only. These files are the working material for the port itself.

- c-comparison/c-compare/ — the reference (clausecker/radsort @ f69e816,
  radixsort_permuted.c) with a driver that (a) uses the same input generator as
  the Rust harness and (b) makes the block size a runtime parameter so both
  sides can apply the same policy.
- c-comparison/cxx/ — the same algorithm as a template over the record type, so
  record shapes other than the paper's 8 byte pair can be measured.
- c-comparison/rust-compare/ — the Rust side, including core.rs verbatim.
- c-comparison/*.log — interleaved A/B runs (alternating binaries within each
  round, median of ratios) and instruction counts.

Result: with the same block policy the port matches the reference within noise on
five record shapes (pair, u64, u128, 32 and 128 byte records) at 1e5 and 1e6
records. The remaining difference was the panic-recovery store, which the
needs_drop gate removes for every type this repository supports.
