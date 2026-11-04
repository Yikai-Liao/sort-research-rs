//! Bench-only LSD baseline for the wrapper records used here.
//!
//! `rust_radsort` sorts only types that implement `radsort::Key`, i.e. bare
//! scalars. The `f64` wrappers and the padded `u64` records below are structs,
//! so without this adapter the block LSD would have no full-buffer LSD baseline
//! on exactly the records it is meant to help with. It extracts the same key the
//! block adapter uses. This lives in the bench harness only:
//! `src/other/rust_radsort.rs` keeps its current contract.

use std::cmp::Ordering;

use sort_research_rs::other::radix_support::Key;
use sort_test_tools::Sort;

pub struct SortImpl;

impl Sort for SortImpl {
    fn name() -> String {
        "rust_radsort_keyed".into()
    }

    #[inline]
    fn sort<T>(data: &mut [T])
    where
        T: Ord,
    {
        Keyed::sort(data);
    }

    fn sort_by<T, F>(_data: &mut [T], _compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        panic!("sort_by not supported");
    }
}

pub fn supports_type<T>() -> bool {
    <T as Keyed>::supported()
}

trait Keyed: Sized {
    fn supported() -> bool;
    fn sort(data: &mut [Self]);
}

impl<T> Keyed for T {
    default fn supported() -> bool {
        false
    }
    default fn sort(_data: &mut [Self]) {
        panic!("Type not supported");
    }
}

impl<T: Key> Keyed for T
where
    <T as Key>::Value: radsort::Key,
{
    fn supported() -> bool {
        T::SUPPORTED
    }
    fn sort(data: &mut [Self]) {
        assert!(T::SUPPORTED, "Type not supported");
        radsort::sort_by_key(data, |v| v.key());
    }
}
