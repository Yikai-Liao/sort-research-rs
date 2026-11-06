//! Stable radix sort with block partitions above a default 64 KiB tile budget.
//!
//! Ordered inputs return or reverse stably; nearly ordered inputs get a budgeted
//! insertion pass. Large nodes partition by an MSD digit, a sampled repeated key,
//! or an end run separated by the highest varying bit. Tiles finish with insertion,
//! an MSD partition, or LSD counting passes over reusable scratch.
//!
//! Vocabulary: a *node* is a slice larger than the tile budget; it is split by
//! a stable block partition. A *tile* fits the budget and is finished here, with
//! insertion sort, a tile-local MSD partition, or LSD counting passes. A *digit*
//! is the window of key bits one pass handles ([`Digit`]); a *cursor table*
//! holds one `u16` position per bucket.
//!
//! Constant bits are skipped. Wide keys whose variation fits one 64-bit half use
//! a narrowed view. The block engine uses O(sqrt(n)) scratch; tile scratch is bounded.
//! For deterministic keys, equal keys retain their input order and key panics
//! preserve the record set.
//!
//! Each recursive tile MSD level keeps a cursor table of up to 32 KiB on the
//! stack. Depth is bounded by ceil(key bits / MIN_TILE_DIGIT_BITS), at most 22
//! levels for u128. Actual frames depend on the compiler; callers on small-stack
//! threads must budget for these tables as well as ordinary call frames.

use std::mem::MaybeUninit;

use crate::other::radix_support::{Key, RecordType, Unsigned};
use crate::other::rust_block_lsd::core as block;

type KeyValue<T> = <T as Key>::Value;

/// Initial split width; also the block engine's tuned width.
const SPLIT_BITS: usize = 8;

/// Split fan-out is bounded by the block engine's widest digit.
const MAX_SPLIT_BITS: usize = block::MAX_RADIX_BITS;

/// At 14 bits the u16 cursor table occupies 32 KiB.
const MAX_TILE_DIGIT_BITS: usize = 14;
/// Maximum cursor table length.
const MAX_DIGIT_BUCKETS: usize = 1 << MAX_TILE_DIGIT_BITS;
/// Narrowest digit a leaf may use.
const MIN_TILE_DIGIT_BITS: usize = 6;
/// Default tile size in bytes of records; a tile always holds at least one record.
const TILE_BYTES: usize = 64 * 1024;
/// Cost model: an MSD level is one counting pass, and the insertion finish that
/// ends the descent costs about one more. Charging a level as three passes kept
/// three-pass plans on flat passes, which measured 11-38% slower for 32-bit keys
/// at 1K-64K records.
const MSD_FINISH_PASSES: usize = 1;
/// Tiles and buckets at or below this many records are finished with an
/// insertion sort instead of paying for a bucket table per remaining digit.
const INSERTION_MAX: usize = 48;
/// u16 cursors bound a tile to 65535 records.
const MAX_TILE_RECORDS: usize = u16::MAX as usize;
/// Keys an occupancy sample reads to judge whether a digit divides its range.
const DIGIT_SAMPLE_LEN: usize = 128;

