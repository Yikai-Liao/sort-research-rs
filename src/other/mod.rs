#[cfg(any(
    feature = "rust_block_lsd",
    feature = "rust_pa",
    feature = "rust_basic_lsd",
    feature = "rust_basic_msd",
    feature = "rust_lsd11",
    feature = "rust_radsort_cached",
    feature = "rust_radsort_unopt"
))]
#[macro_use]
pub mod radix_support;

#[cfg(feature = "rust_lsd11")]
pub mod rust_lsd11;
#[cfg(feature = "rust_radsort_cached")]
pub mod rust_radsort_cached;
#[cfg(feature = "rust_radsort_unopt")]
pub mod rust_radsort_unopt;

#[cfg(any(feature = "rust_basic_lsd", feature = "rust_basic_msd"))]
mod radix_basic;

#[cfg(feature = "rust_basic_lsd")]
pub mod rust_basic_lsd;

#[cfg(feature = "rust_basic_msd")]
pub mod rust_basic_msd;

#[cfg(feature = "rust_block_lsd")]
pub mod rust_block_lsd;

#[cfg(feature = "rust_radsort")]
pub mod rust_radsort;

#[cfg(feature = "rust_afsort")]
pub mod rust_afsort;

// Call simdsort sort via FFI.
#[cfg(feature = "cpp_simdsort")]
pub mod cpp_simdsort;

// Call vqsort sort via FFI.
#[cfg(feature = "cpp_vqsort")]
pub mod cpp_vqsort;

// Call intel_avx512 sort via FFI.
#[cfg(feature = "cpp_intel_avx512")]
pub mod cpp_intel_avx512;

// Call singelisort sort via FFI.
#[cfg(feature = "singeli_singelisort")]
pub mod singeli_singelisort;

#[cfg(feature = "evolution")]
pub mod sort_evolution;

#[cfg(feature = "small_sort")]
pub mod small_sort;

#[cfg(feature = "partition_point")]
pub mod partition_point;

#[cfg(feature = "partition")]
pub mod partition;

#[cfg(feature = "selection")]
pub mod selection;

#[cfg(feature = "rust_pa")]
pub mod rust_pa;
