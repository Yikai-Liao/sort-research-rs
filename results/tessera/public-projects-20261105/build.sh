#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
python3 prepare.py
python3 prepare_ch.py
if [[ ! -d benchmark-1.9.4 ]]; then
    python3 - <<'PY'
from pathlib import Path
import urllib.request,tarfile
p=Path('benchmark.tar.gz')
p.write_bytes(urllib.request.urlopen('https://github.com/google/benchmark/archive/refs/tags/v1.9.4.tar.gz').read())
with tarfile.open(p) as t:t.extractall('.',filter='data')
p.unlink()
PY
fi
cmake -S benchmark-1.9.4 -B benchmark-build -DCMAKE_BUILD_TYPE=Release -DBENCHMARK_ENABLE_TESTING=OFF -DBENCHMARK_ENABLE_INSTALL=OFF
cmake --build benchmark-build -j 2
RUSTFLAGS='-C target-cpu=native' cargo build --release --locked
clang++ -std=c++23 -O3 -march=native -DNDEBUG -I benchmark-1.9.4/include -c starrocks_bench.cpp -o starrocks_bench.o
clang++ starrocks_bench.o target/release/libtessera_public_project_screen.a benchmark-build/src/libbenchmark.a -ldl -lpthread -lm -o starrocks_bench
clang++ -std=c++23 -O3 -march=native -DNDEBUG -I vendor clickhouse_example.cpp ch_stopwatch.cpp target/release/libtessera_public_project_screen.a -ldl -lpthread -lm -o clickhouse_example
