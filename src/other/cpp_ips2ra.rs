//! IPS²Ra, a sequential super-scalar radix sort.
//!
//! The adapter exposes the upstream sequential entry point for the primitive
//! keys and record layouts it can sort without a conversion pass. IPS²Ra is
//! unstable and keeps bucket buffers, so it belongs to the low-extra-space
//! unstable radix group rather than to the stable lineup.

use std::cmp::Ordering;

use sort_test_tools::ffi_types::{Record128, Record16};

sort_impl!(
    "cpp_ips2ra_unstable",
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
    fn ips2ra_unstable_i32(data: *mut i32, len: usize);
    fn ips2ra_unstable_u32(data: *mut u32, len: usize);
    fn ips2ra_unstable_u64(data: *mut u64, len: usize);
    fn ips2ra_unstable_record16(data: *mut Record16, len: usize);
    fn ips2ra_unstable_record128(data: *mut Record128, len: usize);
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
    i32 => ips2ra_unstable_i32,
    u32 => ips2ra_unstable_u32,
    u64 => ips2ra_unstable_u64,
    Record16 => ips2ra_unstable_record16,
    Record128 => ips2ra_unstable_record128,
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
