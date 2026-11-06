//! Stable radix sort with block partitions above a 64 KiB tile budget.
//!
//! Ordered inputs return or reverse stably; nearly ordered inputs get a budgeted
//! insertion pass. Large nodes partition by an MSD digit, a sampled repeated key,
//! or an end run separated by the highest varying bit. Tiles finish with insertion,
//! an MSD partition, or LSD counting passes over reusable scratch.
//!
//! Constant bits are skipped. Wide keys whose variation fits one 64-bit half use
//! a narrowed view. The block engine uses O(sqrt(n)) scratch; tile scratch is bounded.
//! Equal keys retain their input order, and key panics preserve the record set.

use std::mem::MaybeUninit;

use crate::other::radix_support::{Key, Unsigned};
use crate::other::rust_block_lsd::core as block;

type KeyValue<T> = <T as Key>::Value;

/// Initial split width; also the block engine's tuned width.
const SPLIT_BITS: usize = 8;

/// Split fan-out is bounded by the block engine's 12-bit tables.
const MAX_SPLIT_BITS: usize = 12;

/// A 128-key sample cannot fill a quarter of the buckets at 10 bits or more.
const WIDENING_SAMPLE_BITS: usize = 10;
/// At 14 bits the u16 cursor table occupies 32 KiB.
const MAX_TILE_DIGIT_BITS: usize = 14;
/// Narrowest digit a leaf may use.
const MIN_TILE_DIGIT_BITS: usize = 6;
/// Maximum tile size in bytes of records.
const TILE_BYTES: usize = 64 * 1024;
/// One tile partition costs five traversals (histogram, scatter, copy back);
/// three two-traversal LSD passes are its break-even.
const MSD_LEVEL_PASSES: usize = 3;
/// Tiles at or below this many records are finished with an insertion sort
/// instead of paying for a bucket table per remaining digit.
const INSERTION_MAX: usize = 48;
/// u16 cursors bound a tile to 65535 records.
const MAX_TILE_RECORDS: usize = u16::MAX as usize;
/// Keys a leaf samples to judge whether its top window divides the tile.
const DIGIT_SAMPLE_LEN: usize = 128;

/// Counts distinct digits in an evenly spaced sample.
fn sample_distinct_digits<T: Key>(arr: &[T], digit: Digit) -> usize {
    let mask = digit.mask::<T>();
    let shift = T::Value::from_usize(digit.shift);

    let mut seen = [0u64; MAX_TILE_BUCKETS / 64];
    let sample_len = DIGIT_SAMPLE_LEN.min(arr.len());
    let step = (arr.len() / sample_len).max(1);
    let mut distinct = 0;
    for index in (0..arr.len()).step_by(step).take(sample_len) {
        let bucket = ((arr[index].key() & mask) >> shift).to_usize();
        let word = bucket / 64;
        let bit = 1u64 << (bucket % 64);
        if seen[word] & bit == 0 {
            seen[word] |= bit;
            distinct += 1;
        }
    }
    distinct
}

/// Keys a node samples to look for one value that a large share of it repeats.
const MODE_SAMPLE_LEN: usize = 64;

/// A value repeated by at least a quarter of the sample gets an equality partition.
const MODE_SHARE_DENOMINATOR: usize = 4;

/// Consecutive keys per sampled run.
const MODE_SAMPLE_RUN_LEN: usize = 8;

/// Samples eight runs of eight keys, avoiding stride aliasing with periodic keys.
fn sample_keys<T: Key>(arr: &[T], keys: &mut [KeyValue<T>]) {
    let runs = keys.len() / MODE_SAMPLE_RUN_LEN;
    let step = (arr.len() / runs).max(1);
    let mut len = 0;
    for run in 0..runs {
        let start = run * step;
        for offset in 0..MODE_SAMPLE_RUN_LEN {
            let index = start + offset;
            if index >= arr.len() {
                break;
            }
            keys[len] = arr[index].key();
            len += 1;
        }
    }
    // Short inputs pad the sample with the first key.
    let first = keys[0];
    keys[len..].fill(first);
}

