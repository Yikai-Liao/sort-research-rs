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
//! Four decisions are made from one counted quantity each, not from thresholds:
//!
//! - **Sort at all**: a monotone input is answered by one probe (`probe_order`),
//!   ascending immediately and descending by a reversal with its equal-key runs
//!   repaired. Nothing else in the sort can see order, and the probe costs two
//!   comparisons on random data because it gives up at the first inversion.
//! - **Split or stream**: count the passes the node needs if it is streamed, and
//!   split whenever there is more than one of them (`split_pays`). The
//!   tile boundary already says whether the working set is cache resident, so
//!   the counting rule needs no rate, no bandwidth and no cache size.
//! - **How wide a split peels**: a window that concentrates the keys buys nothing,
//!   so a sampled node widens its digit until a quarter of its buckets are
//!   occupied (up to `SPLIT_RADIX_MAX`). Zipf-shaped keys that used to
//!   repeat the same one-bucket level get a level that divides: -17 % of the
//!   work at 1M, measured in instructions, with random nodes unmoved because
//!   their window already passes the sample.
//! - **Leaf digit width and placement**: wider digits mean fewer passes but a
//!   bigger bucket table, so the width is the widest whose table is no larger
//!   than the tile itself (`leaf_radix`), and the windows are placed on the
//!   varying bits rather than on a fixed grid (`plan_digits_packed`).
//! - **How a leaf finishes**: one MSD level over the tile when that is strictly
//!   cheaper than the flat counting plan, an insertion sort below its crossover,
//!   and the flat plan otherwise. The three are the same partition kernel called
//!   with different widths, not three algorithms.
//!
//! Constant bits are dropped everywhere, including in the split's fan-out: a
//! digit that cannot divide the data would otherwise buy 256 empty buckets, and
//! a node whose keys differ only in their low byte pays one pass instead of
//! `key width / digit width`.
//!
//! Records are never lost or duplicated, including through a panic. Every leaf
//! keeps the complete record set in one buffer for the whole of every pass:
//! the counting leaf reads a key before it moves the record, the insertion leaf
//! only swaps, and the MSD level reads the tile and writes only the scratch,
//! which is a complete copy of the set by the time it returns. A panic from the
//! key extractor therefore leaves every record reachable, and the counting leaf
//! copies the scratch buffer back before resuming, but only for types that own a
//! resource -- a duplicated record is a double free there, while a plain key
//! type only risks an arbitrary order, so integer keys compile without any
//! recovery at all.
//! `sort` is stable: equal keys keep their input order through the block
//! engine's permutation and through every leaf.

use std::mem::MaybeUninit;

use crate::other::radix_support::{Key, Unsigned};
use crate::other::rust_block_lsd::core as block;

type SortKey<T> = <T as Key>::Value;

/// Bits per digit for the MSD split and for the block engine's own LSD passes.
/// The block engine's block-size coefficient is tuned for this width.
const RADIX: usize = 8;

/// Widest digit the split may peel. The block engine's bucket table is sized for
/// 12 bits (it asserts the bound), and a wider fan-out makes every write in the
/// split's last pass land in a different 40 KiB region -- measured in section 21
/// as a loss even when the extra buckets are all occupied.
const SPLIT_RADIX_MAX: usize = 12;
/// Widest digit a leaf may use. The cursor table is `2 << width` bytes of
/// `u16`, so this is a 32 KiB stack array in the worst case; the leaf only
/// clears and scans the part its own digit uses.
const LEAF_RADIX_MAX: usize = 14;
/// Narrowest digit a leaf may use.
const LEAF_RADIX_MIN: usize = 6;
/// Largest tile finished with LSD passes, in bytes of record data.
const TILE_BYTES: usize = 64 * 1024;
/// Flat passes that one MSD level inside the tile costs: histogram one, scatter
/// two, copy back two, over two traversals per flat pass. It is the leaf's
/// break-even as well as its price in the routing model, so the two can never
/// drift apart.
const MSD_LEVEL_PASSES: usize = 3;
/// Tiles at or below this many records are finished with an insertion sort
/// instead of paying for a bucket table per remaining digit.
const INSERTION_MAX: usize = 48;
/// Cursors are `u16`, which caps a tile at 65535 records. That is a constant
/// (1 MiB even for 16-byte records), so the auxiliary space stays O(sqrt n).
const MAX_TILE_RECORDS: usize = u16::MAX as usize;
/// Keys a leaf samples to judge whether its top window divides the tile.
const MSD_SAMPLE: usize = 128;

