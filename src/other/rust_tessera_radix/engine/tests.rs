//! Stable-reference checks, key-panic recovery, and historical regressions.

use super::*;

/// Equality includes input position so stable-reference checks observe tie order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Item<K> {
    key: K,
    index: u32,
}

impl<K: Unsigned> Key for Item<K> {
    type Value = K;

    fn key(&self) -> K {
        self.key
    }
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn check<T: Key + Copy + PartialEq + std::fmt::Debug>(mut input: Vec<T>, tile_bytes: usize) {
    let mut expected = input.clone();
    expected.sort_by_key(Key::key);
    sort_with_tile_bytes(&mut input, tile_bytes);
    if input != expected {
        let mismatch = input
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap();
        panic!(
            "tile={tile_bytes} len={} first mismatch at {mismatch}: got {:?}, want {:?}",
            input.len(),
            input[mismatch],
            expected[mismatch]
        );
    }
}

// Tracked identities expose lost records, duplicates and premature drops.
mod tracked {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    static CALLS: AtomicUsize = AtomicUsize::new(0);
    static PANIC_AT: AtomicUsize = AtomicUsize::new(usize::MAX);
    static DROPS: Mutex<Vec<u64>> = Mutex::new(Vec::new());
    // Both panic sweeps use these process-wide counters; cargo runs tests in
    // parallel, so they must not arm each other's panics.
    pub static TEST_LOCK: Mutex<()> = Mutex::new(());

    pub struct Tracked {
        pub id: u64,
        pub key: u64,
    }

    impl Drop for Tracked {
        fn drop(&mut self) {
            DROPS.lock().unwrap().push(self.id);
        }
    }

    impl Key for Tracked {
        type Value = u64;

        fn key(&self) -> u64 {
            let call = CALLS.fetch_add(1, Ordering::Relaxed) + 1;
            assert_ne!(
                call,
                PANIC_AT.load(Ordering::Relaxed),
                "the armed key panic"
            );
            self.key
        }
    }

    pub fn reset() {
        CALLS.store(0, Ordering::Relaxed);
        PANIC_AT.store(usize::MAX, Ordering::Relaxed);
        DROPS.lock().unwrap().clear();
    }

    pub fn arm(call: usize) {
        PANIC_AT.store(call, Ordering::Relaxed);
    }

    pub fn drops() -> Vec<u64> {
        DROPS.lock().unwrap().clone()
    }

    pub fn disarm() {
        PANIC_AT.store(usize::MAX, Ordering::Relaxed);
    }

    /// How many key calls the last sort made; the sweep has to stay inside
    /// that range or it arms a panic the sort never reaches.
    pub fn calls() -> usize {
        CALLS.load(Ordering::Relaxed)
    }
}

fn check_key_panics(len: usize, tile_bytes: usize, seed: u64, key_mask: u64) {
    let _guard = tracked::TEST_LOCK.lock().unwrap();
    let build = || -> Vec<tracked::Tracked> {
        let mut rng = Rng(seed ^ len as u64);
        (0..len)
            .map(|id| tracked::Tracked {
                id: id as u64,
                key: rng.next() & key_mask,
            })
            .collect()
    };
    // Probe and armed runs use identical records: insertion's call count depends on keys.
    tracked::reset();
    let mut probe = build();
    sort_with_tile_bytes(&mut probe, tile_bytes);
    let calls = tracked::calls();
    drop(probe);
    let expected: Vec<u64> = (0..len as u64).collect();
    for step in 0..40 {
        let panic_at = 1 + step * calls / 40;
        tracked::reset();
        let mut records = build();
        tracked::arm(panic_at);
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            sort_with_tile_bytes(&mut records, tile_bytes);
        }));
        tracked::disarm();
        assert!(
            outcome.is_err(),
            "len={len} panic_at={panic_at} did not panic"
        );
        let mut ids: Vec<u64> = records.iter().map(|record| record.id).collect();
        ids.sort_unstable();
        assert_eq!(ids, expected, "len={len} panic_at={panic_at} lost records");
        assert!(
            tracked::drops().is_empty(),
            "records dropped during sorting"
        );
        drop(records);
        let mut dropped = tracked::drops();
        dropped.sort_unstable();
        assert_eq!(
            dropped, expected,
            "len={len} panic_at={panic_at} drop count"
        );
    }
}

