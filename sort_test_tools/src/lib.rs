#![feature(macro_metavar_expr_concat)]

pub trait Sort {
    fn name() -> String;

    fn sort<T>(v: &mut [T])
    where
        T: Ord;

    fn sort_by<T, F>(v: &mut [T], compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering;

    /// Whether [`Sort::sort`] accepts `T`. Key based sorts accept the types they
    /// have a key conversion for, not every `Ord` type. Tests use this to skip a
    /// type instead of failing on the expected rejection.
    fn supports_type<T>() -> bool
    where
        T: Ord,
    {
        true
    }

    /// Whether [`Sort::sort_by`] is implemented. Sorts that only take a fixed
    /// key reject it rather than falling back to comparison sorting.
    fn supports_sort_by() -> bool {
        true
    }
}

pub mod ffi_types;
pub mod patterns;
pub mod tests;
