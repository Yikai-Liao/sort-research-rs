//! Stable tiled radix sort: MSD partition, LSD tiles.
//!
//! The engine lives in [`engine`]; this file only declares the sort identity
//! seen by the benchmark harness and the tests, mirroring the block LSD adapter.

mod engine;

radix_adapter!("rust_tessera_stable_radix", |data| engine::sort(data));
