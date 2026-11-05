# IPS²Ra

Vendored from https://github.com/ips4o/ips2ra at
`83f6b76ca308b277b2acc3a36426a3baa3c9cb73` (BSD-2-Clause, see `LICENSE`).

Only the sequential `ips2ra::sort(begin, end, extractor)` entry point is used.
IPS²Ra is unstable. Its default configuration allocates 256 buckets with a
block buffer of about 2 KiB per bucket (about 512 KiB fixed), plus swap buffers
and task metadata, so it is a low-extra-space radix sort, not an in-place sort.

## Local modifications

`scheduler.hpp` and `memory.hpp` guard the TBB include and the global queue
with `IPS2RA_NO_TBB`. The sequential sorter constructs `Scheduler` but never
calls into its queue; the guard lets `build.rs` build the serial configuration
without a system TBB. The parallel entry point still requires TBB.

## Supported adapter types

`i32`, `u32`, `u64`, and `Record16`/`Record128` keyed by their leading
`u64`. `u128` is intentionally not exposed: upstream's sampling path calls
`clz`/`ctz` helpers that have no 128-bit overload.
