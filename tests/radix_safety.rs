//! Safe key implementations may panic or change their answers. Neither may
//! invalidate ownership, including mutations made by the last key call.
#![cfg(any(feature = "rust_tessera_radix", feature = "rust_msd_radix"))]

use sort_research_rs::other::radix_support::Key;
use std::cell::{Cell, RefCell};

#[cfg(feature = "rust_msd_radix")]
use sort_research_rs::other::rust_msd_radix::sort as msd;
#[cfg(feature = "rust_tessera_radix")]
use sort_research_rs::other::rust_tessera_radix::sort_keyed as tessera;

const LEN: usize = 64;

struct Context<'a> {
    calls: Cell<usize>,
    key: &'a dyn Fn(usize, usize) -> u64,
    drops: Vec<Cell<usize>>,
}

struct Record<'a> {
    id: usize,
    payload: Box<usize>,
    edits: RefCell<Box<usize>>,
    calls: Cell<usize>,
    context: &'a Context<'a>,
}

impl Key for Record<'_> {
    type Value = u64;

    fn key(&self) -> u64 {
        let call = self.context.calls.get() + 1;
        self.context.calls.set(call);
        self.calls.set(self.calls.get() + 1);
        // A recovery from stale bytes would retain an already freed Box.
        self.edits.replace(Box::new(self.calls.get()));
        (self.context.key)(call, self.id)
    }
}

impl Drop for Record<'_> {
    fn drop(&mut self) {
        let drops = &self.context.drops[self.id];
        drops.set(drops.get() + 1);
    }
}

fn owners(
    sort: impl FnOnce(&mut [Record<'_>]),
    key: impl Fn(usize, usize) -> u64,
    expect_panic: bool,
) -> usize {
    let context = Context {
        calls: Cell::new(0),
        key: &key,
        drops: (0..LEN).map(|_| Cell::new(0)).collect(),
    };
    let mut records: Vec<_> = (0..LEN)
        .map(|id| Record {
            id,
            payload: Box::new(id),
            edits: RefCell::new(Box::new(0)),
            calls: Cell::new(0),
            context: &context,
        })
        .collect();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sort(&mut records)));
    assert_eq!(outcome.is_err(), expect_panic);
    assert!(context.drops.iter().all(|count| count.get() == 0));
    records.sort_unstable_by_key(|record| record.id);
    for (id, record) in records.iter().enumerate() {
        assert_eq!(record.id, id);
        assert_eq!(*record.payload, id);
        assert_eq!(**record.edits.borrow(), record.calls.get());
    }
    assert_eq!(
        records
            .iter()
            .map(|record| record.calls.get())
            .sum::<usize>(),
        context.calls.get()
    );
    drop(records);
    assert!(context.drops.iter().all(|count| count.get() == 1));
    context.calls.get()
}

#[derive(Clone, Copy)]
struct CopyRecord<'a> {
    id: usize,
    edits: &'a Cell<usize>,
    context: &'a Context<'a>,
}
impl Key for CopyRecord<'_> {
    type Value = u64;
    fn key(&self) -> u64 {
        let call = self.context.calls.get() + 1;
        self.context.calls.set(call);
        self.edits.set(self.edits.get() + 1);
        (self.context.key)(call, self.id)
    }
}

fn copy_records(
    sort: impl FnOnce(&mut [CopyRecord<'_>]),
    key: impl Fn(usize, usize) -> u64,
    retain_ids: bool,
) -> (bool, usize) {
    let context = Context {
        calls: Cell::new(0),
        key: &key,
        drops: Vec::new(),
    };
    let edits: Vec<_> = (0..LEN).map(|_| Cell::new(0)).collect();
    let mut records: Vec<_> = edits
        .iter()
        .enumerate()
        .map(|(id, edits)| CopyRecord {
            id,
            edits,
            context: &context,
        })
        .collect();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sort(&mut records)));
    // Even with a changing key, every Copy value and its shared payload must
    // remain initialized. A changing key has no promised sorting result.
    for row in &records {
        assert!(row.id < LEN);
        assert!(std::ptr::eq(row.edits, &edits[row.id]));
        let _ = row.edits.get();
    }
    if retain_ids {
        records.sort_unstable_by_key(|row| row.id);
        assert_eq!(
            records.iter().map(|row| row.id).collect::<Vec<_>>(),
            (0..LEN).collect::<Vec<_>>()
        );
    }
    assert_eq!(
        edits.iter().map(Cell::get).sum::<usize>(),
        context.calls.get()
    );
    (result.is_err(), context.calls.get())
}