/// Finds a key repeated by at least a quarter of the sample.
fn sample_mode<V: Unsigned>(keys: &mut [V]) -> Option<V> {
    let len = keys.len();
    keys.sort_unstable();
    let mut best = &keys[..1];
    for run in keys.chunk_by(|a, b| a == b) {
        if run.len() > best.len() {
            best = run;
        }
    }
    (best.len() * MODE_SHARE_DENOMINATOR >= len).then_some(best[0])
}

/// Whether `bit` separates a minority of the sample, and which side is the
/// minority: `true` when the records with the bit set are the few.
fn sample_minority_bit<V: Unsigned>(keys: &[V], bit: V) -> Option<bool> {
    let zero = V::from_usize(0);
    let set = keys.iter().filter(|key| (**key & bit) != zero).count();
    if set * 8 <= keys.len() {
        Some(true)
    } else if set * 8 >= keys.len() * 7 {
        Some(false)
    } else {
        None
    }
}

/// Records at one end that a split may waste a whole pass on. The run is
/// finished by an insertion sort, which is quadratic, hence the cap.
const OUTLIER_RUN_MAX: usize = 1024;

/// A short run at one end of a node whose records disagree with every other
/// record on the node's top bit, and the variation of the rest.
struct OutlierRun {
    /// The run sits at the start of the node (`true`) or at its end.
    head: bool,
    /// Whether the run is the side with the bit set.
    set: bool,
    len: usize,
    body_varying: u128,
}

/// Finds a short end run separated by the top bit and scans the body
/// for variation. A scattered minority stops the scan at its first occurrence.
fn outlier_run<T: Key>(arr: &[T], keys: &[KeyValue<T>], bit: KeyValue<T>) -> Option<OutlierRun> {
    let minority_set = sample_minority_bit(keys, bit)?;
    let zero = KeyValue::<T>::from_usize(0);
    let is_minority = |key: KeyValue<T>| ((key & bit) != zero) == minority_set;
    let cap = OUTLIER_RUN_MAX.min(arr.len() / 2);
    let mut head = 0;
    while head < cap && is_minority(arr[head].key()) {
        head += 1;
    }
    let mut tail = 0;
    while tail < cap && is_minority(arr[arr.len() - 1 - tail].key()) {
        tail += 1;
    }
    // The minority must occupy exactly one end.
    if (head == 0) == (tail == 0) {
        return None;
    }
    let (head_run, len) = if head > 0 {
        (true, head)
    } else {
        (false, tail)
    };
    if len == cap {
        return None;
    }
    let (start, end) = if head_run {
        (len, arr.len())
    } else {
        (0, arr.len() - len)
    };
    let mut union = arr[start].key();
    let mut intersection = union;
    for record in &arr[start + 1..end] {
        let key = record.key();
        if is_minority(key) {
            return None;
        }
        union |= key;
        intersection = intersection & key;
    }
    Some(OutlierRun {
        head: head_run,
        set: minority_set,
        len,
        body_varying: union.to_u128() ^ intersection.to_u128(),
    })
}

/// Scans for inversions and uses budgeted stable insertion on key copies or
/// records. On failure the input remains a stable permutation.
fn budgeted_insertion_sort<T, K: Ord>(
    arr: &mut [T],
    budget: usize,
    key: impl Fn(&T) -> K,
) -> Option<usize> {
    let mut swaps = 0;
    let mut start = 0;
    while let Some(offset) = arr[start..]
        .windows(2)
        .position(|pair| key(&pair[0]) > key(&pair[1]))
    {
        let index = start + offset + 1;
        let (prefix, tail) = arr.split_at_mut(index);
        let (mut current, prefix) = prefix.split_last_mut().unwrap();
        // The scan already compared this pair, so swap without re-reading keys.
        std::mem::swap(current, &mut tail[0]);
        swaps += 1;
        if swaps > budget {
            return None;
        }
        for before in prefix.iter_mut().rev() {
            if key(before) <= key(current) {
                break;
            }
            std::mem::swap(before, current);
            current = before;
            swaps += 1;
            if swaps > budget {
                return None;
            }
        }
        start = index;
    }
    Some(swaps)
}

