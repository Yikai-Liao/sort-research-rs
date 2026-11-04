#![cfg(feature = "rust_block_lsd")]

use sort_research_rs::other::rust_block_lsd::{self, Key};
use std::cmp::Ordering;

type TestSort = rust_block_lsd::SortImpl;

// The library disables unit tests. Include the same backend source here to
// exercise configurable digits and explicit block sizes through its own tests.
#[path = "../src/other/rust_block_lsd/core.rs"]
mod backend;
mod key {
    pub use sort_research_rs::other::rust_block_lsd::{Key, Unsigned};
}

#[test]
fn f128_preserves_quotient_order_and_full_records() {
    use sort_test_tools::ffi_types::F128;
    let mut input: Vec<_> = [i32::MIN, -1, 0, 1, i32::MAX]
        .into_iter()
        .chain((0..4096).rev().map(|i| i32::MAX - i))
        .cycle()
        .take(8193)
        .map(F128::new)
        .collect();
    let mut expected = input.clone();
    expected.sort();
    rust_block_lsd::sort(&mut input);
    assert_eq!(input, expected);
    for pair in input.windows(2) {
        assert_eq!(pair[0].cmp(&pair[1]), pair[0].key().cmp(&pair[1].key()));
    }
}

#[derive(Clone, Copy, Debug)]
struct Record<const N: usize> {
    key: u64,
    ordinal: usize,
    payload: [u8; N],
}

impl<const N: usize> Key for Record<N> {
    type Value = u64;
    fn key(&self) -> u64 {
        self.key
    }
}

impl<const N: usize> PartialEq for Record<N> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<const N: usize> Eq for Record<N> {}
impl<const N: usize> PartialOrd for Record<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<const N: usize> Ord for Record<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

fn check_payloads<const N: usize>() {
    for len in [
        0, 1, 2, 127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 4095, 4096, 4097, 16385,
    ] {
        for pattern in 0..6 {
            let mut actual: Vec<_> = (0..len)
                .map(|ordinal| {
                    let mixed = (ordinal as u64).wrapping_mul(0x9e3779b97f4a7c15);
                    let key = match pattern {
                        0 => mixed,
                        1 => mixed % 7,
                        2 => (u64::MAX & !65535) | (mixed & 65535),
                        3 => ((len - ordinal) / 4) as u64,
                        4 => 42,
                        _ => {
                            if ordinal + 1 == len {
                                43
                            } else {
                                42
                            }
                        }
                    };
                    Record {
                        key,
                        ordinal,
                        payload: [mixed as u8; N],
                    }
                })
                .collect();
            let mut expected = actual.clone();
            expected.sort();
            rust_block_lsd::sort(&mut actual);
            // Record equality deliberately ignores payload; compare all fields here.
            for (a, e) in actual.iter().zip(&expected) {
                assert_eq!(
                    (a.key, a.ordinal, a.payload),
                    (e.key, e.ordinal, e.payload),
                    "len={len}, pattern={pattern}, payload={N}"
                );
            }
        }
    }
}

#[test]
fn stable_payloads_and_block_tails() {
    check_payloads::<0>();
    check_payloads::<16>();
    check_payloads::<112>();
}

#[test]
fn all_integer_widths() {
    macro_rules! check {
        ($($ty:ty),*) => {$({
            let input = [<$ty>::MIN, <$ty>::MAX, 0, 1, 0, 2];
            let mut actual: Vec<$ty> = input.into_iter().cycle().take(1025).collect();
            let mut expected = actual.clone();
            expected.sort();
            rust_block_lsd::sort(&mut actual);
            assert_eq!(actual, expected);
        })*};
    }
    check!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
}

#[test]
fn unsupported_type_preserves_input() {
    let mut input = vec![String::from("b"), String::from("a")];
    let original = input.clone();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        rust_block_lsd::sort(&mut input);
    }));
    let message = result.unwrap_err();
    assert_eq!(message.downcast_ref::<&str>(), Some(&"Type not supported"));
    assert_eq!(input, original);
}

#[test]
fn unsupported_sort_by_preserves_input_and_does_not_call_comparator() {
    let mut input = [2u64, 1];
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        rust_block_lsd::sort_by(&mut input, |_, _| panic!("comparator was called"));
    }));
    let message = result.unwrap_err();
    assert_eq!(
        message.downcast_ref::<&str>(),
        Some(&"sort_by not supported")
    );
    assert_eq!(input, [2, 1]);
}

#[path = "support/radix_tests.rs"]
#[macro_use]
mod radix_tests;
radix_functionality!(TestSort);
