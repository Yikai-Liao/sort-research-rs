# Block LSD research artifacts

This branch holds historical measurements and publication drafts for
[feat/block-lsd](https://github.com/Yikai-Liao/sort-research-rs/tree/feat/block-lsd).
It is separate from the code branch and is not intended for merging.
The branch began with the root commit `c306e53`; closeout is appended as a new
commit rather than replacing that archive.

Corrected code candidate: **22463e9ef3e59a917afe8fcd1ed1a52ca123d9b7**.
The closeout did not compile or execute it, run tests/Miri/ASan, or collect new
performance measurements. Earlier passing tests and benchmark numbers are
historical evidence and do not validate the corrected candidate.

## Contents and provenance

| path | contents |
| --- | --- |
| HANDOFF.md | Chinese closeout report and remaining validation |
| drafts/ | revised issue reply, PR body and checklist; not published |
| results/block_lsd.json | unchanged historical critcmp export, 106 entries |
| results/block_lsd_all.txt | generated median, mean and standard-deviation summary |
| results/TABLES.md | generated random timing and peak-extra-heap tables |
| results/provenance.json | JSON hash, candidate revision and per-entry log classification |
| NUMBERS.md | generated medians, corrected log paths and provenance limitations |
| analysis_block_lsd/ | all standard HTML plots regenerated from the saved JSON |
| figures/ | PNGs regenerated from the same medians |
| logs/ | aggregate Criterion logs and historical peak-extra-heap output |
| logs/contaminated/ | discarded batch, retained only as an archive |
| scripts/refresh.py | repeatable processing of saved data; runs no sorting code |
| scripts/check_saved.py | checks all generated HTML payloads against JSON medians |
| scripts/graph_bench_result/ | graph tools copied from code candidate 22463e9, with explicit historical machine labels |
| scripts/run-*.sh | historical benchmark commands; hard-coded paths, not closeout validation |
| c-comparison/ | archived C/C++ and Rust comparison harnesses and logs; not final candidate evidence |

Of the 106 JSON entries, 82 random cases match the rounded mean/slope and
confidence intervals in their clean aggregate logs. The logs do not include raw
samples to recompute the medians; median values come from the JSON. The other
24 pattern entries are saved baseline estimates whose original measurement log
has not been recovered. `logs/pattern-bench.log` contains a later run and cannot
serve as their original log. The reply/PR numeric comparisons omit those rows;
the archive retains them with explicit labels.

Individual measurement commit IDs were not recorded. The previous candidate
head was `429f6f7`; it must not be treated as proven provenance for every run.

The historical machine was a Skylake-SP Xeon Gold 6140, 2.3 GHz nominal, shared
6 vCPU Linux VM, with nightly 2026-10-08. Warm-up was 2 s and measurement 4 s.
Hot and cold both generate inputs through batch setup. Cold additionally
perturbs prediction state; the harness deliberately avoids measuring loading
from main memory. Hot does not repeatedly sort a single input buffer.

## Regenerate saved artifacts without running benchmarks

From any directory:

```sh
python /path/to/artifacts/scripts/refresh.py
```

For HTML and PNGs, use an isolated Python environment with the versions in
`scripts/requirements-plots.txt`, then:

```sh
python /path/to/artifacts/scripts/refresh.py --plots
```

The plot tools use stored machine metadata rather than detecting the current
host. Each HTML has a provenance notice and each PNG has a historical-data title.
The corrected candidate is labeled unmeasured; pattern rows are baseline-only. `fill.py` now refreshes tables
without rewriting the reviewed drafts. `check_saved.py` compares every HTML
data point against the JSON, including throughput and relative-speedup values. `mktable.py`, `pattern_table.py` and
`coverage.py` use paths relative to this archive.

The C/C++ comparisons and fixed-block ablations helped explore the port, but
have not been repeated after removing the unsound recovery optimization. They
are retained as historical working material only.
