// Times the templated reference port on several record shapes so the Rust
// backend can be compared beyond the paper's 8 byte pair.
//
// Usage: driver <pair|u64|u128|rec32|rec128> <n> <reps>
// Prints one CSV line per type: <type>,<n>,<median_ns>,<ns_per_elem>,<checksum>
#include "prefix_radix.hpp"

#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>

namespace {

constexpr uint64_t SEED = 0xdeadbeefc0febabeull;

inline uint64_t xs(uint64_t a) {
	a ^= a << 13;
	a ^= a >> 7;
	a ^= a << 17;
	return a;
}

// Consumes ceil(sizeof(K)/8) xorshift words, little endian, the same way the
// Rust harness generates keys.
template <class K>
struct UnsignedOf {
	using type = std::make_unsigned_t<K>;
};
template <>
struct UnsignedOf<unsigned __int128> {
	using type = unsigned __int128;
};

template <class K>
K next_value(uint64_t &a) {
	using U = typename UnsignedOf<K>::type;
	U v = 0;
	for (std::size_t i = 0; i < sizeof(K); i += 8) {
		v |= U(a) << (i * 8);
		a = xs(a);
	}
	return K(v);
}

template <std::size_t Bytes>
void fill_payload(std::array<std::byte, Bytes> &payload, uint64_t &a) {
	for (std::size_t i = 0; i < Bytes; i += 8) {
		uint64_t word = a;
		a = xs(a);
		std::memcpy(payload.data() + i, &word, Bytes - i < 8 ? Bytes - i : 8);
	}
}

// Writes a fresh input and returns an order independent checksum.
template <class Rec>
uint64_t fill(std::vector<Rec> &data) {
	uint64_t a = SEED;
	uint64_t sum = 0;
	prs::KeyOf key_of;
	for (auto &rec : data) {
		if constexpr (requires { rec.payload; }) {
			rec.key = next_value<decltype(rec.key)>(a);
			sum += uint64_t(rec.key);
			fill_payload(rec.payload, a);
			for (std::byte b : rec.payload) sum += uint64_t(b);
		} else {
			rec = next_value<Rec>(a);
			sum += uint64_t(rec);
		}
	}
	return sum;
}

template <class Rec>
bool is_sorted(const std::vector<Rec> &data) {
	prs::KeyOf key_of;
	for (std::size_t i = 1; i < data.size(); i++) {
		if (key_of(data[i - 1]) > key_of(data[i])) return false;
	}
	return true;
}

template <class Rec>
uint64_t checksum(const std::vector<Rec> &data) {
	prs::KeyOf key_of;
	uint64_t sum = 0;
	for (const auto &rec : data) {
		sum += uint64_t(key_of(rec));
		if constexpr (requires { rec.payload; }) {
			for (std::byte b : rec.payload) sum += uint64_t(b);
		}
	}
	return sum;
}

template <class Rec>
void run(const char *label, std::size_t n, int reps) {
	std::vector<Rec> data(n);
	std::vector<double> times;
	times.reserve(reps);
	uint64_t sum = 0;

	for (int r = 0; r < reps; r++) {
		sum = fill(data);
		uint64_t before = checksum(data);

		auto begin = std::chrono::steady_clock::now();
		{
			prs::Sorter<Rec, 8> sorter(data.data(), n, prs::block_policy<Rec, 8>(n));
			sorter.run();
		}
		auto end = std::chrono::steady_clock::now();
		times.push_back(std::chrono::duration<double, std::nano>(end - begin).count());

		if (!is_sorted(data)) std::fprintf(stderr, "%s: not sorted\n", label);
		if (checksum(data) != before) std::fprintf(stderr, "%s: checksum mismatch\n", label);
	}

	std::sort(times.begin(), times.end());
	double median = times[times.size() / 2];
	std::printf("%s,%zu,%.0f,%.3f,%llu\n", label, n, median, median / double(n),
	            (unsigned long long)sum);
}

struct Named {
	const char *name;
	void (*run)(const char *, std::size_t, int);
};

template <class Rec>
Named entry(const char *name) {
	return Named{name, &run<Rec>};
}

} // namespace

int main(int argc, char **argv) {
	using Pair = prs::Record<uint32_t, 4>;
	using Rec32 = prs::Record<uint64_t, 24>;
	using Rec128 = prs::Record<uint64_t, 120>;
	using U128 = unsigned __int128;

	const Named types[] = {
	    entry<Pair>("pair"),
	    entry<uint64_t>("u64"),
	    entry<U128>("u128"),
	    entry<Rec32>("rec32"),
	    entry<Rec128>("rec128"),
	};

	std::size_t n = argc > 2 ? std::strtoull(argv[2], nullptr, 10) : 1000000;
	int reps = argc > 3 ? std::atoi(argv[3]) : 7;

	if (argc < 2) {
		std::fprintf(stderr, "usage: %s <pair|u64|u128|rec32|rec128> <n> <reps>\n", argv[0]);
		return 1;
	}
	for (const auto &t : types) {
		if (!std::strcmp(t.name, argv[1])) {
			t.run(t.name, n, reps);
			return 0;
		}
	}
	std::fprintf(stderr, "unknown type %s\n", argv[1]);
	return 1;
}