/// Maximum cursor table length.
const MAX_TILE_BUCKETS: usize = 1 << MAX_TILE_DIGIT_BITS;

/// One pass worth of key bits: `width` bits starting at `shift`.
#[derive(Clone, Copy)]
struct Digit {
    shift: usize,
    width: usize,
}

impl Digit {
    #[inline]
    fn mask<T: Key>(self) -> KeyValue<T> {
        (T::Value::from_usize((1usize << self.width) - 1)) << T::Value::from_usize(self.shift)
    }
}

/// Capacity for packed digit windows over a 128-bit key.
const MAX_DIGITS: usize = 48;

/// Scans until all candidate bits have appeared in both states, or the input ends.
/// The parent partition digit is constant within a child and excluded by the caller.
fn scan_varying_bits<T: Key>(arr: &[T], candidate: u128) -> u128 {
    if candidate == 0 {
        return 0;
    }
    let first = arr[0].key();
    let (mut union, mut intersection) = (first, first);
    let mut index = 1;
    while index < arr.len() {
        let end = (index + 64).min(arr.len());
        for record in &arr[index..end] {
            let key = record.key();
            union |= key;
            intersection = intersection & key;
        }
        let seen = union.to_u128() ^ intersection.to_u128();
        if seen & candidate == candidate {
            break;
        }
        index = end;
    }
    union.to_u128() ^ intersection.to_u128()
}

