//! Complete the same fixed label after first grouping its highest active byte.
//!
//! This changes physical execution within one classifier partition. Every
//! originally active byte is still scattered once per record; local runs use
//! the existing dense budget and oversized runs retain the block backend.

use super::labeler::{Labeler, Masked};
use super::{block, dense, work, NativeKey, Work};

pub(super) fn distribute_local<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    stats: &mut Work,
) -> usize {
    distribute_impl::<T, U, F, false, false, false>(arr, label, block_size, 0, None, false, stats)
}

pub(super) fn distribute_cached<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    workspace_threshold: usize,
    stats: &mut Work,
) -> usize {
    distribute_impl::<T, U, F, true, false, false>(
        arr,
        label,
        block_size,
        workspace_threshold,
        None,
        false,
        stats,
    )
}

pub(super) fn distribute_sampled<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    workspace_threshold: usize,
    stats: &mut Work,
) -> usize {
    distribute_impl::<T, U, F, true, true, false>(
        arr,
        label,
        block_size,
        workspace_threshold,
        None,
        false,
        stats,
    )
}

/// Reuse a mask covering every varying bit of the observed labels. Extra bits
/// are permitted; zero proves that every label is equal. Bits outside the mask
/// must be constant throughout this partition.
pub(super) fn distribute_prepared<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    workspace_threshold: usize,
    label_mask: U,
    stats: &mut Work,
) -> usize {
    distribute_impl::<T, U, F, true, false, false>(
        arr,
        label,
        block_size,
        workspace_threshold,
        Some(label_mask),
        false,
        stats,
    )
}

/// Apply the sampling guard with the same mask contract as `distribute_prepared`.
pub(super) fn distribute_prepared_sampled<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    workspace_threshold: usize,
    label_mask: U,
    stats: &mut Work,
) -> usize {
    distribute_impl::<T, U, F, true, true, false>(
        arr,
        label,
        block_size,
        workspace_threshold,
        Some(label_mask),
        false,
        stats,
    )
}

/// Prepared sampling with grouped writes for digits covered by one varying bit.
pub(super) fn distribute_prepared_grouped<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    workspace_threshold: usize,
    label_mask: U,
    stats: &mut Work,
) -> usize {
    distribute_impl::<T, U, F, true, true, false>(
        arr,
        label,
        block_size,
        workspace_threshold,
        Some(label_mask),
        true,
        stats,
    )
}

/// Compact projected digits also shrink the fixed bucket workspace when fewer
/// than eight output bits vary. The dense admission budget remains unchanged.
pub(super) fn distribute_prepared_packed<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    workspace_threshold: usize,
    label_mask: U,
    stats: &mut Work,
) -> usize {
    distribute_impl::<T, U, F, true, true, true>(
        arr,
        label,
        block_size,
        workspace_threshold,
        Some(label_mask),
        true,
        stats,
    )
}

pub(super) fn distribute_reduced<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    workspace_threshold: usize,
    stats: &mut Work,
) -> usize {
    distribute_impl::<T, U, F, true, false, true>(
        arr,
        label,
        block_size,
        workspace_threshold,
        None,
        false,
        stats,
    )
}

pub(super) fn distribute_sampled_reduced<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    workspace_threshold: usize,
    stats: &mut Work,
) -> usize {
    distribute_impl::<T, U, F, true, true, true>(
        arr,
        label,
        block_size,
        workspace_threshold,
        None,
        false,
        stats,
    )
}

fn distribute_impl<
    T: Copy,
    U: NativeKey,
    F: Labeler<T, U>,
    const CACHE_AWARE: bool,
    const SAMPLE_GUARD: bool,
    const REDUCED_RADIX: bool,
