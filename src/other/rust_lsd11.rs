//! Stable 11-bit LSD reference: fewer scatter passes, larger histograms.
//! The final partial digit uses only its remaining bits. Constant digits skip
//! scatter. One uninitialized full-record buffer and two bucket tables are used.
use super::radix_support::{Key, Unsigned};
use std::{mem::MaybeUninit, ptr};

fn lsd11<T: Key>(input: &mut [T]) {
    if input.len() < 2 || std::mem::size_of::<T>() == 0 {
        return;
    }
    let max_range = 1usize << 11.min(T::Value::BITS);
    let mut counts = vec![0usize; max_range];
    let mut ends = vec![0usize; max_range];
    let mut scratch = Vec::with_capacity(input.len());
    scratch.resize_with(input.len(), MaybeUninit::<T>::uninit);
    let buffer = scratch.as_mut_ptr().cast::<T>();
    let original = input.as_mut_ptr();
    let mut in_scratch = false;
    for shift in (0..T::Value::BITS).step_by(11) {
        let width = 11.min(T::Value::BITS - shift);
        let range = 1usize << width;
        let mask = T::Value::from_usize(range - 1);
        let digit = |value: &T| ((value.key() >> T::Value::from_usize(shift)) & mask).to_usize();
        let (source, destination) = if in_scratch {
            (buffer, original)
        } else {
            (original, buffer)
        };
        // SAFETY: the active allocation contains the complete owning sequence.
        let source = unsafe { std::slice::from_raw_parts(source, input.len()) };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            counts[..range].fill(0);
            for value in source {
                counts[digit(value)] += 1;
            }
            if counts[..range].iter().any(|&n| n == input.len()) {
                return false;
            }
            let mut offset = 0;
            for b in 0..range {
                let n = counts[b];
                counts[b] = offset;
                offset += n;
                ends[b] = offset;
            }
            for value in source {
                let b = digit(value);
                assert!(counts[b] < ends[b], "key changed during radix scatter");
                // SAFETY: destination is disjoint and has input.len() slots;
                // the checked cursor selects an unused slot in its bucket.
                unsafe {
                    ptr::copy_nonoverlapping(value, destination.add(counts[b]), 1);
                }
                counts[b] += 1;
            }
            true
        }));
        match result {
            Ok(true) => in_scratch = !in_scratch,
            Ok(false) => {}
            Err(payload) => {
                if in_scratch {
                    // SAFETY: preserve the live source and all key-side edits.
                    unsafe {
                        ptr::copy_nonoverlapping(buffer, original, input.len());
                    }
                }
                std::panic::resume_unwind(payload);
            }
        }
    }
    if in_scratch {
        // SAFETY: scratch now owns every initialized record; move them back.
        unsafe {
            ptr::copy_nonoverlapping(buffer, original, input.len());
        }
    }
}
radix_adapter!("rust_lsd11_stable_radix", lsd11);
