//! Order proof with branch-free reductions inside fixed-size tiles.
use super::*;

const TILE: usize = 128;

/// Return whether the range is complete, preserving equal-key record order.
/// An unsuccessful proof leaves the range untouched for ordinary preparation.
pub(super) fn ordered<T: Radixable<K>, K: RadixKey>(arr: &mut [T], stats: &mut Work) -> bool
where
    Converted<T, K>: NativeKey,
{
    if arr.len() < 2 {
        return true;
    }
    work! { stats.order_records += 1; }
    let first = arr[0].into_key_type();
    let (different, key) = match first_different::<T, K>(arr, first, stats) {
        Some(found) => found,
        None => return true,
    };
    // The equal prefix and its strict relation to this first different key
    // already prove every pair before `different`. Verify the remaining pairs.
    let descending = key < first;
    let (is_ordered, has_equal_keys) = if descending {
        monotone::<T, K, true>(&arr[different..], stats)
    } else {
        monotone::<T, K, false>(&arr[different..], stats)
    };
    if !is_ordered {
        return false;
    }
    if descending {
        arr.reverse();
        if different > 1 || has_equal_keys {
            super::pa::restore_equal_groups::<T, K>(arr, stats);
        }
    }
    true
}

// Full equal tiles need only a reduction, including on all-equal narrow keys.
// Locate the exact first different key only within the first failing tile.
fn first_different<T: Radixable<K>, K: RadixKey>(
    arr: &[T],
    first: Converted<T, K>,
    stats: &mut Work,
) -> Option<(usize, Converted<T, K>)>
where
    Converted<T, K>: NativeKey,
{
    let zero = Converted::<T, K>::from_u128(0);
    let mut chunks = arr[1..].chunks_exact(TILE);
    for (tile, chunk) in chunks.by_ref().enumerate() {
        // Any differing bit makes the tile unequal. Keep the reduction in the
        // native key representation rather than packing per-record predicates.
        let mut difference = zero;
        for item in chunk {
            work! { stats.order_records += 1; }
            difference |= item.into_key_type() ^ first;
        }
        if difference != zero {
            for (offset, item) in chunk.iter().enumerate() {
                work! { stats.order_records += 1; }
                let key = item.into_key_type();
                if key != first {
                    return Some((1 + tile * TILE + offset, key));
                }
            }
        }
    }
    let remainder = chunks.remainder();
    let start = arr.len() - remainder.len();
    for (offset, item) in remainder.iter().enumerate() {
        work! { stats.order_records += 1; }
        let key = item.into_key_type();
        if key != first {
            return Some((start + offset, key));
        }
    }
    None
}

