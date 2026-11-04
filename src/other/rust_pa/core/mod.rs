//! Serial L/H/P/F screen. No production entry point changes.
//! Each node releases its distribution workspace before visiting children.
#![cfg_attr(not(feature = "rust_pa_work"), allow(unused_variables, unused_mut))]
use crate::other::rust_pa::{RadixKey, Radixable};
use std::{mem::MaybeUninit, ptr};
mod block;
mod labeler;
mod local_distribution;
mod ordered_tiled;
mod pa;
mod packed_projection;
mod prefix_band;
use labeler::Labeler;
pub use pa::{sort_pa_experiment, sort_pa_with_threshold, PaExperiment};

type Converted<T, K> = <<T as Radixable<K>>::Key as RadixKey>::Key;

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    L,
    H,
    P,
    F,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub mode: Mode,
    /// Zero selects ceil(log256(node length)); otherwise fixed byte budget.
    pub bytes: usize,
    /// For same-coarse-prefix comparisons; zero uses the label-byte rule.
    pub coarse_bits: usize,
    pub block: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Work {
    pub projection_nodes: usize,
    pub projection_records: usize,
    pub identity_nodes: usize,
    pub nodes: usize,
    pub node_records: usize,
    pub prepare_records: usize,
    pub order_records: usize,
    pub candidate_records: usize,
    pub label_records: usize,
    pub label_evals: usize,
    pub scatter_records: usize,
    pub dense_nodes: usize,
    pub block_nodes: usize,
    pub passes: usize,
    pub compact_records: usize,
    pub tail_records: usize,
    pub runs_records: usize,
    pub unresolved_records: usize,
    pub leaves: usize,
    pub leaf_records: usize,
    pub max_depth: usize,
    pub max_path_passes: usize,
    pub frontier_nodes: usize,
    pub equal_nodes: usize,
    pub band_nodes: usize,
    pub band_records: usize,
    pub terminal_band_nodes: usize,
    pub probe_records: usize,
    pub leaf_lengths: [usize; 129],
}

impl Default for Work {
    fn default() -> Self {
        Self {
            projection_nodes: 0,
            projection_records: 0,
            identity_nodes: 0,
            nodes: 0,
            node_records: 0,
            prepare_records: 0,
            order_records: 0,
            candidate_records: 0,
            label_records: 0,
            label_evals: 0,
            scatter_records: 0,
            dense_nodes: 0,
            block_nodes: 0,
            passes: 0,
            compact_records: 0,
            tail_records: 0,
            runs_records: 0,
            unresolved_records: 0,
            leaves: 0,
            leaf_records: 0,
            max_depth: 0,
            max_path_passes: 0,
            frontier_nodes: 0,
            equal_nodes: 0,
            band_nodes: 0,
            band_records: 0,
            terminal_band_nodes: 0,
            probe_records: 0,
            leaf_lengths: [0; 129],
        }
    }
}

// Compile out all diagnostic updates from timed binaries.
macro_rules! work { ($($body:tt)*) => { #[cfg(feature="rust_pa_work")] { $($body)* } }; }
pub(crate) use work;

/// Native converted-key operations. Keeping narrow keys narrow matters in the
/// scatter loop; the classifier never needs a wider integer than the key.
pub trait NativeKey:
    Copy
    + Ord
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitXor<Output = Self>
    + std::ops::BitOrAssign
    + std::ops::BitAndAssign
    + std::ops::Shr<usize, Output = Self>
    + std::ops::Add<Output = Self>
{
    const BITS: usize;
    fn from_u128(x: u128) -> Self;
    fn leading_zeros(self) -> u32;
    fn count_ones(self) -> u32;
    fn byte(self, index: usize) -> usize;
    fn mask(bits: usize) -> Self;
    fn to_u128(self) -> u128;
    /// Architecture selection and fallback belong to the dependency.
    fn pack_bits<I: hakmem::isa::Isa>(self, mask: Self, cpu: I) -> u32;
}
macro_rules! native {
    ($($t:ty),*) => {$(impl NativeKey for $t {
        const BITS:usize=<$t>::BITS as usize;
        #[inline] fn from_u128(x:u128)->Self { x as Self }
        #[inline] fn to_u128(self)->u128 { self as u128 }
        #[inline] fn pack_bits<I: hakmem::isa::Isa>(self, mask: Self, cpu: I)->u32 {
            cpu.pext(self, mask) as u32
        }

        #[inline] fn leading_zeros(self)->u32 { self.leading_zeros() }
        #[inline] fn count_ones(self)->u32 { self.count_ones() }
        #[inline] fn byte(self,index:usize)->usize { ((self>>(index*8))&255) as usize }
        #[inline] fn mask(bits:usize)->Self {
            if bits==Self::BITS as usize { Self::MAX } else { ((1 as Self)<<bits)-1 }
        }
    })*};
}
native!(u8, u16, u32, u64, u128);