#[cfg(feature = "rust_tessera_radix")]
#[test]
fn tessera_copy_key_panics_preserve_records() {
    for digits in [1, 2, 3] {
        let targets: &[usize] = match digits {
            1 => &[133, 165, 196],
            2 => &[133, 197, 260, 300, 324],
            _ => &[133, 197, 324, 400, 452],
        };
        for &panic_at in targets {
            let (panicked, calls) = copy_records(
                |records| tessera(records),
                |call, id| {
                    if call == panic_at {
                        std::panic::panic_any(call);
                    }
                    let window = match digits {
                        1 => 1,
                        2 => (1u64 << 31) | 1,
                        _ => (1u64 << 63) | (1u64 << 31) | 1,
                    };
                    (id % 2) as u64 * window
                },
                true,
            );
            assert!(panicked);
            assert_eq!(calls, panic_at);
        }
    }
}

#[cfg(feature = "rust_msd_radix")]
#[test]
fn msd_copy_key_panics_preserve_records() {
    for panic_at in [129, 131] {
        let (panicked, calls) = copy_records(
            |records| msd(records),
            |call, id| {
                if call == panic_at {
                    std::panic::panic_any(call);
                }
                (id % 2) as u64
            },
            true,
        );
        assert!(panicked);
        assert_eq!(calls, panic_at);
    }
}

#[cfg(feature = "rust_msd_radix")]
#[test]
fn msd_copy_changed_keys_remain_valid() {
    for empty_bucket in [false, true] {
        let (panicked, _) = copy_records(
            |records| msd(records),
            |call, id| {
                if empty_bucket {
                    match call {
                        1..=64 => (id % 2) as u64,
                        65..=128 => 0,
                        _ => 1,
                    }
                } else if call <= 128 {
                    (id % 2) as u64
                } else {
                    0
                }
            },
            false,
        );
        assert!(panicked);
    }
}

#[cfg(feature = "rust_tessera_radix")]
#[test]
fn tessera_changed_keys_preserve_owners() {
    // Four probe calls, 64 copied keys, then 64 variation calls. Alternating
    // keys exhaust the insertion trial's budget, reaching a flat counting pass.
    let calls = owners(
        |records| tessera(records),
        |call, id| match call {
            1..=132 => (id % 2) as u64,
            133..=196 => 0,
            _ => 1,
        },
        false,
    );
    assert_eq!(
        calls, 196,
        "scatter uses the histogram classifications without another key call"
    );

    let calls = owners(
        |records| tessera(records),
        |call, id| if call <= 196 { (id % 2) as u64 } else { 0 },
        false,
    );
    // The old second extraction would overfill a bucket while remaining inside
    // the tile. Cached classifications avoid that second answer entirely.
    assert_eq!(calls, 196);
}

#[cfg(feature = "rust_tessera_radix")]
#[test]
fn tessera_later_pass_restores_scratch_owners() {
    let calls = owners(
        |records| tessera(records),
        |call, id| {
            if call == 197 {
                std::panic::panic_any(call);
            }
            (id % 2) as u64 * ((1 << 31) | 1)
        },
        true,
    );
    assert_eq!(
        calls, 197,
        "the second histogram must restore its scratch source"
    );
}

#[cfg(feature = "rust_tessera_radix")]
#[test]
fn tessera_changed_keys_with_matching_counts() {
    owners(
        |records| tessera(records),
        |call, id| {
            let key = (id % 2) as u64;
            if call <= 196 {
                key
            } else {
                1 - key
            }
        },
        false,
    );
}

#[cfg(feature = "rust_tessera_radix")]
#[test]
fn tessera_copy_keys_still_require_initialized_output() {
    #[derive(Clone, Copy)]
    struct CopyRecord<'a> {
        id: usize,
        payload: &'a usize,
        calls: &'a Cell<usize>,
    }
    impl Key for CopyRecord<'_> {
        type Value = u64;
        fn key(&self) -> u64 {
            let call = self.calls.get() + 1;
            self.calls.set(call);
            match call {
                1..=132 => (self.id % 2) as u64,
                133..=196 => 0,
                _ => 1,
            }
        }
    }
    let calls = Cell::new(0);
    let payloads: Vec<_> = (0..LEN).collect();
    let mut records: Vec<_> = payloads
        .iter()
        .enumerate()
        .map(|(id, payload)| CopyRecord {
            id,
            payload,
            calls: &calls,
        })
        .collect();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tessera(&mut records)));
    assert!(outcome.is_ok());
    assert_eq!(calls.get(), 196);
    for (id, record) in records.iter().enumerate() {
        assert_eq!(record.id, id);
        assert_eq!(*record.payload, id);
    }
}

#[cfg(feature = "rust_tessera_radix")]
#[test]
fn tessera_copy_cache_initializes_two_pass_scratch() {
    #[derive(Clone, Copy)]
    struct Byte(u8);
    impl Key for Byte {
        type Value = u64;
        fn key(&self) -> u64 {
            self.0 as u64
        }
    }
    // The record area ends at an odd address. The first of two LSD passes
    // caches digits beside the cursor table and initializes the Copy scratch
    // prefix; the second pass reuses it without another classification.
    let mut input: Vec<_> = (0..257).map(|i| Byte(((i * 73) % 251) as u8)).collect();
    let mut expected: Vec<_> = input.iter().map(|r| r.0).collect();
    expected.sort();
    tessera(&mut input);
    assert_eq!(input.iter().map(|r| r.0).collect::<Vec<_>>(), expected);
}

