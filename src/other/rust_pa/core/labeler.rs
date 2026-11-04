//! Keep a record classifier's execution context attached to its hot loops.
use super::{Classifier, Converted, NativeKey};
use crate::other::rust_pa::{RadixKey, Radixable};
use std::marker::PhantomData;

pub(super) trait Labeler<T, U> {
    fn classify(&self, record: &T) -> U;

    #[inline(always)]
    fn in_context<R>(&self, kernel: impl FnOnce() -> R) -> R {
        kernel()
    }
}

// Existing distribution callers keep their ordinary closure interface.
impl<T, U, F: Fn(&T) -> U> Labeler<T, U> for F {
    #[inline(always)]
    fn classify(&self, record: &T) -> U {
        self(record)
    }
}

pub(super) struct Classified<'a, T, K, C> {
    pub classifier: &'a C,
    pub marker: PhantomData<fn(T, K)>,
    #[cfg(feature = "rust_pa_work")]
    pub evaluations: &'a std::cell::Cell<usize>,
}

impl<T: Radixable<K>, K: RadixKey, C: Classifier<Converted<T, K>>> Labeler<T, C::Label>
    for Classified<'_, T, K, C>
where
    Converted<T, K>: NativeKey,
{
    #[inline(always)]
    fn classify(&self, record: &T) -> C::Label {
        #[cfg(feature = "rust_pa_work")]
        self.evaluations.set(self.evaluations.get() + 1);
        self.classifier.label(record.into_key_type())
    }

    #[inline(always)]
    fn in_context<R>(&self, kernel: impl FnOnce() -> R) -> R {
        self.classifier.in_context(kernel)
    }
}

pub(super) struct Masked<'a, F, U> {
    pub labeler: &'a F,
    pub mask: U,
}

impl<T, U: NativeKey, F: Labeler<T, U>> Labeler<T, U> for Masked<'_, F, U> {
    #[inline(always)]
    fn classify(&self, record: &T) -> U {
        self.labeler.classify(record) & self.mask
    }

    #[inline(always)]
    fn in_context<R>(&self, kernel: impl FnOnce() -> R) -> R {
        self.labeler.in_context(kernel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct Guarded {
        active: Cell<bool>,
        entries: Cell<usize>,
        mask: u8,
    }

    impl Labeler<u16, u8> for Guarded {
        fn classify(&self, record: &u16) -> u8 {
            assert!(self.active.get(), "kernel lost its classifier context");
            *record as u8 & self.mask
        }

        fn in_context<R>(&self, kernel: impl FnOnce() -> R) -> R {
            self.entries.set(self.entries.get() + 1);
            let previous = self.active.replace(true);
            let result = kernel();
            self.active.set(previous);
            result
        }
    }

    #[test]
    fn dense_and_recycling_kernels_retain_context_through_normalization() {
        for mask in [1, 3] {
            for block in [3, 17] {
                let labeler = Guarded {
                    active: Cell::new(false),
                    entries: Cell::new(0),
                    mask,
                };
                let label = Masked {
                    labeler: &labeler,
                    mask,
                };
                let input: Vec<u16> = (0..257).map(|id| id * 73).collect();
                let mut expected = input.clone();
                expected.sort_by_key(|record| *record as u8 & mask);
                let mut actual = input.clone();
                {
                    let mut sorter =
                        super::super::block::Sorter::new(&mut actual, usize::from(mask) + 1, block);
                    if mask == 1 {
                        sorter.step_grouped(&label, 0, 1);
                    } else {
                        sorter.step(&label, 0);
                    }
                    sorter.compact();
                }
                assert_eq!(actual, expected);
                assert_eq!(labeler.entries.get(), 1);
                assert!(!labeler.active.get());

                let mut actual = input;
                super::super::dense(
                    &mut actual,
                    &label,
                    1,
                    mask,
                    &mut super::super::Work::default(),
                );
                assert_eq!(actual, expected);
                assert_eq!(labeler.entries.get(), 2);
                assert!(!labeler.active.get());
            }
        }
    }
}