#[cfg(test)]
fn mask(bits: usize) -> u128 {
    <u128 as NativeKey>::mask(bits)
}

trait Classifier<U: NativeKey> {
    type Label: NativeKey;
    const USE_COMPACT_RADIX: bool = false;
    fn label(&self, key: U) -> Self::Label;
    fn remaining(&self, key: U) -> usize;

    #[inline(always)]
    fn in_context<R>(&self, kernel: impl FnOnce() -> R) -> R {
        kernel()
    }
}

struct Identity;
impl<U: NativeKey> Classifier<U> for Identity {
    type Label = U;
    #[inline]
    fn label(&self, key: U) -> U {
        key
    }
    fn remaining(&self, _: U) -> usize {
        0
    }
}

#[derive(Clone, Copy)]
struct Prefix<U> {
    low: usize,
    mask: U,
}
impl<U: NativeKey> Classifier<U> for Prefix<U> {
    type Label = U;
    #[inline]
    fn label(&self, key: U) -> U {
        (key >> self.low) & self.mask
    }
    fn remaining(&self, _: U) -> usize {
        self.low
    }
}

struct Frontier<U> {
    local_mask: U,
    pivot: U,
    low: usize,
    high: U,
    equal: U,
    exits: [u8; 128],
}
impl<U: NativeKey> Frontier<U> {
    fn new(pivot: U, width: usize, coarse: usize) -> Self {
        let low = width - coarse;
        let pivot = pivot & U::mask(width);
        let high = pivot >> low;
        let suffix = pivot & U::mask(low);
        let mut exits = [0; 128];
        for b in 0..low {
            let above = (suffix >> (b + 1)).count_ones() as usize;
            exits[b] = (above
                + if (pivot >> b) & U::from_u128(1) == U::from_u128(0) {
                    b + 1
                } else {
                    0
                }) as u8;
        }
        Self {
            local_mask: U::mask(width),
            pivot,
            low,
            high,
            equal: high + U::from_u128(suffix.count_ones() as u128),
            exits,
        }
    }
}
impl<U: NativeKey> Classifier<U> for Frontier<U> {
    type Label = U;
    #[inline]
    fn label(&self, key: U) -> U {
        let key = key & self.local_mask;
        let high = key >> self.low;
        if high < self.high {
            return high;
        }
        if high > self.high {
            return high + U::from_u128(self.low as u128);
        }
        let diff = key ^ self.pivot;
        if diff == U::from_u128(0) {
            self.equal
        } else {
            self.high
                + U::from_u128(self.exits[U::BITS - 1 - diff.leading_zeros() as usize] as u128)
        }
    }
    fn remaining(&self, key: U) -> usize {
        let key = key & self.local_mask;
        if key >> self.low != self.high {
            return self.low;
        }
        let diff = key ^ self.pivot;
        if diff == U::from_u128(0) {
            0
        } else {
            U::BITS - 1 - diff.leading_zeros() as usize
        }
    }
}

pub fn sort<T: Radixable<K>, K: RadixKey>(arr: &mut [T], config: Config) -> Work
where
    Converted<T, K>: NativeKey,
{
    assert!(config.block > 0 && config.bytes <= 16 && config.coarse_bits <= 128);
    let mut stats = Work::default();
    if arr.len() < 2 || std::mem::size_of::<T>() == 0 {
        return stats;
    }
    match config.mode {
        Mode::L => node::<T, K, 0>(arr, arr[0].type_size(), config, &mut stats, 0, 0),
        Mode::H => node::<T, K, 1>(arr, arr[0].type_size(), config, &mut stats, 0, 0),
        Mode::P => node::<T, K, 2>(arr, arr[0].type_size(), config, &mut stats, 0, 0),
        Mode::F => node::<T, K, 3>(arr, arr[0].type_size(), config, &mut stats, 0, 0),
    }
    stats
}

