// Highway VQSort, pinned at 1.4.0 (2607d3b5b0113992fe84d3848859eae13b3b52c1).
//
// VQSort is unstable and allocates no heap memory. The uint128_t entry sorts the
// complete 128-bit key; it is not a 64-bit key with a payload. The per-type
// translation units are compiled by build.rs and provide the dispatched VQSort
// overloads.

#include "hwy/contrib/sort/vqsort.h"

#include <stdint.h>

#include "shared.h"

static_assert(sizeof(U128) == sizeof(hwy::uint128_t));
static_assert(alignof(U128) == alignof(hwy::uint128_t));

extern "C" {
// --- i32 ---

void vqsort_i32(int32_t* data, size_t len) {
    hwy::VQSort(data, len, hwy::SortAscending{});
}

// --- u16 ---

void vqsort_u16(uint16_t* data, size_t len) {
    hwy::VQSort(data, len, hwy::SortAscending{});
}

// --- u32 ---

void vqsort_u32(uint32_t* data, size_t len) {
    hwy::VQSort(data, len, hwy::SortAscending{});
}

// --- u64 ---

void vqsort_u64(uint64_t* data, size_t len) {
    hwy::VQSort(data, len, hwy::SortAscending{});
}

// --- u128 ---

void vqsort_u128(U128* data, size_t len) {
    hwy::VQSort(reinterpret_cast<hwy::uint128_t*>(data), len, hwy::SortAscending{});
}
}  // extern "C"
