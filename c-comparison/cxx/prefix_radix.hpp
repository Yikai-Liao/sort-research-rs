// Faithful C++ translation of the reference block LSD radix sort
// (clausecker/radsort @ f69e816, radixsort_permuted.c), templated over the
// record type so record shapes other than the paper's 8 byte pair can be
// measured. Algorithm, permutation bookkeeping and the compact step are
// unchanged; two things the reference hard-codes are parameters here: the block
// size, and the record type with its key.
#pragma once

#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <new>
#include <type_traits>
#include <utility>

namespace prs {

// A record with a radix key and an opaque payload, matching the Rust harness.
// PayloadBytes is chosen so sizeof(Record) equals the shape under test.
template <class K, std::size_t PayloadBytes>
struct Record {
	K key;
	std::array<std::byte, PayloadBytes> payload;
};

// Reads the sort key of a record: its first field, or the value itself for
// primitive keys.
struct KeyOf {
	template <class R>
	auto operator()(const R &rec) const {
		if constexpr (requires { rec.key; }) {
			return rec.key;
		} else {
			return rec;
		}
	}
};

// Block LSD radix sort: one step per RSHIFT bits, blocks of block_size records,
// 2*RADIX scratch blocks plus a permutation of all blocks.
template <class Rec, unsigned RSHIFT, class KeyOfFn = KeyOf>
class Sorter {
	using Key = std::remove_cvref_t<decltype(KeyOfFn{}(std::declval<const Rec &>()))>;
	static_assert(sizeof(Key) <= 16, "key wider than 128 bits");

      public:
	using Size = std::size_t;

	static constexpr unsigned RADIX = 1u << RSHIFT;
	static constexpr unsigned NSCRATCH = 2 * RADIX;
	static constexpr unsigned PASSES = (sizeof(Key) * 8 + RSHIFT - 1) / RSHIFT;

	struct Bucket {
		Rec *start, *end;
	};

	struct Partial {
		unsigned index;
		unsigned length;
	};

	Sorter(Rec *pairs, Size n, Size block_size)
	    : pairs(pairs),
	      n(n),
	      block_size(block_size),
	      perm(static_cast<unsigned *>(::operator new(sizeof(unsigned) * nperm()))),
	      perm2(static_cast<unsigned *>(::operator new(sizeof(unsigned) * nperm()))),
	      assignments(static_cast<unsigned char *>(::operator new(nperm()))),
	      scratch(static_cast<Rec *>(::operator new(sizeof(Rec) * NSCRATCH * block_size))) {
		initialize();
	}

	~Sorter() {
		::operator delete(perm);
		::operator delete(perm2);
		::operator delete(assignments);
		::operator delete(scratch);
	}

	Sorter(const Sorter &) = delete;
	Sorter &operator=(const Sorter &) = delete;

	void run() {
		for (unsigned pass = 0; pass < PASSES; pass++) {
			step(pass * RSHIFT);
		}
		compact();
	}

      private:
	Size nperm() const { return NSCRATCH + n / block_size; }

	Rec *blockat(Size i) {
		return i < NSCRATCH ? scratch + i * block_size : pairs + (i - NSCRATCH) * block_size;
	}

	static unsigned bucketof(const Rec &rec, unsigned shift) {
		return static_cast<unsigned>((KeyOfFn{}(rec) >> shift) & (RADIX - 1));
	}

	Size blocksize(Size i, Size *i_partial) const {
		Size len = block_size;
		assert(*i_partial < RADIX);
		if (partials[*i_partial].index == i) {
			len = partials[*i_partial].length;
			++*i_partial;
		}
		return len;
	}

	void initialize() {
		fill = RADIX + n / block_size + 1;
		std::memset(perm, 0, sizeof(unsigned) * nperm());
		std::memset(perm2, 0, sizeof(unsigned) * nperm());
		std::memset(assignments, 0, nperm());
		Size i = 0;
		for (; i < RADIX; i++) {
			perm[i] = static_cast<unsigned>(i);
		}
		for (; i < fill - 1; i++) {
			perm[i] = static_cast<unsigned>(i + (NSCRATCH - RADIX));
		}
		for (; i < nperm(); i++) {
			perm[i] = static_cast<unsigned>(i - (fill - 1) + RADIX);
		}
		Size n_tail = n % block_size;
		std::memcpy(blockat(RADIX), pairs + block_size * (n / block_size), n_tail * sizeof(Rec));
		for (auto &p : partials) {
			p.index = 0;
			p.length = 0;
		}
		partials[0].index = static_cast<unsigned>(fill - 1);
		partials[0].length = static_cast<unsigned>(n_tail);
	}