>(
    arr: &mut [T],
    label: &F,
    block_size: usize,
    workspace_threshold: usize,
    label_mask: Option<U>,
    grouped: bool,
    stats: &mut Work,
) -> usize {
    if arr.len() < 2 {
        return 0;
    }
    let variation = label.in_context(|| match label_mask {
        Some(mask) => mask,
        None => {
            let first = label.classify(&arr[0]);
            let mut variation = U::from_u128(0);
            for item in arr.iter() {
                variation |= label.classify(item) ^ first;
            }
            work! { stats.label_records += arr.len(); }
            variation
        }
    });
    let levels = (U::BITS - variation.leading_zeros() as usize + 7) / 8;
    if levels == 0 {
        return 0;
    }
    let passes = (0..levels).filter(|&i| variation.byte(i) != 0).count();
    let budget = 512usize
        .checked_mul(block_size)
        .and_then(|n| n.checked_mul(std::mem::size_of::<T>()))
        .expect("budget overflow");
    if passes == 1 || dense_fits::<T>(arr.len(), levels, budget) {
        return finish_label::<T, U, F, REDUCED_RADIX>(
            arr, label, levels, variation, block_size, budget, grouped, stats,
        );
    }

    let high_byte = levels - 1;
    if CACHE_AWARE {
        // An explicit offline tuning parameter, independent of the unchanged
        // allocation budget. Sorting performs no hardware discovery.
        if budget <= workspace_threshold {
            return finish_label::<T, U, F, REDUCED_RADIX>(
                arr, label, levels, variation, block_size, budget, grouped, stats,
            );
        }
        if SAMPLE_GUARD
            && arr.len() >= 64
            && !sample_runs_fit_dense(arr, label, high_byte, budget, stats)
        {
            return finish_label::<T, U, F, REDUCED_RADIX>(
                arr, label, levels, variation, block_size, budget, grouped, stats,
            );
        }
        let mut lengths = [0usize; 256];
        label.in_context(|| {
            for item in arr.iter() {
                lengths[label.classify(item).byte(high_byte)] += 1;
            }
        });
        work! { stats.label_records += arr.len(); }
        if lengths
            .iter()
            .any(|&length| !dense_fits::<T>(length, high_byte, budget))
        {
            // Reject before moving records; the existing variation scan is
            // sufficient to execute the original whole-label path.
            return finish_label::<T, U, F, REDUCED_RADIX>(
                arr, label, levels, variation, block_size, budget, grouped, stats,
            );
        }
    }
    work! {
        stats.block_nodes += 1;
        stats.passes += 1;
        stats.scatter_records += arr.len();
    }
    // Only the selected high byte is active during the first split.
    let high_variation = variation & (U::mask(levels * 8) ^ U::mask(high_byte * 8));
    block_bytes::<T, U, F, REDUCED_RADIX>(
        arr,
        label,
        levels,
        high_variation,
        block_size,
        grouped,
        stats,
    );
    // The root workspace has been released. Only one run's dense or block
    // workspace can be live while completing the remaining label bytes.
    let low_variation = variation & U::mask(high_byte * 8);
    let mut start = 0;
    while start < arr.len() {
        let end = label.in_context(|| {
            let high = label.classify(&arr[start]).byte(high_byte);
            work! { stats.label_records += 1; }
            let mut end = start + 1;
            while end < arr.len() {
                let next_high = label.classify(&arr[end]).byte(high_byte);
                work! { stats.label_records += 1; }
                if next_high != high {
                    break;
                }
                end += 1;
            }
            end
        });
        finish_label::<T, U, F, REDUCED_RADIX>(
            &mut arr[start..end],
            label,
            high_byte,
            low_variation,
            block_size,
            budget,
            grouped,
            stats,
        );
        start = end;
    }
    // Even singleton runs execute all originally active lower bytes. Thus each
    // record consumes the same pass count as the existing distribution path,
    // and callers can keep their current per-path pass accounting.
    passes
}

fn dense_fits<T>(length: usize, levels: usize, budget: usize) -> bool {
    length
        .checked_mul(std::mem::size_of::<T>())
        .and_then(|bytes| bytes.checked_add((levels + 2) * 256 * std::mem::size_of::<usize>()))
        .map_or(false, |bytes| bytes <= budget)
}

