// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2025, 2026 Robert Clausecker <clausecker@zib.de>
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
// TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
// PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
// TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

//! Stable block LSD radix sort adapted from Robert Clausecker's
//! BSD-2-Clause C implementation, `radixsort_permuted.c`.
//! Reference revision: f69e816c3cd79d312cd67aea5b9cf1c338c1b371.
//! <https://github.com/clausecker/radsort/blob/f69e816c3cd79d312cd67aea5b9cf1c338c1b371/radixsort_permuted.c>
//! <https://arxiv.org/abs/2607.05302>
//!
//! Ownership invariant: the active block sequence contains exactly one logical
//! owner of every record. Other physical copies are stale bytes, never dropped
//! or borrowed as records. Key extraction borrows only a live source record.
//! Scratch is MaybeUninit<T>; before returning or resuming a key panic, compact
//! moves every live owner back into the input slice. Payloads need no Copy/Clone.

use super::key::{Key, Unsigned};
use std::{marker::PhantomData, mem::MaybeUninit, ptr};

type SortKey<T> = <T as Key>::Value;
type PanicPayload = Box<dyn std::any::Any + Send>;

// Position of the first uncommitted record. A key panic happens before writing
// that record, so the committed outputs plus this unread suffix still own the
// complete logical input sequence.
struct ScatterPosition {
    input: usize,
    offset: usize,
    output: usize,
}

/// # Block LSD stable sort
///
/// Sort by converted radix key, preserving the order of equal keys.
/// `radix` is the number of bits per digit (1..=8). The block size is chosen
/// from the input length and element size; 8 bits per digit is recommended.
///
/// For n elements of s bytes and R = 2^radix buckets, the block size in elements
/// is `max(1, ceil(4 * sqrt(9*n / (2*R*s))), floor(1024/s))`. This balances the
/// block scratch and descriptor storage with an approximately 1 KiB block
/// floor. The coefficient is tuned for radix 8, not proven optimal.
///
/// This single-threaded sort uses O(sqrt(n)) auxiliary space for fixed radix
/// and record size. Nonconstant inputs use the block backend at every size;
/// constant digits are skipped.
///
/// # Panics
///
/// Panics on invalid parameters or unrepresentable allocation sizes or block
/// indices. If key extraction panics, the original record set is retained,
/// including payloads, and its order is unspecified.
///
/// # Example
///
/// ```
/// use sort_research_rs::other::rust_block_lsd::sort;
/// let mut arr = [42u32, 7, 42, 0, u32::MAX];
/// sort(&mut arr);
/// assert_eq!(arr, [0, 7, 42, 42, u32::MAX]);
/// ```
pub fn sort<T>(arr: &mut [T], radix: usize)
where
    T: Key,
{
    // Widths above 8 are only reachable from the experiment harness; the
    // adapter keeps 8, which the block-size coefficient is tuned for.
    assert!((1..=12).contains(&radix), "radix must be in 1..=12");
    let record_bytes = std::mem::size_of::<T>();
    if arr.len() < 2 || record_bytes == 0 {
        return;
    }
    sort_with_block_size(
        arr,
        radix,
        default_block_size(arr.len(), record_bytes, radix),
    );
}

/// Sorts with an explicit block size. `sort` derives one from the input; the
/// reference implementation uses a fixed 512. Exposed so tests and benchmarks can
/// pin the configuration instead of guessing it from `n`.
pub fn sort_with_block_size<T>(arr: &mut [T], radix: usize, block_size: usize)
where
    T: Key,
{
    let (union, intersection) = varying_key_bits::<T>(arr);
    sort_with_variation(arr, radix, block_size, union, intersection);
}

/// Same sort, with a variation scan the caller already paid for.
pub(crate) fn sort_with_variation<T>(
    arr: &mut [T],
    radix: usize,
    block_size: usize,
    union: SortKey<T>,
    intersection: SortKey<T>,
) where
    T: Key,
{
    if union == intersection {
        return;
    }
    // The C reference has four byte passes. Derive masks from the converted
    // key width to handle wide keys and partial digits without truncation.
    let bits = T::Value::BITS;
    let range = 1usize << radix;
    let levels = bits.div_ceil(radix);
    let mut sorter = Sorter::new(arr, range, block_size);
    for level in 0..levels {
        let shift = level * radix;
        let width = radix.min(bits - shift);
        let mask = T::Value::from_usize((1usize << width) - 1) << T::Value::from_usize(shift);
        if union & mask != intersection & mask {
            if let Some(payload) = sorter.step(mask, shift) {
                // Finish materializing a permutation before resuming the user's
                // panic. This uses existing blocks, without an O(n) backup.
                sorter.compact();
                std::panic::resume_unwind(payload);
            }
        }
    }
    sorter.compact();
}

