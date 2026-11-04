//! Archived full PA UnifiedGroups policy, with its original Copy contract.
//! The closed adapter uses only built-in nonpanicking numeric conversions.
use super::radix_support::Key;
use sort_test_tools::ffi_types::RadixRecord;
use std::{mem::MaybeUninit, ptr};
#[allow(dead_code, unused_imports)]
mod core;
sort_impl!("rust_pa_unified_groups_stable_radix");

// Conversion-only subset of the original traits. The PA modules never invoke
// the original crate's Dispatcher or other sorting algorithms through them.
pub(crate) trait RadixKey {
    type Key: core::NativeKey;
    fn into_keytype(&self) -> Self::Key;
}
impl<T: Key> RadixKey for T
where
    T::Value: core::NativeKey,
{
    type Key = T::Value;
    #[inline]
    fn into_keytype(&self) -> Self::Key {
        Key::key(self)
    }
}
pub(crate) trait Radixable<K: RadixKey>: Copy {
    type Key: RadixKey;
    fn key(&self) -> Self::Key;
    #[inline]
    fn into_key_type(&self) -> <Self::Key as RadixKey>::Key {
        self.key().into_keytype()
    }
    #[inline]
    fn type_size(&self) -> usize {
        <<Self::Key as RadixKey>::Key as core::NativeKey>::BITS
    }
}
impl<T: Key + Copy> Radixable<T::Value> for T
where
    T::Value: core::NativeKey + Key<Value = T::Value>,
{
    type Key = T::Value;
    #[inline]
    fn key(&self) -> Self::Key {
        Key::key(self)
    }
}
fn insertion_sort_key<T: Radixable<K>, K: RadixKey>(leaf: &mut [T]) {
    for i in 1..leaf.len() {
        let key = leaf[i].into_key_type();
        if leaf[i - 1].into_key_type() <= key {
            continue;
        }
        let (mut lo, mut hi) = (0, i);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if leaf[mid].into_key_type() <= key {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let mut temporary = MaybeUninit::<T>::uninit();
        // SAFETY: i and lo are in bounds. The record is Copy, source/destination
        // overlap only in the ptr::copy that shifts the insertion interval.
        unsafe {
            let base = leaf.as_mut_ptr();
            ptr::copy_nonoverlapping(base.add(i), temporary.as_mut_ptr(), 1);
            ptr::copy(base.add(lo), base.add(lo + 1), i - lo);
            ptr::copy_nonoverlapping(temporary.as_ptr(), base.add(lo), 1);
        }
    }
}

// Restrict this historical adapter to known nonpanicking extraction paths.
// It does not inherit the newer Block LSD adapter's owning/panic guarantees.
trait Supported: Key + Copy {}
macro_rules! supported {
    ($($t:ty),* $(,)?) => {$(impl Supported for $t {})*};
}
supported!(
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    bool,
    char,
    ()
);
impl<const N: usize> Supported for RadixRecord<N> {}
trait Dispatch: Sized {
    fn supported() -> bool;
    fn run(data: &mut [Self]) -> core::Work;
}
impl<T> Dispatch for T {
    default fn supported() -> bool {
        false
    }
    default fn run(_data: &mut [Self]) -> core::Work {
        panic!("Type not supported");
    }
}
impl<T: Supported> Dispatch for T
where
    T::Value: core::NativeKey + Key<Value = T::Value>,
{
    fn supported() -> bool {
        true
    }
    fn run(data: &mut [Self]) -> core::Work {
        let size = std::mem::size_of::<T>();
        if data.len() < 2 || size == 0 {
            return core::Work::default();
        }
        let block = (1024 / size)
            .max((4.0 * (9.0 * data.len() as f64 / (512.0 * size as f64)).sqrt()).ceil() as usize)
            .max(1);
        core::sort_pa_with_threshold::<T, T::Value>(
            data,
            core::PaExperiment::UnifiedGroups,
            block,
            16 * 1024 * 1024,
        )
    }
}
pub fn supports_type<T>() -> bool {
    <T as Dispatch>::supported()
}
pub fn sort<T: Ord>(data: &mut [T]) {
    <T as Dispatch>::run(data);
}
#[cfg(feature = "rust_pa_work")]
pub fn diagnose<T: Ord>(data: &mut [T]) -> core::Work {
    <T as Dispatch>::run(data)
}
pub fn sort_by<T, F: FnMut(&T, &T) -> std::cmp::Ordering>(_data: &mut [T], _compare: F) {
    panic!("sort_by not supported");
}