/// Predict only whether to pay for the full histogram. A sample may reject a
/// profitable split; every accepted split still needs the exact population check.
fn sample_runs_fit_dense<T: Copy, U: NativeKey, F: Labeler<T, U>>(
    arr: &[T],
    label: &F,
    high_byte: usize,
    budget: usize,
    stats: &mut Work,
) -> bool {
    debug_assert!(arr.len() >= 64);
    let span = arr.len() - 1;
    let stride = span / 63;
    let remainder = span % 63;
    let mut lengths = [0u8; 256];
    let mut maximum = 0usize;
    for i in 0..64 {
        // Equivalent to floor(i * span / 63), with unique indices from 0 to
        // span and no overflowing product even for a very large input.
        let index = i * stride + (i * remainder) / 63;
        let length = &mut lengths[label.classify(&arr[index]).byte(high_byte)];
        *length += 1;
        maximum = maximum.max(usize::from(*length));
    }
    work! { stats.label_records += 64; }
    // ceil(n * maximum / 64) without forming n * maximum. The first product
    // and the sum are at most n; the remaining numerator is at most 4095.
    let predicted = (arr.len() / 64) * maximum + ((arr.len() % 64) * maximum + 63) / 64;
    dense_fits::<T>(predicted, high_byte, budget)
}

/// Complete the indicated low bytes without changing the fixed label or
/// introducing another physical split. The supplied variation is from the
/// whole classifier partition, so this remains a same-scatter-work experiment.
fn finish_label<T: Copy, U: NativeKey, F: Labeler<T, U>, const REDUCED_RADIX: bool>(
    arr: &mut [T],
    label: &F,
    levels: usize,
    variation: U,
    block_size: usize,
    budget: usize,
    grouped: bool,
    stats: &mut Work,
) -> usize {
    let passes = (0..levels).filter(|&i| variation.byte(i) != 0).count();
    work! {
        stats.passes += passes;
        stats.scatter_records += passes * arr.len();
    }
    if dense_fits::<T>(arr.len(), levels, budget) {
        work! { stats.dense_nodes += 1; }
        dense(arr, label, levels, variation, stats);
    } else {
        work! { stats.block_nodes += 1; }
        block_bytes::<T, U, F, REDUCED_RADIX>(
            arr, label, levels, variation, block_size, grouped, stats,
        );
    }
    passes
}

/// Strip constant label bits only when that also reduces the bucket count.
/// In each processed byte, every digit is (digit & v.byte) | c for one common
/// c; removing c preserves order and equality in that byte. Unprocessed bytes
/// need not be constant. Normalized active digits are at most v.byte.
fn block_bytes<T: Copy, U: NativeKey, F: Labeler<T, U>, const REDUCED: bool>(
    arr: &mut [T],
    label: &F,
    levels: usize,
    variation: U,
    block_size: usize,
    grouped: bool,
    stats: &mut Work,
) {
    let maximum = if REDUCED {
        (0..levels)
            .map(|byte| variation.byte(byte))
            .max()
            .unwrap_or(0)
    } else {
        255
    };
    if maximum < 255 {
        let normalized = Masked {
            labeler: label,
            mask: variation,
        };
        let mut sorter = block::Sorter::new(arr, maximum + 1, block_size);
        for byte in 0..levels {
            if variation.byte(byte) != 0 {
                if grouped && variation.byte(byte).is_power_of_two() {
                    sorter.step_grouped(&normalized, byte, variation.byte(byte) as u8);
                } else {
                    sorter.step(&normalized, byte);
                }
            }
        }
        sorter.compact();
        work! {
            stats.compact_records += sorter.compact_records;
            stats.tail_records += sorter.tail_records;
        }
    } else {
        // Ordinary full-byte labels keep the original extraction hot loop.
        let mut sorter = block::Sorter::new(arr, 256, block_size);
        for byte in 0..levels {
            if variation.byte(byte) != 0 {
                if grouped && variation.byte(byte).is_power_of_two() {
                    sorter.step_grouped(label, byte, variation.byte(byte) as u8);
                } else {
                    sorter.step(label, byte);
                }
            }
        }
        sorter.compact();
        work! {
            stats.compact_records += sorter.compact_records;
            stats.tail_records += sorter.tail_records;
        }
    }
}

