mod core;
use super::radix_support as key;

pub use super::radix_support::{Key, Unsigned};

radix_adapter!("rust_block_lsd_stable_radix", |data| core::sort(data, 8));