/// How many distinct digits an evenly spaced sample shows for one window.
///
/// The most significant window is the only one an MSD level may use -- a lower
/// one would order the tile by the wrong bits -- and it can cover a single bit
/// that few records set, which makes the partition divide nothing. The sample
/// makes that visible for the price of one pass over 128 keys.
fn sampled_buckets<T: Key>(arr: &[T], digit: Digit) -> usize {
    let mask = digit.mask::<T>();
    let shift = T::Value::from_usize(digit.shift);
    // A plain array: 2 KiB of zero-fill is below what a 1024-key sample costs,
    // and the bitmap is smaller than the 16 KiB cursor table it once mirrored.
    let mut seen = [0u64; MAX_BUCKETS / 64];
    let sample = MSD_SAMPLE.min(arr.len());
    let step = (arr.len() / sample).max(1);
    let mut distinct = 0;
    for index in (0..arr.len()).step_by(step).take(sample) {
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
const PLAN_DIGITS: usize = 48;

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

/// Walks the buckets a split just produced, extracting every key once and
/// accumulating the child's variation while the bucket boundary is still open, so
/// no child pays for a second scan of its own keys. The boundary comes from
/// comparing digits rather than from the split's cursor table, which is the
/// cheaper source here; inside a tile the cursor table was measured 48 % faster,
/// so the two walks stay separate.
fn for_each_bucket<T: Key>(
    arr: &mut [T],
    digit: Digit,
    mut finish: impl FnMut(&mut [T], Variation<T>),
) {
    let mask = digit.mask::<T>();
    let shift = T::Value::from_usize(digit.shift);
    let bucket_of = |key: SortKey<T>| ((key & mask) >> shift).to_usize();
    let mut start = 0;
    while start < arr.len() {
        let first = arr[start].key();
        let bucket = bucket_of(first);
        let (mut union, mut intersection) = (first, first);
        let mut end = start + 1;
        while end < arr.len() {
            let key = arr[end].key();
            if bucket_of(key) != bucket {
                break;
            }
            union |= key;
            intersection = intersection & key;
            end += 1;
        }
        finish(
            &mut arr[start..end],
            Variation {
                union,
                intersection,
            },
        );
        start = end;
    }
}

/// Writes the passes a leaf will run into `digits`, highest first, and returns
/// how many there are. Every pass covers only varying bits: constant bits cannot
/// change the order, so dropping whole digits keeps a node full of duplicates
/// cheap, and narrowing a partially varying digit keeps a skewed node cheap,
/// because both a split's bucket count and a leaf's digit width follow the digit
/// they are given.
///
/// The windows pack, instead of following a fixed grid over the key width.
/// The grid splits a cluster of varying bits that straddles a cell boundary into
/// two passes and gives one pass to every other cell, which is why a key with 31
/// varying bits scattered over 62 used to take six passes where three windows
/// suffice: a window runs from the lowest varying bit not yet consumed up to the
/// highest one within `width` bits, and only that span.
///
/// Passes come out most significant first, the order the leaf consumes them in
/// (it walks the plan backwards, so the last entry runs first). Disjoint spans
/// cannot interleave two keys' digits differently from the key order.
///
/// The plan stays a slice of a caller-owned array: this toolchain's layout query
/// ICEs when a function passes an array whose length is a path constant.
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
    /// 0 runs the leaf as shipped, 1 runs the flat plan only, 2 runs the MSD level
    /// only. The two non-zero values are the two halves of the leaf ablation.
    leaf_mode_test: u8,
}

/// The knobs a sort can be given. The defaults are the measured constants;
/// measurement code pins one field instead of the library growing a function
/// per combination of them.
#[derive(Clone, Copy)]
pub struct Policy {
    /// Largest leaf, in bytes of record data. Caps the auxiliary space.
    pub tile_bytes: usize,
    /// Tiles at or below this many records are finished with an insertion sort.
    pub insertion_max: usize,

}


impl Default for Policy {
    fn default() -> Self {
        Policy {
            tile_bytes: TILE_BYTES,
            insertion_max: INSERTION_MAX,
        }
    }
}

/// Sorts by converted radix key, stably, in O(n) time and O(sqrt(n)) auxiliary
/// space. See the module documentation for the shape of the algorithm.
pub(super) fn sort<T: Key>(arr: &mut [T]) {
    sort_with_policy(arr, Policy::default());
}

/// Sorts without the tile's MSD level: the measurement surface for whether that
/// level still earns its keep. Not part of the sort's contract.
pub(super) fn sort_with_leaf_plan<T: Key>(arr: &mut [T]) {
    run(arr, Policy::default(), 1);
}

/// Sorts using only the tile's MSD level, never the flat plan: the other half of
/// the leaf ablation. Measurement only.
pub(super) fn sort_with_msd_only<T: Key>(arr: &mut [T]) {
    run(arr, Policy::default(), 2);
}

/// Sorts with an explicit policy. Every knob is a local cost decision: the tile
/// caps the auxiliary space on its own, so which leaf finishes a tile never
/// changes the O(sqrt n) bound.
pub(super) fn sort_with_policy<T: Key>(arr: &mut [T], policy: Policy) {
    run(arr, policy, 0);
}

fn run<T: Key>(arr: &mut [T], policy: Policy, leaf_mode_test: u8) {
    let record_bytes = std::mem::size_of::<T>();
    if arr.len() < 2 || record_bytes == 0 {
        return;
    }
    let tile_bytes = policy
        .tile_bytes
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
        insertion_max: policy.insertion_max,
        leaf_mode_test,
    };
    context.sort_node(arr, variation);
}

