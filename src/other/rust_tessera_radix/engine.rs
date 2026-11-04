//! Tessera: a stable radix sort that tiles the input.
//!
//! Successor of the "PA" experiments. The top of the recursion partitions by the
//! most significant bits that still differ, using the block-permutation engine
//! from [`rust_block_lsd`](crate::other::rust_block_lsd); every child that fits
//! the tile budget is finished with stable counting passes over a private
//! buffer. The block engine keeps the auxiliary space at the same O(sqrt(n)) as
//! block LSD, and the leaves run inside a cache tile whose size is a constant.
//!
//! Why this shape: an LSD pass streams the whole array once per digit, so at
//! n = 1M a u64 pays eight cold passes of ~5 ns per record. After one split the
//! children are ~31 KiB; the remaining passes over a child stay in L1/L2 and
//! cost a third as much each, which is why this sort improves with n where
//! block LSD degrades.
//!
//! Three decisions use measured costs instead of thresholds:
//!
//! - **Split or stream**: compare streaming the node cold against the split plus
//!   its cache-resident passes (`split_pays`).
//! - **Leaf digit width**: wider digits mean fewer passes but a bigger bucket
//!   table, so the width follows the tile size (`leaf_radix`).
//! - **How a leaf finishes**: counting passes, a merge sort or an insertion
//!   sort, whichever is cheapest at that size.
//!
//! Constant bits are dropped everywhere (`plan_digits`), including in the
//! split's fan-out: a digit that cannot divide the data would otherwise buy 256
//! empty buckets, and a node whose keys differ only in their low byte pays one
//! pass instead of `key width / digit width`.

use std::mem::MaybeUninit;

use crate::other::radix_support::{Key, Unsigned};
use crate::other::rust_block_lsd::core as block;

type SortKey<T> = <T as Key>::Value;

/// Bits per digit for the MSD split and for the block engine's own LSD passes.
/// The block engine's block-size coefficient is tuned for this width.
const RADIX: usize = 8;
/// Widest digit a cache-resident leaf may use. Wider digits mean fewer passes
/// over the tile, which is the whole cost there; the price is a bucket table of
/// `4 << LEAF_RADIX_MAX` bytes (8 KiB here) that has to fit next to the tile in
/// L1. Past this width the table costs more than the pass it removes: 12 bits
/// measured 1-2 % worse than 11 on 128-bit keys and made no difference on 64.
const LEAF_RADIX_MAX: usize = 11;
/// Largest tile finished with LSD passes, in bytes of record data.
const TILE_BYTES: usize = 64 * 1024;
/// Tiles at or below this many records are finished with an insertion sort
/// instead of paying for a bucket table per remaining digit.
const INSERTION_MAX: usize = 48;
/// Tiles at or below this many records are finished with a merge sort. Placed by
/// measurement; see EXPERIMENTS.md.
const MERGE_MAX: usize = 384;
/// Cursors are `u16`, which caps a tile at 65535 records. That is a constant
/// (1 MiB even for 16-byte records), so the auxiliary space stays O(sqrt n).
const MAX_TILE_RECORDS: usize = u16::MAX as usize;
/// Bucket capacity of a leaf pass. The literal appears in the array type:
/// rustc's layout computation hits an ICE on a const-generic bound spelled
/// through an inner associated constant here.
const MAX_BUCKETS: usize = 1 << LEAF_RADIX_MAX;

/// One pass worth of key bits: `width` bits starting at `shift`.
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
}

/// Slots in a pass list. Packed windows are never narrower than the gap
/// between two varying clusters, so in practice a plan is shorter than the fixed
/// grid's; the bound is one window per varying bit.
const PLAN_DIGITS: usize = 128;

/// The bits that can still differ inside one node: every bit set in some key,
/// and every bit set in all of them. Constant bits cannot change the order, so
/// both routes use this to skip work.
struct Variation<T: Key> {
    union: SortKey<T>,
    intersection: SortKey<T>,
}

// Manual impls: the bound belongs to the key type, which is `Copy`, not to the
// record `T`, which need not be.
impl<T: Key> Clone for Variation<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Key> Copy for Variation<T> {}