/// Records per block for the block-permutation engine.
///
/// Grow blocks with n so scratch and the block tables both stay O(sqrt(n)).
/// With b elements per block, scratch costs 2*R*b*s bytes. The two u32
/// permutation tables and the bucket IDs cost about 9*n/b bytes. Their sum is
/// minimized at b = sqrt(9*n / (2*R*s)). Convert before multiplying to avoid
/// overflowing usize on large slices.
///
/// Both constants below are deviations from the reference implementation, which
/// uses a fixed 512 records because it only ever sorts one record type.
/// Ablation over the benchmark types found this policy faster than the fixed
/// size at every length below ~4M records and equal above it, while the
/// auxiliary space now follows sqrt(n).
pub(crate) fn default_block_size(len: usize, record_bytes: usize, radix: usize) -> usize {
    const SQRT_SCALE: f64 = 4.0;
    const BLOCK_BYTES_FLOOR: usize = 1024;
    let range = 1usize << radix;
    let descriptor_bytes = 9.0 * len as f64;
    let scratch_bytes_per_element = 2.0 * range as f64 * record_bytes as f64;
    let balanced = (descriptor_bytes / scratch_bytes_per_element).sqrt();
    // Favor larger blocks to reduce descriptor and rollover work, at the cost
    // of more scratch. The factor 4 is tuned for radix 8. A roughly 1 KiB floor
    // limits bookkeeping when the square-root optimum gives tiny blocks;
    // records larger than that still need at least one element per block.
    let scaled = (SQRT_SCALE * balanced).ceil() as usize;
    scaled.max(BLOCK_BYTES_FLOOR / record_bytes).max(1)
}

/// One stable partition pass by the digit selected by `shift` and `radix`.
///
/// The engine keeps its records scattered over physical blocks; after this call
/// the slice again holds them contiguously, ordered by that digit and stable
/// within each bucket. Used by the MSD front end of the hybrid sorts.
#[cfg(feature = "rust_tessera_radix")]
pub(crate) fn partition_digit<T: Key>(arr: &mut [T], radix: usize, shift: usize) {
    let record_bytes = std::mem::size_of::<T>();
    if arr.len() < 2 || record_bytes == 0 {
        return;
    }
    assert!((1..=12).contains(&radix), "radix must be in 1..=12");
    let bits = T::Value::BITS;
    assert!(shift < bits, "digit shift outside the key");
    let width = radix.min(bits - shift);
    let mask = (T::Value::from_usize((1usize << width) - 1)) << T::Value::from_usize(shift);
    let block_size = default_block_size(arr.len(), record_bytes, radix);
    let mut sorter = Sorter::new(arr, 1usize << radix, block_size);
    if let Some(payload) = sorter.step(mask, shift) {
        // Same recovery contract as `sort`: materialize the permutation, then
        // let the caller's panic continue.
        sorter.compact();
        std::panic::resume_unwind(payload);
    }
    sorter.compact();
}


pub(crate) fn varying_key_bits<T: Key>(arr: &[T]) -> (SortKey<T>, SortKey<T>) {
    let dummy = &arr[0];
    // The C reference always scatters every digit. Sparse and repeated keys
    // would pay for passes that cannot change their order. Union/intersection
    // identifies those digits without allocating a histogram or adding a
    // BitXor bound to Unsigned.
    let mut union = dummy.key();
    let mut intersection = union;
    let bits = T::Value::BITS;
    let zero = T::Value::from_usize(0);
    let mut full_mask = zero;
    // Build the complete key mask in bytes, including keys wider than usize.
    for shift in (0..bits).step_by(8) {
        full_mask |= T::Value::from_usize((1usize << 8.min(bits - shift)) - 1)
            << T::Value::from_usize(shift);
    }
    // A full pre-scan adds work to wide random inputs. Stop after this prefix
    // if every key bit already varies; otherwise use one uninterrupted,
    // vectorizable scan for sparse or skewed keys. The reference implementation
    // has no such scan and always scatters every digit; this pass turns whole
    // passes into no-ops on constant digits, and costs one early-exiting read
    // of the first 1025 records otherwise.
    let boundary = arr.len().min(1025);
    for item in &arr[1..boundary] {
        let key = item.key();
        union |= key;
        intersection = intersection & key;
    }
    if union != full_mask || intersection != zero {
        for item in &arr[boundary..] {
            let key = item.key();
            union |= key;
            intersection = intersection & key;
        }
    }
    (union, intersection)
}

