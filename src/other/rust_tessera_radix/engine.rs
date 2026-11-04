//! Tessera: a stable radix sort that tiles the input.
//!
//! Successor of the "PA" experiments. The top of the recursion partitions by the
//! most significant varying digit with the block-permutation engine from
//! [`rust_block_lsd`](crate::other::rust_block_lsd); every child that fits the tile
//! budget is finished with an ordinary LSD counting pass over a private buffer.
//! The block engine keeps the auxiliary space at the same O(sqrt(n)) as block
//! LSD, and the LSD leaves run entirely inside a cache tile.
//!
//! Why this shape: at n = 1M, an 8-bit LSD pass costs about 5.5 ns per record
//! because every pass streams the whole array through memory. After one MSD
//! split at 256 buckets the children are ~31 KiB, so their remaining seven
//! passes stay in L1/L2 and cost roughly a quarter as much together as one cold
//! pass. Skewed inputs are handled by the per-node variation filter: a child in
//! which a digit is constant does not pay for that digit at all.

use std::mem::MaybeUninit;

use crate::other::radix_support::{Key, Unsigned};
use crate::other::rust_block_lsd::core as block;

type SortKey<T> = <T as Key>::Value;

/// Bits per digit. The block engine's block-size coefficient is tuned for this
/// width, and 256 leaf buckets keep the cursor array in L1.
const RADIX: usize = 8;
/// Largest tile finished with LSD passes, in bytes of record data.
const TILE_BYTES: usize = 64 * 1024;
/// One digit per 8 key bits covers every supported key width. The literal
/// appears in the array types: rustc's layout computation hits an ICE on a
/// const-generic bound spelled through an inner associated constant here.
const MAX_BUCKETS: usize = 1 << RADIX;

/// One digit of the converted key, highest digit first in a [`Plan`].
#[derive(Clone, Copy)]
struct Digit {
    shift: usize,
    width: usize,
}

impl Digit {
    #[inline]
    fn mask<T: Key>(self) -> SortKey<T> {
        (T::Value::from_usize((1usize << self.width) - 1)) << T::Value::from_usize(self.shift)
    }

    #[inline]
    fn extract<T: Key>(self, key: SortKey<T>) -> usize {
        ((key & self.mask::<T>()) >> T::Value::from_usize(self.shift)).to_usize()
    }

    #[inline]
    fn varies<T: Key>(self, union: SortKey<T>, intersection: SortKey<T>) -> bool {
        let mask = self.mask::<T>();
        union & mask != intersection & mask
    }
}

/// The digits still worth a pass for one node, highest first.
///
/// A fixed inline array keeps recursion free of per-node allocation; the width
/// bound is the key width divided by the digit width.
#[derive(Clone, Copy)]
struct Plan {
    digits: [Digit; 16],
    len: usize,
}

impl Plan {
    fn all<T: Key>() -> Self {
        let bits = T::Value::BITS;
        let mut plan = Plan {
            digits: [Digit { shift: 0, width: 0 }; 16],
            len: 0,
        };
        for shift in (0..bits).step_by(RADIX).rev() {
            debug_assert!(plan.len < plan.digits.len());
            plan.digits[plan.len] = Digit {
                shift,
                width: RADIX.min(bits - shift),
            };
            plan.len += 1;
        }
        plan
    }

    /// Keeps only the digits whose bits vary between `union` and
    /// `intersection`. Constant digits cannot change the order, so no pass runs
    /// for them; this is what keeps a node full of duplicate keys cheap.
    fn filter<T: Key>(self, union: SortKey<T>, intersection: SortKey<T>) -> Self {
        let mut out = self;
        out.len = 0;
        for i in 0..self.len {
            let digit = self.digits[i];
            if digit.varies::<T>(union, intersection) {
                out.digits[out.len] = digit;
                out.len += 1;
            }
        }
        out
    }

    fn is_empty(self) -> bool {
        self.len == 0
    }

    /// The same plan without its most significant digit.
    fn tail(self) -> Self {
        debug_assert!(self.len > 0);
        let mut out = self;
        out.digits.copy_within(1..self.len, 0);
        out.len -= 1;
        out
    }
}

struct Context<T> {
    /// Scratch for the LSD leaves; holds the largest tile seen so far.
    scratch: Vec<MaybeUninit<T>>,
    tile_bytes: usize,
}

