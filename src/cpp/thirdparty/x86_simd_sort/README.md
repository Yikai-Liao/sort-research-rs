# x86-simd-sort

Vendored from https://github.com/numpy/x86-simd-sort at
`6e059b14975df2e3c74260ec905167a1be5a34ca` (BSD-3-Clause, see `LICENSE.md`).

The adapter includes `src/x86simdsort-static-incl.h` and is compiled with
`-march=native`, so the build selects the AVX2 or AVX-512 translation. Only
direct integer sorting is used; key-value and object interfaces are not.

## Supported adapter types

`i32`, `u32`, and `u64`. The important correction over the previous vendored
version is that `u64` is passed as `uint64_t*`; the old adapter reinterpreted
it as `int64_t*`, which reverses the order of the upper half of the unsigned
range.
