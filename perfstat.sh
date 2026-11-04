#!/bin/bash
set -u
B=/root/code/sort-research-rs-c-compare
run() { # name exe args
  for i in 1 2 3; do
    perf stat -e cycles,instructions,branch-misses,L1-dcache-load-misses,LLC-load-misses \
      taskset -c 2 "$2" $3 $4 2>&1 >/dev/null | awk -F'#' '{gsub(/ /,"",$1); print $1}'
  done
}
for n in 100000 1000000; do
  echo "== n=$n gcc =="
  for i in 1 2 3; do perf stat -e cycles,instructions,branch-misses,L1-dcache-load-misses,LLC-load-misses taskset -c 2 $B/c-compare/driver_dyn $n 7 policy 2>&1 >/dev/null | rg -o '^\s*[0-9,]+\s+\w' | tr -d ' ' | tr '\n' ' '; echo; done
  echo "== n=$n clang =="
  for i in 1 2 3; do perf stat -e cycles,instructions,branch-misses,L1-dcache-load-misses,LLC-load-misses taskset -c 2 $B/c-compare/driver_dyn_clang $n 7 policy 2>&1 >/dev/null | rg -o '^\s*[0-9,]+\s+\w' | tr -d ' ' | tr '\n' ' '; echo; done
  echo "== n=$n rust =="
  for i in 1 2 3; do perf stat -e cycles,instructions,branch-misses,L1-dcache-load-misses,LLC-load-misses taskset -c 2 $B/target/release/rust-compare $n 7 auto 2>&1 >/dev/null | rg -o '^\s*[0-9,]+\s+\w' | tr -d ' ' | tr '\n' ' '; echo; done
done
