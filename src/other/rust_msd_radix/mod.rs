//! Ordinary MSD radix sort: the unstable speed reference of the radix family.
//!
//! This is deliberately *not* a stable sort and deliberately has no auxiliary
//! buffer: it permutes in place, cycling each record to its bucket, and pays for
//! that with an unstable result. It exists so the tiled stable sort can be read
//! against the cheapest shape radix sorting can take on the same keys, in the
//! same harness, on the same machine. Nothing here is a candidate for the
//! repository's stable-sort lineup.
//!
//! Shape: one histogram per branching digit, prefix-summed into bucket borders,
//! then a cycle-following permutation inside the node. Digits whose bits are
//! constant in a node are skipped, and nodes of at most [`LEAF`] records are
//! finished with a key comparison insertion sort.

use std::cmp::Ordering;

use super::radix_support::{Key, Unsigned};
use super::rust_block_lsd::core as block;

type SortKey<T> = <T as Key>::Value;

/// Bits per digit.
const RADIX: usize = 8;
/// Nodes at or below this many records use insertion sort.
const LEAF: usize = 24;
// Array lengths below are spelled as literals: this toolchain's layout query
// ICEs on an array type whose length is a path constant in a nested module.
const MAX_BUCKETS: usize = 1 << RADIX;

/// Sorts by converted radix key. Unstable.
pub fn sort<T: Key>(arr: &mut [T]) {
    let bits = T::Value::BITS;
    if arr.len() < 2 {
        return;
    }
    let (union, intersection) = block::varying_key_bits(arr);
    sort_node(arr, bits, union, intersection);
}

fn sort_node<T: Key>(arr: &mut [T], bits: usize, union: SortKey<T>, intersection: SortKey<T>) {
    if arr.len() < 2 || bits == 0 {
        return;
    }
    // Highest varying digit that is still inside the key.
    let width = RADIX.min(bits);
    let shift = bits - width;
    let mask = (T::Value::from_usize((1usize << width) - 1)) << T::Value::from_usize(shift);
    if union & mask == intersection & mask {
        // This digit cannot change the order; continue with the bits below it.
        sort_node(arr, shift, union, intersection);
        return;
    }
    if arr.len() <= LEAF {
        insertion_sort(arr);
        return;
    }

    let buckets = 1usize << width;
    let shift_bits = T::Value::from_usize(shift);
    let mut starts = [0usize; 257];
    for record in arr.iter() {
        starts[((record.key() & mask) >> shift_bits).to_usize() + 1] += 1;
    }
    for bucket in 0..buckets {
        starts[bucket + 1] += starts[bucket];
    }
    // Borders stay valid for the whole permutation; `next` is the write cursor.
    let borders = starts;
    permute(arr, &borders, mask, shift_bits, buckets);

    if shift == 0 {
        return;
    }
    let mut start = 0;
    for bucket in 0..buckets {
        let end = borders[bucket + 1];
        if end - start > 1 {
            let child = &mut arr[start..end];
            let (child_union, child_intersection) = block::varying_key_bits(child);
            sort_node(child, shift, child_union, child_intersection);
        }
        start = end;
    }
}

/// Cycle-following permutation into the counted bucket borders.
fn permute<T: Key>(
    arr: &mut [T],
    borders: &[usize; 257],
    mask: SortKey<T>,
    shift: SortKey<T>,
    buckets: usize,
) {
    let mut next = *borders;
    for bucket in 0..buckets {
        while next[bucket] < borders[bucket + 1] {
            // Pick up the record sitting at this bucket's write cursor and put
            // it where it belongs; repeat with whatever that slot displaced.
            let mut record = unsafe { std::ptr::read(arr.as_ptr().add(next[bucket])) };
            let mut target = ((record.key() & mask) >> shift).to_usize();
            while target != bucket {
                let slot = next[target];
                next[target] += 1;
                let displaced = unsafe { std::ptr::read(arr.as_ptr().add(slot)) };
                unsafe { std::ptr::write(arr.as_mut_ptr().add(slot), record) };
                record = displaced;
                target = ((record.key() & mask) >> shift).to_usize();
            }
            unsafe { std::ptr::write(arr.as_mut_ptr().add(next[bucket]), record) };
            next[bucket] += 1;
        }
    }
}

/// Stable insertion sort on converted keys. Only used for tiny nodes, where the
/// stability it happens to provide is not part of the contract.
fn insertion_sort<T: Key>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1].key().cmp(&arr[j].key()) == Ordering::Greater {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check<T: Key + Copy + Ord + std::fmt::Debug>(mut input: Vec<T>) {
        let mut expected = input.clone();
        expected.sort();
        sort(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn sorts_every_width() {
        let mut x = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for &len in &[0usize, 1, 2, 3, 25, 100, 4097] {
            check::<u32>((0..len).map(|_| next() as u32).collect());
            check::<u64>((0..len).map(|_| next()).collect());
            check::<u128>(
                (0..len)
                    .map(|_| ((next() as u128) << 64) | next() as u128)
                    .collect(),
            );
        }
    }

    #[test]
    fn skewed_keys_sort() {
        let mut input = vec![7u64; 20_000];
        input.extend((0..500).map(|i| i as u64 * 977));
        check(input);
    }
}