#[cfg(all(test, feature = "rust_pa_work"))]
mod tests {
    use super::super::{distribute, Work};
    use super::{
        dense_fits, distribute_cached, distribute_local, distribute_prepared,
        distribute_prepared_sampled, distribute_sampled,
    };
    use std::cell::Cell;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Record {
        key: u32,
        id: u32,
        payload: [u8; 120],
    }

    fn assert_same_work(expected: &Work, actual: &Work) {
        macro_rules! compare {
            ($($field:ident),+ $(,)?) => {
                // Keep the comparison exhaustive as Work gains fields.
                let Work { $($field: _),+ } = expected;
                $(assert_eq!(expected.$field, actual.$field, stringify!($field));)+
            };
        }
        compare!(
            projection_nodes,
            projection_records,
            identity_nodes,
            nodes,
            node_records,
            prepare_records,
            order_records,
            candidate_records,
            label_records,
            label_evals,
            scatter_records,
            dense_nodes,
            block_nodes,
            passes,
            compact_records,
            tail_records,
            runs_records,
            unresolved_records,
            leaves,
            leaf_records,
            max_depth,
            max_path_passes,
            frontier_nodes,
            equal_nodes,
            band_nodes,
            band_records,
            terminal_band_nodes,
            probe_records,
            leaf_lengths,
        );
    }

    #[test]
    fn prepared_identity_and_prefix_preserve_work_except_the_label_scan() {
        for length in [257usize, 16_385] {
            for prefix in [false, true] {
                let mut input: Vec<Record> = (0..length)
                    .map(|id| {
                        let raw_label =
                            0x80a5a0 | ((id.wrapping_mul(2654435761) as u32) & 0x050005);
                        Record {
                            key: if prefix {
                                0xa0000000 | (raw_label << 4) | (id % 16) as u32
                            } else {
                                raw_label
                            },
                            id: id as u32,
                            payload: [id as u8; 120],
                        }
                    })
                    .collect();
                input.reverse();
                let label = |record: &Record| {
                    if prefix {
                        (record.key >> 4) & 0x00ffffff
                    } else {
                        record.key
                    }
                };
                let (union, intersection) = input.iter().fold(
                    (input[0].key, input[0].key),
                    |(union, intersection), record| (union | record.key, intersection & record.key),
                );
                let mask = if prefix {
                    ((union ^ intersection) >> 4) & 0x00ffffff
                } else {
                    union ^ intersection
                };
                let mut expected = input.clone();
                expected.sort_by_key(&label);

                for sampled in [false, true] {
                    let mut old = input.clone();
                    let mut new = input.clone();
                    let old_calls = Cell::new(0usize);
                    let new_calls = Cell::new(0usize);
                    let old_label = |record: &Record| {
                        old_calls.set(old_calls.get() + 1);
                        label(record)
                    };
                    let new_label = |record: &Record| {
                        new_calls.set(new_calls.get() + 1);
                        label(record)
                    };
                    let mut old_work = Work::default();
                    let mut new_work = Work::default();
                    let (old_passes, new_passes) = if sampled {
                        (
                            distribute_sampled(&mut old, &old_label, 17, usize::MAX, &mut old_work),
                            distribute_prepared_sampled(
                                &mut new,
                                &new_label,
                                17,
                                usize::MAX,
                                mask,
                                &mut new_work,
                            ),
                        )
                    } else {
                        (
                            distribute_cached(&mut old, &old_label, 17, usize::MAX, &mut old_work),
                            distribute_prepared(
                                &mut new,
                                &new_label,
                                17,
                                usize::MAX,
                                mask,
                                &mut new_work,
                            ),
                        )
                    };
                    assert_eq!(old, expected);
                    assert_eq!(new, expected);
                    assert_eq!(old_passes, new_passes);
                    assert_eq!(old_calls.get(), new_calls.get() + length + 1);
                    assert_eq!(old_work.label_records, new_work.label_records + length);
                    old_work.label_records -= length;
                    assert_same_work(&old_work, &new_work);
                    if length == 257 {
                        assert_eq!(new_work.dense_nodes, 1);
                        assert_eq!(new_work.block_nodes, 0);
                    } else {
                        assert_eq!(new_work.dense_nodes, 0);
                        assert_eq!(new_work.block_nodes, 1);
                    }
                }
            }
        }
    }

