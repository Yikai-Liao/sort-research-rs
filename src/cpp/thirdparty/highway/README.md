# Highway / VQSort

Vendored from https://github.com/google/highway at release `1.4.0`,
`2607d3b5b0113992fe84d3848859eae13b3b52c1` (Apache-2.0, see `LICENSE`).

`build.rs` compiles the per-type VQSort translation units for `i32`, `u16`,
`u32`, `u64`, and `uint128_t`, plus the Highway runtime files needed for
dynamic target dispatch. VQSort is unstable and allocates no heap memory; it
uses about 1.2 KiB of stack and a small thread-local random state.

`uint128_t` is a complete 128-bit key. It is not `K64V64` (a 64-bit key with
a 64-bit payload) and it is not the repository's 128-byte `Record128`.
