//! DovetailSort in its serial configuration.
//!
//! DovetailSort detects frequently occurring keys and merges them after the
//! light keys have been sorted. It is stable and allocates a full-length
//! temporary record array, so it is the "stable, O(n) workspace" integer-radix
//! comparison. The name has no "unstable" marker, which makes the shared test
//! suite check stability for every supported type.

use std::cmp::Ordering;

use sort_test_tools::ffi_types::{Record128, Record16};

sort_impl!(
    "cpp_dovetail_stable",
    fn supports_type<T>() -> bool
    where
        T: Ord,
    {
        <T as CppSort>::supported()
    },
    fn supports_sort_by() -> bool {
        false
    }
);

extern "C" {
    fn dovetail_stable_i32(data: *mut i32, len: usize);
    fn dovetail_stable_u32(data: *mut u32, len: usize);
    fn dovetail_stable_u64(data: *mut u64, len: usize);
    fn dovetail_stable_record16(data: *mut Record16, len: usize);
    fn dovetail_stable_record128(data: *mut Record128, len: usize);
}

trait CppSort: Sized {
    fn supported() -> bool;
    fn sort(data: &mut [Self]);
}

impl<T> CppSort for T {
    default fn supported() -> bool {
        false
    }

    default fn sort(_data: &mut [Self]) {
        panic!("Type not supported");
    }
}

macro_rules! intrinsic_sort {
    ($($ty:ty => $sort_fn:ident),* $(,)?) => {$(
        impl CppSort for $ty {
            fn supported() -> bool {
                true
            }

            fn sort(data: &mut [Self]) {
                unsafe {
                    $sort_fn(data.as_mut_ptr(), data.len());
                }
            }
        }
    )*};
}

intrinsic_sort!(
    i32 => dovetail_stable_i32,
    u32 => dovetail_stable_u32,
    u64 => dovetail_stable_u64,
    Record16 => dovetail_stable_record16,
    Record128 => dovetail_stable_record128,
);

pub fn supports_type<T: Ord>() -> bool {
    <T as CppSort>::supported()
}

pub fn sort<T: Ord>(data: &mut [T]) {
    <T as CppSort>::sort(data);
}

pub fn sort_by<T, F: FnMut(&T, &T) -> Ordering>(_data: &mut [T], _compare: F) {
    panic!("sort_by not supported");
}