/// Branch hint for a condition that is almost never true, without a nightly
/// feature. The cold marker tells the optimiser to keep this path out of line,
/// so the hot loop falls through on the common case instead of taking a jump
/// over the rare one. Same shape as the `branches` crate.
#[cold]
#[inline(never)]
fn cold_path() {}

#[inline]
fn unlikely(condition: bool) -> bool {
    if condition {
        cold_path();
    }
    condition
}

#[derive(Clone, Copy)]
struct PartialBlock {
    order_index: usize,
    // Only this prefix contains initialized records.
    len: usize,
}

struct BucketWriter<T> {
    next: *mut T,
    end: *mut T,
}

// Unlike C's fixed integer pairs, generic T may contain invalid zero bit patterns.
// Reserve uninitialized slots and read only recorded live prefixes. The borrow
// marker retains exclusive input access while raw pointers address the two fixed
// allocations; physical IDs name scratch first, then full input blocks.
struct BlockStorage<'a, T> {
    input: *mut T,
    input_len: usize,
    block_size: usize,
    scratch_blocks: usize,
    block_count: usize,
    scratch: Vec<MaybeUninit<T>>,
    input_borrow: PhantomData<&'a mut [T]>,
}

impl<'a, T> BlockStorage<'a, T> {
    fn new(input: &'a mut [T], bucket_count: usize, block_size: usize) -> Self {
        let scratch_blocks = 2 * bucket_count;
        let full_blocks = input.len() / block_size;
        let block_count = full_blocks
            .checked_add(scratch_blocks)
            .expect("block count overflow");
        assert!(block_count <= u32::MAX as usize);
        let scratch_len = scratch_blocks
            .checked_mul(block_size)
            .expect("scratch size overflow");
        let mut scratch = Vec::with_capacity(scratch_len);
        scratch.resize_with(scratch_len, MaybeUninit::<T>::uninit);
        // The input tail has no full physical block. Start it in scratch;
        // the first bucket_count scratch blocks remain free for output.
        let tail_len = input.len() % block_size;
        // SAFETY: the source has tail_len initialized records and the disjoint
        // scratch destination has room for at least that many records.
        unsafe {
            ptr::copy_nonoverlapping(
                input.as_ptr().add(full_blocks * block_size),
                scratch
                    .as_mut_ptr()
                    .cast::<T>()
                    .add(bucket_count * block_size),
                tail_len,
            );
        }
        Self {
            input: input.as_mut_ptr(),
            input_len: input.len(),
            block_size,
            scratch_blocks,
            block_count,
            scratch,
            input_borrow: PhantomData,
        }
    }

    fn block_ptr(&mut self, physical: usize) -> *mut T {
        assert!(physical < self.block_count);
        // SAFETY: the bounds check selects a full block in one of the two
        // allocations, which stay alive and are never resized during sorting.
        unsafe {
            if physical < self.scratch_blocks {
                self.scratch
                    .as_mut_ptr()
                    .cast::<T>()
                    .add(physical * self.block_size)
            } else {
                self.input
                    .add((physical - self.scratch_blocks) * self.block_size)
            }
        }
    }
}

// At pass boundaries, block_order maps logical order to physical storage.
// Its bucket_count..active_blocks_end range holds the current record sequence;
// the prefix and suffix are free blocks. partial_blocks records short prefixes.
struct Sorter<'a, T> {
    blocks: BlockStorage<'a, T>,
    bucket_count: usize,
    block_order: Vec<u32>,
    next_block_order: Vec<u32>,
    // Bucket IDs for output blocks, used to build next_block_order after scatter.
    // u16 keeps digit widths above 8 available to the experiment harness.
    bucket_ids: Vec<u16>,
    partial_blocks: Vec<PartialBlock>,
    active_blocks_end: usize,
    input_materialized: bool,
}