#[cfg(feature = "rust_tessera_radix")]
#[test]
fn tessera_noncopy_cached_digits_align_after_odd_records() {
    // No Copy and no Drop, with one-byte alignment. This exceeds the cursor
    // table's capacity, forcing cached digits into the odd-ended scratch tail.
    struct Byte(u8);
    impl Key for Byte {
        type Value = u64;
        fn key(&self) -> u64 {
            self.0 as u64
        }
    }
    let mut input: Vec<_> = (0..16385).map(|i| Byte(((i * 73) % 251) as u8)).collect();
    let mut counts = [0usize; 256];
    for row in &input {
        counts[row.0 as usize] += 1;
    }
    tessera(&mut input);
    assert!(input.windows(2).all(|pair| pair[0].0 <= pair[1].0));
    for row in &input {
        counts[row.0 as usize] -= 1;
    }
    assert_eq!(counts, [0; 256]);
}

#[cfg(feature = "rust_tessera_radix")]
#[test]
fn tessera_changed_keys_preserve_exclusive_borrows() {
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
            match call {
                1..=132 => (self.id % 2) as u64,
                133..=196 => 0,
                _ => 1,
            }
        }
    }
    assert!(!std::mem::needs_drop::<Borrowed<'_>>());
    let calls = Cell::new(0);
    let mut slots: Vec<_> = (0..LEN).collect();
    let mut records: Vec<_> = slots
        .iter_mut()
        .enumerate()
        .map(|(id, slot)| Borrowed {
            id,
            slot,
            calls: &calls,
        })
        .collect();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tessera(&mut records)));
    assert!(outcome.is_ok());
    assert_eq!(calls.get(), 196);
    records.sort_unstable_by_key(|record| record.id);
    for (id, record) in records.iter_mut().enumerate() {
        assert_eq!(record.id, id);
        assert_eq!(*record.slot, id);
        *record.slot += LEN;
    }
    drop(records);
    assert_eq!(slots, (LEN..2 * LEN).collect::<Vec<_>>());
}

#[cfg(feature = "rust_msd_radix")]
#[test]
fn msd_key_panics_preserve_owners() {
    let calls = owners(
        |records| {
            msd(records);
            assert!(records
                .windows(2)
                .all(|pair| pair[0].id % 2 <= pair[1].id % 2));
        },
        |_, id| (id % 2) as u64,
        false,
    );
    // Variation and histogram take 128 calls. Cover the first picked record,
    // a cycle's displaced record, later cycles, and recursive variation scans.
    for panic_at in [129, 130, 131, 132, 160, calls] {
        let reached = owners(
            |records| msd(records),
            |call, id| {
                if call == panic_at {
                    std::panic::panic_any(call);
                }
                (id % 2) as u64
            },
            true,
        );
        assert_eq!(reached, panic_at);
    }
}

#[cfg(feature = "rust_msd_radix")]
#[test]
fn msd_changed_keys_preserve_owners() {
    let calls = owners(
        |records| msd(records),
        |call, id| match call {
            1..=64 => (id % 2) as u64,
            65..=128 => 0,
            _ => 1,
        },
        true,
    );
    assert_eq!(calls, 129);
    let calls = owners(
        |records| msd(records),
        |call, id| if call <= 128 { (id % 2) as u64 } else { 0 },
        true,
    );
    assert_eq!(calls, 161);
}

#[cfg(feature = "rust_msd_radix")]
#[test]
fn msd_key_panics_preserve_exclusive_borrows() {
    struct Borrowed<'a> {
        id: usize,
        slot: &'a mut usize,
        calls: &'a Cell<usize>,
        panic_at: usize,
    }
    impl Key for Borrowed<'_> {
        type Value = u64;
        fn key(&self) -> u64 {
            let call = self.calls.get() + 1;
            self.calls.set(call);
            if call == self.panic_at {
                std::panic::panic_any(call);
            }
            (self.id % 2) as u64
        }
    }
    assert!(!std::mem::needs_drop::<Borrowed<'_>>());
    for panic_at in [129, 131] {
        let calls = Cell::new(0);
        let mut slots: Vec<_> = (0..LEN).collect();
        let mut records: Vec<_> = slots
            .iter_mut()
            .enumerate()
            .map(|(id, slot)| Borrowed {
                id,
                slot,
                calls: &calls,
                panic_at,
            })
            .collect();
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| msd(&mut records)));
        assert_eq!(
            outcome.unwrap_err().downcast_ref::<usize>(),
            Some(&panic_at)
        );
        records.sort_unstable_by_key(|record| record.id);
        for (id, record) in records.iter_mut().enumerate() {
            assert_eq!(record.id, id);
            assert_eq!(*record.slot, id);
            *record.slot += LEN;
        }
        drop(records);
        assert_eq!(slots, (LEN..2 * LEN).collect::<Vec<_>>());
    }
}