fn monotone<T: Radixable<K>, K: RadixKey, const DESCENDING: bool>(
    arr: &[T],
    stats: &mut Work,
) -> (bool, bool)
where
    Converted<T, K>: NativeKey,
{
    let pairs = arr.len() - 1;
    let complete = pairs - pairs % TILE;
    let left_tiles = arr[..pairs].chunks_exact(TILE);
    let right_tiles = arr[1..].chunks_exact(TILE);
    let mut has_equal_keys = false;
    // Offset slices include the pairs crossing record-tile boundaries. Each
    // comparison is independent; the direction is specialized outside the loop.
    for (left, right) in left_tiles.zip(right_tiles) {
        let mut is_ordered = true;
        for offset in 0..TILE {
            work! { stats.order_records += 2; }
            let left = left[offset].into_key_type();
            let right = right[offset].into_key_type();
            let pair_ordered = if DESCENDING {
                left >= right
            } else {
                left <= right
            };
            // Software-wide comparisons become scalar predicate packing when
            // reduced across the tile. An early exit keeps those comparisons
            // scalar; native-word keys retain the vectorizable tile reduction.
            if Converted::<T, K>::BITS > usize::BITS as usize && !pair_ordered {
                return (false, false);
            }
            is_ordered &= pair_ordered;
            if DESCENDING {
                has_equal_keys |= left == right;
            }
        }
        if !is_ordered {
            return (false, false);
        }
    }
    for pair in arr[complete..].windows(2) {
        work! { stats.order_records += 2; }
        let left = pair[0].into_key_type();
        let right = pair[1].into_key_type();
        let out_of_order = if DESCENDING {
            left < right
        } else {
            left > right
        };
        if out_of_order {
            return (false, false);
        }
        if DESCENDING {
            has_equal_keys |= left == right;
        }
    }
    (true, has_equal_keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
    struct Record {
        key: u128,
        id: usize,
        payload: [u64; 2],
    }

    impl Radixable<u128> for Record {
        type Key = u128;

        fn key(&self) -> u128 {
            self.key
        }
    }

    fn records(keys: impl IntoIterator<Item = u128>) -> Vec<Record> {
        keys.into_iter()
            .enumerate()
            .map(|(id, key)| Record {
                key,
                id,
                payload: [id as u64 ^ 0xdead_beef, !(id as u64)],
            })
            .collect()
    }

    #[test]
    fn equal_and_ascending_ranges_keep_identity_and_payload() {
        for n in [0, 1, 2, 127, 128, 129, 130, 257, 258, 1027] {
            for keys in [
                vec![u128::MAX; n],
                (0..n).map(|i| (1u128 << 120) + (i / 3) as u128).collect(),
            ] {
                let mut actual = records(keys);
                let expected = actual.clone();
                assert!(ordered::<_, u128>(&mut actual, &mut Work::default()));
                assert_eq!(actual, expected, "n={n}");
            }
            let mut narrow: Vec<u8> = (0..n).map(|i| (i * 255 / n.max(1)) as u8).collect();
            let expected = narrow.clone();
            assert!(ordered::<_, u8>(&mut narrow, &mut Work::default()));
            assert_eq!(narrow, expected, "narrow n={n}");
        }
    }

    #[test]
    fn first_difference_can_follow_an_equal_tile_or_land_in_the_tail() {
        let n = 4 * TILE + 5;
        for different in [1, TILE - 1, TILE, TILE + 1, 2 * TILE, n - 1] {
            let mut actual =
                records((0..n).map(|i| (1u128 << 120) + i.saturating_sub(different - 1) as u128));
            let expected = actual.clone();
            assert!(ordered::<_, u128>(&mut actual, &mut Work::default()));
            assert_eq!(actual, expected, "different={different}");
        }
    }

    #[test]
    fn each_key_bit_detects_an_interior_outlier_across_tiles_and_tail() {
        let n = 3 * TILE + 5;
        let base = 1u128 << 120;
        for bit in 0..128 {
            for outlier in [1, TILE - 1, TILE, TILE + 1, 2 * TILE - 1, 2 * TILE, n - 2] {
                let mut keys = vec![base; n];
                keys[outlier] ^= 1u128 << bit;
                let expected = keys.clone();
                assert!(!ordered::<_, u128>(&mut keys, &mut Work::default()));
                assert_eq!(keys, expected, "bit={bit} outlier={outlier}");

                let mut actual = records(expected);
                let expected = actual.clone();
                assert!(!ordered::<_, u128>(&mut actual, &mut Work::default()));
                assert_eq!(actual, expected, "record bit={bit} outlier={outlier}");
            }
        }
    }

    #[test]
    fn descending_groups_spanning_tiles_preserve_duplicate_order() {
        let lengths = [TILE + 1, 2, 1, TILE - 1, TILE + 3, 2];
        let keys = lengths
            .iter()
            .copied()
            .enumerate()
            .flat_map(|(group, len)| {
                std::iter::repeat((1u128 << 120) + (10 - group) as u128).take(len)
            });
        let mut actual = records(keys);
        let mut expected = actual.clone();
        expected.sort_by_key(|item| item.key);
        assert!(ordered::<_, u128>(&mut actual, &mut Work::default()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn descending_equal_prefix_is_repaired_when_the_suffix_is_unique() {
        let n = 3 * TILE + 5;
        let mut actual = records((0..n).map(|i| {
            let low = if i <= TILE { n } else { n - i };
            (1u128 << 120) + low as u128
        }));
        let mut expected = actual.clone();
        expected.sort_by_key(|item| item.key);
        assert!(ordered::<_, u128>(&mut actual, &mut Work::default()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unique_descending_range_is_completed() {
        let mut actual = records((0..3 * TILE + 5).rev().map(|i| (1u128 << 120) + i as u128));
        let mut expected = actual.clone();
        expected.sort_by_key(|item| item.key);
        assert!(ordered::<_, u128>(&mut actual, &mut Work::default()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn disorder_at_tile_boundaries_is_rejected_without_writes() {
        let n = 4 * TILE + 5;
        for descending in [false, true] {
            for inversion in [0, 1, TILE - 1, TILE, TILE + 1, 2 * TILE + 1, n - 2] {
                let mut actual = records((0..n).map(|i| {
                    let low = if descending { n - i } else { i };
                    (1u128 << 120) + low as u128
                }));
                actual.swap(inversion, inversion + 1);
                let expected = actual.clone();
                assert!(!ordered::<_, u128>(&mut actual, &mut Work::default()));
                assert_eq!(
                    actual, expected,
                    "descending={descending} inversion={inversion}"
                );
            }
        }
    }
}