impl<T: Key> Variation<T> {
    fn varying(self) -> u128 {
        self.union.to_u128() & !self.intersection.to_u128()
    }
}

/// Writes the passes a node will really run into `digits`, highest first, and
/// returns how many there are.
///
/// Consider digits of `width` bits covering the key, and keep only what varies:
/// constant bits cannot change the order, so no pass runs for them. Dropping
/// whole digits is what keeps a node full of duplicate keys cheap; narrowing a
/// partially varying digit is what keeps a skewed node cheap, because both a
/// split's bucket count and a leaf's digit width follow the digit it is given.
/// Keys that differ only in their top bit would otherwise pay a 256-bucket pass
/// to make two buckets.
///
/// The plan stays a slice of a caller-owned array: this toolchain's layout query
/// ICEs when a function passes an array whose length is a path constant.
/// The passes a leaf runs, packing windows instead of using a fixed grid.
///
/// The fixed grid splits a cluster of varying bits that straddles a cell
/// boundary into two passes and gives one pass to every other cell, which is
/// why a key with 31 varying bits scattered over 62 used to take six passes
/// where three windows suffice. Here each window covers the span from the lowest
/// varying bit not yet consumed up to the highest one within `width` bits, and
/// only that span, so a window is never wider than its cluster needs.
///
/// Passes come out most significant window first, the order the leaf consumes
/// them in (it walks the plan backwards, so the last entry runs first). Disjoint
/// spans cannot interleave two keys' digits differently from the key order.
fn plan_digits_packed(varying: u128, width: usize, digits: &mut [Digit]) -> usize {
    let mut remaining = varying;
    let mut len = 0;
    while remaining != 0 {
        let high = (u128::BITS - 1 - remaining.leading_zeros()) as usize;
        let window_low = (high + 1).saturating_sub(width);
        let window = if window_low == 0 {
            u128::MAX
        } else {
            u128::MAX << window_low
        };
        let inside = remaining & window;
        let low = inside.trailing_zeros() as usize;
        debug_assert!(len < digits.len());
        digits[len] = Digit {
            shift: low,
            width: high - low + 1,
        };
        len += 1;
        remaining &= !window;
    }
    len
}

/// The passes the block engine would run: a fixed `width`-bit grid, which is what
/// the streaming route really costs.
fn plan_digits<T: Key>(width: usize, variation: Variation<T>, digits: &mut [Digit]) -> usize {
    let varying_bits = variation.varying();
    let mut len = 0;
    let mut shift = T::Value::BITS;
    while shift > 0 {
        let width = width.min(shift);
        shift -= width;
        let span = ((1u128 << width) - 1) << shift;
        let varying = varying_bits & span;
        if varying == 0 {
            continue;
        }
        // Absolute bit positions: the span already carries the digit's place.
        let low = varying.trailing_zeros() as usize;
        let high = u128::BITS as usize - 1 - varying.leading_zeros() as usize;
        debug_assert!(len < digits.len());
        digits[len] = Digit {
            shift: low,
            width: high - low + 1,
        };
        len += 1;
    }
    len
}

/// Set `RADIX_TRACE=1` to print every routing decision. The trace costs one
/// atomic read per node and is the only way to see why an input took a path.
fn trace() -> bool {
    static TRACE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *TRACE.get_or_init(|| std::env::var_os("RADIX_TRACE").is_some())
}

struct Context<T> {
    /// Scratch for the leaves; holds the largest tile seen so far.
    scratch: Vec<MaybeUninit<T>>,
    tile_bytes: usize,
    /// Tiles at or below this many records use an insertion sort.
    insertion_max: usize,
    /// Tiles at or below this many records use a merge sort instead of counting
    /// passes: past the insertion crossover the bucket table of a counting pass
    /// still costs more than a merge level while the tile is small.
    merge_max: usize,
}

/// Sorts by converted radix key, stably, in O(n) time and O(sqrt(n)) auxiliary
/// space. See the module documentation for the shape of the algorithm.
pub fn sort<T: Key>(arr: &mut [T]) {
    sort_with_policy(arr, TILE_BYTES, MERGE_MAX, INSERTION_MAX);
}