/// Counts distinct digits in an evenly spaced sample.
fn sample_distinct_digits<T: Key>(arr: &[T], digit: Digit) -> usize {
    let mask = digit.mask::<T>();
    let shift = T::Value::from_usize(digit.shift);

    let mut seen = [0u64; MAX_DIGIT_BUCKETS / 64];
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

/// Fills `keys` with runs of `MODE_SAMPLE_RUN_LEN` consecutive keys spread over
/// the node, which avoids stride aliasing with periodic keys.
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

/// Finds a key repeated by at least `1 / MODE_SHARE_DENOMINATOR` of the sample.
/// Leaves the sample sorted; later uses may only depend on the multiset.
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

/// A side holding at most `1 / MINORITY_DENOMINATOR` of the sample is the minority.
const MINORITY_DENOMINATOR: usize = 8;

/// Whether `bit` separates a minority of the sample, and which side is the
/// minority: `true` when the records with the bit set are the few.
fn sample_minority_bit<V: Unsigned>(keys: &[V], bit: V) -> Option<bool> {
    let zero = V::from_usize(0);
    let set = keys.iter().filter(|key| (**key & bit) != zero).count();
    let clear = keys.len() - set;
    if set * MINORITY_DENOMINATOR <= keys.len() {
        Some(true)
    } else if clear * MINORITY_DENOMINATOR <= keys.len() {
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

/// Number of bits needed to represent `value`; 0 for 0.
#[inline]
fn bit_len(value: u128) -> usize {
    (u128::BITS - value.leading_zeros()) as usize
}

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

/// Capacity for packed digit windows over a 128-bit key. A bound of 22 is
/// sufficient, but shrinking this array awaits an isolated performance check.
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

/// The window over the highest remaining bits that a pass of `width` bits
/// clears, snapped to the lowest varying bit it contains: constant bits below the
/// top one stay out of the digit. Returns the digit and the bits it clears.
fn top_window(remaining: u128, width: usize) -> (Digit, u128) {
    let high = remaining.ilog2() as usize;
    let window = u128::MAX << (high + 1).saturating_sub(width);
    let low = (remaining & window).trailing_zeros() as usize;
    let digit = Digit {
        shift: low,
        width: high - low + 1,
    };
    (digit, window)
}

/// Packs varying bits into windows, highest first; the LSD passes consume the
/// plan in reverse and each window skips the constant bits below it.
///
/// A window spans at most `width` bits. When every bit below the top one varies,
/// the run is instead tiled evenly over the passes that `width` would need, so
/// 32 bits in three passes become 11 + 11 + 10 rather than 14 + 14 + 4. Both
/// plans move each record three times and cover the same bits, but the even one
/// clears, counts and walks a quarter of the bucket slots. Gaps keep the wide
/// windows, which jump over them.
fn plan_digits(varying: u128, width: usize, digits: &mut [Digit]) -> usize {
    let top = bit_len(varying);
    // Passes left to tile the contiguous run over; `None` keeps fixed windows.
    let mut even = (varying.count_ones() as usize == top).then_some(top.div_ceil(width));
    let mut remaining = varying;
    let mut len = 0;
    while remaining != 0 {
        let window_width = match even {
            Some(left) if left > 0 => bit_len(remaining).div_ceil(left),
            // Gaps keep windows of the full budget that jump over them.
            _ => width,
        };
        let (digit, window) = top_window(remaining, window_width);
        debug_assert!(len < digits.len());
        debug_assert!(digit.width <= width);
        digits[len] = digit;
        len += 1;
        remaining &= !window;
        even = even.map(|left| left.saturating_sub(1));
    }
    len
}

/// A layout-preserving view of a wide key's varying 64-bit half.
/// The discarded half is constant across the root, so order and equality are preserved.
#[repr(transparent)]
#[derive(Clone, Copy)]
struct KeyHalf<const SHIFT: usize, T: Key>(T);

impl<const SHIFT: usize, T: Key> Key for KeyHalf<SHIFT, T> {
    type Value = u64;
    #[inline]
    fn key(&self) -> u64 {
        (self.0.key().to_u128() >> SHIFT) as u64
    }
}

struct Sorter<T> {
    // Copy values in this prefix stay initialized across tiles. Grow its length
    // only after a complete cached pass; non-Copy scratch may hold stale bytes.
    scratch: Vec<MaybeUninit<T>>,
    tile_bytes: usize,
}

impl<T> Sorter<T> {
    fn new(tile_bytes: usize) -> Self {
        Sorter {
            scratch: Vec::new(),
            tile_bytes,
        }
    }
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
        .min(MAX_TILE_RECORDS.saturating_mul(record_bytes));
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
        // When the window is the whole array, estimating the swap count on copied
        // keys repeats work the records themselves can do, so spend the budget in
        // place and keep the order when it lands. A shorter window keeps the copy:
        // a failed trial on large records would move far more bytes than keys.
        let swaps = if probe == arr.len() {
            budgeted_insertion_sort(arr, probe, Key::key)
        } else {
            let mut keys: Vec<KeyValue<T>> = arr[..probe].iter().map(Key::key).collect();
            match budgeted_insertion_sort(&mut keys, probe, |&key| key) {
                // A shorter window cannot speak for the records outside it.
                Some(0) => None,
                Some(swaps) => budgeted_insertion_sort(arr, swaps * arr.len() / probe, Key::key),
                None => None,
            }
        };
        if swaps.is_some() {
            return;
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

    if T::Value::BITS > 64 {
        if varying >> 64 == 0 {
            sort_key_half::<0, T>(arr, varying, tile_bytes);
            return;
        }
        if varying & ((1u128 << 64) - 1) == 0 {
            sort_key_half::<64, T>(arr, varying, tile_bytes);
            return;
        }
    }
    Sorter::new(tile_bytes).sort_node(arr, varying);
}

/// Narrows at the root only; descendants retain the same key view.
fn sort_key_half<const SHIFT: usize, T: Key>(arr: &mut [T], varying: u128, tile_bytes: usize) {
    // SAFETY: KeyHalf is repr(transparent) over T. This reborrow is the only
    // reference used during sorting, with identical length, layout and alignment.
    let narrowed: &mut [KeyHalf<SHIFT, T>] =
        unsafe { &mut *(arr as *mut [T] as *mut [KeyHalf<SHIFT, T>]) };
    Sorter::new(tile_bytes).sort_node(narrowed, varying >> SHIFT);
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
#[inline(never)]
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
/// Source holds len initialized records; destination has len writable slots in
/// a separate allocation, and cursor has MAX_DIGIT_BUCKETS u16 slots. For cached
/// passes, any cache_tail is disjoint from records and holds len aligned u16s.
/// For uncached passes, T is Copy and destination is already fully initialized.
/// 0 < len <= u16::MAX. Partial non-Copy output must not be read or dropped.
#[inline]
unsafe fn counting_pass<T: Key, const CACHED: bool>(
    source: *const T,
    destination: *mut T,
    len: usize,
    digit: Digit,
    cursor: *mut u16,
    cache_tail: *mut u16,
) -> u16 {
    let mask = digit.mask::<T>();
    let shift = T::Value::from_usize(digit.shift);
    let buckets = 1usize << digit.width;
    let bucket_ids = if CACHED && len + buckets <= MAX_DIGIT_BUCKETS {
        cursor.add(buckets)
    } else {
        cache_tail
    };
    for slot in 0..buckets {
        // SAFETY: the caller guarantees room for `buckets` entries.
        *cursor.add(slot) = 0;
    }
    let mut largest = 0u16;
    for index in 0..len {
        // SAFETY: the caller guarantees `len` initialized records.
        let key = (*source.add(index)).key();
        let bucket = ((key & mask) >> shift).to_usize();
        if CACHED {
            // SAFETY: the scratch tail has len aligned u16 slots.
            *bucket_ids.add(index) = bucket as u16;
        }
        // SAFETY: the digit mask keeps the index below `buckets`, and no bucket
        // can hold more than `len <= u16::MAX` records.
        let count = *cursor.add(bucket) + 1;
        *cursor.add(bucket) = count;
        largest = largest.max(count);
    }
    let mut offset = 0u16;
    for slot in 0..buckets {
        // SAFETY: as above.
        let count = *cursor.add(slot);
        *cursor.add(slot) = offset;
        offset += count;
    }
    for index in 0..len {
        let record = source.add(index);
        let bucket = if !CACHED {
            // Copy records use an already initialized destination. A changing key
            // may duplicate values, but cannot create invalid or uninitialized T.
            (((*record).key() & mask) >> shift).to_usize()
        } else {
            // SAFETY: histogram initialized every cached classification before
            // any record was copied. Scatter runs no user code on this path.
            *bucket_ids.add(index) as usize
        };
        let slot = *cursor.add(bucket);
        *cursor.add(bucket) = slot.wrapping_add(1);
        let target = if CACHED {
            slot as usize
        } else {
            (slot as usize).min(len - 1)
        };
        // SAFETY: cached classifications give disjoint output intervals. The
        // uncached Copy path keeps every destination slot initialized even if
        // classifications change; clamping keeps all writes in the allocation.
        std::ptr::write(destination.add(target), std::ptr::read(record));
    }
    largest
}

/// Chooses the widest cursor table that fits beside the tile, capped by
/// record count to avoid clearing mostly empty buckets on wide records.
fn tile_digit_bits(records: usize, record_bytes: usize) -> usize {
    let tile_bytes = records.saturating_mul(record_bytes);
    // The widest table that still fits: `2 * 2^width` bytes of cursors.
    let widest = (tile_bytes / 2).max(1).ilog2() as usize;
    // Limit empty-bucket work to at most eight times the record count.
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
        // At larger widths the sample cannot fill a quarter of the buckets.
        if (1usize << width) <= 4 * DIGIT_SAMPLE_LEN {
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
        let top = bit_len(varying);
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
        // Three outcomes need four buckets because `partition_by` takes powers
        // of two; the fourth is unused, so the greater band runs to the end.
        let ends = block::partition_by(arr, 4, classify);
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

    fn reserve_scratch(&mut self, len: usize, cache: bool, buckets: usize) -> (*mut T, *mut u16) {
        // Cached digits first use the unused suffix of the existing cursor
        // table. Larger caches share the record allocation, without another
        // allocation or zero initialization. Initialized Copy scratch needs no cache.
        let tail = cache && len + buckets > MAX_DIGIT_BUCKETS;
        if tail {
            let bytes = std::mem::size_of::<T>();
            let extra = (len * std::mem::size_of::<u16>() + 1).div_ceil(bytes);
            let required = len
                .checked_add(extra)
                .expect("tile scratch capacity overflow");
            if self.scratch.capacity() < required {
                self.scratch.reserve_exact(required - self.scratch.len());
            }
        }
        if self.scratch.len() < len {
            self.scratch.reserve_exact(len - self.scratch.len());
            if !T::COPY {
                self.scratch.resize_with(len, MaybeUninit::<T>::uninit);
            }
        }
        let records = self.scratch.as_mut_ptr().cast::<T>();
        let ids = if tail {
            // SAFETY: capacity includes len records followed by 2*len+1 bytes.
            // u16 alignment needs at most one byte; the regions do not overlap.
            unsafe {
                let start = records.add(len).cast::<u8>();
                start
                    .add(start.align_offset(std::mem::align_of::<u16>()))
                    .cast::<u16>()
            }
        } else {
            std::ptr::null_mut()
        };
        (records, ids)
    }

    /// Partitions a tile through scratch, then finishes each bucket.
    fn partition_tile(&mut self, arr: &mut [T], digit: Digit, varying: u128) {
        let len = arr.len();
        let cache = !T::COPY || self.scratch.len() < len;
        let (scratch, cache_tail) = self.reserve_scratch(len, cache, 1 << digit.width);
        let mut counts = MaybeUninit::<[u16; MAX_DIGIT_BUCKETS]>::uninit();
        let counts_ptr = counts.as_mut_ptr().cast::<u16>();
        let tile = arr.as_mut_ptr();
        let mut guard = RestoreTile {
            tile,
            len,
            source: tile,
        };
        let mut destination = scratch;
        let largest = unsafe {
            if cache {
                guard.pass::<true>(&mut destination, digit, counts_ptr, cache_tail)
            } else {
                guard.pass::<false>(&mut destination, digit, counts_ptr, cache_tail)
            }
        };
        if T::COPY && cache {
            // SAFETY: the cached pass initialized every reserved record slot.
            unsafe { self.scratch.set_len(len) };
        }
        drop(guard);
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
        // Widest counting table this tile can afford; LSD spreads it over passes.
        let table_width = tile_digit_bits(len, std::mem::size_of::<T>());
        let mut digits = [Digit { shift: 0, width: 0 }; MAX_DIGITS];
        let passes = plan_digits(varying, table_width, &mut digits);
        // Compare an MSD descent against the flat plan in passes over the tile.
        // A level plus its finish cannot undercut two flat passes, so shorter
        // plans skip the occupancy sample.
        if passes > MSD_FINISH_PASSES + 1 {
            // One partition finishes the tile, so it takes the widest table rather
            // than a pass-plan window; a narrower digit leaves more to insertion.
            let (digit, _) = top_window(varying, table_width);
            // The tile is non-empty, so its sample is non-empty too. Repeats in
            // the sample cap the split; a sample with no repeats at all is limited
            // by its length, not by the keys, so the digit's buckets apply.
            // `max(1)` is redundant in value but not in codegen: it tells the
            // optimizer the divisor below is non-zero, and removing it measured
            // as a slower `sort_tile` (an extra zero check and different register
            // allocation).
            let distinct = sample_distinct_digits(arr, digit).max(1);
            let fanout = if distinct >= DIGIT_SAMPLE_LEN.min(len) {
                1usize << digit.width
            } else {
                distinct
            };
            // Estimate the descent until the largest child fits insertion; each
            // level is one counting pass and the finish is one more.
            let mut largest = len;
            let mut depth = 0;
            while largest > INSERTION_MAX && depth < passes {
                depth += 1;
                largest /= fanout;
            }
            // The tile holds more than INSERTION_MAX records, so the loop ran.
            // As above, `max(1)` is kept for the optimizer, not for the value.
            let level_cost = depth.max(1) + MSD_FINISH_PASSES;
            if level_cost < passes {
                self.partition_tile(arr, digit, varying);
                return;
            }
        }
        // A Copy buffer needs cached classification only while its initialized
        // prefix grows. Later passes and tiles reuse valid Copy values directly.
        let mut cache = !T::COPY || self.scratch.len() < len;
        let buckets = if cache {
            let width = if T::COPY {
                digits[passes - 1].width
            } else {
                digits[..passes]
                    .iter()
                    .map(|digit| digit.width)
                    .max()
                    .unwrap()
            };
            1 << width
        } else {
            0
        };
        let (scratch_ptr, cache_tail) = self.reserve_scratch(len, cache, buckets);
        debug_assert!(len <= MAX_TILE_RECORDS);
        let mut counts = MaybeUninit::<[u16; MAX_DIGIT_BUCKETS]>::uninit();
        let counts_ptr = counts.as_mut_ptr().cast::<u16>();
        let tile = arr.as_mut_ptr();
        let mut guard = RestoreTile {
            tile,
            len,
            source: tile,
        };
        let mut destination = scratch_ptr;
        for &digit in digits[..passes].iter().rev() {
            // One branch per pass keeps each scatter loop free of the decision.
            if cache {
                unsafe { guard.pass::<true>(&mut destination, digit, counts_ptr, cache_tail) };
                if T::COPY {
                    // SAFETY: the cached pass initialized the scratch prefix.
                    unsafe { self.scratch.set_len(len) };
                    cache = false;
                }
            } else {
                unsafe { guard.pass::<false>(&mut destination, digit, counts_ptr, cache_tail) };
            }
        }
    }
}

/// `source` holds initialized records from the last successful pass. Cached
/// passes preserve the record set; a changing Copy key may give arbitrary values.
/// Restores it to the tile if it resides in scratch.
struct RestoreTile<T> {
    tile: *mut T,
    len: usize,
    source: *mut T,
}

impl<T: Key> RestoreTile<T> {
    /// Safety: both buffers have len slots; cursor has MAX_DIGIT_BUCKETS slots.
    /// For an uncached pass, T is Copy and both buffers are initialized. If the
    /// cursor suffix cannot fit cached IDs, cache_tail has len aligned u16 slots.
    #[inline]
    unsafe fn pass<const CACHED: bool>(
        &mut self,
        destination: &mut *mut T,
        digit: Digit,
        cursor: *mut u16,
        cache_tail: *mut u16,
    ) -> u16 {
        let largest = counting_pass::<T, CACHED>(
            self.source,
            *destination,
            self.len,
            digit,
            cursor,
            cache_tail,
        );
        std::mem::swap(&mut self.source, destination);
        largest
    }
}

impl<T> Drop for RestoreTile<T> {
    fn drop(&mut self) {
        if self.source != self.tile {
            // SAFETY: source holds initialized records in scratch; tile holds
            // Copy values or stale non-Copy bytes. The allocations do not overlap.
            unsafe {
                std::ptr::copy_nonoverlapping(self.source, self.tile, self.len);
            }
        }
    }
}

#[cfg(test)]
mod tests;
