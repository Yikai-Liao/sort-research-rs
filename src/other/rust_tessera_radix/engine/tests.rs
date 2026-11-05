//! Unit tests for the tile engine: correctness against a stable reference, the
//! panic contract, and the tile-budget sweep. Behavioural coverage of the public
//! surface lives in the tests/ directory instead.

use super::*;

/// A record whose key type is wider or narrower than its payload: equal keys then
/// have to keep their input order, which is what stability means here.
#[derive(Clone, Copy, Debug)]
struct Item<K> {
    key: K,
    index: u32,
}

impl<K: Copy + Ord> PartialEq for Item<K> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.index == other.index
    }
}

impl<K: Copy + Ord> Eq for Item<K> {}

impl<K: Copy + Ord> PartialOrd for Item<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Copy + Ord> Ord for Item<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
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

fn check<T: Key + Copy + Ord + std::fmt::Debug>(mut input: Vec<T>, tile_bytes: usize) {
    let mut expected = input.clone();
    expected.sort();
    sort_with_policy(
        &mut input,
        Policy {
            tile_bytes,
            ..Policy::default()
        },
    );
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

// A record that owns a resource, counts its drops, and has a key that can
// be made to panic on a chosen call. The recoverability contract is about
// types like this: a duplicated record is a double free, a lost one leaks.
mod tracked {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    static CALLS: AtomicUsize = AtomicUsize::new(0);
    static PANIC_AT: AtomicUsize = AtomicUsize::new(usize::MAX);
    static DROPS: Mutex<Vec<u64>> = Mutex::new(Vec::new());

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

/// The three leaves reach their keys at different points of a pass, so each
/// is armed to panic on many different calls. Whatever the leaf and whatever
/// the call, the record set has to survive exactly once and no record may be
/// dropped twice.
#[test]
fn a_key_panic_never_loses_or_duplicates_a_record() {
    // The interesting panics are the ones inside a level that is writing
    // into the live buffer, and where those sit in the call sequence depends
    // on the leaf, so the whole sequence is swept instead of guessed. The
    // sweep is sized from the calls an unarmed sort really makes: the leaf
    // changed shape since this test was written and a fixed upper bound
    // started arming panics that never happen.
    for &len in &[20usize, 300, 2000] {
        // Identical data for the probe and for every armed run: the number of
        // key calls depends on the data (insertion is data dependent), and
        // arming inside a range measured on other data skipped the panic.
        let build = || -> Vec<tracked::Tracked> {
            let mut rng = Rng(0x2436_8f5c_1e97_b3d1 ^ len as u64);
            (0..len)
                .map(|id| tracked::Tracked {
                    id: id as u64,
                    key: rng.next(),
                })
                .collect()
        };
        tracked::reset();
        let mut probe = build();
        sort_with_policy(&mut probe, Policy::default());
        let calls = tracked::calls();
        drop(probe);
        // 0..40 exclusive: the last point must stay inside the measured call
        // count, or it arms a panic the sort never reaches.
        for step in 0..40 {
            let panic_at = 1 + step * calls / 40;
            tracked::reset();
            let mut records = build();
            let expected: Vec<u64> = (0..len as u64).collect();
            tracked::arm(panic_at);
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                sort_with_policy(&mut records, Policy::default());
            }));
            assert!(outcome.is_err(), "len={len} panic_at={panic_at} did not panic");
            let mut ids: Vec<u64> = records.iter().map(|record| record.id).collect();
            ids.sort_unstable();
            assert_eq!(
                ids, expected,
                "len={len} panic_at={panic_at} left the record set different"
            );
            assert!(
                tracked::drops().is_empty(),
                "len={len} panic_at={panic_at} dropped a record while the tile still held it"
            );
            tracked::disarm();
            drop(records);
            let mut dropped = tracked::drops();
            dropped.sort_unstable();
            assert_eq!(
                dropped, expected,
                "len={len} panic_at={panic_at} did not drop every record exactly once"
            );
        }
    }
}

/// Routing and leaf choices are arithmetic on sampled shapes, so they can be
/// pinned instead of measured: a change here is a decision change, and this table
/// is what tells a simplification apart from a regression.
#[test]
fn route_table() {
    // (node bytes, records, flat passes, largest child share, split?); the last two
    // columns check the trace numbers the rule reads, not an input to it
    let routes = [
        // uniform u64 at 1M: 8 passes, a well-divided first digit
        (8_000_000usize, 1_000_000usize, 8usize, 0.0107f64, true),
        // two values: one pass cannot pay for a split
        (8_000_000, 1_000_000, 1, 0.5303, false),
        // keys piled into one bucket: seven passes against five for a split plus a
        // tile-sized child, so the split still pays -- this is the zipf case, where
        // both routes measured the same
        (8_000_000, 1_000_000, 7, 0.9980, true),
        // a node only just over the tile with a narrow digit
        (65_552, 8_194, 2, 0.0100, false),
        // 99 % sorted: the top digit separates the stragglers
        (8_000_000, 1_000_000, 8, 0.9980, true),
    ];
    for (bytes, records, flat, largest, expected) in routes {
        // The router prices the child at the best case the leaf can reach; see the
        // comment at the call site for the 25 % that assuming otherwise cost.
        let hot = flat.min(MSD_LEVEL_PASSES);
        assert_eq!(
            split_pays(flat, hot),
            expected,
            "{records} records of {} bytes, {flat} passes, largest {largest}",
            bytes / records
        );
    }

    // A well-divided tile is one level; a clustered one is its flat plan.
    assert_eq!(tile_cost(8, 3_906, 1.0 / 128.0, INSERTION_MAX), MSD_LEVEL_PASSES);
    assert_eq!(tile_cost(8, 3_906, 0.99, INSERTION_MAX), 8);
    assert_eq!(tile_cost(2, 3_906, 1.0 / 128.0, INSERTION_MAX), 2);

    // The tile's digit width is the widest table that still fits the tile.
    assert_eq!(leaf_radix(3_906, 8), 13);
    assert_eq!(leaf_radix(200, 8), 9);
    assert_eq!(leaf_radix(4, 8), LEAF_RADIX_MIN);
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

    let mut rng = Rng(7);
    for &len in &[2usize, 100, 5000] {
        let input: Vec<Item<u16>> = (0..len)
            .map(|index| Item {
                key: (rng.next() % 4) as u16,
                index: index as u32,
            })
            .collect();
        let mut sorted = input.clone();
        sorted.sort_by_key(|item| item.key);
        let mut actual = input.clone();
        sort_with_policy(&mut actual, Policy::default());
        for (a, e) in actual.iter().zip(sorted.iter()) {
            assert_eq!(a, e);
        }
    }
}

#[test]
fn random_configurations_match_a_stable_reference() {
    // Random key shapes crossed with random tile budgets, checked against the
    // standard library's stable sort. The payload is the input index, so an
    // equal-key pair out of order fails the comparison: this is the one test
    // that exercises stability and correctness together over shapes no
    // hand-written case names (sparse masks, one varying byte, two clusters).

    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    let tiles = [4 * 1024usize, 16 * 1024, 64 * 1024, 256 * 1024, 1 << 20];
    for round in 0..2000 {
        let len = (rng.next() % 8000) as usize;
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
        expected.sort();
        let mut actual = input;
        sort_with_policy(
            &mut actual,
            Policy {
                tile_bytes: tiles[(round % tiles.len() as u64) as usize],
                ..Policy::default()
            },
        );
        for (position, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
            assert_eq!(
                (a.key, a.index),
                (e.key, e.index),
                "round {round} shape {shape} len {len} at {position}"
            );
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
