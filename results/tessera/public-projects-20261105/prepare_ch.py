#!/usr/bin/env python3
"""Adapt the upstream ClickHouse example/benchmark; retain its generator and timer."""
from pathlib import Path
import hashlib,json
ROOT=Path(__file__).resolve().parent
vendor=ROOT/'vendor'
for name in ['ch_allocator.h','ch_stopwatch.h','ch_stopwatch.cpp','ch_example.cpp']:
    s=(vendor/name).read_text()
    # Keep C++ standard and PCG includes, supply only project dependencies via shim.
    s=''.join(l for l in s.splitlines(keepends=True) if not (l.startswith('#include <Common/') or l.startswith('#include <base/') or l.startswith('#include <IO/') or l.startswith('#include <Core/') or l.startswith('#include <Examples/') or l.startswith('#include <pdqsort.h>') or l.startswith('#include <pcg_random.hpp>')))
    (vendor/('adapted_'+name)).write_text(s)
s=(vendor/'adapted_ch_example.cpp').read_text()
s='#include "ch_shim.h"\n'+s
s=s.replace('int mainEntryExampleRadixSort(int argc, char ** argv)','int main(int argc, char ** argv)')
s=s.replace('std::setprecision(3)','std::setprecision(9)')
needle='        if (method == 8) sort8(data.data(), n, limit);'
assert s.count(needle)==1
s=s.replace(needle,needle+'\n        if (method == 9) tessera_u64(data.data(), n);')
# Exact reference and conservation check outside the upstream timed region.
needle='    {\n        Stopwatch watch;\n\n        if (method == 1)'
assert s.count(needle)==1
s=s.replace(needle,'    auto reference = data;\n    std::sort(reference.begin(), reference.end());\n\n'+needle)
s=s.replace('    bool ok = true;','    bool ok = (data == reference);')
s=s.replace('    return 0;\n}', '    if (ok) std::cerr << "Reference check: OK\\n";\n    return ok ? 0 : 1;\n}')
(ROOT/'clickhouse_example.cpp').write_text(s)
(ROOT/'ch_stopwatch.cpp').write_text('#include "ch_shim.h"\n'+(vendor/'adapted_ch_stopwatch.cpp').read_text())
manifest=json.loads((ROOT/'manifest.json').read_text())
base='https://raw.githubusercontent.com/ClickHouse/ClickHouse/c8c7411269eb045ef70cf6e90dbc997d60dfa76f/'
paths={'ch_example.cpp':'src/Common/examples/radix_sort.cpp','ch_benchmark.cpp':'src/Common/benchmarks/radix_sort.cpp','ch_allocator.h':'src/Common/AllocatorWithMemoryTracking.h','ch_stopwatch.h':'src/Common/Stopwatch.h','ch_stopwatch.cpp':'src/Common/Stopwatch.cpp','ch_pdqsort.h':'contrib/pdqsort/pdqsort.h'}
for name in ['pcg_random.hpp','pcg_extras.hpp','pcg_uint128.hpp']:paths[name]='base/pcg-random/'+name
for name,path in paths.items():manifest['upstream'][name]={'url':base+path,'sha256':hashlib.sha256((vendor/name).read_bytes()).hexdigest()}
manifest['clickhouse_scope']='Common/examples/radix_sort.cpp, UInt64 full-sort methods 1/2/5 plus Tessera method 9. Original pcg64, Stopwatch, sort dispatch and checks retained; seed fixed, reporting precision increased, exact reference check added outside timer. Allocator memory tracking replaced with malloc/free; database getPermutation benchmark not run.'
(ROOT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Prepared upstream ClickHouse example benchmark')
