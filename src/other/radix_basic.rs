//! Plain stable counting-scatter baselines. They move records without cloning
//! payloads; one uninitialized full-size scratch allocation is reused.
use super::radix_support::{Key, Unsigned};
use std::{mem::MaybeUninit, ptr};
fn digit<T: Key>(v: &T, shift: usize) -> usize {
    ((v.key() >> T::Value::from_usize(shift)) & T::Value::from_usize(255)).to_usize()
}
fn histogram<T: Key>(input: &[T], shift: usize) -> [usize; 256] {
    let mut counts = [0; 256];
    for v in input {
        counts[digit(v, shift)] += 1;
    }
    counts
}
fn offsets(counts: &[usize; 256]) -> [usize; 257] {
    let mut starts = [0; 257];
    for i in 0..256 {
        starts[i + 1] = starts[i] + counts[i];
    }
    starts
}
fn scratch<T>(n: usize) -> Vec<MaybeUninit<T>> {
    let mut out = Vec::with_capacity(n);
    out.resize_with(n, MaybeUninit::uninit);
    out
}
// The caller provides a disjoint allocation of input.len() slots. Source stays
// the complete owning sequence until scatter finishes. Partial output never drops.
fn scatter<T: Key>(input: &[T], output: *mut T, shift: usize, starts: &[usize; 257]) {
    let mut heads = *starts;
    for value in input {
        let d = digit(value, shift);
        assert!(heads[d] < starts[d + 1], "key changed during radix scatter");
        // SAFETY: checked bucket limit selects an allocated destination slot;
        // a typed raw copy preserves payload ownership bits and padding.
        unsafe {
            ptr::copy_nonoverlapping(value, output.add(heads[d]), 1);
        }
        heads[d] += 1;
    }
}
pub(super) fn lsd<T: Key>(input: &mut [T]) {
    if input.len() < 2 || std::mem::size_of::<T>() == 0 {
        return;
    }
    let mut storage = scratch::<T>(input.len());
    let buffer = storage.as_mut_ptr().cast::<T>();
    let original = input.as_mut_ptr();
    let mut in_scratch = false;
    for shift in (0..T::Value::BITS).step_by(8) {
        let (source, destination) = if in_scratch {
            (buffer, original)
        } else {
            (original, buffer)
        };
        // SAFETY: the active source is fully initialized after every committed
        // pass; destination is a separate allocation, with no active owners.
        let source = unsafe { std::slice::from_raw_parts(source, input.len()) };
        // Classification may mutate an owning record through interior
        // mutability, even during histogram construction. Catch the entire
        // pass so the live scratch sequence is always restored on unwinding.
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let counts = histogram(source, shift);
            if counts.iter().any(|&n| n == input.len()) {
                return false;
            }
            scatter(source, destination, shift, &offsets(&counts));
            true
        }));
        let did_scatter = match outcome {
            Ok(done) => done,
            Err(payload) => {
                if in_scratch {
                    // SAFETY: scratch is the complete owning sequence, with
                    // every interior modification made before the panic.
                    unsafe {
                        ptr::copy_nonoverlapping(buffer, original, input.len());
                    }
                }
                std::panic::resume_unwind(payload);
            }
        };
        if !did_scatter {
            continue;
        }
        in_scratch = !in_scratch;
    }
    if in_scratch {
        // SAFETY: the completed sequence lives in scratch; input is stale.
        unsafe {
            ptr::copy_nonoverlapping(buffer, original, input.len());
        }
    }
}
pub(super) fn msd<T: Key>(input: &mut [T]) {
    if input.len() < 2 || std::mem::size_of::<T>() == 0 {
        return;
    }
    let mut storage = scratch::<T>(input.len());
    msd_rec(input, &mut storage, T::Value::BITS - 8);
}
fn msd_rec<T: Key>(input: &mut [T], scratch: &mut [MaybeUninit<T>], shift: usize) {
    let counts = histogram(input, shift);
    if counts.iter().any(|&n| n == input.len()) {
        if shift >= 8 {
            msd_rec(input, scratch, shift - 8);
        }
        return;
    }
    let starts = offsets(&counts);
    let output = scratch.as_mut_ptr().cast::<T>();
    // If classification panics, input is unchanged and scratch drops no T.
    scatter(input, output, shift, &starts);
    // SAFETY: complete scatter initialized every scratch slot, preserving the
    // full permutation. Move it back without dropping stale input copies.
    unsafe {
        ptr::copy_nonoverlapping(output, input.as_mut_ptr(), input.len());
    }
    if shift >= 8 {
        for i in 0..256 {
            let (start, end) = (starts[i], starts[i + 1]);
            if end - start > 1 {
                msd_rec(&mut input[start..end], &mut scratch[start..end], shift - 8);
            }
        }
    }
}