/// Sorts by converted radix key, stably, in O(n) time and O(sqrt(n)) auxiliary
/// space. See the module documentation for the shape of the algorithm.
pub fn sort<T: Key>(arr: &mut [T]) {
    sort_with_tile_bytes(arr, TILE_BYTES);
}

/// Sorts with an explicit tile budget, in bytes of record data. `sort` uses the
/// tuned default; benchmarks pin the budget to make the trade-off visible.
pub fn sort_with_tile_bytes<T: Key>(arr: &mut [T], tile_bytes: usize) {
    let record_bytes = std::mem::size_of::<T>();
    if arr.len() < 2 || record_bytes == 0 {
        return;
    }
    let tile_bytes = tile_bytes.max(record_bytes);
    let (union, intersection) = block::varying_key_bits(arr);
    let plan = Plan::all::<T>().filter::<T>(union, intersection);
    if plan.is_empty() {
        return;
    }
    let mut context = Context {
        scratch: Vec::new(),
        tile_bytes,
    };
    context.sort_node(arr, plan);
}

impl<T: Key> Context<T> {
    fn sort_node(&mut self, arr: &mut [T], plan: Plan) {
        if arr.len() < 2 {
            return;
        }
        let tile_bytes = self.tile_bytes;
        if plan.is_empty() || arr.len().saturating_mul(std::mem::size_of::<T>()) <= tile_bytes {
            self.lsd_leaf(arr, plan);
            return;
        }
        // MSD split: one stable block-permutation pass over the whole node.
        let digit = plan.digits[0];
        block::partition_digit(arr, digit.width, digit.shift);
        self.descend(arr, digit, plan.tail());
    }

    /// Walks the freshly partitioned node, and sorts each bucket as a separate
    /// child. The walk extracts every key once and accumulates the variation of
    /// the child while the bucket boundary is still open, so a child does not
    /// pay for a second scan of its own keys.
    fn descend(&mut self, arr: &mut [T], digit: Digit, tail: Plan) {
        let mask = digit.mask::<T>();
        let shift = T::Value::from_usize(digit.shift);
        let mut start = 0;
        while start < arr.len() {
            let first = arr[start].key();
            let bucket = ((first & mask) >> shift).to_usize();
            let mut union = first;
            let mut intersection = first;
            let mut end = start + 1;
            while end < arr.len() {
                let key = arr[end].key();
                if ((key & mask) >> shift).to_usize() != bucket {
                    break;
                }
                union |= key;
                intersection = intersection & key;
                end += 1;
            }
            let child = tail.filter::<T>(union, intersection);
            self.sort_node(&mut arr[start..end], child);
            start = end;
        }
    }

    fn set_scratch_len(&mut self, len: usize) {
        if self.scratch.len() < len {
            self.scratch.clear();
            self.scratch.reserve_exact(len);
            self.scratch.resize_with(len, MaybeUninit::<T>::uninit);
        }
    }

