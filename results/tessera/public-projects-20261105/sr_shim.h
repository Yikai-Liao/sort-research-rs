#pragma once
#include <algorithm>
#include <atomic>
#include <bit>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <immintrin.h>
#include <memory>
#include <vector>
#include <type_traits>

#define UNLIKELY(x) __builtin_expect(!!(x),0)
#define DCHECK(x) ((void)0)
#define DCHECK_LE(x,y) ((void)0)
#define TEST_SYNC_POINT(x) ((void)0)
#define MFV_AVX512BW(...) __attribute__((target("avx512f,avx512bw"))) __VA_ARGS__
#define MFV_AVX2(...) __attribute__((target("avx2"))) __VA_ARGS__
namespace starrocks {
namespace BitUtil { inline void* safe_aligned_alloc(size_t a,size_t n){ return aligned_alloc(a,(n+a-1)/a*a); } }
template<class T> struct InlinePermuteItem { T inline_value; uint32_t index_in_chunk; };
template<class T> using InlinePermutation=std::vector<InlinePermuteItem<T>>;
using Tie=std::vector<uint8_t>;
struct SmallPermuteItem {uint32_t index_in_chunk;};
using SmallPermutation=std::vector<SmallPermuteItem>;
template<class T> static inline void restore_inline_permutation(const InlinePermutation<T>& inlined,SmallPermutation& output) {
    for(int i=0;i<inlined.size();i++) output[i].index_in_chunk=inlined[i].index_in_chunk;
}
struct Slice {int compare(const Slice&)const{return 0;}};
struct GermanString {int compare(const GermanString&)const{return 0;}};
struct DateValue {int32_t julian()const{return 0;}};
struct TimestampValue {int64_t timestamp()const{return 0;}};
}
#include "vendor/adapted_sr_raw_container.h"
#include "vendor/sr_simd.h"
#include "vendor/adapted_sr_comparator.h"
#include "vendor/adapted_starrocks_radix_sort.h"
#include "vendor/sr_pdqsort.h"

extern "C" void tessera_i32(void*,size_t);
extern "C" void tessera_i64(void*,size_t);
extern "C" void tessera_i128(void*,size_t);
extern "C" void tessera_f32(void*,size_t);
extern "C" void tessera_f64(void*,size_t);
extern "C" void tessera_u64(uint64_t*,size_t);
template<class T> void call_tessera(starrocks::InlinePermuteItem<T>* a,size_t n) {
    if constexpr(std::is_same_v<T,int32_t>)tessera_i32(a,n);
    if constexpr(std::is_same_v<T,int64_t>)tessera_i64(a,n);
    if constexpr(std::is_same_v<T,float>)tessera_f32(a,n);
    if constexpr(std::is_same_v<T,double>)tessera_f64(a,n);
}
