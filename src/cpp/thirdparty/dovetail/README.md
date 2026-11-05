# DovetailSort

Vendored from https://github.com/ucrparlay/DovetailSort at
`f4630a264679143dfc909cd9612ffa39b27bb76e` (MIT, see `LICENSE`).

Built with `PARLAY_SEQUENTIAL`, which is the algorithm's serial configuration:
the main recursion still samples, detects frequently occurring keys, recurses
and merges; one worker only changes how internal work is scheduled.

The adapter calls `parlay::integer_sort_inplace2(slice, get_key)`. Despite the
name, the function allocates a full-length temporary record array and writes the
result back to the input range. DovetailSort is stable and uses O(n) extra space.

## Supported adapter types

`i32`, `u32`, `u64`, and `Record16`/`Record128` keyed by their leading
`u64`. `u128` is not claimed until it has been verified separately.
