#![cfg(feature = "rust_tessera_radix")]

use sort_test_tools::instantiate_sort_tests;

type TestSort = sort_research_rs::other::rust_tessera_radix::SortImpl;

// Same contract and same suite as the block LSD adapter: every type the key
// trait covers, with unsupported types and `sort_by` declared up front.
instantiate_sort_tests!(TestSort);