#[test]
fn panic_preserves_records() {
    let cases: &[(usize, u64)] = if cfg!(miri) {
        &[(12, u64::MAX), (80, 0xfffff)]
    } else {
        &[(20, u64::MAX), (80, 0xfffff), (2000, u64::MAX)]
    };
    for &(len, mask) in cases {
        check_key_panics(len, TILE_BYTES, 0x2436_8f5c_1e97_b3d1, mask);
    }
}

#[test]
fn split_panic_recovers() {
    check_key_panics(200, 256, 0x9f4a_2c31_7b68_d105, u64::MAX);
}

#[test]
fn tile_partition_caches_changed_keys() {
    use std::cell::Cell;

    struct Borrowed<'a> {
        id: usize,
        slot: &'a mut usize,
        calls: &'a Cell<usize>,
    }
    impl Key for Borrowed<'_> {
        type Value = u64;
        fn key(&self) -> u64 {
            let call = self.calls.get() + 1;
            self.calls.set(call);
            u64::from(call > 64)
        }
    }
    let calls = Cell::new(0);
    let mut slots: Vec<_> = (0..64).collect();
    let mut records: Vec<_> = slots
        .iter_mut()
        .enumerate()
        .map(|(id, slot)| Borrowed {
            id,
            slot,
            calls: &calls,
        })
        .collect();
    let mut sorter = Sorter {
        scratch: Vec::new(),
        tile_bytes: TILE_BYTES,
    };
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sorter.partition_tile(&mut records, Digit { shift: 0, width: 1 }, 1);
    }));
    assert!(outcome.is_ok());
    assert_eq!(calls.get(), 64);
    for (id, record) in records.iter_mut().enumerate() {
        assert_eq!(record.id, id);
        assert_eq!(*record.slot, id);
        *record.slot += 64;
    }
    drop(records);
    assert_eq!(slots, (64..128).collect::<Vec<_>>());
}

#[test]
fn copy_scratch_reuse_keeps_initialized_records() {
    use std::cell::Cell;
    #[derive(Clone, Copy)]
    struct Record<'a> {
        id: usize,
        payload: &'a usize,
        calls: &'a Cell<usize>,
        changing: &'a Cell<bool>,
    }
    impl Key for Record<'_> {
        type Value = u64;
        fn key(&self) -> u64 {
            let call = self.calls.get() + 1;
            self.calls.set(call);
            if self.changing.get() {
                u64::from(call > 64)
            } else {
                (self.id % 2) as u64
            }
        }
    }
    let calls = Cell::new(0);
    let changing = Cell::new(false);
    let payloads: Vec<_> = (0..64).collect();
    let build = || {
        payloads
            .iter()
            .enumerate()
            .map(|(id, payload)| Record {
                id,
                payload,
                calls: &calls,
                changing: &changing,
            })
            .collect::<Vec<_>>()
    };
    let mut sorter = Sorter {
        scratch: Vec::new(),
        tile_bytes: TILE_BYTES,
    };
    let mut warm = build();
    sorter.sort_tile(&mut warm, 1);
    assert_eq!(
        calls.get(),
        64,
        "new scratch must use cached classification"
    );
    drop(warm);
    calls.set(0);
    changing.set(true);
    let mut input = build();
    sorter.sort_tile(&mut input, 1);
    assert_eq!(
        calls.get(),
        128,
        "initialized scratch must avoid another cache"
    );
    for row in input {
        assert!(row.id < 64);
        assert_eq!(*row.payload, row.id);
    }
}

