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