/// Packs varying bits into windows of at most `width` bits, highest first.
/// The LSD passes consume the plan in reverse. Windows skip constant gaps.
fn plan_digits(varying: u128, width: usize, digits: &mut [Digit]) -> usize {
    let mut remaining = varying;
    let mut len = 0;
    while remaining != 0 {
        let high = remaining.ilog2() as usize;
        let window_low = (high + 1).saturating_sub(width);
        let window = u128::MAX << window_low;
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

/// A layout-preserving view of a wide key's varying 64-bit half.
/// The discarded half is constant across the root, so order and equality are preserved.
#[repr(transparent)]
struct KeyHalf<const SHIFT: usize, T: Key>(T);

impl<const SHIFT: usize, T: Key> Key for KeyHalf<SHIFT, T> {
    type Value = u64;
    #[inline]
    fn key(&self) -> u64 {
        (self.0.key().to_u128() >> SHIFT) as u64
    }
}

struct Sorter<T> {
    scratch: Vec<MaybeUninit<T>>,
    tile_bytes: usize,
}

/// Sorts by converted radix key, stably, in O(n) time and O(sqrt(n)) auxiliary
/// space. See the module documentation for the shape of the algorithm.
pub(super) fn sort<T: Key>(arr: &mut [T]) {
    sort_with_tile_bytes(arr, TILE_BYTES);
}

fn sort_with_tile_bytes<T: Key>(arr: &mut [T], tile_bytes: usize) {
    let record_bytes = std::mem::size_of::<T>();
    if arr.len() < 2 || record_bytes == 0 {
        return;
    }
    let tile_bytes = tile_bytes
        .max(record_bytes)
        .min(MAX_TILE_RECORDS * record_bytes);
    // Probe before the variation scan so ordered inputs pay only one full read.
    let sampled = probe_order(arr);
    match sampled {
        Order::Ascending => return,
        Order::Descending => {
            reverse_stably(arr);
            return;
        }
        Order::Flat | Order::Unsorted => {}
    }
    if sampled == Order::Unsorted {
        let probe = (tile_bytes / record_bytes).max(2).min(arr.len());
        let probe_swaps = {
            let mut keys: Vec<KeyValue<T>> = arr[..probe].iter().map(Key::key).collect();
            budgeted_insertion_sort(&mut keys, probe, |&key| key)
        };
        match probe_swaps {
            None => {}
            Some(0) => {
                if probe == arr.len() {
                    return;
                }
            }
            Some(swaps) => {
                let budget = swaps * arr.len() / probe;
                if budgeted_insertion_sort(arr, budget, Key::key).is_some() {
                    return;
                }
            }
        }
    }
    let varying = block::varying_key_bits(arr);
    if varying == 0 {
        return;
    }
    // Only confirm flat samples after ruling out a constant array.
    if sampled == Order::Flat {
        match confirm_order(arr) {
            Order::Ascending => return,
            Order::Descending => {
                reverse_stably(arr);
                return;
            }
            _ => {}
        }
    }

    if T::Value::BITS > 64 && T::Value::BITS <= 128 {
        if varying >> 64 == 0 {
            sort_key_half::<0, T>(arr, varying, tile_bytes);
            return;
        }
        if varying & ((1u128 << 64) - 1) == 0 {
            sort_key_half::<64, T>(arr, varying, tile_bytes);
            return;
        }
    }
    let mut sorter = Sorter {
        scratch: Vec::new(),
        tile_bytes,
    };
    sorter.sort_node(arr, varying);
}

/// Narrows at the root only; descendants retain the same key view.
fn sort_key_half<const SHIFT: usize, T: Key>(arr: &mut [T], varying: u128, tile_bytes: usize) {
    // SAFETY: KeyHalf is repr(transparent) over T. This reborrow is the only
    // reference used during sorting, with identical length, layout and alignment.
    let narrowed: &mut [KeyHalf<SHIFT, T>] =
        unsafe { &mut *(arr as *mut [T] as *mut [KeyHalf<SHIFT, T>]) };
    let mut sorter = Sorter::<KeyHalf<SHIFT, T>> {
        scratch: Vec::new(),
        tile_bytes,
    };
    sorter.sort_node(narrowed, varying >> SHIFT);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Order {
    Ascending,
    Descending,
    /// Every sampled pair was equal; gaps may still contain differing keys.
    Flat,
    Unsorted,
}

/// Spread-out pairs the strided gate may look at before the full order pass.
const ORDER_SAMPLES: usize = 4096;

/// Rejects unordered input with strided adjacent pairs, then confirms order
/// with a full scan. Equal-only samples defer confirmation to the caller.
fn probe_order<T: Key>(arr: &[T]) -> Order {
    let stride = (arr.len() / ORDER_SAMPLES).max(1);
    let mut ascending = true;
    let mut descending = true;
    for index in (stride..arr.len()).step_by(stride) {
        // SAFETY: the range guarantees 1 <= index < arr.len() for both accesses.
        let before = unsafe { arr.get_unchecked(index - 1) }.key();
        let after = unsafe { arr.get_unchecked(index) }.key();
        ascending &= before <= after;
        descending &= before >= after;
        if !ascending && !descending {
            return Order::Unsorted;
        }
    }
    if ascending && descending {
        return Order::Flat;
    }
    confirm_order(arr)
}

/// Determines direction from the first unequal key, then confirms that order.
fn confirm_order<T: Key>(arr: &[T]) -> Order {
    let first = arr[0].key();
    let mut keys = arr[1..].iter().map(Key::key);
    let Some(key) = keys.find(|&key| key != first) else {
        return Order::Ascending;
    };
    if first < key {
        keys.try_fold(key, |a, b| (a <= b).then_some(b))
            .map_or(Order::Unsorted, |_| Order::Ascending)
    } else {
        keys.try_fold(key, |a, b| (a >= b).then_some(b))
            .map_or(Order::Unsorted, |_| Order::Descending)
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

/// Stable insertion sort on converted keys.
fn insertion_sort<T: Key>(arr: &mut [T]) {
    let _ = budgeted_insertion_sort(arr, usize::MAX, Key::key);
}

/// Histogram, prefix sum, then stable scatter. Returns the largest bucket.
///
/// # Safety
/// `source` holds `len` initialized records; `destination` has `len` writable
/// slots in a separate allocation. `cursor` has room for `1 << digit.width`
/// entries. `len <= u16::MAX`, as enforced by the tile-size clamp.
#[inline]
unsafe fn counting_pass<T: Key>(
    source: *const T,
    destination: *mut T,
    len: usize,
    digit: Digit,
    cursor: *mut u16,
) -> u16 {
    let mask = digit.mask::<T>();
    let shift = T::Value::from_usize(digit.shift);
    let buckets = 1usize << digit.width;
    for slot in 0..buckets {
        // SAFETY: the caller guarantees room for `buckets` entries.
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
    let mut largest = 0u16;
    for slot in 0..buckets {
        // SAFETY: as above.
        let count = *cursor.add(slot);
        *cursor.add(slot) = offset;
        offset += count;
        largest = largest.max(count);
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
            *cursor.add(bucket) = slot.wrapping_add(1);
            // Limit the target index to the destination tile.
            let target = (slot as usize).min(len - 1);
            std::ptr::write(destination.add(target), std::ptr::read(record));
        }
    }
    largest
}

/// Chooses the widest cursor table that fits beside the tile, capped by
/// record count to avoid clearing mostly empty buckets on wide records.
fn tile_digit_bits(records: usize, record_bytes: usize) -> usize {
    let tile_bytes = records.saturating_mul(record_bytes);
    // The widest table that still fits: `2 * 2^width` bytes of cursors.
    let widest = (tile_bytes / 2).max(1).ilog2() as usize;
    // Limit empty-bucket work to at most two bits above the record count.
    let by_records = records.max(1).ilog2() as usize + 3;
    widest
        .min(by_records)
        .clamp(MIN_TILE_DIGIT_BITS, MAX_TILE_DIGIT_BITS)
}

/// Chooses an initial width targeting half-tile children, then widens if needed.
fn choose_split_digit<T: Key>(arr: &[T], top: usize, tile_bytes: usize) -> Digit {
    let record_bytes = std::mem::size_of::<T>();
    let tile_records = (tile_bytes / record_bytes).max(64);
    let target = (tile_records / 2).max(1);
    let children = (arr.len() / target).max(2);
    let bits = (children - 1).ilog2() + 1;
    // Wider fan-out increased page/TLB pressure in the 10M measurements.
    let mut width = (bits as usize).clamp(3, SPLIT_BITS).min(top).max(1);

    // Widen a concentrated top window until a quarter of its buckets are sampled.
    while width < top && width + 2 <= MAX_SPLIT_BITS {
        // At larger widths the sample cannot meet the occupancy threshold.
        if width < WIDENING_SAMPLE_BITS {
            let probe = Digit {
                shift: top - width,
                width,
            };

            if sample_distinct_digits(arr, probe) * 4 >= 1usize << width {
                break;
            }
        }
        // Clamp to the highest varying bit: a 9-bit node previously widened
        // to 10 bits and underflowed the digit shift.
        width = (width + 2).min(top);
    }
    Digit {
        shift: top - width,
        width,
    }
}

impl<T: Key> Sorter<T> {
    fn sort_node(&mut self, arr: &mut [T], varying: u128) {
        let records = arr.len();
        if records < 2 || varying == 0 {
            return;
        }
        let record_bytes = std::mem::size_of::<T>();
        let node_bytes = records.saturating_mul(record_bytes);
        if node_bytes <= self.tile_bytes {
            self.sort_tile(arr, varying);
            return;
        }
        // One digit finishes a narrow span; wider spans benefit from tiled children.
        let low = varying.trailing_zeros() as usize;
        let top = (u128::BITS - varying.leading_zeros()) as usize;
        let span = top - low;

        if span <= SPLIT_BITS {
            // One block partition finishes the node without recursion.
            block::partition_digit(arr, SPLIT_BITS, low);
            return;
        }
        // Amortize the key sample over at least four tiles.
        if node_bytes >= 4 * self.tile_bytes {
            let mut keys = [KeyValue::<T>::from_usize(0); MODE_SAMPLE_LEN];
            sample_keys(arr, &mut keys);
            // An equality partition finishes the repeated key immediately.
            if let Some(equal) = sample_mode(&mut keys) {
                self.partition_equal(arr, equal, varying);
                return;
            }
            // Lift a short end run rather than move the whole node to separate it.
            let bit = KeyValue::<T>::from_usize(1) << KeyValue::<T>::from_usize(top - 1);
            if let Some(outlier) = outlier_run(arr, &keys, bit) {
                self.sort_outlier_run(arr, outlier);
                return;
            }
        }
        let digit = choose_split_digit(arr, top, self.tile_bytes);
        self.partition_digit(arr, digit, varying);
    }

    /// Partitions by digit, then sorts each unfinished child.
    fn partition_digit(&mut self, arr: &mut [T], digit: Digit, varying: u128) {
        let record_bytes = std::mem::size_of::<T>();
        // Bucket boundaries come from the block engine. The split digit is
        // constant inside each child, so its bits are excluded from the next scan.
        let candidate = varying & !digit.mask::<T>().to_u128();
        let tile_bytes = self.tile_bytes;
        let mut visit = |child: &mut [T]| {
            if child.len().saturating_mul(record_bytes) <= tile_bytes {
                let varying = scan_varying_bits(child, candidate);
                self.sort_tile(child, varying);
            }
        };
        let ends = block::partition_digit_with_ends(arr, digit.width, digit.shift, &mut visit);
        let mut start = 0;
        for &end in &ends {
            if end > start {
                let child = &mut arr[start..end];
                if child.len().saturating_mul(record_bytes) > tile_bytes {
                    let varying = scan_varying_bits(child, candidate);
                    self.sort_node(child, varying);
                }
            }
            start = end;
        }
    }

    /// Partitions below, equal to, and above a sampled key. The equal band is done.
    fn partition_equal(&mut self, arr: &mut [T], equal: KeyValue<T>, parent_varying: u128) {
        let classify = move |record: &T| {
            let key = record.key();
            usize::from(key >= equal) + usize::from(key > equal)
        };
        let ends = block::partition_by(arr, 4, classify);
        // The block engine supplies the boundaries of all three bands.
        for range in [0..ends[0], ends[1]..arr.len()] {
            let child = &mut arr[range];
            if child.len() > 1 {
                let varying = scan_varying_bits(child, parent_varying);
                self.sort_node(child, varying);
            }
        }
    }

    /// Moves a run of disagreeing records to the end its bit belongs to, then
    /// finishes both parts: the body by the ordinary recursion, the run by
    /// insertion.
    fn sort_outlier_run(&mut self, arr: &mut [T], outlier: OutlierRun) {
        let len = arr.len();
        let run = outlier.len;
        // The run belongs before the body when its bit is clear and after it when
        // set: every record outside the run has the opposite bit.
        let before = !outlier.set;
        if outlier.head && !before {
            arr.rotate_left(run);
        } else if !outlier.head && before {
            arr.rotate_right(run);
        }
        let body = if before {
            &mut arr[run..]
        } else {
            &mut arr[..len - run]
        };
        self.sort_node(body, outlier.body_varying);
        let lifted = if before {
            &mut arr[..run]
        } else {
            &mut arr[len - run..]
        };
        insertion_sort(lifted);
    }

    fn reserve_scratch(&mut self, len: usize) {
        if self.scratch.len() < len {
            self.scratch.reserve_exact(len - self.scratch.len());
            self.scratch.resize_with(len, MaybeUninit::<T>::uninit);
        }
    }

    /// Partitions a tile through scratch, copies back, then finishes each bucket.
    fn partition_tile(&mut self, arr: &mut [T], digit: Digit, varying: u128) {
        let len = arr.len();
        self.reserve_scratch(len);
        let scratch = self.scratch.as_mut_ptr().cast::<T>();
        let mut counts = MaybeUninit::<[u16; MAX_TILE_BUCKETS]>::uninit();
        let counts_ptr = counts.as_mut_ptr().cast::<u16>();
        // SAFETY: scratch has room for len records in a separate allocation.
        // A key panic leaves the complete source tile intact.
        let largest = unsafe { counting_pass(arr.as_ptr(), scratch, len, digit, counts_ptr) };
        // SAFETY: scratch now holds the complete record set; the tile holds stale copies.
        unsafe { std::ptr::copy_nonoverlapping(scratch, arr.as_mut_ptr(), len) };
        // With insertion-sized buckets, whole-tile insertion has the same inversion
        // count as separate bucket sorts and skips the boundary walk.
        if largest as usize <= INSERTION_MAX {
            insertion_sort(arr);
            return;
        }
        // Read boundaries from the completed cursors; recomputing them from keys
        // measured 48% slower when there was roughly one bucket per record.
        let candidate = varying & !digit.mask::<T>().to_u128();
        let ends = unsafe { std::slice::from_raw_parts(counts_ptr, 1usize << digit.width) };
        let mut start = 0;
        for &end in ends {
            let end = end as usize;
            debug_assert!(start <= end && end <= len);
            // Insertion-sized children need no variation scan.
            if end - start > INSERTION_MAX {
                let child = &arr[start..end];
                let varying = scan_varying_bits(child, candidate);
                self.sort_tile(&mut arr[start..end], varying);
            } else if end > start + 1 {
                insertion_sort(&mut arr[start..end]);
            }
            start = end;
        }
    }

    /// Finishes a tile by insertion, an MSD partition, or LSD counting passes.
    fn sort_tile(&mut self, arr: &mut [T], varying: u128) {
        let len = arr.len();
        if len < 2 || varying == 0 {
            return;
        }
        if len <= INSERTION_MAX {
            insertion_sort(arr);
            return;
        }
        let mut digits = [Digit { shift: 0, width: 0 }; MAX_DIGITS];
        let passes = plan_digits(
            varying,
            tile_digit_bits(len, std::mem::size_of::<T>()),
            &mut digits,
        );
        // Use an MSD partition only when it costs strictly fewer passes.
        if passes > MSD_LEVEL_PASSES {
            let distinct = sample_distinct_digits(arr, digits[0]);
            // Estimate the partition depth from sampled occupancy until the largest
            // child fits insertion; each level costs MSD_LEVEL_PASSES flat passes.
            let distinct = distinct.max(1);
            let mut largest = len;
            let mut depth = 0;
            while largest > INSERTION_MAX && depth < passes {
                depth += 1;
                largest /= distinct;
            }
            let level_cost = MSD_LEVEL_PASSES * depth.max(1);
            if level_cost < passes {
                self.partition_tile(arr, digits[0], varying);
                return;
            }
        }
        self.reserve_scratch(len);
        let scratch_ptr = self.scratch.as_mut_ptr().cast::<T>();
        debug_assert!(len <= MAX_TILE_RECORDS);
        // The counting pass initializes only the used prefix of the cursor table.
        let mut counts = MaybeUninit::<[u16; MAX_TILE_BUCKETS]>::uninit();
        let counts_ptr = counts.as_mut_ptr().cast::<u16>();

        // Restore the live source on normal return and on key panic.
        let tile = arr.as_mut_ptr();
        let mut guard = RestoreTile {
            tile,
            len,
            source: tile,
        };
        let mut destination: *mut T = scratch_ptr;
        for &digit in digits[..passes].iter().rev() {
            // SAFETY: source holds the complete record set; destination has room
            // for len records in a separate allocation. Both buffers remain live.
            unsafe { counting_pass(guard.source, destination, len, digit, counts_ptr) };
            std::mem::swap(&mut guard.source, &mut destination);
        }
    }
}

/// `source` holds the complete record set from the last successful pass.
/// Restores it to the tile if it resides in scratch.
struct RestoreTile<T> {
    tile: *mut T,
    len: usize,
    source: *mut T,
}

impl<T> Drop for RestoreTile<T> {
    fn drop(&mut self) {
        if self.source != self.tile {
            // SAFETY: source holds the complete set in scratch; tile holds stale
            // copies, and the two allocations do not overlap.
            unsafe {
                std::ptr::copy_nonoverlapping(self.source, self.tile, self.len);
            }
        }
    }
}

#[cfg(test)]
mod tests;
