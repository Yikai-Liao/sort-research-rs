#!/usr/bin/env bash
# Paired timing: tessera (A1), tessera_a2 (A2, same function), tessera_copy
# (identical engine copy, placement control), tessera_cached (Copy records
# forced onto the bucket-id cache). One process per cycle, pinned to CPU 2.
set -euo pipefail
BIN=/root/code/tessera-cache-exp/target/release/build/sort_research_rs/0c7e65725a0b5808/out/radix_ab-0c7e65725a0b5808
OUT=/root/code/tessera-cache-exp/raw
cd /root/code/tessera-cache-exp/wt
COMMON=(--types u32,u64,u128,record16,record128 --patterns random,random_d256,zipf13
        --variants tessera,tessera_a2,tessera_copy,tessera_cached)
for cycle in 1 2 3 4 5; do
  export RADIX_RAW=$OUT/cycle$cycle.csv
  : > "$RADIX_RAW"
  taskset -c 2 "$BIN" "${COMMON[@]}" --sizes 1024,4096 --batch 32 --rounds 41 > "$OUT/cycle$cycle.log"
  taskset -c 2 "$BIN" "${COMMON[@]}" --sizes 16384 --batch 8 --rounds 41 >> "$OUT/cycle$cycle.log"
  taskset -c 2 "$BIN" "${COMMON[@]}" --sizes 65536 --batch 2 --rounds 31 >> "$OUT/cycle$cycle.log"
  taskset -c 2 "$BIN" "${COMMON[@]}" --sizes 1000000 --batch 1 --rounds 11 >> "$OUT/cycle$cycle.log"
  echo "cycle $cycle done $(date +%T)"
done