    #[test]
    fn prepared_zero_mask_does_not_read_labels_or_move_records() {
        for length in [0, 1, 129] {
            let input: Vec<Record> = (0..length)
                .map(|id| Record {
                    key: 0xa5,
                    id,
                    payload: [id as u8; 120],
                })
                .rev()
                .collect();
            let label = |_: &Record| -> u32 { panic!("constant mask reads no labels") };
            for sampled in [false, true] {
                let mut actual = input.clone();
                let mut stats = Work::default();
                let passes = if sampled {
                    distribute_prepared_sampled(&mut actual, &label, 17, usize::MAX, 0, &mut stats)
                } else {
                    distribute_prepared(&mut actual, &label, 17, usize::MAX, 0, &mut stats)
                };
                assert_eq!(actual, input);
                assert_eq!(passes, 0);
                assert_same_work(&stats, &Work::default());
            }
        }
    }

    #[test]
    fn prepared_conservative_mask_sorts_three_labels_stably() {
        // The classifier's range is 0/1/2 even when this partition contains
        // only 0/1 or a constant 1. A conservative mask may add work safely.
        for distinct in [1, 2, 3] {
            let input: Vec<Record> = (0..16_385u32)
                .rev()
                .map(|id| Record {
                    key: if distinct == 1 { 1 } else { id % distinct },
                    id,
                    payload: [id as u8; 120],
                })
                .collect();
            let mut expected = input.clone();
            expected.sort_by_key(|record| record.key);
            for sampled in [false, true] {
                let mut actual = input.clone();
                let mut stats = Work::default();
                let label = |record: &Record| record.key;
                let passes = if sampled {
                    distribute_prepared_sampled(&mut actual, &label, 17, usize::MAX, 3, &mut stats)
                } else {
                    distribute_prepared(&mut actual, &label, 17, usize::MAX, 3, &mut stats)
                };
                assert_eq!(actual, expected);
                assert_eq!(passes, 1);
                assert_eq!(stats.block_nodes, 1);
                assert_eq!(stats.scatter_records, input.len());
                assert_eq!(stats.label_records, 0);
            }
        }
    }

    #[test]
    fn reduced_radix_preserves_constant_bits_empty_buckets_and_tails() {
        const N: usize = 16_385;
        for mask in [0x050005u32, 0x000001, 0x050001] {
            let mut input: Vec<Record> = (0..N)
                .map(|id| Record {
                    key: 0x80a5a0 | ((id.wrapping_mul(2654435761) as u32) & mask),
                    id: id as u32,
                    payload: [id as u8; 120],
                })
                .collect();
            input.reverse();
            let mut expected = input.clone();
            expected.sort_by_key(|r| r.key);
            let mut ordinary = input.clone();
            let mut reduced = input.clone();
            let mut combined = input;
            let mut old = Work::default();
            let mut new = Work::default();
            let mut both = Work::default();
            let label = |r: &Record| r.key;
            let p = distribute_cached(&mut ordinary, &label, 17, usize::MAX, &mut old);
            let q = super::distribute_reduced(&mut reduced, &label, 17, usize::MAX, &mut new);
            let z =
                super::distribute_sampled_reduced(&mut combined, &label, 17, usize::MAX, &mut both);
            assert_eq!(ordinary, expected);
            assert_eq!(reduced, expected);
            assert_eq!(combined, expected);
            assert_eq!((p, old.scatter_records), (q, new.scatter_records));
            assert_eq!((p, old.scatter_records), (z, both.scatter_records));
            assert!(new.block_nodes > 0);
        }
    }