// --- Cost model -----------------------------------------------------------
//
// The split decision below is the same shape as the block-size formula in
// `rust_block_lsd::core`: minimize predicted cost instead of tuning a byte
// threshold. The constants are machine facts about moving record bytes, not
// properties of a data distribution.

/// The passes a tile-sized range really costs: the flat plan, or one MSD level
/// per level of shrinking when that is cheaper.
///
/// The router and the leaf both ask this, so they cannot price the same tile
/// differently: a window that divides only a little leaves one child holding
/// almost everything, and a level then buys nothing over the flat plan.
fn tile_cost(
    flat_passes: usize,
    records: usize,
    largest_share: f64,
    insertion_max: usize,
) -> usize {
    if flat_passes <= MSD_LEVEL_PASSES {
        return flat_passes;
    }
    let share = largest_share.clamp(f64::MIN_POSITIVE, 1.0);
    let mut largest = records as f64;
    let mut depth = 0;
    while largest > insertion_max as f64 && depth < flat_passes {
        depth += 1;
        largest *= share;
    }
    (MSD_LEVEL_PASSES * depth).min(flat_passes)
}


/// Whether an MSD split costs less than running the passes level by level.
///
/// Both sides are counted in passes over the node, so nothing about bandwidth,
/// latency or cache size enters: streaming costs one pass per varying digit;
/// splitting costs the split itself plus the passes of the child that keeps the
/// work, each discounted by what a cache-resident working set is worth. A child
/// too large for the tile earns no discount, which is what makes the rule stop
/// splitting once the children are still large.
fn split_pays(cold_passes: usize) -> bool {
    // One rule, no machine facts: while the node needs more than one pass, split it.
    // A split costs SPLIT_PASSES passes over the node and leaves children that the
    // tile boundary then finishes in cache; streaming costs cold_passes passes over
    // the whole node, which is more than one pass of the same bytes. The pass count
    // is what decides, and the ratio that used to appear here is folded into the
    // tile boundary: a node over the tile is cold by construction.
    cold_passes >= 2
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

/// Sorts one node level by level over the whole array, least significant digit
/// first: the radix algorithm itself, without the recursion.
///
/// This is what a node gets when the model finds that splitting it costs more
/// than the passes it replaces -- a node with one or two varying digits, or one
/// whose keys pile into a single bucket. Each pass is the same stable block
/// permutation the split uses, so the code is the split without its children,
/// and the digit windows come from the same planner the leaves use, so a node
/// pays only for the bits that vary.
///
/// Out of line: at 1M the model rarely takes this branch and keeping it out of
/// line leaves the split's code in one piece, but at 10M every child of the root
/// takes it, so it must not be marked cold -- that arms the optimizer against the
/// path that most of the data ends up on.
#[inline(never)]
fn stream_node<T: Key>(arr: &mut [T], variation: Variation<T>) {
    let mut digits = [Digit { shift: 0, width: 0 }; PLAN_DIGITS];
    let passes = plan_digits_packed(variation.varying(), RADIX, &mut digits);
    // Least significant window first. The engine is created once for all the
    // passes and compacts once, which is the difference between this and calling
    // the single-pass partition in a loop: that paid a full compaction per pass.
    let mut plan = [(0usize, 0usize); PLAN_DIGITS];
    for (pass, slot) in plan[..passes].iter_mut().enumerate() {
        let digit = digits[passes - 1 - pass];
        *slot = (digit.width, digit.shift);
    }
    block::sort_with_digits(arr, RADIX, &plan[..passes]);
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
/// `len` writable slots of a different allocation. `counts` must point at
/// room for `1 << digit.width` entries that need not be initialised: this
/// pass writes every entry it later reads. `len` must not exceed `u16::MAX`:
/// the cursors are `u16`, and `sort_with_policy` clamps the tile to
/// `MAX_TILE_RECORDS` for exactly this reason. The array
/// length is not in the signature on purpose: this toolchain ICEs on an
/// array-typed parameter whose length is a path constant.
#[inline]
unsafe fn counting_pass<T: Key>(
    source: *const T,
    destination: *mut T,
    len: usize,
    digit: Digit,
    cursor: *mut u16,
) {
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
            *cursor.add(bucket) = slot.wrapping_add(1);
            // The clamp is dead for a key that is a function of the record, and it
            // is what keeps a key that is not one from writing outside the tile:
            // `Key` is a safe trait, so a wrong implementation may garble the order
            // but must not be able to cause memory unsafety.
            let target = (slot as usize).min(len - 1);
            std::ptr::write(destination.add(target), std::ptr::read(record));
        }
    }
}