    /// Ordinary stable LSD counting sort over the tile, driven by a private
    /// buffer. One pass per remaining digit, ping-ponging between the tile and
    /// the buffer.
    fn lsd_leaf(&mut self, arr: &mut [T], plan: Plan) {
        let len = arr.len();
        if len < 2 || plan.is_empty() {
            return;
        }
        self.set_scratch_len(len);
        let scratch_ptr = self.scratch.as_mut_ptr().cast::<T>();
        let mut counts = [0usize; MAX_BUCKETS];

        // The record set lives in `source` for the whole pass. A panic from the
        // key extractor therefore never loses or duplicates a record: if the
        // source is the tile, the tile is untouched; if it is the scratch
        // buffer, the buffer still holds every record and is copied back.
        let mut live_in_scratch = false;
        let mut run = |live_in_scratch: &mut bool, counts: &mut [usize; MAX_BUCKETS]| {
            let mut source: *mut T = arr.as_mut_ptr();
            let mut destination: *mut T = scratch_ptr;
            let mut source_is_tile = true;
            for index in (0..plan.len).rev() {
                let digit = plan.digits[index];
                let mask = digit.mask::<T>();
                let shift = T::Value::from_usize(digit.shift);
                // SAFETY: the tile holds `len` initialized records, and the
                // scratch buffer has held `len` records since its allocation.
                let records = unsafe { std::slice::from_raw_parts(source, len) };
                counts[..1 << digit.width].fill(0);
                for record in records {
                    counts[((record.key() & mask) >> shift).to_usize()] += 1;
                }
                let mut offset = 0;
                for count in counts[..1 << digit.width].iter_mut() {
                    let current = *count;
                    *count = offset;
                    offset += current;
                }
                *live_in_scratch = !source_is_tile;
                for record in records {
                    let bucket = ((record.key() & mask) >> shift).to_usize();
                    let slot = counts[bucket];
                    counts[bucket] += 1;
                    // SAFETY: `slot` counts the records already placed in this
                    // bucket, so it stays inside that bucket's output interval.
                    unsafe {
                        std::ptr::write(destination.add(slot), std::ptr::read(record));
                    }
                }
                std::mem::swap(&mut source, &mut destination);
                source_is_tile = !source_is_tile;
            }
            source_is_tile
        };

        let ended_in_tile = if const { std::mem::needs_drop::<T>() } {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run(&mut live_in_scratch, &mut counts)
            })) {
                Ok(ended_in_tile) => ended_in_tile,
                Err(payload) => {
                    if live_in_scratch {
                        // SAFETY: the scratch buffer holds the complete record
                        // set; the tile only holds stale duplicates or a stale
                        // prefix from the previous pass.
                        unsafe {
                            std::ptr::copy_nonoverlapping(scratch_ptr, arr.as_mut_ptr(), len);
                        }
                    }
                    std::panic::resume_unwind(payload);
                }
            }
        } else {
            run(&mut live_in_scratch, &mut counts)
        };

        if !ended_in_tile {
            // SAFETY: an odd number of passes left the complete record set in
            // the scratch buffer; the tile slots being overwritten are stale
            // records that were already moved out.
            unsafe {
                std::ptr::copy_nonoverlapping(scratch_ptr, arr.as_mut_ptr(), len);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
    }

    fn check<T: Key + Copy + Ord + std::fmt::Debug>(mut input: Vec<T>, tile_bytes: usize) {
        let mut expected = input.clone();
        expected.sort();
        sort_with_tile_bytes(&mut input, tile_bytes);
        assert_eq!(input, expected);
    }

    #[test]
    fn every_width_and_tile_rounds_to_zero() {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        for &len in &[0usize, 1, 2, 3, 7, 8, 9, 100, 255, 256, 257, 1000, 4097, 20_000] {
            let u32s: Vec<u32> = (0..len).map(|_| rng.next() as u32).collect();
            check(u32s, TILE_BYTES);
            let u64s: Vec<u64> = (0..len).map(|_| rng.next()).collect();
            for &tile in &[1usize, 64, 1024, 8192, TILE_BYTES] {
                check(u64s.clone(), tile);
            }
            let u128s: Vec<u128> = (0..len)
                .map(|_| ((rng.next() as u128) << 64) | rng.next() as u128)
                .collect();
            check(u128s, TILE_BYTES);
        }
    }

    #[test]
    fn duplicates_stay_stable() {
        // Equal keys carry their input index, so a stable sort must keep the
        // index order inside every run of equal keys.
        #[derive(Clone, Copy, Debug)]
        struct Item {
            key: u16,
            index: u32,
        }
        impl PartialEq for Item {
            fn eq(&self, other: &Self) -> bool {
                self.key == other.key && self.index == other.index
            }
        }
        impl Eq for Item {}
        impl PartialOrd for Item {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Item {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.key.cmp(&other.key)
            }
        }
        impl Key for Item {
            type Value = u16;
            fn key(&self) -> u16 {
                self.key
            }
        }

        let mut rng = Rng(7);
        for &len in &[2usize, 100, 5000] {
            let input: Vec<Item> = (0..len)
                .map(|index| Item {
                    key: (rng.next() % 4) as u16,
                    index: index as u32,
                })
                .collect();
            let mut sorted = input.clone();
            sorted.sort_by_key(|item| item.key);
            let mut actual = input.clone();
            sort_with_tile_bytes(&mut actual, TILE_BYTES);
            for (a, e) in actual.iter().zip(sorted.iter()) {
                assert_eq!(a, e);
            }
        }
    }

    #[test]
    fn constant_keys_are_no_ops() {
        let mut input = vec![42u64; 10_000];
        let expected = input.clone();
        sort(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn skewed_keys_sort() {
        let mut rng = Rng(11);
        let mut input: Vec<u32> = Vec::new();
        for _ in 0..50_000 {
            input.push(0);
        }
        for _ in 0..1_000 {
            input.push(rng.next() as u32);
        }
        check(input, TILE_BYTES);
    }
}