fn node<T: Radixable<K>, K: RadixKey, const MODE: u8>(
    arr: &mut [T],
    cap: usize,
    config: Config,
    stats: &mut Work,
    depth: usize,
    path: usize,
) where
    Converted<T, K>: NativeKey,
{
    if arr.len() < 2 || cap == 0 {
        return;
    }
    work! { stats.max_depth = stats.max_depth.max(depth); }
    if arr.len() <= 128 {
        work! { stats.leaves += 1; stats.leaf_records += arr.len(); stats.leaf_lengths[arr.len()] += 1; }
        super::insertion_sort_key::<T, K>(arr);
        return;
    }
    let first = arr[0].into_key_type();
    let (mut union, mut intersection, mut previous, mut pivot) = (first, first, first, first);
    let (mut ascending, mut descending, mut votes) = (true, true, 0usize);
    // Candidate selection is fused into the same full variation/order scan.
    for item in arr.iter() {
        let key = item.into_key_type();
        union |= key;
        intersection &= key;
        ascending &= previous <= key;
        descending &= previous >= key;
        previous = key;
        if MODE == 3 {
            if votes == 0 {
                pivot = key;
                votes = 1;
            } else if key == pivot {
                votes += 1;
            } else {
                votes -= 1;
            }
        }
    }
    work! { stats.prepare_records += arr.len(); stats.order_records += arr.len(); if MODE == 3 { stats.candidate_records += arr.len(); } }
    if ascending {
        return;
    }
    if descending {
        arr.reverse();
        let mut start = 0;
        while start < arr.len() {
            work! { stats.order_records += 1; }
            let key = arr[start].into_key_type();
            let mut end = start + 1;
            while end < arr.len() && {
                work! { stats.order_records += 1; }
                arr[end].into_key_type() == key
            } {
                end += 1;
            }
            arr[start..end].reverse();
            start = end;
        }
        return;
    }
    let width =
        (Converted::<T, K>::BITS - (union ^ intersection).leading_zeros() as usize).min(cap);
    if width == 0 {
        return;
    }
    let bytes = if config.bytes == 0 {
        ((usize::BITS as usize - (arr.len() - 1).leading_zeros() as usize) + 7) / 8
    } else {
        config.bytes
    };
    let coarse = if MODE == 0 {
        width
    } else if config.coarse_bits != 0 {
        width.min(config.coarse_bits)
    } else if MODE == 3 && width > 8 * bytes {
        8 * bytes - 1
    } else {
        width.min(8 * bytes)
    };
    work! { stats.nodes += 1; stats.node_records += arr.len(); }
    if MODE == 0 {
        distribute_children::<T, K, _, MODE>(arr, &Identity, config, stats, depth, path);
    } else if MODE == 3 {
        work! { stats.frontier_nodes += 1; }
        let classifier = Frontier::new(pivot, width, coarse);
        distribute_children::<T, K, _, MODE>(arr, &classifier, config, stats, depth, path);
    } else {
        let classifier = Prefix {
            low: width - coarse,
            mask: Converted::<T, K>::mask(coarse),
        };
        distribute_children::<T, K, _, MODE>(arr, &classifier, config, stats, depth, path);
    }
}

fn distribute_children<
    T: Radixable<K>,
    K: RadixKey,
    C: Classifier<Converted<T, K>>,
    const MODE: u8,