    #[test]
    fn local_label_completion_preserves_stability_and_scatter_work() {
        const N: usize = 16_385;
        const HEAVY: usize = 12_288;
        const BLOCK: usize = 17;
        let mut input: Vec<Record> = (0..N)
            .map(|id| {
                let (high, low) = if id < HEAVY {
                    (0x20, (id % 17) as u32)
                } else if id == N - 1 {
                    (0xfe, 0x91)
                } else {
                    (0x40 + (id % 8) as u32, (id % 31) as u32)
                };
                let mut payload = [0; 120];
                for (offset, byte) in payload.iter_mut().enumerate() {
                    *byte = (id.wrapping_mul(31) ^ offset.wrapping_mul(17)) as u8;
                }
                Record {
                    // Bytes 0 and 2 vary; byte 1 is deliberately constant.
                    key: (high << 16) | 0x5a00 | low,
                    id: id as u32,
                    payload,
                }
            })
            .collect();
        let mut state = 42u64;
        for end in (1..input.len()).rev() {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            input.swap(end, (state % (end as u64 + 1)) as usize);
        }
        let mut expected = input.clone();
        expected.sort_by_key(|record| record.key);
        assert!(expected.windows(2).any(|pair| pair[0].key == pair[1].key));
        assert_eq!(input.iter().filter(|r| r.key >> 16 == 0xfe).count(), 1);

        let budget = 512 * BLOCK * std::mem::size_of::<Record>();
        assert!(!dense_fits::<Record>(N, 3, budget));
        assert!(!dense_fits::<Record>(HEAVY, 2, budget));
        assert!(dense_fits::<Record>(512, 2, budget));

        let label = |record: &Record| record.key;
        let mut reference = input.clone();
        let mut cached = input.clone();
        let mut sampled = input.clone();
        let mut local = input;
        let mut reference_work = Work::default();
        let mut cached_work = Work::default();
        let mut sampled_work = Work::default();
        let mut local_work = Work::default();
        let reference_passes = distribute(&mut reference, &label, BLOCK, &mut reference_work);
        let local_passes = distribute_local(&mut local, &label, BLOCK, &mut local_work);
        let cached_passes =
            distribute_cached(&mut cached, &label, BLOCK, 1024 * 1024, &mut cached_work);
        let sampled_passes =
            distribute_sampled(&mut sampled, &label, BLOCK, 1024 * 1024, &mut sampled_work);

        // Full-record equality checks stable duplicate identities and every
        // payload byte, including the single-record high-byte run.
        for (position, expected) in expected.iter().enumerate() {
            assert_eq!(
                &reference[position], expected,
                "reference record {position}"
            );
            assert_eq!(&local[position], expected, "local record {position}");
            assert_eq!(&cached[position], expected, "cached record {position}");
            assert_eq!(&sampled[position], expected, "sampled record {position}");
        }
        assert_eq!(reference_passes, 2);
        assert_eq!(local_passes, reference_passes);
        assert_eq!(reference_work.scatter_records, 2 * N);
        assert_eq!(local_work.scatter_records, reference_work.scatter_records);
        assert_eq!(reference_work.block_nodes, 1);
        assert_eq!(reference_work.dense_nodes, 0);
        // One top-byte block distribution and one oversized low-byte run.
        // Eight small runs plus the singleton use separate dense completions.
        assert_eq!(local_work.block_nodes, 2);
        assert_eq!(local_work.dense_nodes, 9);
        assert_eq!(local_work.passes, 11);
        // The oversized run rejects cached completion after exactly one
        // extra histogram scan, before changing the original scatter path.
        assert_eq!(cached_passes, reference_passes);
        assert_eq!(cached_work.scatter_records, reference_work.scatter_records);
        assert_eq!(cached_work.block_nodes, reference_work.block_nodes);
        assert_eq!(cached_work.dense_nodes, reference_work.dense_nodes);
        assert_eq!(cached_work.passes, reference_work.passes);
        assert_eq!(cached_work.compact_records, reference_work.compact_records);
        assert_eq!(cached_work.tail_records, reference_work.tail_records);
        assert_eq!(cached_work.label_records, reference_work.label_records + N);
        // The heavy run is rejected from 64 labels instead of the exact
        // histogram, retaining precisely the cached path's whole-label work.
        assert_eq!(sampled_passes, cached_passes);
        assert_eq!(sampled_work.scatter_records, cached_work.scatter_records);
        assert_eq!(sampled_work.block_nodes, cached_work.block_nodes);
        assert_eq!(sampled_work.dense_nodes, cached_work.dense_nodes);
        assert_eq!(sampled_work.passes, cached_work.passes);
        assert_eq!(sampled_work.compact_records, cached_work.compact_records);
        assert_eq!(sampled_work.tail_records, cached_work.tail_records);
        assert_eq!(
            sampled_work.label_records + N - 64,
            cached_work.label_records
        );
    }

