#![cfg(feature = "rust_block_lsd")]

// Upstream functionality tests that apply to fixed-width radix keys. Comparator
// based tests and variable-length keys (String) are out of scope for this sort.
#[path = "support/radix_tests.rs"]
#[macro_use]
mod radix_tests;

radix_functionality!(sort_research_rs::other::rust_block_lsd::SortImpl);
