//! Stable tiled radix sort: MSD partition, LSD tiles.
//!
//! The engine lives in [`engine`]; this file only declares the sort identity
//! seen by the benchmark harness and the tests, mirroring the block LSD adapter.

mod engine;

use super::radix_support::Key;

radix_adapter!("rust_tessera_stable_radix", |data| engine::sort(data));

pub use engine::Policy;

/// Sorts with an explicit policy. Measurement code pins one knob here instead of
/// the library growing an entry point per combination of them.
pub fn sort_with_policy<T: Key>(data: &mut [T], policy: Policy) {
    engine::sort_with_policy(data, policy);
}

/// Sorts without the tile's MSD level. Measurement only.
pub fn sort_with_leaf_plan<T: Key>(data: &mut [T]) {
    engine::sort_with_leaf_plan(data);
}
