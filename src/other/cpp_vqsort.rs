//! Highway VQSort for the integer types it directly supports.
//!
//! VQSort is unstable and does not allocate heap memory. `u128` is a complete
//! 128-bit key, not a 64-bit key with a payload.

use std::cmp::Ordering;

sort_impl!(
    "cpp_vqsort",
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
    fn vqsort_i32(data: *mut i32, len: usize);
    fn vqsort_u16(data: *mut u16, len: usize);
    fn vqsort_u32(data: *mut u32, len: usize);
    fn vqsort_u64(data: *mut u64, len: usize);
    fn vqsort_u128(data: *mut u128, len: usize);
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
    i32 => vqsort_i32,
    u16 => vqsort_u16,
    u32 => vqsort_u32,
    u64 => vqsort_u64,
    u128 => vqsort_u128,
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
