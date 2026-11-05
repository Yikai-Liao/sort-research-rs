// x86-simd-sort, pinned at 6e059b14975df2e3c74260ec905167a1be5a34ca.
//
// The old adapter reinterpreted uint64_t as int64_t, which reverses the order of
// the upper half of the unsigned range. The static interface is called with the
// actual element type so unsigned comparisons are used. Compilation with
// -march=native selects either the AVX2 or the AVX-512 translation.

#include "thirdparty/x86_simd_sort/src/x86simdsort-static-incl.h"

#include <stdint.h>

#include "shared.h"

extern "C" {
// --- i32 ---

void intel_avx512_i32(int32_t* data, size_t len) {
    x86simdsortStatic::qsort(data, len);
}

// --- u32 ---

void intel_avx512_u32(uint32_t* data, size_t len) {
    x86simdsortStatic::qsort(data, len);
}

// --- u64 ---

void intel_avx512_u64(uint64_t* data, size_t len) {
    x86simdsortStatic::qsort(data, len);
}
}  // extern "C"