/// Sorts with an explicit tile budget, in bytes of record data. Benchmarks pin
/// the budget to make the trade-off visible.
pub fn sort_with_tile_bytes<T: Key>(arr: &mut [T], tile_bytes: usize) {
    sort_with_policy(arr, tile_bytes, MERGE_MAX, INSERTION_MAX);
}

/// Sorts with an explicit leaf policy. Leaf handling is a local cost decision:
/// the tile is bounded by the budget, so the auxiliary space stays O(sqrt n)
/// whatever the leaf does, and picking the cheapest leaf per size is free. This
/// entry point exists so the crossovers are placed by measurement.
pub fn sort_with_policy<T: Key>(
    arr: &mut [T],
    tile_bytes: usize,
    merge_max: usize,
    insertion_max: usize,
) {
    let record_bytes = std::mem::size_of::<T>();
    if arr.len() < 2 || record_bytes == 0 {
        return;
    }
    let tile_bytes = tile_bytes
        .max(record_bytes)
        .min(MAX_TILE_RECORDS * record_bytes);
    let (union, intersection) = block::varying_key_bits(arr);
    let variation = Variation {
        union,
        intersection,
    };
    if variation.varying() == 0 {
        return;
    }
    // An ordered input needs no passes at all. The probe gives up at the first
    // inversion, so a random array pays two comparisons, while an ascending one
    // pays a single read and returns.
    match probe_order(arr) {
        Order::Ascending => return,
        Order::Descending => {
            reverse_stably(arr);
            return;
        }
        Order::Unsorted => {}
    }
    let mut context = Context {
        scratch: Vec::new(),
        tile_bytes,
        insertion_max,
        merge_max,
    };
    context.sort_node(arr, variation);
}

// --- Cost model -----------------------------------------------------------
//
// The split decision below is the same shape as the block-size formula in
// `rust_block_lsd::core`: minimize predicted cost instead of tuning a byte
// threshold. The constants are machine facts about moving record bytes, not
// properties of a data distribution.

/// A pass costs the same per record whatever the record width, until the memory
/// term takes over: the per-record work (key, bucket, cursor, store) has a floor
/// of 2.5 ns, and wide records add their traffic at the L1 write rate. Measured
/// on cache-resident tiles: 2.3 ns per 4-byte record, 2.6 ns per 8-byte record.
/// `llvm-mca` on the scatter loop puts its ideal throughput at 2.0 cycles per
/// record, so a tile pass running at ~6 cycles per record is memory bound rather
/// than store bound; the floor is what the model needs, not what the hardware
/// can achieve.
const HOT_NS_PER_RECORD: f64 = 2.5;
const HOT_BYTES_PER_NS: f64 = 3.0;

/// A cold pass keeps the same per-record floor and streams through the working
/// set at the scatter bandwidth, fitted to the measured block LSD passes:
/// 3.4 GB/s at 1.6 MiB, 2.1 at 8 MiB, 1.7 at 16 MiB, 1.05 at 80 MiB. The power
/// law below is within 25 % of those points; a cold scatter is that slow because
/// it writes to 256 streams at once, where memcpy reaches ~10 GB/s. Note the
/// measured floor of 3.0 ns per record: below a few MiB the passes are flat.
const COLD_BANDWIDTH_PER_NS: f64 = 3.4;
const COLD_BANDWIDTH_REFERENCE: f64 = 2_097_152.0;
const COLD_BANDWIDTH_EXPONENT: f64 = 0.30;
const COLD_NS_PER_RECORD_FLOOR: f64 = 3.0;

