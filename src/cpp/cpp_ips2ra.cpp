// IPS²Ra, pinned at 83f6b76ca308b277b2acc3a36426a3baa3c9cb73.
//
// Only the sequential entry point is used. IPS²Ra is unstable and keeps
// per-bucket block buffers plus task metadata, so it is the low-extra-space
// unstable radix baseline, not an in-place sort.

#include "thirdparty/ips2ra/ips2ra.hpp"

#include <cstdlib>
#include <stdint.h>

#include "shared.h"

namespace {

// IPS²Ra orders the extracted key with <. Signed keys are mapped to unsigned
// without changing their order.
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

template <typename T, typename Extractor>
void sort_impl(T* data, size_t len, Extractor extractor) noexcept {
    try {
        ips2ra::sort(data, data + len, extractor);
    } catch (...) {
        // No exception may cross the C ABI boundary.
        std::abort();
    }
}

}  // namespace

extern "C" {
// --- i32 ---

void ips2ra_unstable_i32(int32_t* data, size_t len) {
    sort_impl(data, len, I32Key{});
}

// --- u32 ---

void ips2ra_unstable_u32(uint32_t* data, size_t len) {
    sort_impl(data, len, ips2ra::Config<>::identity{});
}

// --- u64 ---

void ips2ra_unstable_u64(uint64_t* data, size_t len) {
    sort_impl(data, len, ips2ra::Config<>::identity{});
}

// --- records keyed by their leading u64 ---

void ips2ra_unstable_record16(Record16* data, size_t len) {
    sort_impl(data, len, Record16Key{});
}

void ips2ra_unstable_record128(Record128* data, size_t len) {
    sort_impl(data, len, Record128Key{});
}
}  // extern "C"
