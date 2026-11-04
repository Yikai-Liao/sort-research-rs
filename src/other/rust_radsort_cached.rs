//! Indirect stable LSD from the pinned radsort crate. Keys are cached once,
//! key/index records are sorted, then the original payloads are permuted.
use super::radix_support::Key;
sort_impl!("rust_radsort_cached_stable_radix");
trait Cached: Sized {
    fn supported() -> bool;
    fn sort(data: &mut [Self]);
}
impl<T> Cached for T {
    default fn supported() -> bool {
        false
    }
    default fn sort(_data: &mut [Self]) {
        panic!("Type not supported");
    }
}
impl<T: Key> Cached for T
where
    T::Value: radsort::Key,
{
    fn supported() -> bool {
        T::SUPPORTED
    }
    fn sort(data: &mut [Self]) {
        sort_keyed(data);
    }
}
pub fn supports_type<T>() -> bool {
    <T as Cached>::supported()
}
pub fn sort<T: Ord>(data: &mut [T]) {
    <T as Cached>::sort(data);
}
pub fn sort_keyed<T: Key>(data: &mut [T])
where
    T::Value: radsort::Key,
{
    assert!(T::SUPPORTED, "Type not supported");
    // Extraction finishes before any original record moves. A key panic drops
    // only temporary scalar keys/indices, leaving owners and edits in input.
    radsort::sort_by_cached_key(data, Key::key);
}
pub fn sort_by<T, F: FnMut(&T, &T) -> std::cmp::Ordering>(_data: &mut [T], _compare: F) {
    panic!("sort_by not supported");
}