impl<'a, T> Sorter<'a, T> {
    fn new(input: &'a mut [T], bucket_count: usize, block_size: usize) -> Self {
        let full_blocks = input.len() / block_size;
        let tail_len = input.len() % block_size;
        let blocks = BlockStorage::new(input, bucket_count, block_size);
        let active_blocks_end = bucket_count + full_blocks + 1;
        let mut block_order = vec![0; blocks.block_count];
        for (i, block) in block_order.iter_mut().enumerate() {
            *block = if i < bucket_count {
                i
            } else if i < active_blocks_end - 1 {
                i + bucket_count
            } else {
                i - (active_blocks_end - 1) + bucket_count
            } as u32;
        }
        let mut partial_blocks = vec![
            PartialBlock {
                order_index: blocks.block_count,
                len: 0
            };
            bucket_count
        ];
        partial_blocks[0] = PartialBlock {
            order_index: active_blocks_end - 1,
            len: tail_len,
        };
        let block_count = blocks.block_count;
        Self {
            blocks,
            bucket_count,
            block_order,
            next_block_order: vec![0; block_count],
            bucket_ids: vec![0; block_count],
            partial_blocks,
            active_blocks_end,
            input_materialized: false,
        }
    }

    fn next_block_len(&self, logical: usize, partial: &mut usize) -> usize {
        if *partial < self.partial_blocks.len()
            && self.partial_blocks[*partial].order_index == logical
        {
            let length = self.partial_blocks[*partial].len;
            *partial += 1;
            length
        } else {
            self.blocks.block_size
        }
    }

    #[cfg(debug_assertions)]
    fn validate(&self) {
        let mut seen = vec![false; self.block_order.len()];
        for &physical in &self.block_order {
            assert!(!seen[physical as usize]);
            seen[physical as usize] = true;
        }
        assert!(self.active_blocks_end <= self.block_order.len());
        assert!(self
            .partial_blocks
            .windows(2)
            .all(|p| p[0].order_index < p[1].order_index));
        assert!(self
            .partial_blocks
            .iter()
            .all(|p| p.order_index >= self.bucket_count
                && p.order_index < self.active_blocks_end
                && p.len < self.blocks.block_size));
        let mut partial = 0;
        let total: usize = (self.bucket_count..self.active_blocks_end)
            .map(|i| self.next_block_len(i, &mut partial))
            .sum();
        assert_eq!(total, self.blocks.input_len);
    }

