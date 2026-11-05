//! Stable tiled radix sort: MSD partition, LSD tiles.
//!
//! The engine lives in [`engine`]; this file only declares the sort identity
//! seen by the benchmark harness and the tests, mirroring the block LSD adapter.

mod engine;

pub use super::radix_support::{Key, Unsigned};

radix_adapter!("rust_tessera_stable_radix", |data| engine::sort(data));

/// Sorts with an explicit LSD tile budget in bytes of record data.
pub fn sort_with_tile_bytes<T: Key>(data: &mut [T], tile_bytes: usize) {
    engine::sort_with_tile_bytes(data, tile_bytes);
}

/// Which leaf strategy to use once a tile is too large for insertion or merge.
pub use engine::LeafMode;

/// Sorts with an explicit leaf strategy. Exists so the flat counting plan and
/// the MSD level inside the tile can be measured in one process, interleaved,
/// instead of across runs on a shared machine.
pub fn sort_with_leaf_mode<T: Key>(data: &mut [T], mode: LeafMode) {
    engine::sort_with_leaf_mode(data, mode);
}

/// Sorts with an explicit tile budget and leaf-policy thresholds: tiles up to
/// `insertion_max` records use an insertion sort, up to `merge_max` a merge
/// sort, larger ones counting passes. The leaf policy never changes the O(sqrt
/// n) auxiliary space, because the tile itself is capped by the budget.
pub fn sort_with_policy<T: Key>(
    data: &mut [T],
    tile_bytes: usize,
    merge_max: usize,
    insertion_max: usize,
) {
    engine::sort_with_policy(data, tile_bytes, merge_max, insertion_max);
}
