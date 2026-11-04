#![cfg(feature = "rust_block_lsd")]
use sort_research_rs::other::radix_support::Key;
use sort_test_tools::Sort;
use std::cell::Cell;
use std::cmp::Ordering;

thread_local! {
    static SKEW: Cell<bool> = const { Cell::new(false) };
    static CALLS: Cell<usize> = const { Cell::new(0) };
    static PANIC_AT: Cell<usize> = const { Cell::new(usize::MAX) };
    static DROPS: Cell<usize> = const { Cell::new(0) };
}
// Owning, so the backend maintains the exact recovery offset for it. Recovery
// after a panic is then observable as one drop per original record: a lost
// record drops fewer, a duplicated one drops more.
struct Item<const N: usize> {
    identity: usize,
    payload: [u8; N],
}
impl<const N: usize> Clone for Item<N> {
    fn clone(&self) -> Self {
        Self {
            identity: self.identity,
            payload: self.payload,
        }
    }
}
impl<const N: usize> Drop for Item<N> {
    fn drop(&mut self) {
        DROPS.with(|d| d.set(d.get() + 1));
    }
}
fn key_value(i: usize) -> u64 {
    SKEW.with(|skew| {
        if skew.get() {
            (i % 2) as u64
        } else {
            (i as u64).wrapping_mul(0x9e37) & 65535
        }
    })
}
impl<const N: usize> PartialEq for Item<N> {
    fn eq(&self, other: &Self) -> bool {
        key_value(self.identity) == key_value(other.identity)
    }
}
impl<const N: usize> Eq for Item<N> {}
impl<const N: usize> PartialOrd for Item<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<const N: usize> Ord for Item<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        key_value(self.identity).cmp(&key_value(other.identity))
    }
}
impl<const N: usize> Key for Item<N> {
    type Value = u64;
    fn key(&self) -> u64 {
        let call = CALLS.with(|n| {
            let old = n.get();
            n.set(old + 1);
            old
        });
        if PANIC_AT.with(|n| n.get() == call) {
            std::panic::panic_any(call);
        }
        key_value(self.identity)
    }
}

fn panic_retention<S: Sort, const PAD: usize>() {
    // Every target is a full sort that panics partway, and Miri interprets each
    // one. Narrow it to one skew and a few sizes and targets there.
    let skews: &[bool] = if cfg!(miri) { &[false] } else { &[false, true] };
    for &skew in skews {
        SKEW.with(|v| v.set(skew));
        let sizes: &[usize] = if cfg!(miri) {
            &[2, 17]
        } else {
            &[2, 3, 17, 127, 128, 129, 257, 1025, 8193]
        };
        for &n in sizes {
            let original: Vec<_> = (0..n)
                .map(|identity| Item {
                    identity,
                    payload: [identity as u8; PAD],
                })
                .collect();
            PANIC_AT.with(|v| v.set(usize::MAX));
            CALLS.with(|v| v.set(0));
            let mut normal = original.clone();
            S::sort(&mut normal);
            let total = CALLS.with(|v| v.get());
            let mut targets = if cfg!(miri) {
                vec![0, 3 * n, total - 1]
            } else {
                vec![
                    0,
                    n - 1,
                    n,
                    n + 1,
                    2 * n - 1,
                    2 * n,
                    3 * n,
                    3 * n + n / 2,
                    4 * n - 1,
                    total - 1,
                ]
            };
            targets.sort_unstable();
            targets.dedup();
            for target in targets.into_iter().filter(|&v| v < total) {
                let mut data = original.clone();
                CALLS.with(|v| v.set(0));
                PANIC_AT.with(|v| v.set(target));
                DROPS.with(|v| v.set(0));
                let panic =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| S::sort(&mut data)))
                        .unwrap_err();
                assert_eq!(
                    panic.downcast_ref::<usize>(),
                    Some(&target),
                    "unexpected recovery panic: n={n}, pad={PAD}"
                );
                PANIC_AT.with(|v| v.set(usize::MAX));
                // Every original record must be dropped exactly once, whether
                // it was in the slice or still in a scratch block.
                drop(data);
                assert_eq!(
                    DROPS.with(|v| v.get()),
                    n,
                    "record lost or duplicated by recovery: n={n}, target={target}, pad={PAD}"
                );
                let mut data = original.clone();
                data.sort_unstable_by_key(|v| v.identity);
                assert!(
                    data.iter()
                        .zip(&original)
                        .all(|(a, b)| a.identity == b.identity && a.payload == b.payload),
                    "original set changed: n={n}, target={target}, pad={PAD}"
                );
            }
        }
    }
}

#[repr(transparent)]
struct PartialFloat(f32);
impl PartialEq for PartialFloat {
    fn eq(&self, b: &Self) -> bool {
        self.0 == b.0
    }
}
impl Eq for PartialFloat {}
impl PartialOrd for PartialFloat {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}
impl Ord for PartialFloat {
    fn cmp(&self, b: &Self) -> Ordering {
        self.0.partial_cmp(&b.0).unwrap()
    }
}
impl Key for PartialFloat {
    type Value = u32;
    fn key(&self) -> u32 {
        sort_research_rs::other::radix_support::partial_f32_key(self.0)
    }
}