    /// One digit pass: scatter every record into its bucket, then rebuild the
    /// block order from the order the buckets filled.
    fn step(&mut self, mask: SortKey<T>, shift: usize) -> Option<PanicPayload>
    where
        T: Key,
    {
        let (mut buckets, mut counts) = self.begin_pass();
        let mut position = ScatterPosition {
            input: self.bucket_count,
            offset: 0,
            output: self.bucket_count,
        };
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let shift_bits = T::Value::from_usize(shift);
            self.scatter(&mut buckets, &mut counts, &mut position, |value| {
                ((value.key() & mask) >> shift_bits).to_usize()
            });
        }))
        .err();
        if panic.is_some() {
            // Do not call a failed extractor again. Appending every unread
            // record to bucket zero completes a valid (unsorted) pass. Recycling
            // still follows the original read-before-reserve protocol.
            self.scatter(&mut buckets, &mut counts, &mut position, |_| 0);
        }
        self.active_blocks_end = position.output;
        self.finish_pass(&buckets, counts);
        #[cfg(debug_assertions)]
        self.validate();
        panic
    }

    /// Reserve the first `bucket_count` free blocks, one per bucket, and give
    /// every bucket an empty write cursor. Counts start at one because that
    /// block is already assigned to the bucket.
    fn begin_pass(&mut self) -> (Vec<BucketWriter<T>>, Vec<usize>) {
        let mut buckets = Vec::with_capacity(self.bucket_count);
        for i in 0..self.bucket_count {
            let out = self.blocks.block_ptr(self.block_order[i] as usize);
            // SAFETY: block_ptr() returns storage for block_size elements.
            buckets.push(BucketWriter {
                next: out,
                end: unsafe { out.add(self.blocks.block_size) },
            });
            self.bucket_ids[i] = i as u16;
        }
        (buckets, vec![1; self.bucket_count])
    }

    /// Rebuild the logical block order from the order the buckets filled, and
    /// record where each bucket's partially filled last block sits.
    fn finish_pass(&mut self, buckets: &[BucketWriter<T>], counts: Vec<usize>) {
        // A separate starts Vec would allocate once per digit. Counts are dead
        // after scatter, so reuse their allocation for prefix offsets and then
        // advance those offsets while rebuilding the block order.
        let mut starts = counts;
        let mut start = self.bucket_count;
        for count in &mut starts {
            let next = start + *count;
            *count = start;
            start = next;
        }
        for i in 0..self.active_blocks_end {
            let b = self.bucket_ids[i] as usize;
            let j = starts[b];
            starts[b] += 1;
            self.next_block_order[j] = self.block_order[i];
            let base = self.blocks.block_ptr(self.block_order[i] as usize);
            // SAFETY: next/end belong to the same block-sized allocation.
            if unsafe { base.add(self.blocks.block_size) } == buckets[b].end {
                self.partial_blocks[b] = PartialBlock {
                    order_index: j,
                    len: unsafe { buckets[b].next.offset_from(base) as usize },
                };
            }
        }
        assert!(self.active_blocks_end + self.bucket_count <= self.block_order.len());
        self.next_block_order[..self.bucket_count].copy_from_slice(
            &self.block_order[self.active_blocks_end..self.active_blocks_end + self.bucket_count],
        );
        self.next_block_order[self.active_blocks_end + self.bucket_count..]
            .copy_from_slice(&self.block_order[self.active_blocks_end + self.bucket_count..]);
        self.active_blocks_end += self.bucket_count;
        std::mem::swap(&mut self.block_order, &mut self.next_block_order);
    }

    // Pass setup and block reordering need state that scatter does not.
    // Keep their optimization regions separate so that state does not force
    // digit masks and bucket cursors out of registers in the hot loop.
    #[inline(never)]
    fn scatter<F: FnMut(&T) -> usize>(
        &mut self,
        buckets: &mut [BucketWriter<T>],
        counts: &mut [usize],
        position: &mut ScatterPosition,
        mut classify: F,
    ) {
        let mut partial = self
            .partial_blocks
            .partition_point(|p| p.order_index < position.input);
        while position.input < self.active_blocks_end {
            let input = position.input;
            let source = self.blocks.block_ptr(self.block_order[input] as usize);
            let length = self.next_block_len(input, &mut partial);
            let begin = position.offset;
            let mut write = |offset: usize| {
                // SAFETY: only the initialized, unread suffix is read. Blocks
                // become output only after all of their input records are read.
                // Borrow the actual live record. Never materialize a second T:
                // it may own a String, Box, Rc, or other non-Copy payload.
                // This borrow ends before the bitwise move below.
                let b = classify(unsafe { &*source.add(offset) });
                // SAFETY: bucket_count is 1 << radix and mask keeps the width
                // bits below radix, so the index is in bounds; the check would
                // otherwise run once per record in the hot loop.
                let bucket = unsafe { buckets.get_unchecked_mut(b) };
                let next = bucket.next;
                let end = bucket.end;
                // SAFETY: next is an unused cell of a reserved output block;
                // source and destination may overlap, so use ptr::copy.
                let next = unsafe {
                    ptr::copy(source.add(offset), next, 1);
                    next.add(1)
                };
                bucket.next = next;
                // Exact recovery offset. A panic can only come from the key
                // extractor above, so the next unread record must be recorded
                // before the move, otherwise recovery duplicates or loses a
                // record. Only types with destructors can observe that: a
                // duplicated Copy record is inert bits. The store is therefore
                // skipped for them, which LLVM folds away per concrete type and
                // which measured 3% of the hot loop on 8 byte records.
                if const { core::mem::needs_drop::<T>() } {
                    position.offset = offset + 1;
                }
                if unlikely(next == end) {
                    debug_assert!(position.output <= input);
                    let out = self
                        .blocks
                        .block_ptr(self.block_order[position.output] as usize);
                    *bucket = BucketWriter {
                        next: out,
                        end: unsafe { out.add(self.blocks.block_size) },
                    };
                    self.bucket_ids[position.output] = b as u16;
                    counts[b] += 1;
                    position.output += 1;
                }
            };
            // Four records per bucket-cursor reload. Measured cheaper than one
            // record at a time on 8 byte records; the size scaling keeps the
            // reload amortization for wider records without spilling cursors.
            let group =
                (4 * std::mem::size_of::<usize>() / std::mem::size_of::<T>().max(1)).clamp(1, 4);
            let complete = begin + (length - begin) / group * group;
            for start in (begin..complete).step_by(group) {
                for offset in 0..group {
                    write(start + offset);
                }
            }
            for offset in complete..length {
                write(offset);
            }
            position.input += 1;
            position.offset = 0;
        }
    }

    fn compact(&mut self) {
        // Sorting is finished. Reuse the output buffer as the inverse map:
        // physical block ID -> position in block_order.
        let mut block_positions = std::mem::take(&mut self.next_block_order);
        for i in 0..self.block_order.len() {
            block_positions[self.block_order[i] as usize] = i as u32;
        }
        let mut start = 0;
        let mut partial = 0;
        let mut free_logical = 0;
        let mut free_physical = self.block_order[free_logical] as usize;
        for destination in self.blocks.scratch_blocks..self.block_order.len() {
            let input = destination - self.bucket_count;
            let mut output = block_positions[destination] as usize;
            let source = self.block_order[input] as usize;
            if output > input && output < self.active_blocks_end {
                debug_assert!(
                    free_logical < self.bucket_count || free_logical >= self.active_blocks_end
                );
                let from = self.blocks.block_ptr(destination);
                let to = self.blocks.block_ptr(free_physical);
                // SAFETY: both physical IDs are valid full blocks. Free block
                // receives live output which would otherwise be overwritten.
                // ptr::copy preserves uninitialized padding without reading it
                // as T; later element reads use only the live prefix.
                unsafe {
                    ptr::copy(from, to, self.blocks.block_size);
                }
                self.block_order[free_logical] = destination as u32;
                self.block_order[output] = free_physical as u32;
                block_positions[free_physical] = output as u32;
                block_positions[destination] = free_logical as u32;
                free_logical = output;
                output = block_positions[destination] as usize;
            }
            let length = self.next_block_len(input, &mut partial);
            let from = self.blocks.block_ptr(source);
            assert!(start + length <= self.blocks.input_len);
            // Completed partials can only shorten preceding output: start <=
            // (destination - scratch_blocks) * block_size, and
            // start + length never exceeds this destination block's end.
            // Any still-live destination block was evacuated above; earlier
            // physical destinations are already consumed.
            // SAFETY: length valid source elements are moved into the next
            // logical output interval. ptr::copy permits overlap as memmove.
            unsafe {
                ptr::copy(from, self.blocks.input.add(start), length);
            }
            self.block_order[input] = destination as u32;
            self.block_order[output] = source as u32;
            block_positions[source] = output as u32;
            block_positions[destination] = input as u32;
            start += length;
            if input != output {
                free_physical = source;
                free_logical = output;
            }
        }
        for input in self.block_order.len() - self.bucket_count..self.active_blocks_end {
            let source = self.block_order[input] as usize;
            assert!(source < self.blocks.scratch_blocks);
            let length = self.next_block_len(input, &mut partial);
            let from = self.blocks.block_ptr(source);
            assert!(start + length <= self.blocks.input_len);
            unsafe {
                ptr::copy(from, self.blocks.input.add(start), length);
            }
            start += length;
        }
        assert_eq!(start, self.blocks.input_len);
        self.input_materialized = true;
    }
}

