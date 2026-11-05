// DovetailSort, pinned at f4630a264679143dfc909cd9612ffa39b27bb76e.
//
// The build defines PARLAY_SEQUENTIAL, so this is the algorithm's serial
// configuration. The entry point is integer_sort_inplace2: despite the name it
// allocates a full-length temporary record array, and writes the result back to
// the input range. It is stable and uses O(n) extra space.

#include "thirdparty/dovetail/parlay/integer_sort.h"

#include <cstdlib>
#include <stdint.h>

#include "shared.h"

namespace {

struct I32Key {
    uint32_t operator()(int32_t value) const noexcept {
        return static_cast<uint32_t>(value) ^ 0x80000000u;
    }
};

struct Record16Key {
    uint64_t operator()(const Record16& value) const noexcept { return value.key; }
};

struct Record128Key {
    uint64_t operator()(const Record128& value) const noexcept { return value.key; }
};

struct IdentityKey {
    template <typename T>
    T operator()(T value) const noexcept {
        return value;
    }
};

template <typename T, typename GetKey>
void sort_impl(T* data, size_t len, GetKey get_key) noexcept {
    try {
        parlay::integer_sort_inplace2(parlay::make_slice(data, data + len), get_key);
    } catch (...) {
        // No exception may cross the C ABI boundary. The benchmark process has
        // no recovery path for a failed allocation inside the sort.
        std::abort();
    }
}

}  // namespace

extern "C" {
// --- i32 ---

void dovetail_stable_i32(int32_t* data, size_t len) {
    sort_impl(data, len, I32Key{});
}

// --- u32 ---

void dovetail_stable_u32(uint32_t* data, size_t len) {
    sort_impl(data, len, IdentityKey{});
}

// --- u64 ---

void dovetail_stable_u64(uint64_t* data, size_t len) {
    sort_impl(data, len, IdentityKey{});
}

// --- records keyed by their leading u64 ---

void dovetail_stable_record16(Record16* data, size_t len) {
    sort_impl(data, len, Record16Key{});
}

void dovetail_stable_record128(Record128* data, size_t len) {
    sort_impl(data, len, Record128Key{});
}
}  // extern "C"
