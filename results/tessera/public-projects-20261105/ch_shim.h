#pragma once
#include <algorithm>
#include <bit>
#include <cassert>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <iomanip>
#include <iostream>
#include <limits>
#include <vector>

using UInt64=uint64_t;
#define ALWAYS_INLINE inline __attribute__((always_inline))
#define NO_INLINE __attribute__((noinline))
#define NO_STACK_PROTECTOR __attribute__((no_stack_protector))
#define likely(x) __builtin_expect(!!(x),1)
#define chassert(x) assert(x)
#define USE_X86_64_MULTITARGET_CODE 1
namespace DB {
enum class TargetArch { GenuineIntel };
inline bool isArchSupported(TargetArch) { return true; } // Verified GenuineIntel host.
template<class T> T parse(const char* p){return static_cast<T>(std::strtoull(p,nullptr,10));}
}
using std::bit_cast;
using std::make_unsigned_t;
template<class T> constexpr bool is_integer=std::is_integral_v<T>;
template<class T> constexpr bool is_unsigned_v=std::is_unsigned_v<T>;
#include "vendor/ch_pdqsort.h"
// These base/sort declarations are used by the unmeasured MSD helpers.
template<class It,class C> void sort(It a,It b,C c){std::sort(a,b,c);}
template<class It,class C> bool trySort(It a,It b,C c){return pdqsort_try_sort(a,b,c);}

#include "vendor/adapted_ch_allocator.h"
inline void* allocateWithMemoryTracking(size_t bytes,size_t alignment) {
    if(alignment) {
        void* p=nullptr;
        if(posix_memalign(&p,alignment,bytes))throw std::bad_alloc();
        return p;
    }
    void* p=std::malloc(bytes);
    if(!p)throw std::bad_alloc();
    return p;
}
inline void deallocateWithMemoryTracking(void* p,size_t) noexcept {std::free(p);}
#include "vendor/adapted_clickhouse_radix_sort.h"
#include "vendor/adapted_ch_stopwatch.h"
#include "vendor/pcg_random.hpp"
// The example normally chooses a random seed. Freeze it so every method sees identical input.
inline uint64_t randomSeed(){const char* s=std::getenv("BENCH_SEED");return s?std::strtoull(s,nullptr,10):0x5EED1234ULL;}
extern "C" void tessera_u64(uint64_t*,size_t);
