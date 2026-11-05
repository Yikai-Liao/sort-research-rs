//! x86-simd-sort for the integer types it directly supports.
//!
//! The adapter calls the unsigned entry with the actual element type. The old
//! version cast `u64` to `i64`, which sorted the upper half of the unsigned
//! range in reverse.

use std::cmp::Ordering;

sort_impl!(
    "cpp_intel_avx512",
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
    fn intel_avx512_i32(data: *mut i32, len: usize);
    fn intel_avx512_u32(data: *mut u32, len: usize);
    fn intel_avx512_u64(data: *mut u64, len: usize);
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
    i32 => intel_avx512_i32,
    u32 => intel_avx512_u32,
    u64 => intel_avx512_u64,
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