#[repr(C)]
#[derive(Clone)]
struct NonCopyRecord {
    value: u64,
    identity: u64,
    payload: [u8; 16],
}
impl PartialEq for NonCopyRecord {
    fn eq(&self, b: &Self) -> bool {
        self.value == b.value
    }
}
impl Eq for NonCopyRecord {}
impl PartialOrd for NonCopyRecord {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}
impl Ord for NonCopyRecord {
    fn cmp(&self, b: &Self) -> Ordering {
        self.value.cmp(&b.value)
    }
}
impl Key for NonCopyRecord {
    type Value = u64;
    fn key(&self) -> u64 {
        self.value
    }
}

fn contracts<S: Sort>() {
    let mut strings = vec!["b".to_owned(), "a".to_owned()];
    let original = strings.clone();
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| S::sort(&mut strings)))
        .unwrap_err();
    assert_eq!(panic.downcast_ref::<&str>(), Some(&"Type not supported"));
    assert_eq!(strings, original);
    let mut input = [3i32, 1, 2];
    let called = Cell::new(false);
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        S::sort_by(&mut input, |a, b| {
            called.set(true);
            a.cmp(b)
        })
    }))
    .unwrap_err();
    assert_eq!(panic.downcast_ref::<&str>(), Some(&"sort_by not supported"));
    assert!(!called.get());
    assert_eq!(input, [3, 1, 2]);
}

fn views<S: Sort>() {
    let mut records: Vec<_> = (0..4097)
        .map(|i| NonCopyRecord {
            value: (i * 37 % 17) as u64,
            identity: i,
            payload: [i as u8; 16],
        })
        .collect();
    let mut expected = records.clone();
    expected.sort();
    S::sort(&mut records);
    assert!(records
        .iter()
        .zip(&expected)
        .all(|(a, b)| a.value == b.value && a.identity == b.identity && a.payload == b.payload));
    let mut floats: Vec<_> = [
        -0.0f32,
        0.0,
        -1.0,
        0.0,
        -0.0,
        2.0,
        f32::NEG_INFINITY,
        f32::INFINITY,
    ]
    .into_iter()
    .cycle()
    .take(1025)
    .map(PartialFloat)
    .collect();
    let mut expected: Vec<_> = floats.iter().map(|v| v.0).collect();
    expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
    S::sort(&mut floats);
    assert_eq!(
        floats.iter().map(|v| v.0.to_bits()).collect::<Vec<_>>(),
        expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
    let mut tuples = vec![
        (i32::MAX, i32::MIN),
        (0, 2),
        (0, -2),
        (i32::MIN, i32::MAX),
        (0, 2),
    ];
    let mut expected = tuples.clone();
    expected.sort();
    S::sort(&mut tuples);
    assert_eq!(tuples, expected);
    let mut cells: Vec<_> = (0..1025).rev().map(|i| (Cell::new(i % 17), i)).collect();
    let mut expected = cells.clone();
    expected.sort();
    S::sort(&mut cells);
    assert_eq!(cells, expected);
    let mut kib: Vec<_> = (0..1025)
        .rev()
        .map(|i| sort_test_tools::ffi_types::FFIOneKibiByte::new(i % 17))
        .collect();
    let mut expected = kib.clone();
    expected.sort();
    S::sort(&mut kib);
    assert_eq!(kib, expected);
}

macro_rules! checks {
    ($module:ident,$sort:ty) => {
        mod $module {
            #[test]
            fn views() {
                super::views::<$sort>();
            }
            #[test]
            fn contracts() {
                super::contracts::<$sort>();
            }
            #[test]
            fn panic_group_four() {
                super::panic_retention::<$sort, 0>();
            }
            #[test]
            fn panic_group_two() {
                super::panic_retention::<$sort, 8>();
            }
            #[test]
            fn panic_scalar() {
                super::panic_retention::<$sort, 112>();
            }
        }
    };
}
#[cfg(feature = "rust_block_lsd")]
checks!(block, sort_research_rs::other::rust_block_lsd::SortImpl);

#[test]
fn declared_capabilities_match_rejections() {
    macro_rules! check {
        ($module:path) => {{
            use $module as adapter;
            assert!(adapter::supports_type::<std::cell::Cell<i32>>());
            assert!(adapter::supports_type::<(i32, i32)>());
            assert!(adapter::supports_type::<(Cell<i32>, i32)>());
            assert!(adapter::supports_type::<
                sort_test_tools::ffi_types::FFIOneKibiByte,
            >());
            assert!(!adapter::supports_type::<String>());
            assert!(!adapter::supports_type::<(u128, u128)>());
        }};
    }
    #[cfg(feature = "rust_block_lsd")]
    check!(sort_research_rs::other::rust_block_lsd);
}
