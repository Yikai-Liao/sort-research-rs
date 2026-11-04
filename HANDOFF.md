# Hand-off: block LSD radix sort

State of this work, what is still open, and how to check any number in the drafts.
Written for whoever picks this up (including future me).

## Where things stand

| item | state |
| --- | --- |
| Code branch | sort-research-rs, branch feat/block-lsd, commit 429f6f7, pushed to the fork |
| Issue #35 | draft reply ready in drafts/issue-reply.md, not posted |
| PR | draft body ready in drafts/pr-body.md, not opened |
| Tests | release, ASan and Miri green (details below) |
| Benchmarks | 106 critcmp benchmarks, baseline block_lsd, hot and cold |
| Graphs | analysis_block_lsd/ (repo standard, HTML) and figures/ (static PNG) |

Nothing in this branch is for merging. The reviewable code is the other branch.

## What the code branch contains

- src/other/rust_block_lsd/core.rs — the port of the reference block LSD radix
  sort, about 700 lines with the BSD-2-Clause notices kept.
- src/other/rust_block_lsd.rs — adapter, feature rust_block_lsd, sort name
  rust_block_lsd_stable_radix.
- src/other/radix_support.rs — Key/Unsigned contract and the adapter macro
  shared by the three radix sorts.
- tests/block_lsd_functionality.rs — the standard suite, 233 tests.
- tests/block_lsd.rs — 8 tests: block tails, record widths, integer widths,
  the two rejection paths.
- tests/radix_owned.rs, tests/radix_views.rs — 2 + 6 tests: non-Copy records,
  drop counts, key extractor that panics mid-pass.
- benches/extra_memory.rs — peak extra heap, the evidence for O(sqrt(n)).
- util/graph_bench_result/util.py — meta info entries for the two new sort
  names, otherwise the graph tool cannot plot them.

## Verification commands and their results

    cargo test --release --features rust_block_lsd,rust_radsort
      8 + 233 + 233 + 2 + 6 tests pass

    RUSTFLAGS=-Zsanitizer=address cargo test --release -Zbuild-std \
      --target x86_64-unknown-linux-gnu --features rust_block_lsd,rust_radsort
      same counts, no sanitizer reports

    cargo +nightly miri test --release --features rust_block_lsd,rust_radsort \
      --test block_lsd | --test radix_owned | --test radix_views
      8 + 2 + 6 pass; shapes are reduced on purpose, the reason is in the test
      comments (a Miri run interprets every instruction of every pass)

## Decisions already taken, and why

1. Block size is derived from n and the record size instead of the reference's
   fixed 512. Formula and reasoning are in core.rs::sort; a fixed block is still
   one call away through sort_with_block_size.
2. The panic-recovery offset is gated on needs_drop::<T>(). Copy records skip it
   (3% of the hot loop), records with destructors keep exact recovery. The test
   for that path now counts drops, so a lost or duplicated record fails the test.
3. No numbers from the C reference or the C++ template comparison appear in the
   issue reply or the PR body. Those comparisons guided the port, they are not
   part of the claim; see c-comparison/.
4. The two sort_test_tools::Sort declarations (supports_type, supports_sort_by)
   are part of the same commit series but can be split out if the maintainer
   prefers.

## Open questions for the maintainer

Written into both drafts, repeated here so they are not lost:

1. Panic contract: is the needs_drop split acceptable, or is exact recovery
   wanted for every type at the 3% cost?
2. Vendored implementation versus a dependency on the reference port.
3. Keep benches/extra_memory.rs as a new bench target?
4. Split the Sort capability declarations into their own PR?

## How to reproduce the numbers

Benchmarks (criterion settings are the ones util/run_benchmarks.py uses):

    cd sort-research-rs
    cargo bench --features \
      rust_block_lsd,rust_radsort,cold_benchmarks,bench_type_u128,\
bench_type_f64,bench_type_u64_16b,bench_type_u64_128b \
      --bench bench -- --warm-up-time 2 --measurement-time 4 --noplot \
      --save-baseline block_lsd
    critcmp --export block_lsd > block_lsd.json

BENCH_REGEX per batch (the order the data was collected in) is in scripts/.

Graphs: the repo tool needs a CPU name it recognises, and its table does not
contain this machine, so the copy used here (identical except cpu_info.py) is
not included; edit that one function or run the tool on your own machine:

    python util/graph_bench_result/graph_all.py block_lsd.json

Static PNGs come from scripts/plot.py, which reads the same JSON.

## Reading the data honestly

- Against rust_radsort_radix (same family, O(n) scratch) the port wins everywhere
  measured at n >= 1e5: 0.95x at 100k u64, 0.61x at 1M, 0.45-0.67x on 128 byte
  records.
- Against rust_std_stable it is around parity on u64 at 1M (cold 42.8 vs 43.1 ms,
  hot 47.2 vs 44.9 ms) and clearly ahead on wide records and f64 (0.37-0.81x).
- Against rust_ipnsort_unstable it loses on u64 and u128 by 1.5-1.8x. That is
  expected: ipnsort is an unstable in-place comparison sort, its 23 comparisons
  per element at 1e7 happen in registers, while radix moves every record to
  memory once per digit.
- Below 1e4 records the port loses to everything: the 2*256 block tables and the
  ~0.5 MB scratch floor dominate. The memory table shows the same effect from the
  other side (1000 u64 records: 0.54 MB).
- Ordered input: the pre-scan cuts 8 passes to 3, which beats rust_radsort_radix
  by 2.6x at 1M, but the adaptive comparison sorts are still 20-40x faster there.

## Caveats

- One hot-cache batch was measured while other work competed for the cores and
  came out about 2x slow. It was thrown away and re-measured; everything in logs/
  is from the clean runs. If a number here disagrees with an older log, the older
  one is the contaminated batch.
- Run-to-run variation on this shared VM is a few percent; read ratios, not
  absolute milliseconds.
- 1000 was in the regex batches but criterion skipped the empty/short cases, so
  the smallest measured size is 1e4.
