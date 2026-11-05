# IPS⁴o

Vendored upstream: https://github.com/ips4o/ips4o (see the license headers in
the sources).

The adapter uses the sequential `ips4o::sort(begin, end)` entry point. IPS⁴o
is unstable and is the sample-sort reference in the radix experiments.

## Local modification

`scheduler.hpp` and `memory.hpp` guard the TBB include and the global queue
with `IPS4O_NO_TBB`, mirroring the IPS²Ra integration. The sequential sorter
constructs `Scheduler` but never calls its queue, so `build.rs` can build the
serial configuration without system TBB. The parallel entry point still
requires TBB.