impl<T> Drop for Sorter<'_, T> {
    fn drop(&mut self) {
        // At pass boundaries the directory is a complete owning sequence.
        // Restore it even if unwinding starts outside key extraction (for
        // example an allocation failure with an unwinding error handler).
        // Types without destructors do not maintain the recovery offset and
        // cannot observe a duplicated record, so there is nothing to restore.
        if !self.input_materialized && const { core::mem::needs_drop::<T>() } {
            self.compact();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_digit_widths_and_explicit_block_boundaries() {
        // Miri interprets every instruction and every 8-bit pass fills
        // 2*256*block_size bucket slots, so it runs a reduced shape: a partial
        // final digit (3), a full byte digit (8), one small block, and inputs
        // up to one block plus one.
        let radixes: &[usize] = if cfg!(miri) {
            &[3, 8]
        } else {
            &[1, 2, 3, 4, 5, 6, 7, 8]
        };
        let block_sizes: &[usize] = if cfg!(miri) {
            &[3]
        } else {
            &[1, 3, 17, 64, 128]
        };
        let max_len = if cfg!(miri) { 129 } else { 1025 };
        for &radix in radixes {
            for &block_size in block_sizes {
                for len in [
                    2,
                    block_size * 2 - 1,
                    block_size * 2,
                    block_size * 2 + 1,
                    max_len,
                ] {
                    let mut input: Vec<u128> = (0..len)
                        .map(|i| {
                            // Exercise low and high bits, including a partial final digit.
                            let x = (i as u128).wrapping_mul(0x9e3779b97f4a7c15);
                            (x << 64) | (x.rotate_left(17) % 311)
                        })
                        .collect();
                    let mut expected = input.clone();
                    expected.sort();
                    sort_with_block_size(&mut input, radix, block_size);
                    assert_eq!(
                        input, expected,
                        "radix={radix}, block={block_size}, len={len}"
                    );
                }
            }
        }
    }

    #[test]
    fn unwind_between_passes_restores_owners() {
        use std::{cell::Cell, rc::Rc};
        struct Owned {
            key: u64,
            id: usize,
            payload: Box<usize>,
            drops: Rc<Cell<usize>>,
        }
        impl Key for Owned {
            type Value = u64;
            fn key(&self) -> u64 {
                self.key
            }
        }
        impl Drop for Owned {
            fn drop(&mut self) {
                self.drops.set(self.drops.get() + 1);
            }
        }
        let drops = Rc::new(Cell::new(0));
        let mut data: Vec<_> = (0..1025)
            .map(|id| Owned {
                key: (id % 7) as u64,
                id,
                payload: Box::new(id),
                drops: drops.clone(),
            })
            .collect();
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut sorter = Sorter::new(&mut data, 256, 3);
            assert!(sorter.step(255, 0).is_none());
            std::panic::panic_any(17usize);
        }))
        .unwrap_err();
        assert_eq!(panic.downcast_ref::<usize>(), Some(&17));
        assert_eq!(drops.get(), 0);
        data.sort_unstable_by_key(|v| v.id);
        assert!(data
            .iter()
            .enumerate()
            .all(|(i, v)| v.id == i && *v.payload == i));
        drop(data);
        assert_eq!(drops.get(), 1025);
    }

    #[test]
    fn float_total_order_preserves_every_bit() {
        macro_rules! check {
            ($float:ty, $bits:ty, $nan:expr, $negative_nan:expr) => {{
                let radixes: &[usize] = if cfg!(miri) {
                    &[8]
                } else {
                    &[1, 2, 3, 4, 5, 6, 7, 8]
                };
                let cyclic_len = if cfg!(miri) { 129 } else { 1025 };
                let random_len = if cfg!(miri) { 64 } else { 4096 };
                for &radix in radixes {
                    let special: [$bits; 8] = [
                        <$float>::NEG_INFINITY.to_bits(),
                        (-1.0 as $float).to_bits(),
                        (-0.0 as $float).to_bits(),
                        (0.0 as $float).to_bits(),
                        (1.0 as $float).to_bits(),
                        <$float>::INFINITY.to_bits(),
                        $nan,
                        $negative_nan,
                    ];
                    let mut input: Vec<_> = special
                        .into_iter()
                        .cycle()
                        .take(cyclic_len)
                        .chain((0..random_len).map(|i| (i as $bits).wrapping_mul(0x9e3779b9)))
                        .map(<$float>::from_bits)
                        .collect();
                    let mut expected = input.clone();
                    expected.sort_by(<$float>::total_cmp);
                    // The automatic block size has a ~1 KiB floor, which costs
                    // 2*256*block_size slot writes per pass. Miri uses an
                    // explicit small block instead; the floor is covered by the
                    // release and ASan runs.
                    if cfg!(miri) {
                        sort_with_block_size(&mut input, radix, 3);
                    } else {
                        sort(&mut input, radix);
                    }
                    assert_eq!(
                        input.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                        expected.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                    );
                }
            }};
        }
        check!(f32, u32, 0x7fc00001, 0xffc00002);
        check!(f64, u64, 0x7ff8000000000001, 0xfff8000000000002);
    }
}