/// How well a split on the top varying bits would divide the node: up to
/// `SPLIT_SAMPLES` keys spread over the node, bucketed exactly as the split
/// would bucket them. Returns how many buckets are occupied and the largest
/// bucket's share.
///
/// This is what tells the model whether a split would actually shrink the
/// working set. A digit that puts everything in one bucket divides nothing: the
/// recursion would pay a pass and gain none, and the input's own locality (a
/// scatter into one stream is nearly sequential) already made that pass cheap.
fn sample_split_shape<T: Key>(arr: &[T], varying: u128) -> (usize, f64) {
    let top = (u128::BITS - varying.leading_zeros()) as usize;
    let width = RADIX.min(top);
    let shift = top - width;
    let mask = ((1u128 << width) - 1) << shift;
    let mut buckets = [0u32; 1 << RADIX];
    let mut sampled = 0u32;
    let mut largest = 0u32;
    // Contiguous runs of `SPLIT_RUN` records, spread over the node. A strided
    // sample is cheaper but can alias with a repeating input pattern (a stride
    // that is a multiple of the pattern period sees a single bucket and calls a
    // well-spread digit narrow); runs cannot.
    let step = (arr.len() / SPLIT_SAMPLES).max(1);
    let mut block = 0;
    while block < SPLIT_SAMPLES {
        let start = block * step;
        let end = (start + SPLIT_RUN).min(arr.len());
        let mut index = start;
        while index < end {
            let key = arr[index].key().to_u128();
            let bucket = ((key & mask) >> shift) as usize;
            buckets[bucket] += 1;
            largest = largest.max(buckets[bucket]);
            sampled += 1;
            index += 1;
        }
        block += SPLIT_RUN;
    }
    let distinct = buckets.iter().filter(|&&count| count > 0).count();
    (distinct, largest as f64 / sampled.max(1) as f64)
}

/// Keys the split-shape sample may look at.
const SPLIT_SAMPLES: usize = 1024;
/// Records read back to back in each sample run.
const SPLIT_RUN: usize = 32;
/// Reading a node sequentially moves bytes faster than `hot` assumes, because
/// nothing is written: the split's read pass is the only one that does not
/// scatter. 6 GB/s is what a wide node gave on this VM.
const SPLIT_READ_BYTES_PER_NS: f64 = 6.0;
/// Floor for one split pass, in ns per record.
const SPLIT_NS_PER_RECORD_MIN: f64 = 1.5;

fn hot_ns_per_record(record_bytes: usize) -> f64 {
    HOT_NS_PER_RECORD.max(record_bytes as f64 / HOT_BYTES_PER_NS)
}

fn cold_ns_per_record(node_bytes: usize, records: usize) -> f64 {
    let record_bytes = node_bytes as f64 / records.max(1) as f64;
    let bandwidth = COLD_BANDWIDTH_PER_NS
        * (node_bytes as f64 / COLD_BANDWIDTH_REFERENCE).powf(-COLD_BANDWIDTH_EXPONENT);
    COLD_NS_PER_RECORD_FLOOR.max(record_bytes / bandwidth.min(HOT_BYTES_PER_NS))
}

/// Cost of one MSD split, in units of a cache-resident pass.
///
/// The split is not bandwidth limited: its scatter writes into 2R blocks that
/// stay resident, so the block scatter, the materialization of the permutation
/// and the child-boundary scan together cost about four hot passes per record.
/// Measured 6.1 ns per 4-byte record and 8.9 ns per 8-byte record, i.e. 1.1-1.5
/// ns per byte against a hot pass at 0.33 ns per byte.
const SPLIT_HOT_PASSES: f64 = 4.0;

/// Whether an MSD split costs less than streaming the node cold.
///
/// Not a threshold but the two predicted totals: streaming costs
/// `cold_passes` cold passes, splitting costs the split itself plus
/// `hot_passes` cache-resident passes for the whole node. Both pass counts are
/// what this node would really run, so a wide key with sixteen digits and a
/// narrow one with two are compared on their own work.
fn split_pays(
    node_bytes: usize,
    records: usize,
    tile_bytes: usize,
    cold_passes: usize,
    hot_passes: usize,
    largest_child: f64,
) -> bool {
    if cold_passes < 2 || records == 0 {
        return false;
    }
    let hot = hot_ns_per_record(node_bytes / records);
    let streamed = cold_passes as f64 * cold_ns_per_record(node_bytes, records);
    // The child that carries most of the records is what the recursion has to
    // keep working on; the split only pays if moving that work into a smaller
    // working set is cheaper than the split itself. A child that fits the tile
    // runs its passes in cache, one that does not is still streaming.
    let share = largest_child.clamp(1.0 / 256.0, 1.0);
    let child_bytes = ((node_bytes as f64 * share) as usize).max(1);
    let child_records = ((records as f64 * share) as usize).max(1);
    let child_pass = if child_bytes <= tile_bytes {
        hot
    } else {
        cold_ns_per_record(child_bytes, child_records)
    };
    // The split reads the node sequentially `SPLIT_HOT_PASSES` times and writes
    // nothing until the fan-out; charging it the scatter bandwidth of a pass
    // overestimated it by about 2x.
    let split_pass =
        SPLIT_NS_PER_RECORD_MIN.max(node_bytes as f64 / records as f64 / SPLIT_READ_BYTES_PER_NS);
    let split = SPLIT_HOT_PASSES as f64 * split_pass + hot_passes as f64 * child_pass;
    split < streamed
}

