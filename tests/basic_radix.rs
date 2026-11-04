#![cfg(any(
    feature = "rust_basic_lsd",
    feature = "rust_basic_msd",
    feature = "rust_lsd11",
    feature = "rust_radsort_cached",
    feature = "rust_radsort_unopt"
))]
#[path = "support/radix_tests.rs"]
#[macro_use]
mod radix_tests;
#[cfg(feature = "rust_basic_lsd")]
mod lsd {
    radix_functionality!(sort_research_rs::other::rust_basic_lsd::SortImpl);
}
#[cfg(feature = "rust_basic_msd")]
mod msd {
    radix_functionality!(sort_research_rs::other::rust_basic_msd::SortImpl);
}

#[cfg(feature = "rust_lsd11")]
mod wide {
    radix_functionality!(sort_research_rs::other::rust_lsd11::SortImpl);
}
#[cfg(feature = "rust_radsort_cached")]
mod cached {
    radix_functionality!(sort_research_rs::other::rust_radsort_cached::SortImpl);
}
#[cfg(feature = "rust_radsort_unopt")]
mod unopt {
    radix_functionality!(sort_research_rs::other::rust_radsort_unopt::SortImpl);
}