>(
    arr: &mut [T],
    classifier: &C,
    config: Config,
    stats: &mut Work,
    depth: usize,
    path: usize,
) where
    Converted<T, K>: NativeKey,
{
    #[cfg(feature = "rust_pa_work")]
    let label_evals = std::cell::Cell::new(0usize);
    let label = |item: &T| {
        work! { label_evals.set(label_evals.get()+1); }
        classifier.label(item.into_key_type())
    };
    let passes = distribute(arr, &label, config.block, stats);
    work! { stats.max_path_passes = stats.max_path_passes.max(path+passes); }
    if MODE == 0 {
        work! { stats.label_evals += label_evals.get(); }
        return;
    }
    // distribute() owns all scratch. None survives into a recursive call.
    let mut start = 0;
    while start < arr.len() {
        let current = label(&arr[start]);
        let remaining = classifier.remaining(arr[start].into_key_type());
        let mut end = start + 1;
        while end < arr.len() && label(&arr[end]) == current {
            end += 1;
        }
        work! { stats.runs_records += end-start; }
        if end - start > 1 && remaining > 0 {
            work! { stats.unresolved_records += end-start; }
            if MODE == 1 {
                node::<T, K, 0>(
                    &mut arr[start..end],
                    remaining,
                    config,
                    stats,
                    depth + 1,
                    path + passes,
                );
            } else {
                node::<T, K, MODE>(
                    &mut arr[start..end],
                    remaining,
                    config,
                    stats,
                    depth + 1,
                    path + passes,
                );
            }
        }
        start = end;
    }
    work! { stats.label_evals += label_evals.get(); }
}

fn distribute<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block: usize,
    stats: &mut Work,
) -> usize {
    let variation = label.in_context(|| {
        let first = label.classify(&arr[0]);
        let mut variation = U::from_u128(0);
        for item in arr.iter() {
            variation |= label.classify(item) ^ first;
        }
        variation
    });
    work! { stats.label_records += arr.len(); }
    let levels = (U::BITS - variation.leading_zeros() as usize + 7) / 8;
    if levels == 0 {
        return 0;
    }
    let passes = (0..levels).filter(|&i| variation.byte(i) != 0).count();
    work! { stats.passes += passes; stats.scatter_records += passes*arr.len(); }
    // Identical budget for L/H/P/F, including dense histograms and cursors.
    let dense_bytes = arr
        .len()
        .checked_mul(std::mem::size_of::<T>())
        .and_then(|b| b.checked_add((levels + 2) * 256 * std::mem::size_of::<usize>()));
    let budget = 512usize
        .checked_mul(block)
        .and_then(|n| n.checked_mul(std::mem::size_of::<T>()))
        .expect("budget overflow");
    if dense_bytes.map_or(false, |bytes| bytes <= budget) {
        work! { stats.dense_nodes += 1; }
        dense(arr, label, levels, variation, stats);
    } else {
        work! { stats.block_nodes += 1; }
        let mut sorter = block::Sorter::new(arr, 256, block);
        for i in 0..levels {
            if variation.byte(i) != 0 {
                sorter.step(label, i);
            }
        }
        sorter.compact();
        work! { stats.compact_records += sorter.compact_records; stats.tail_records += sorter.tail_records; }
    }
    passes
}

fn dense<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    levels: usize,
    variation: U,
    stats: &mut Work,
) {
    label.in_context(|| {
        let mut hist = vec![[0usize; 256]; levels];
        for item in arr.iter() {
            let key = label.classify(item);
            for i in 0..levels {
                hist[i][key.byte(i)] += 1;
            }
        }
        work! { stats.label_records += arr.len(); }
        let mut scratch = vec![MaybeUninit::<T>::uninit(); arr.len()];
        let input = arr.as_mut_ptr();
        let buffer = scratch.as_mut_ptr().cast::<T>();
        let mut in_buffer = false;
        for i in 0..levels {
            if variation.byte(i) == 0 {
                continue;
            }
            let mut heads = [0usize; 256];
            let mut ends = [0usize; 256];
            let mut offset = 0;
            for b in 0..256 {
                heads[b] = offset;
                offset += hist[i][b];
                ends[b] = offset;
            }
            let (source, destination) = if in_buffer {
                (buffer, input)
            } else {
                (input, buffer)
            };
            // SAFETY: the source is initialized after the preceding complete pass;
            // allocations are disjoint, every cursor stays inside its own bucket.
            for j in 0..arr.len() {
                let item = unsafe { &*source.add(j) };
                let b = label.classify(item).byte(i);
                assert!(heads[b] < ends[b], "key changed during distribution");
                unsafe {
                    ptr::copy_nonoverlapping(item, destination.add(heads[b]), 1);
                }
                heads[b] += 1;
            }
            assert_eq!(heads, ends, "key changed during distribution");
            in_buffer = !in_buffer;
        }
        if in_buffer {
            unsafe {
                ptr::copy_nonoverlapping(buffer, input, arr.len());
            }
            work! { stats.compact_records += arr.len(); }
        }
    })
}