/// Digit width for a leaf: the widest digit whose cursor table is no larger
/// than the tile itself.
///
/// Wider digits mean fewer passes, and the table is only worth its footprint
/// while it stays smaller than the records it is indexing. That single rule
/// reproduces both measured optima exactly, which a pass-count-plus-table cost
/// model did not: a 31 KiB tile of 8-byte records picks 13 bits (`2 << 13` is
/// 16 KiB, `2 << 14` would be 32 KiB and measured 21 % slower, the table having
/// become the working set), and a 61 KiB tile of 16-byte records picks 14
/// (32 KiB table against 61 KiB of records, measured best at 1M).
fn leaf_radix(records: usize, record_bytes: usize) -> usize {
    if let Some(w) = leaf_width_override() {
        return w;
    }
    let tile_bytes = records.saturating_mul(record_bytes);
    // The widest table that still fits: `2 * 2^width` bytes of cursors.
    let widest = (usize::BITS - (tile_bytes / 2).max(1).leading_zeros()) as usize - 1;
    // A digit wider than the records can fill is not free: the counter table still
    // has to be cleared and scanned, and every bucket still costs a boundary read.
    // A 128-byte tile of 278 records was picking 13 bits, i.e. 8192 buckets for 278
    // records; capping at two bits above the record count keeps the table in the
    // same order as the data it sorts.
    let by_records = (usize::BITS - records.max(1).leading_zeros()) as usize + 2;
    widest.min(by_records).clamp(LEAF_RADIX_MIN, LEAF_RADIX_MAX)
}

