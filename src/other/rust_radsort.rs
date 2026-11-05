use std::cmp::Ordering;

sort_impl!("rust_radsort_radix");

trait RadSort: Sized {
    fn supported() -> bool;
    fn sort(data: &mut [Self]);
}

impl<T> RadSort for T {
    default fn supported() -> bool {
        false
    }

    default fn sort(_data: &mut [Self]) {
        panic!("Type not supported");
    }
}

impl<T: radsort::Key> RadSort for T {
    fn supported() -> bool {
        true
    }

    fn sort(data: &mut [Self]) {
        radsort::sort(data);
    }
}

pub fn supports_type<T: Ord>() -> bool {
    <T as RadSort>::supported()
}

pub fn sort<T: Ord>(data: &mut [T]) {
    <T as RadSort>::sort(data);
}

/// Sort records by an extracted key. This is the record counterpart of
/// `radsort::sort`, which only accepts types implementing `radsort::Key`.
#[cfg(feature = "rust_block_lsd")]
pub fn sort_keyed<T>(data: &mut [T])
where
    T: crate::other::radix_support::Key,
    T::Value: radsort::Key,
{
    radsort::sort_by_key(data, |value| value.key());
}

pub fn sort_by<T, F: FnMut(&T, &T) -> Ordering>(_data: &mut [T], _compare: F) {
    panic!("sort_by not supported");
}