    #[test]
    fn cached_completion_checks_budget_and_all_high_byte_runs() {
        const N: usize = 8705;
        let input: Vec<Record> = (0..N)
            .map(|id| Record {
                key: (((id % 16) as u32) << 16) | 0x5a00 | ((id / 16) % 13) as u32,
                id: id as u32,
                payload: [id as u8; 120],
            })
            .collect();
        let mut expected = input.clone();
        expected.sort_by_key(|record| record.key);
        let label = |record: &Record| record.key;

        for block_size in [16, 17] {
            let budget = 512 * block_size * std::mem::size_of::<Record>();
            assert!(!dense_fits::<Record>(N, 3, budget));
            assert!(dense_fits::<Record>((N + 15) / 16, 2, budget));
            let mut reference = input.clone();
            let mut cached = input.clone();
            let mut sampled = input.clone();
            let mut reference_work = Work::default();
            let mut cached_work = Work::default();
            let mut sampled_work = Work::default();
            let reference_passes =
                distribute(&mut reference, &label, block_size, &mut reference_work);
            let cached_passes = distribute_cached(
                &mut cached,
                &label,
                block_size,
                1024 * 1024,
                &mut cached_work,
            );
            let sampled_passes = distribute_sampled(
                &mut sampled,
                &label,
                block_size,
                1024 * 1024,
                &mut sampled_work,
            );
            for (position, expected) in expected.iter().enumerate() {
                assert_eq!(&reference[position], expected);
                assert_eq!(&cached[position], expected);
                assert_eq!(&sampled[position], expected);
            }
            assert_eq!(cached_passes, reference_passes);
            assert_eq!(sampled_passes, cached_passes);
            assert_eq!(cached_work.scatter_records, reference_work.scatter_records);
            assert_eq!(sampled_work.scatter_records, cached_work.scatter_records);
            assert_eq!(sampled_work.block_nodes, cached_work.block_nodes);
            assert_eq!(sampled_work.dense_nodes, cached_work.dense_nodes);
            assert_eq!(sampled_work.passes, cached_work.passes);
            assert_eq!(sampled_work.compact_records, cached_work.compact_records);
            assert_eq!(sampled_work.tail_records, cached_work.tail_records);
            if block_size == 16 {
                assert_eq!(budget, 1024 * 1024);
                assert_eq!(cached_work.block_nodes, 1);
                assert_eq!(cached_work.dense_nodes, 0);
                assert_eq!(cached_work.label_records, reference_work.label_records);
                assert_eq!(sampled_work.label_records, cached_work.label_records);
            } else {
                let mut local = input.clone();
                let mut local_work = Work::default();
                let local_passes =
                    distribute_local(&mut local, &label, block_size, &mut local_work);
                assert_eq!(cached_passes, local_passes);
                assert!(cached.iter().zip(&local).all(|(a, b)| a == b));
                assert_eq!(cached_work.block_nodes, 1);
                assert_eq!(cached_work.dense_nodes, 16);
                assert_eq!(cached_work.passes, 17);
                assert_eq!(cached_work.compact_records, local_work.compact_records);
                assert_eq!(cached_work.label_records, local_work.label_records + N);
                assert_eq!(sampled_work.label_records, cached_work.label_records + 64);
            }
        }
    }
}