/// Measurement hook: RADIX_LEAF_WIDTH pins the leaf digit width for the same reason
/// RADIX_SPLIT_WIDTH exists -- a sweep then costs one build. Unset in every shipped run.
fn leaf_width_override() -> Option<usize> {
    use std::sync::OnceLock;
    static WIDTH: OnceLock<Option<usize>> = OnceLock::new();
    *WIDTH.get_or_init(|| {
        std::env::var("RADIX_LEAF_WIDTH")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|w| (LEAF_RADIX_MIN..=LEAF_RADIX_MAX).contains(w))
    })
}

/// Measurement hook: RADIX_SPLIT_WIDTH pins the split digit width so a sweep costs
/// one build instead of one build per width. Unset in every shipped run.
fn width_override() -> Option<usize> {
    use std::sync::OnceLock;
    static WIDTH: OnceLock<Option<usize>> = OnceLock::new();
    *WIDTH.get_or_init(|| {
        std::env::var("RADIX_SPLIT_WIDTH")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|w| (1..=LEAF_RADIX_MAX).contains(w))
    })
}

/// Digit width for one MSD split.
///
/// Aim for children of about half the tile budget: large enough that the split
/// pass stays amortized, small enough that the common child needs no further
/// split. The bucket count, and with it the block engine's scratch, then
/// follows the node size instead of being fixed at 256 even for a node of a few
/// thousand records.
fn split_radix(node_len: usize, record_bytes: usize, tile_bytes: usize) -> usize {
    if let Some(width) = width_override() {
        return width;
    }
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
        // What the node costs if it is streamed instead of split: the passes its
        // varying digits need at the split width. That count is the whole rule.
        let mut scratch_digits = [Digit { shift: 0, width: 0 }; PLAN_DIGITS];
        let cold_passes = plan_digits_packed(varying, RADIX, &mut scratch_digits);
        // The digit the split would peel, decided once.
        let top = (u128::BITS - varying.leading_zeros()) as usize;
        let mut radix = split_radix(records, record_bytes, self.tile_bytes)
            .min(top)
            .max(1);
        // A window that puts nearly every record in one bucket buys nothing: the
        // child repeats the same decision one level down and the level's two passes
        // are wasted. An MSD level may only peel the *highest* bits (a lower window
        // would order by the wrong bits), so the single freedom it has is how many
        // of them to peel: widening pulls lower bits into the bucket index and
        // splits a concentrated key set into distinct buckets.
        //
        // The sample costs one pass over MSD_SAMPLE keys, which at the top of a
        // 15 MiB node is nothing, and it only ever widens a node whose sample shows
        // the window does not divide -- a random node is already balanced at the
        // size-chosen width, so its decision does not move.
        while radix < top && radix + 2 <= SPLIT_RADIX_MAX {
            let probe = Digit {
                shift: top - radix,
                width: radix,
            };
            // A quarter of the buckets occupied is the bar: below it the level is
            // mostly one bucket and a wider one is strictly better, above it the
            // extra table and boundary reads cost more than the balance they buy.
            if sampled_buckets(arr, probe) * 4 >= 1usize << radix {
                break;
            }
            radix += 2;
        }
        let digit = Digit {
            shift: top - radix,
            width: radix,
        };
        // nothing left to price: the rule reads one number.
        // One line per node says everything the routing decided, so a trace diff
        // between two builds is a decision diff.
        let routed = split_pays(cold_passes);
        if trace() {
            eprintln!(
                "route len={} bytes={} passes={} split={}",
                records, node_bytes, cold_passes, routed
            );
        }
        if !routed {
            // Splitting would cost more than the passes it replaces. Finish the node
            // with the radix passes it was priced at: no buffer the size of the node,
            // and the variation scan is already done.
            stream_node(arr, variation);
            return;
        }
        // MSD split: one stable block-permutation pass over the whole node.
        // The fan-out follows the node size. A node that is only slightly over
        // the tile budget must not pay for 256 output streams and the block
        // engine's per-bucket scratch floor.
        if trace() {
            eprintln!("  split radix={} shift={}", radix, top - radix);
        }
        block::partition_digit(arr, digit.width, digit.shift);
        // Within a bucket every key shares the digits above the split, so they fall
        // out of the child's variation on their own.
        for_each_bucket(arr, digit, |child, variation| self.sort_node(child, variation));
    }

    fn set_scratch_len(&mut self, len: usize) {
        if self.scratch.len() < len {
            self.scratch.clear();
            self.scratch.reserve_exact(len);
            self.scratch.resize_with(len, MaybeUninit::<T>::uninit);
        }
    }

    /// One stable MSD level over the tile, then each bucket finished on its own.
    ///
    /// The leaf's private buffer is enough for a stable partition into exact
    /// positions: the counting pass writes every record at its bucket's start plus
    /// its rank inside the bucket, which keeps equal keys in order and leaves the
    /// buckets contiguous, so each one can be finished independently. The pass
    /// count then follows how many records the tile holds instead of how many
    /// digits the key has -- 3906 records of 64 bits need six flat passes and one
    /// of these.
    ///
    /// Traffic, in units of one tile traversal: the histogram is one, the scatter
    /// is two, the copy back is two. A flat pass is two, so this pays off from
    /// three flat passes upwards, which is what the caller checks.
    fn msd_leaf(&mut self, arr: &mut [T], digit: Digit) {
        let len = arr.len();
        self.set_scratch_len(len);
        let scratch = self.scratch.as_mut_ptr().cast::<T>();
        let mut counts = MaybeUninit::<[u16; MAX_BUCKETS]>::uninit();
        let counts_ptr = counts.as_mut_ptr().cast::<u16>();
        // SAFETY: the tile holds `len` initialized records, the scratch buffer has
        // room for `len`, and they are distinct allocations. The tile is only read
        // here, so a panic from the key extractor leaves the whole record set in
        // the tile and nothing needs undoing -- unlike the flat plan, this level
        // needs no recovery of its own.
        unsafe { counting_pass(arr.as_ptr(), scratch, len, digit, counts_ptr) };
        // The scatter wrote every record exactly once, so the scratch is a complete
        // record set and the tile is stale. Copying back is therefore valid whether
        // or not the pass finished, and it restores the invariant the rest of the
        // leaf relies on: the tile always holds the set.
        // SAFETY: as above, plus `len` records were written to the scratch.
        unsafe { std::ptr::copy_nonoverlapping(scratch, arr.as_mut_ptr(), len) };
        // Boundaries come from the cursor table rather than from comparing keys:
        // with a bucket per record the comparison walk is one unpredictable branch
        // per record, which measured 48 % slower than reading the ends this pass
        // already computed. A bucket of one record is done; a larger one has its
        // variation re-read, and a flat plan on equal keys returns immediately.
        let buckets = 1usize << digit.width;
        let mut start = 0;
        for bucket in 0..buckets {
            // SAFETY: the pass filled every entry below buckets, and cumulative
            // counts never decrease, so start <= end <= len.
            let end = unsafe { *counts_ptr.add(bucket) } as usize;
            debug_assert!(start <= end && end <= len);
            // A bucket that goes straight to insertion does not need its variation:
            // the scan would only be thrown away, and a leaf-sized bucket is where
            // that scan is a measurable share of the bucket's own work.
            if end - start > self.insertion_max {
                let (union, intersection) = block::varying_key_bits(&arr[start..end]);
                self.lsd_leaf(&mut arr[start..end], Variation { union, intersection });
            } else if end > start + 1 {
                insertion_leaf(&mut arr[start..end]);
            }
            start = end;
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
        // The leaf picks its own digit width: inside the tile the only cost that
        // scales with it is the bucket table, so a wider digit buys fewer passes
        // while the table still fits in L1 next to the records.
        let mut digits = [Digit { shift: 0, width: 0 }; PLAN_DIGITS];
        let passes = plan_digits_packed(
            variation.varying(),
            leaf_radix(len, std::mem::size_of::<T>()),
            &mut digits,
        );
        // Strictly cheaper only: a level that merely ties the flat plan costs a memcpy
        // and a bucket walk for nothing, and measured 5 % worse on `bands8`.
        if self.leaf_mode_test != 1 && passes > MSD_LEVEL_PASSES {
            // One level costs about two and a half flat passes, and the depth it
            // needs is set by how evenly its window divides the tile: a window
            // that only a few records distinguish leaves one child holding
            // almost everything and recurses until the child is small, which is
            // how a locally-clustered tile ended up 3x slower than the flat plan.
            // Those keys also make a flat pass cheap, so the sample pays for
            // itself by keeping this decision honest.
            let distinct = sampled_buckets(arr, digits[0]).max(2);
            let msd_cost = tile_cost(passes, len, 1.0 / distinct as f64, self.insertion_max);
            if trace() {
                eprintln!(
                    "leaf-msd? len={} flat={} distinct={} msd_cost={}",
                    len, passes, distinct, msd_cost
                );
            }
            // Mode 2 asks the other ablation question: without the flat plan, is the
            // level plus its bucket recursion enough? Only a window that actually
            // splits is allowed, so the forced mode cannot recurse forever.
            let take = if self.leaf_mode_test == 2 {
                distinct > 1
            } else {
                msd_cost < passes
            };
            if take {
                self.msd_leaf(arr, digits[0]);
                return;
            }
        }
        self.set_scratch_len(len);
        let scratch_ptr = self.scratch.as_mut_ptr().cast::<T>();
        // A tile is at most `MAX_TILE_RECORDS` records, so 16-bit cursors are
        // enough, and they are the difference between a cursor table that stays
        // in one L1 set and one that does not.
        debug_assert!(len <= MAX_TILE_RECORDS);
        // Deliberately uninitialised: the counting pass clears the digit's own
        // prefix of this table before it reads it, so zeroing all
        // MAX_BUCKETS entries on every leaf would be pure overhead -- 32 KiB of
        // memset against a 31 KiB tile, which measured 14 % on u64 1M.
        let mut counts = MaybeUninit::<[u16; MAX_BUCKETS]>::uninit();
        let counts_ptr = counts.as_mut_ptr().cast::<u16>();

        // The record set lives in exactly one of the two buffers for the whole
        // leaf, so the restore is the same whether a pass returns or the key
        // extractor panics: put the set back if it is not already in the tile.
        // Doing that in a guard, instead of catching the panic only for the
        // types that own a resource, makes the promise hold for every type.
        let mut guard = Restore {
            tile: arr.as_mut_ptr(),
            len,
            scratch: scratch_ptr,
            live: false,
        };
        {
            let mut source: *mut T = guard.tile;
            let mut destination: *mut T = scratch_ptr;
            let mut source_is_tile = true;
            for index in (0..passes).rev() {
                guard.live = !source_is_tile;
                // SAFETY: the source holds `len` initialized records, the
                // destination has room for `len` records, and both are distinct
                // allocations that stay alive for the whole leaf.
                unsafe {
                    counting_pass(source, destination, len, digits[index], counts_ptr);
                }
                std::mem::swap(&mut source, &mut destination);
                source_is_tile = !source_is_tile;
            }
            guard.live = !source_is_tile;
        }
    }
}

/// Puts the record set back in the tile when a leaf stops with it in scratch.
///
/// The scratch buffer holds every record of the tile while a pass is in flight,
/// so one copy back restores the complete set: on an odd number of passes, and on
/// a panic from the key extractor, however far into the pass it happened. A guard
/// covers both, so the promise holds for every record type, not only the owning
/// ones: a duplicated record is a double free there, while a plain key type would
/// otherwise keep stale duplicates in the tile.
/// Raw pointers on purpose: the leaf owns both buffers for the whole pass, and a
/// shared reference to the tile would alias the writes the passes make through it.
struct Restore<T: Key> {
    tile: *mut T,
    len: usize,
    scratch: *mut T,
    live: bool,
}

impl<T: Key> Drop for Restore<T> {
    fn drop(&mut self) {
        if self.live {
            // SAFETY: the scratch buffer holds initialized records for the whole
            // tile; the slots being overwritten are stale records already moved out.
            unsafe {
                std::ptr::copy_nonoverlapping(self.scratch, self.tile, self.len);
            }
        }
    }
}

#[cfg(test)]
mod tests;