/// Merges the two runs `[start, middle)` and `[middle, end)` from `source`
/// into `destination` at the same offsets, stably: on equal keys the left run
/// wins, which is what keeps the whole sort stable.
///
/// # Safety
///
/// Both runs must be sorted, `source` must hold `end` initialized records and
/// `destination` must have room for `end`, in a different allocation.
unsafe fn merge_runs<T: Key>(
    source: *const T,
    destination: *mut T,
    start: usize,
    middle: usize,
    end: usize,
) {
    unsafe {
        if middle >= end {
            std::ptr::copy_nonoverlapping(source.add(start), destination.add(start), end - start);
            return;
        }
        // The common case on partially sorted inputs: the runs already meet in
        // order, so the whole pair is a copy.
        if (*source.add(middle - 1)).key() <= (*source.add(middle)).key() {
            std::ptr::copy_nonoverlapping(source.add(start), destination.add(start), end - start);
            return;
        }
        let (mut left, mut right, mut out) = (start, middle, start);
        while left < middle && right < end {
            let take_left = (*source.add(left)).key() <= (*source.add(right)).key();
            let from = if take_left { left } else { right };
            std::ptr::write(destination.add(out), std::ptr::read(source.add(from)));
            if take_left {
                left += 1;
            } else {
                right += 1;
            }
            out += 1;
        }
        if left < middle {
            std::ptr::copy_nonoverlapping(source.add(left), destination.add(out), middle - left);
            out += middle - left;
        }
        if right < end {
            std::ptr::copy_nonoverlapping(source.add(right), destination.add(out), end - right);
        }
    }
}

enum Order {
    Ascending,
    Descending,
    Unsorted,
}

/// Spread-out pairs the strided gate may look at before the full order pass.
const ORDER_SAMPLES: usize = 4096;

/// Whether the input is already ordered.
///
/// A strided sample answers first: at most `ORDER_SAMPLES` spread-out adjacent
/// pairs, tracking both directions. If the sample is monotone in neither
/// direction the array cannot be ordered, so the full pass is skipped. That
/// matters for the common "sorted, then a few appends" shape, where a full pass
/// would run to the end before finding the inversion. The sample can only
/// *reject*, never accept, so the answer stays exact.
///
/// Keeping this separate from the variation scan also keeps that scan
/// vectorizable; fusing the two cost 5 % on clustered inputs, where the scan
/// runs to the end and the order flags never pay off.
fn probe_order<T: Key>(arr: &[T]) -> Order {
    let stride = (arr.len() / ORDER_SAMPLES).max(1);
    let mut ascending = true;
    let mut descending = true;
    let mut index = stride;
    while index < arr.len() {
        let before = arr[index - 1].key();
        let after = arr[index].key();
        ascending &= before <= after;
        descending &= before >= after;
        if !ascending && !descending {
            return Order::Unsorted;
        }
        index += stride;
    }
    let mut ascending = true;
    let mut descending = true;
    let mut previous = arr[0].key();
    for item in &arr[1..] {
        let key = item.key();
        ascending &= previous <= key;
        descending &= previous >= key;
        previous = key;
        if !ascending && !descending {
            return Order::Unsorted;
        }
    }
    if ascending {
        Order::Ascending
    } else if descending {
        Order::Descending
    } else {
        Order::Unsorted
    }
}

/// Sorts a descending array, stably, by reversing it and then reversing each
/// run of equal keys back. Reversal alone would swap equal keys past each other.
fn reverse_stably<T: Key>(arr: &mut [T]) {
    arr.reverse();
    let mut start = 0;
    while start < arr.len() {
        let key = arr[start].key();
        let mut end = start + 1;
        while end < arr.len() && arr[end].key() == key {
            end += 1;
        }
        arr[start..end].reverse();
        start = end;
    }
}

