#![cfg(feature = "rust_block_lsd")]
use sort_research_rs::other::radix_support::Key;
use sort_test_tools::Sort;
use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    rc::Rc,
};

trait Payload {
    fn number(&self) -> usize;
}
struct Dynamic<'a>(&'a Cell<usize>);
impl Payload for Dynamic<'_> {
    fn number(&self) -> usize {
        17
    }
}
impl Drop for Dynamic<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}
struct Context {
    calls: Cell<usize>,
    panic_at: usize,
    drops: Vec<Cell<usize>>,
    shared_drops: Cell<usize>,
}
// Deliberately neither Copy nor Clone. The numeric key is independent of every
// owning payload, including its trait object and interior-mutable Box.
struct Owned<'a> {
    key: u64,
    id: usize,
    text: String,
    boxed: Box<[u8]>,
    shared: Rc<dyn Payload + 'a>,
    edits: RefCell<Box<usize>>,
    key_calls: Cell<usize>,
    context: &'a Context,
}
impl Drop for Owned<'_> {
    fn drop(&mut self) {
        let n = &self.context.drops[self.id];
        n.set(n.get() + 1);
    }
}
impl Key for Owned<'_> {
    type Value = u64;
    fn key(&self) -> u64 {
        let n = self.context.calls.get();
        self.context.calls.set(n + 1);
        self.key_calls.set(self.key_calls.get() + 1);
        // Replacing an owned pointer is intentional. A stale scratch/input
        // copy would lose these edits or drop an already-freed allocation.
        let old = self.edits.replace(Box::new(self.key_calls.get()));
        drop(old);
        if n == self.context.panic_at {
            std::panic::panic_any(n);
        }
        self.key
    }
}
impl PartialEq for Owned<'_> {
    fn eq(&self, b: &Self) -> bool {
        self.key == b.key
    }
}
impl Eq for Owned<'_> {}
impl PartialOrd for Owned<'_> {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}
impl Ord for Owned<'_> {
    fn cmp(&self, b: &Self) -> Ordering {
        self.key.cmp(&b.key)
    }
}

fn trial<S: Sort>(n: usize, target: usize) -> usize {
    let ctx = Context {
        calls: Cell::new(0),
        panic_at: target,
        drops: (0..n).map(|_| Cell::new(0)).collect(),
        shared_drops: Cell::new(0),
    };
    let root: Rc<dyn Payload> = Rc::new(Dynamic(&ctx.shared_drops));
    let mut input: Vec<_> = (0..n)
        .map(|id| Owned {
            key: (id % 7) as u64,
            id,
            text: format!("载荷\0🙂-{id}"),
            boxed: vec![id as u8; 13].into_boxed_slice(),
            shared: root.clone(),
            edits: RefCell::new(Box::new(0)),
            key_calls: Cell::new(0),
            context: &ctx,
        })
        .collect();
    let text_ptrs: Vec<_> = input.iter().map(|r| r.text.as_ptr()).collect();
    let box_ptrs: Vec<_> = input.iter().map(|r| r.boxed.as_ptr()).collect();
    assert_eq!(Rc::strong_count(&root), n + 1);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| S::sort(&mut input)));
    let calls = ctx.calls.get();
    if target == usize::MAX {
        assert!(result.is_ok());
        let mut expected: Vec<_> = (0..n).collect();
        expected.sort_by_key(|&i| i % 7);
        assert_eq!(input.iter().map(|r| r.id).collect::<Vec<_>>(), expected);
    } else {
        let panic = result.unwrap_err();
        assert_eq!(panic.downcast_ref::<usize>(), Some(&target));
    }
    assert_eq!(
        Rc::strong_count(&root),
        n + 1,
        "sorting must not clone/drop Rc payloads"
    );
    assert!(
        ctx.drops.iter().all(|v| v.get() == 0),
        "no payload may be dropped by the sorter"
    );
    assert_eq!(
        input.iter().map(|r| r.key_calls.get()).sum::<usize>(),
        calls,
        "all interior mutations must remain observable"
    );
    input.sort_unstable_by_key(|r| r.id);
    for (id, row) in input.iter().enumerate() {
        assert_eq!(row.id, id);
        assert_eq!(row.text, format!("载荷\0🙂-{id}"));
        assert_eq!(row.text.as_ptr(), text_ptrs[id]);
        assert_eq!(row.boxed.as_ptr(), box_ptrs[id]);
        assert!(row.boxed.iter().all(|&v| v == id as u8));
        assert_eq!(row.shared.number(), 17);
        assert_eq!(**row.edits.borrow(), row.key_calls.get());
    }
    drop(input);
    assert!(
        ctx.drops.iter().all(|v| v.get() == 1),
        "each owner must drop exactly once"
    );
    assert_eq!(Rc::strong_count(&root), 1);
    drop(root);
    assert_eq!(ctx.shared_drops.get(), 1);
    calls
}
fn owners<S: Sort>() {
    let sizes: &[usize] = if cfg!(miri) || std::env::var_os("RADIX_OWNED_QUICK").is_some() {
        &[129]
    } else {
        &[2, 129, 1025]
    };
    for &n in sizes {
        let total = trial::<S>(n, usize::MAX);
        let mut targets = vec![0, n, n + 1, 2 * n + 1, 3 * n + 1, total - 1];
        targets.sort_unstable();
        targets.dedup();
        for target in targets.into_iter().filter(|&i| i < total) {
            trial::<S>(n, target);
        }
    }
}
#[cfg(feature = "rust_block_lsd")]
#[test]
fn block_owners() {
    owners::<sort_research_rs::other::rust_block_lsd::SortImpl>();
}
#[cfg(feature = "rust_tessera_radix")]
#[test]
fn tessera_owners() {
    owners::<sort_research_rs::other::rust_tessera_radix::SortImpl>();
}
struct KeyOnly {
    key: u64,
    payload: String,
}
impl Key for KeyOnly {
    type Value = u64;
    fn key(&self) -> u64 {
        self.key
    }
}
#[test]
fn key_only_records() {
    macro_rules! check {
        ($module:path) => {{
            use $module as adapter;
            let mut rows = vec![
                KeyOnly {
                    key: 2,
                    payload: "first-two".into(),
                },
                KeyOnly {
                    key: 1,
                    payload: "one".into(),
                },
                KeyOnly {
                    key: 2,
                    payload: "second-two".into(),
                },
            ];
            adapter::sort_keyed(&mut rows);
            assert_eq!(
                rows.iter().map(|r| r.payload.as_str()).collect::<Vec<_>>(),
                ["one", "first-two", "second-two"]
            );
        }};
    }
    #[cfg(feature = "rust_block_lsd")]
    check!(sort_research_rs::other::rust_block_lsd);
    #[cfg(feature = "rust_tessera_radix")]
    check!(sort_research_rs::other::rust_tessera_radix);
}