	void step(unsigned shift) {
		Bucket buckets[RADIX];
		unsigned counts[RADIX];
		Size i_out, i_in, i_partial;

		for (i_out = 0; i_out < RADIX; i_out++) {
			Rec *out_blk = blockat(perm[i_out]);
			buckets[i_out].start = out_blk;
			buckets[i_out].end = out_blk + block_size;
			assignments[i_out] = static_cast<unsigned char>(i_out);
			counts[i_out] = 1;
		}

		i_partial = 0;
		for (i_in = RADIX; i_in < fill; i_in++) {
			Rec *in_blk = blockat(perm[i_in]);
			Size len = blocksize(i_in, &i_partial);

			for (Size j = 0; j < len; j++) {
				unsigned bucket = bucketof(in_blk[j], shift);
				Rec *bucketp = buckets[bucket].start;
				std::memcpy(bucketp++, in_blk + j, sizeof(Rec));
				buckets[bucket].start = bucketp;
				if (bucketp == buckets[bucket].end) {
					Rec *out_blk = blockat(perm[i_out]);
					buckets[bucket].start = out_blk;
					buckets[bucket].end = out_blk + block_size;
					assignments[i_out] = static_cast<unsigned char>(bucket);
					counts[bucket]++;
					i_out++;
				}
			}
		}

		fill = i_out;
		fix_perm(counts, buckets);
	}

	void fix_perm(unsigned counts[RADIX], Bucket buckets[RADIX]) {
		unsigned *newperm = perm2;
		Size starts[RADIX];

		starts[0] = RADIX;
		for (Size i = 1; i < RADIX; i++) {
			starts[i] = starts[i - 1] + counts[i - 1];
		}

		Size i;
		for (i = 0; i < fill; i++) {
			int bucket = assignments[i];
			Size j = starts[bucket]++;
			newperm[j] = perm[i];
			if (blockat(perm[i]) + block_size == buckets[bucket].end) {
				partials[bucket].index = static_cast<unsigned>(j);
				partials[bucket].length =
				    static_cast<unsigned>(block_size - (buckets[bucket].end - buckets[bucket].start));
			}
		}

		assert(i + RADIX <= nperm());
		std::memcpy(newperm, perm + i, RADIX * sizeof *perm);
		i += RADIX;
		std::memcpy(newperm + i, perm + i, (nperm() - i) * sizeof *perm);

		fill += RADIX;
		perm2 = perm;
		perm = newperm;
	}

	void compact() {
		unsigned *p = perm;
		unsigned *pinv = perm2;
		for (Size i = 0; i < nperm(); i++) {
			pinv[p[i]] = static_cast<unsigned>(i);
		}

		Size start = 0;
		Size i_partial = 0;
		Size j_free = p[0];
		Size i_free = 0;
		Size j_out = NSCRATCH;

		for (; j_out < nperm(); j_out++) {
			Size i_in = j_out - RADIX;
			Size i_out = pinv[j_out];
			Size j_in = p[i_in];

			if (i_out > i_in && i_out < fill) {
				std::memcpy(blockat(j_free), blockat(j_out), sizeof(Rec) * block_size);
				p[i_free] = static_cast<unsigned>(j_out);
				p[i_out] = static_cast<unsigned>(j_free);
				pinv[j_free] = static_cast<unsigned>(i_out);
				pinv[j_out] = static_cast<unsigned>(i_free);

				i_free = i_out;
				i_out = pinv[j_out];
			}

			Size blocklen = blocksize(i_in, &i_partial);
			std::memmove(pairs + start, blockat(j_in), blocklen * sizeof(Rec));
			p[i_in] = static_cast<unsigned>(j_out);
			p[i_out] = static_cast<unsigned>(j_in);
			pinv[j_in] = static_cast<unsigned>(i_out);
			pinv[j_out] = static_cast<unsigned>(i_in);
			start += blocklen;

			if (i_in != i_out) {
				j_free = j_in;
				i_free = i_out;
			}
		}

		for (Size i_in = j_out - RADIX; i_in < fill; i_in++) {
			Size j_in = p[i_in];
			Size blocklen = blocksize(i_in, &i_partial);
			std::memcpy(pairs + start, blockat(j_in), blocklen * sizeof(Rec));
			start += blocklen;
		}

		assert(start == n);
	}

	Rec *pairs;
	Size n;
	Size block_size;
	Size fill;
	unsigned *perm;
	unsigned *perm2;
	unsigned char *assignments;
	Partial partials[RADIX];
	Rec *scratch;
};

// The block size the Rust backend derives: b = max(ceil(4*sqrt(9n/(2*R*s))),
// floor(1024/s), 1) with R = 2^RSHIFT and s = sizeof(Rec).
template <class Rec, unsigned RSHIFT>
std::size_t block_policy(std::size_t n) {
	const double record_bytes = double(sizeof(Rec));
	const double balanced = std::sqrt(9.0 * double(n) / (2.0 * double(1u << RSHIFT) * record_bytes));
	std::size_t block = std::size_t(std::ceil(4.0 * balanced));
	std::size_t floor = std::size_t(1024.0 / record_bytes);
	return block < floor ? floor : block;
}

} // namespace prs
