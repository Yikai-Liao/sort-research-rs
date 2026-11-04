pub(crate) mod core;
use super::radix_support as key;

pub use super::radix_support::{Key, Unsigned};

radix_adapter!("rust_block_lsd_stable_radix", |data| core::sort(data, 8));

/// Sorts with an explicit digit width. Only the experiment harness uses widths
/// above 8; the adapter and the public `sort` wrapper keep the tuned default.
pub fn sort_with_radix<T: Key>(data: &mut [T], radix: usize) {
    core::sort(data, radix);
}
