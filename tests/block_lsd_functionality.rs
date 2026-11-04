#![cfg(feature = "rust_block_lsd")]

use sort_test_tools::instantiate_sort_tests;

type TestSort = sort_research_rs::other::rust_block_lsd::SortImpl;

// The standard suite for every type the adapter has a key conversion for. Types
// without a key, and `sort_by`, are declared unsupported by the adapter, so the
// harness skips them rather than failing on the deliberate rejection.
instantiate_sort_tests!(TestSort);