#[test]
fn digit_plan_fits_capacity_for_sparse_and_dense_keys() {
    for width in MIN_TILE_DIGIT_BITS..=MAX_TILE_DIGIT_BITS {
        // Sparse single-bit digits still consume a full planning window. The
        // six-bit spacing reaches the 22-entry bound, including a short tail.
        let sparse = (0..128)
            .step_by(width)
            .fold(0, |mask, bit| mask | (1u128 << bit));
        for varying in [u128::MAX, sparse, 1, 1 << 127] {
            let mut digits = [Digit { shift: 0, width: 0 }; MAX_DIGITS];
            let passes = plan_digits(varying, width, &mut digits);
            assert!(passes <= (128usize).div_ceil(width));
            let mut covered = 0;
            for digit in &digits[..passes] {
                let mask = digit.mask::<u128>();
                assert_eq!(covered & mask, 0);
                covered |= mask;
            }
            assert_eq!(covered & varying, varying);
            if width == MIN_TILE_DIGIT_BITS && varying == sparse {
                assert_eq!(passes, (128usize).div_ceil(MIN_TILE_DIGIT_BITS));
            }
        }
    }
}

/// Bucket slots the plan's passes clear, count and walk.
fn plan_slots(digits: &[Digit]) -> usize {
    digits.iter().map(|digit| 1usize << digit.width).sum()
}

fn plan_widths(varying: u128, width: usize) -> Vec<usize> {
    let mut digits = [Digit { shift: 0, width: 0 }; MAX_DIGITS];
    let passes = plan_digits(varying, width, &mut digits);
    digits[..passes].iter().map(|digit| digit.width).collect()
}

#[test]
fn digit_plan_tiles_contiguous_bits_evenly() {
    // Three passes over 32 bits: fixed windows gave 14 + 14 + 4, which cleared
    // and walked 32 784 bucket slots to separate the same bits.
    assert_eq!(plan_widths(u128::MAX >> 96, 14), [11, 11, 10]);
    // Two passes over 20 bits: 10 + 10 replaces 14 + 6.
    assert_eq!(plan_widths(u128::MAX >> 108, 14), [10, 10]);
    // Spans that already divide evenly keep their widths.
    assert_eq!(plan_widths(u128::MAX >> 100, 14), [14, 14]);
    // The plan keeps the fixed windows when a gap would leave a pass empty,
    // such as here where one high bit and one low bit need one pass each.
    assert_eq!(plan_widths(1 << 127 | 1, 14), [1, 1]);
}

#[test]
fn digit_plan_covers_random_masks() {
    let mut rng = Rng(0x243f_6a88_85a3_08d3);
    let mut digits = [Digit { shift: 0, width: 0 }; MAX_DIGITS];
    for width in MIN_TILE_DIGIT_BITS..=MAX_TILE_DIGIT_BITS {
        for sample in 0..64 {
            let dense = (rng.next() as u128) | ((rng.next() as u128) << 64);
            let mut varying = if sample % 2 == 0 {
                dense
            } else {
                // Sparse masks space one bit per stride, like low-cardinality keys.
                let stride = 1 + (rng.next() as usize % width);
                (0..128)
                    .step_by(stride)
                    .fold(0, |mask, bit| mask | (1u128 << bit))
            };
            if varying == 0 {
                varying = 1;
            }
            let passes = plan_digits(varying, width, &mut digits);
            assert!(passes < MAX_DIGITS);
            assert!(passes <= (128usize).div_ceil(width));
            let mut covered = 0;
            for digit in &digits[..passes] {
                assert!(digit.width <= width);
                let mask = digit.mask::<u128>();
                assert_eq!(covered & mask, 0);
                covered |= mask;
            }
            assert_eq!(covered & varying, varying);
            // Contiguous runs keep the pass count of fixed windows, whose last
            // pass is short; the even plan trades that short pass for even ones.
            let places = (128 - varying.leading_zeros()) as usize;
            if varying.count_ones() as usize == places {
                let fixed = places.div_ceil(width);
                let tail = places - (fixed - 1) * width;
                assert_eq!(passes, fixed);
                assert!(plan_slots(&digits[..passes]) <= (fixed - 1) * (1 << width) + (1 << tail));
            }
        }
    }
}

