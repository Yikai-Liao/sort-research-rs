#![cfg(feature = "rust_pa")]
use sort_research_rs::other::{radix_support::Key, rust_pa as pa};
use sort_test_tools::ffi_types::RadixRecord;

#[test]
fn primitive_keys_include_all_widths_and_extremes() {
    macro_rules! check {
        ($($t:ty),*) => {$(
            assert!(pa::supports_type::<$t>());
            for n in [0, 1, 2, 127, 128, 129, 257, 2049, 10000] {
                let mut a: Vec<$t> = (0..n).map(|i|
                    [<$t>::MIN, <$t>::MAX, 0, 1, (i as u128).wrapping_mul(0x9e3779b97f4a7c15) as $t][i % 5]
                ).collect();
                let mut expected = a.clone(); expected.sort();
                pa::sort(&mut a); assert_eq!(a, expected);
            }
        )*};
    }
    check!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
    let mut chars = vec!['\u{10ffff}', '\0', 'é', 'a', '\0'];
    pa::sort(&mut chars);
    assert_eq!(chars, vec!['\0', '\0', 'a', 'é', '\u{10ffff}']);
    let mut flags = [true, false, true, false];
    pa::sort(&mut flags);
    assert_eq!(flags, [false, false, true, true]);
    pa::sort(&mut [(); 257]);
}
fn records<const N: usize>(n: usize, pattern: usize) {
    let mut a: Vec<_> = (0..n)
        .map(|i| {
            let random = (i as u64).wrapping_mul(0x9e3779b97f4a7c15);
            let key = match pattern {
                0 => random,
                1 => random % 20,
                2 => (i / 3) as u64,
                3 => ((n - i) / 3) as u64,
                4 => 42,
                5 => {
                    if i % 20 == 0 {
                        random
                    } else {
                        u64::MAX / 2
                    }
                }
                _ => (random & 65535) << 40,
            };
            RadixRecord::<N>::new(key, i)
        })
        .collect();
    let mut expected = a.clone();
    expected.sort();
    pa::sort(&mut a);
    assert!(
        a.iter()
            .zip(&expected)
            .all(|(a, b)| a.key == b.key && a.identity == b.identity && a.payload == b.payload),
        "record bytes={} n={n} pattern={pattern}",
        std::mem::size_of::<RadixRecord<N>>()
    );
}
#[test]
fn record32_stability_at_leaf_and_block_boundaries() {
    for n in [0, 1, 2, 17, 127, 128, 129, 255, 256, 257, 2048, 2049, 16385] {
        for pattern in 0..7 {
            records::<16>(n, pattern);
        }
    }
}
#[test]
fn record128_stability_at_leaf_and_block_boundaries() {
    for n in [0, 1, 2, 17, 127, 128, 129, 255, 256, 257, 2048, 2049, 16385] {
        for pattern in 0..7 {
            records::<112>(n, pattern);
        }
    }
}
#[test]
fn record32_stability_above_dense_threshold() {
    for pattern in 0..7 {
        records::<16>(600001, pattern);
    }
}
#[test]
fn record128_stability_above_dense_threshold() {
    for pattern in 0..7 {
        records::<112>(150001, pattern);
    }
}
#[test]
fn unsupported_owners_and_comparators_reject_before_modification() {
    assert!(!pa::supports_type::<String>());
    assert!(!pa::supports_type::<sort_test_tools::ffi_types::F128>());
    let mut a = vec![String::from("b"), String::from("a")];
    let expected = a.clone();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| pa::sort(&mut a))).is_err());
    assert_eq!(a, expected);
    let mut a = [3, 1, 2];
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| pa::sort_by(
            &mut a,
            |_, _| panic!("comparator invoked")
        )))
        .is_err()
    );
    assert_eq!(a, [3, 1, 2]);
}
#[test]
fn fixture_key_matches_ord_and_preserves_size() {
    assert_eq!(std::mem::size_of::<RadixRecord<16>>(), 32);
    assert_eq!(std::mem::size_of::<RadixRecord<112>>(), 128);
    assert!(pa::supports_type::<RadixRecord<16>>());
    assert!(pa::supports_type::<RadixRecord<112>>());
    let a = RadixRecord::<16>::new(2, 0);
    let b = RadixRecord::<16>::new(3, 1);
    assert_eq!(Key::key(&a).cmp(&Key::key(&b)), a.cmp(&b));
}
#[cfg(feature = "rust_pa_work")]
#[test]
fn work_confirms_real_dense_and_block_routes() {
    let mut a: Vec<u32> = (0..10000)
        .map(|i| (i as u32).wrapping_mul(0x9e3779b9))
        .collect();
    let dense = pa::diagnose(&mut a);
    assert!(dense.dense_nodes > 0);
    let mut a: Vec<_> = (0..150001)
        .map(|i| RadixRecord::<112>::new((i as u64).wrapping_mul(0x9e3779b97f4a7c15), i))
        .collect();
    let block = pa::diagnose(&mut a);
    assert!(block.block_nodes > 0);
    eprintln!(
        "PA routes dense={} block={}",
        dense.dense_nodes, block.block_nodes
    );
}