/// Streams one node through the block engine's cold passes.
///
/// Out of line and marked cold: this is the branch the routing model expects to
/// take least often on large inputs, and keeping it out of line leaves the split
/// path's code in one piece for the instruction cache.
#[cold]
#[inline(never)]
fn stream_node<T: Key>(arr: &mut [T], variation: Variation<T>) {
    let block_size = block::default_block_size(arr.len(), std::mem::size_of::<T>(), RADIX);
    block::sort_with_variation(
        arr,
        RADIX,
        block_size,
        variation.union,
        variation.intersection,
    );
}

/// Stable insertion sort on converted keys.
fn insertion_leaf<T: Key>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1].key() > arr[j].key() {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// One stable counting pass: histogram, prefix sum, scatter.
///
/// # Safety
///
/// `source` must point at `len` initialized records and `destination` at
/// `len` writable slots of a different allocation. `counts` must have at least
/// `1 << digit.width` entries, and `len` must fit in `u32`. The array length is
/// not in the signature on purpose: this toolchain ICEs on an array-typed
/// parameter whose length is a path constant.
#[inline]
unsafe fn counting_pass<T: Key>(
    source: *const T,
    destination: *mut T,
    len: usize,
    digit: Digit,
    counts: &mut [u16],
) {
    let mask = digit.mask::<T>();
    let shift = T::Value::from_usize(digit.shift);
    let buckets = 1usize << digit.width;
    let cursor = counts.as_mut_ptr();
    for slot in 0..buckets {
        // SAFETY: `buckets <= MAX_BUCKETS`, the length of `counts`.
        *cursor.add(slot) = 0;
    }
    for index in 0..len {
        // SAFETY: the caller guarantees `len` initialized records.
        let key = (*source.add(index)).key();
        let bucket = ((key & mask) >> shift).to_usize();
        // SAFETY: the digit mask keeps the index below `buckets`, and no bucket
        // can hold more than `len <= u16::MAX` records.
        *cursor.add(bucket) += 1;
    }
    let mut offset = 0u16;
    for slot in 0..buckets {
        // SAFETY: as above.
        let count = *cursor.add(slot);
        *cursor.add(slot) = offset;
        offset += count;
    }
    for index in 0..len {
        // SAFETY: the caller guarantees `len` initialized records; the cursor
        // counts the records already placed in this bucket, so it stays inside
        // that bucket's output interval and below `len`.
        unsafe {
            let record = source.add(index);
            let key = (*record).key();
            let bucket = ((key & mask) >> shift).to_usize();
            let slot = *cursor.add(bucket);
            *cursor.add(bucket) = slot + 1;
            std::ptr::write(destination.add(slot as usize), std::ptr::read(record));
        }
    }
}

/// Digit width for a leaf that will run counting passes over `records`.
///
/// A pass costs `records` moves plus `2^width` table entries, and the pass
/// count falls roughly as `1/width`, so the best width grows with the tile. The
/// rule keeps the table at or below one entry per record, which is where the
/// measured crossovers sit, and never exceeds [`LEAF_RADIX_MAX`].
fn leaf_radix(records: usize) -> usize {
    let bits = usize::BITS as usize - records.leading_zeros() as usize;
    bits.clamp(6, LEAF_RADIX_MAX)
}

/// Digit width for one MSD split.
///
/// Aim for children of about half the tile budget: large enough that the split
/// pass stays amortized, small enough that the common child needs no further
/// split. The bucket count, and with it the block engine's scratch, then
/// follows the node size instead of being fixed at 256 even for a node of a few
/// thousand records.
fn split_radix(node_len: usize, record_bytes: usize, tile_bytes: usize) -> usize {
    let tile_records = (tile_bytes / record_bytes).max(64);
    let target = (tile_records / 2).max(1);
    let children = (node_len / target).max(2);
    let bits = usize::BITS - (children - 1).leading_zeros();
    // Capped at `RADIX`, not at the leaf maximum: a 2048-way fan-out makes every
    // write in the split's last pass land in a different 40 KiB region, and the
    // measured cost of that (u64 10M 35.4 -> 41.4 ns, u128 10M 100 -> 97) is
    // larger than the L1-resident leaves it buys.
    (bits as usize).clamp(3, RADIX)
}

impl<T: Key> Context<T> {
    fn sort_node(&mut self, arr: &mut [T], variation: Variation<T>) {
        let records = arr.len();
        let varying = variation.varying();
        if records < 2 || varying == 0 {
            return;
        }
        let record_bytes = std::mem::size_of::<T>();
        let node_bytes = records.saturating_mul(record_bytes);
        if node_bytes <= self.tile_bytes {
            if trace() {
                eprintln!("leaf  len={} bytes={}", records, node_bytes);
            }
            self.lsd_leaf(arr, variation);
            return;
        }
        // Two predicted totals: stream the node through cold passes the way the
        // block engine would, or pay for one split and finish the children in
        // cache. The pass counts are the ones each route would really run.
        let mut scratch_digits = [Digit { shift: 0, width: 0 }; PLAN_DIGITS];
        let cold_passes = plan_digits::<T>(RADIX, variation, &mut scratch_digits);
        let hot_passes = plan_digits_packed(varying, leaf_radix(records), &mut scratch_digits);
        let (distinct, largest_child) = sample_split_shape(arr, varying);
        if trace() {
            eprintln!(
                "shape len={} distinct={} largest={:.4} cold={} hot={}",
                records, distinct, largest_child, cold_passes, hot_passes
            );
        }
        if !split_pays(
            node_bytes,
            records,
            self.tile_bytes,
            cold_passes,
            hot_passes,
            largest_child,
        ) {
            if trace() {
                eprintln!(
                    "block len={} bytes={} cold_passes={} hot_passes={}",
                    records, node_bytes, cold_passes, hot_passes
                );
            }
            // Splitting would cost more than the passes it replaces. Finish the
            // node with the block engine's LSD pass, which needs no buffer the
            // size of the node. It gets the variation scan already done here.
            stream_node(arr, variation);
            return;
        }
        // MSD split: one stable block-permutation pass over the whole node.
        // The fan-out follows the node size. A node that is only slightly over
        // the tile budget must not pay for 256 output streams and the block
        // engine's per-bucket scratch floor.
        // The split takes the topmost bits that can still differ. Anchoring it
        // at the highest varying bit instead of at a digit boundary keeps every
        // level useful and lets one level peel bits from several leaf digits: a
        // level that peeled no varying bit would pay a pass to make one bucket.
        let top = (u128::BITS - varying.leading_zeros()) as usize;
        let radix = split_radix(records, record_bytes, self.tile_bytes)
            .min(top)
            .max(1);
        let digit = Digit {
            shift: top - radix,
            width: radix,
        };
        if trace() {
            eprintln!(
                "split len={} bytes={} cold_passes={} hot_passes={} radix={}",
                records, node_bytes, cold_passes, hot_passes, radix
            );
        }
        block::partition_digit(arr, digit.width, digit.shift);
        self.descend(arr, digit);
    }

    /// Walks the freshly partitioned node, and sorts each bucket as a separate
    /// child. The walk extracts every key once and accumulates the variation of
    /// the child while the bucket boundary is still open, so a child does not
    /// pay for a second scan of its own keys.
    fn descend(&mut self, arr: &mut [T], digit: Digit) {
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
            // Within a bucket every key shares the digits above the split, so
            // they fall out of the child's variation on their own.
            self.sort_node(
                &mut arr[start..end],
                Variation {
                    union,
                    intersection,
                },
            );
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

    /// Stable bottom-up merge sort over the tile, driven by the same buffer the
    /// counting passes use. A merge level moves every record once and needs no
    /// bucket table, which is what wins while the tile is only a few hundred
    /// records; past that the level count outgrows the digit count.
    fn merge_leaf(&mut self, arr: &mut [T]) {
        let len = arr.len();
        if len < 2 || arr.windows(2).all(|pair| pair[0].key() <= pair[1].key()) {
            return;
        }
        self.set_scratch_len(len);
        let buffer = self.scratch.as_mut_ptr().cast::<T>();
        let mut source = arr.as_mut_ptr();
        let mut destination = buffer;
        let mut source_is_tile = true;
        let mut width = 1;
        loop {
            let mut start = 0;
            while start < len {
                let middle = (start + width).min(len);
                let end = (start + 2 * width).min(len);
                // SAFETY: the source holds `len` initialized records and the
                // destination has room for `len`; they are distinct
                // allocations, and each pair of runs is read before it is
                // written.
                unsafe {
                    merge_runs(source, destination, start, middle, end);
                }
                start = end;
            }
            // The pass just finished: its output is in `destination`.
            std::mem::swap(&mut source, &mut destination);
            source_is_tile = !source_is_tile;
            if 2 * width >= len {
                break;
            }
            width *= 2;
        }
        if !source_is_tile {
            // SAFETY: the whole tile now lives, sorted, in the buffer.
            unsafe { std::ptr::copy_nonoverlapping(buffer, arr.as_mut_ptr(), len) };
        }
    }

    /// Stable LSD counting sort over the tile, driven by a private buffer. One
    /// pass per remaining digit, ping-ponging between the tile and the buffer.
    fn lsd_leaf(&mut self, arr: &mut [T], variation: Variation<T>) {
        let len = arr.len();
        if len < 2 || variation.varying() == 0 {
            return;
        }
        // A counting pass costs a full bucket table before it moves a single
        // record. Below the crossover an insertion sort on converted keys is
        // both smaller and faster, and it is stable, so the tile's contract is
        // unchanged. Skewed inputs land here for every thin tail bucket.
        if len <= self.insertion_max {
            insertion_leaf(arr);
            return;
        }
        if len <= self.merge_max {
            self.merge_leaf(arr);
            return;
        }
        // The leaf picks its own digit width: inside the tile the only cost that
        // scales with it is the bucket table, so a wider digit buys fewer passes
        // while the table still fits in L1 next to the records.
        let mut digits = [Digit { shift: 0, width: 0 }; PLAN_DIGITS];
        let passes = plan_digits_packed(variation.varying(), leaf_radix(len), &mut digits);
        self.set_scratch_len(len);
        let scratch_ptr = self.scratch.as_mut_ptr().cast::<T>();
        // A tile is at most `MAX_TILE_RECORDS` records, so 16-bit cursors are
        // enough, and they are the difference between a cursor table that stays
        // in one L1 set and one that does not.
        debug_assert!(len <= MAX_TILE_RECORDS);
        let mut counts = [0u16; MAX_BUCKETS];

        // The record set lives in `source` for the whole pass. A panic from the
        // key extractor therefore never loses or duplicates a record: if the
        // source is the tile, the tile is untouched; if it is the scratch
        // buffer, the buffer still holds every record and is copied back.
        let mut live_in_scratch = false;
        let mut run = |live_in_scratch: &mut bool, counts: &mut [u16]| {
            let mut source: *mut T = arr.as_mut_ptr();
            let mut destination: *mut T = scratch_ptr;
            let mut source_is_tile = true;
            for index in (0..passes).rev() {
                *live_in_scratch = !source_is_tile;
                // SAFETY: the source holds `len` initialized records, the
                // destination has room for `len` records, and both are distinct
                // allocations that stay alive for the whole leaf.
                unsafe {
                    counting_pass(source, destination, len, digits[index], counts);
                }
                std::mem::swap(&mut source, &mut destination);
                source_is_tile = !source_is_tile;
            }
            source_is_tile
        };

        let ended_in_tile = if const { std::mem::needs_drop::<T>() } {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run(&mut live_in_scratch, &mut counts[..])
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
            run(&mut live_in_scratch, &mut counts[..])
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
        if input != expected {
            let mismatch = input
                .iter()
                .zip(expected.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(0);
            panic!(
                "tile={tile_bytes} len={} first mismatch at {mismatch}: got {:?}, want {:?}",
                input.len(),
                input[mismatch],
                expected[mismatch]
            );
        }
    }

    #[test]
    fn every_width_and_tile_rounds_to_zero() {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        for &len in &[
            0usize, 1, 2, 3, 7, 8, 9, 100, 255, 256, 257, 1000, 4097, 20_000,
        ] {
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