#[test]
fn key_widths_and_tile_boundaries() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    // Miri interprets every byte moved, so it gets the boundary lengths that decide a
    // branch and none of the large ones that only repeat what they already showed.
    let lengths: &[usize] = if cfg!(miri) {
        &[0, 1, 2, 3, 7, 8, 9, 100, 255, 257]
    } else {
        &[0, 1, 2, 3, 7, 8, 9, 100, 255, 256, 257, 1000, 4097, 20_000]
    };
    for &len in lengths {
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
fn duplicate_stability() {
    // Equal keys carry their input index, so a stable sort must keep the
    // index order inside every run of equal keys.

    let mut rng = Rng(7);
    for &len in &[2usize, 100, 5000] {
        let input: Vec<Item<u16>> = (0..len)
            .map(|index| Item {
                key: (rng.next() % 4) as u16,
                index: index as u32,
            })
            .collect();
        check(input, TILE_BYTES);
    }
}

#[test]
fn random_shapes_and_tiles() {
    // Sparse masks, narrow windows and separated clusters, with identity payloads.

    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    let tiles = [4 * 1024usize, 16 * 1024, 64 * 1024, 256 * 1024, 1 << 20];
    // Miri gets the same shapes at a size it can interpret in seconds.
    let rounds = if cfg!(miri) { 12 } else { 2000 };
    let widest = if cfg!(miri) { 200 } else { 8000 };
    for round in 0..rounds {
        let len = (rng.next() % widest) as usize;
        let shape = rng.next() % 6;
        let mut input: Vec<Item<u32>> = Vec::with_capacity(len);
        for index in 0..len {
            let roll = rng.next();
            let key = match shape {
                0 => roll as u32,
                1 => (roll % 4) as u32,
                2 => ((roll % 64) << 26) as u32,
                3 => (((roll % 256) << 8) | (roll % 256)) as u32,
                4 => u32::MAX.wrapping_sub(roll as u32),
                _ => ((roll % 8) * 0x0101_0101) as u32,
            };
            input.push(Item {
                key,
                index: index as u32,
            });
        }
        let mut expected = input.clone();
        expected.sort_by_key(Key::key);
        let mut actual = input;
        sort_with_tile_bytes(&mut actual, tiles[(round % tiles.len() as u64) as usize]);
        for (position, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
            assert_eq!(
                (a.key, a.index),
                (e.key, e.index),
                "round {round} shape {shape} len {len} at {position}"
            );
        }
    }
}

/// Both 64-bit key halves must preserve tie order through the narrowed view.
#[test]
fn narrow_key_stability() {
    let mut rng = Rng(0x7f4a_7c15_9e37_79b9);
    // Miri is ~100x slower; the same shapes fit in a few hundred records.
    let len = if cfg!(miri) { 600 } else { 20_000 };
    let tiles: &[usize] = if cfg!(miri) {
        &[4096, 16384]
    } else {
        &[4096, 65536]
    };
    for shift in [0usize, 64] {
        for &tile in tiles {
            let input: Vec<Item<u128>> = (0..len)
                .map(|index| {
                    // 16 bits of values: enough duplicates to observe stability.
                    let value = (rng.next() & 0xffff) as u128;
                    Item {
                        key: value << shift,
                        index: index as u32,
                    }
                })
                .collect();
            check(input, tile);
        }
    }
}

#[test]
fn constant_keys() {
    let mut input = vec![42u64; 10_000];
    let expected = input.clone();
    sort(&mut input);
    assert_eq!(input, expected);
}

#[test]
fn skewed_keys() {
    let mut rng = Rng(11);
    let mut input: Vec<u32> = Vec::new();
    // 98 % zeros, the rest random: the shape is what matters, not the count, so Miri
    // gets a hundredth of the records.
    let (zeros, stragglers) = if cfg!(miri) {
        (500, 10)
    } else {
        (50_000, 1_000)
    };
    for _ in 0..zeros {
        input.push(0);
    }
    for _ in 0..stragglers {
        input.push(rng.next() as u32);
    }
    check(input, TILE_BYTES);
}

#[test]
fn nine_bit_split() {
    // A concentrated 9-bit node must stop widening before its shift underflows.
    let mut rng = Rng(7);
    let (len, tile) = if cfg!(miri) {
        (2_000, 1024)
    } else {
        (16_384, 1024)
    };
    let input: Vec<u64> = (0..len)
        .map(|_| match rng.next() % 3 {
            0 => 0,
            1 => 1,
            _ => 511,
        })
        .collect();
    check(input, tile);
}

#[test]
fn low_outlier_run() {
    // Low outliers already at the front; displaced body pairs reject ordered input.
    let (len, tile) = if cfg!(miri) {
        (2_048, 256)
    } else {
        (32_768, 1_024)
    };
    let scale = u32::MAX as u64 / 2 + 1;
    let mut input: Vec<u64> = (0..len as u64).map(|x| (x + (1 << 31)) * scale).collect();
    input[0] = (1 << 31) * scale - 2;
    input[1] = (1 << 31) * scale - 1;
    let mut index = 2;
    while index + 1 < input.len() {
        input.swap(index, index + 1);
        index += 100;
    }
    check(input, tile);
}

#[test]
fn high_outlier_run() {
    // The mirror image: the run's bit is set, so the lifted records belong after
    // the body and the front of the node is where they must leave from.
    let (len, tile) = if cfg!(miri) {
        (2_048, 256)
    } else {
        (32_768, 1_024)
    };
    let scale = u32::MAX as u64 / 2 + 1;
    let mut input: Vec<u64> = (0..len as u64).map(|x| (x + 1) * scale).collect();
    input[0] = (1 << 31) * scale + 1;
    input[1] = (1 << 31) * scale + 3;
    let mut index = 2;
    while index + 1 < input.len() {
        input.swap(index, index + 1);
        index += 100;
    }
    check(input, tile);
}

#[test]
fn insertion_swap_budget() {
    // Sorted keys need nothing; a single displaced pair costs one move; the
    // budget stops the count instead of walking the array.
    let mut keys: Vec<u64> = (0..64).collect();
    assert_eq!(budgeted_insertion_sort(&mut keys, 64, |&key| key), Some(0));
    assert_eq!(keys, (0..64).collect::<Vec<u64>>());

    keys.swap(10, 11);
    assert_eq!(budgeted_insertion_sort(&mut keys, 64, |&key| key), Some(1));
    assert_eq!(keys, (0..64).collect::<Vec<u64>>());

    let mut reversed: Vec<u64> = (0..64).rev().collect();
    assert_eq!(budgeted_insertion_sort(&mut reversed, 8, |&key| key), None);
}

#[test]
fn budgeted_insertion() {
    let items = |keys: &[u64]| -> Vec<Item<u64>> {
        keys.iter()
            .enumerate()
            .map(|(index, &key)| Item {
                key,
                index: index as u32,
            })
            .collect()
    };
    // Near-sorted: finishes, and payloads stay in input order inside equal keys.
    let mut records = items(&[0, 1, 1, 2, 3, 3, 4, 5]);
    let len = records.len();
    assert_eq!(
        budgeted_insertion_sort(&mut records, len, Key::key),
        Some(0)
    );
    assert!(records.windows(2).all(|pair| pair[0].key <= pair[1].key));
    let ones: Vec<u32> = records
        .iter()
        .filter(|r| r.key == 1)
        .map(|r| r.index)
        .collect();
    assert_eq!(ones, vec![1, 2]);

    // A reversed body spends far more than one move per record: the pass stops
    // with only swaps behind it, so the records are a permutation of the input.
    let input: Vec<u64> = (0..32).rev().collect();
    let mut records = items(&input);
    assert_eq!(budgeted_insertion_sort(&mut records, 4, Key::key), None);
    let mut keys: Vec<u64> = records.iter().map(|r| r.key).collect();
    keys.sort();
    assert_eq!(keys, (0..32).collect::<Vec<u64>>());
}

#[test]
fn nearly_sorted_input() {
    // One displaced adjacent pair in a hundred fits the sampled insertion budget.
    let (len, tile) = if cfg!(miri) {
        (2_048, 256)
    } else {
        (32_768, 1_024)
    };
    let mut input: Vec<u64> = (0..len as u64).collect();
    let mut index = 0;
    while index + 1 < input.len() {
        input.swap(index, index + 1);
        index += 100;
    }
    check(input, tile);
}
