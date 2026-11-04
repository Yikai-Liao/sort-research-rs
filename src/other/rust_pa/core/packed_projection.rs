//! Highest-varying-bit coordinates with a conservative original-key remainder.
use super::{Classifier, NativeKey};
use std::marker::PhantomData;

// hakmem 0.2 has no architecture-neutral query for hardware PEXT. Keep this
// compatibility boundary here: X86V3 is the library's checked BMI2-capable
// token. Other targets have no token, and PA retains its ordinary classifier.
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "rust_pa_portable")
))]
mod hardware {
    pub type Isa = hakmem::isa::X86V3;
    #[inline]
    pub fn detect() -> Option<Isa> {
        Isa::detect()
    }
}
#[cfg(not(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "rust_pa_portable")
)))]
mod hardware {
    // Signature only; detect never constructs a software extraction token.
    pub type Isa = hakmem::isa::Native;
    #[inline]
    pub fn detect() -> Option<Isa> {
        None
    }
}
pub(super) use hardware::{detect as hardware_gather, Isa as GatherIsa};

pub(super) struct Selection<U> {
    mask: U,
    pub bits: usize,
    pub remaining: usize,
}

impl<U: NativeKey> Selection<U> {
    pub(super) fn new(variation: U, cap: usize, budget: usize) -> Option<Self> {
        let variation = variation & U::mask(cap);
        let q = variation.count_ones() as usize;
        let width = U::BITS - variation.leading_zeros() as usize;
        if q == 0 || q == width || budget == 0 {
            return None;
        }
        let raw = variation.to_u128();
        let low = width.saturating_sub(budget);
        let prefix = if low == 0 {
            raw
        } else {
            raw & !((1u128 << low) - 1)
        };
        let active_bytes = |mask: u128| {
            (0..U::BITS / 8)
                .filter(|&byte| (mask >> (byte * 8)) & 255 != 0)
                .count()
        };
        // A complete tag must not add a whole-record pass over the ordinary
        // first node. Otherwise already-ordered children could make the extra
        // passes pure regression.
        let prefix_passes = active_bytes(prefix >> low);
        let complete = q <= budget && q <= 32 && (q + 7) / 8 <= prefix_passes;
        // Partial compression keeps exactly the ordinary prefix's partition.
        // Spending holes on lower bits changes fanout and can turn ordered
        // large runs into tens of thousands of costly small leaves.
        let selected = if complete { raw } else { prefix };
        let bits = selected.count_ones() as usize;
        if bits == 0 || bits > 32 {
            return None;
        }
        // If the old prefix already covers the whole variation, a same-pass
        // tag only changes representation. Count its shifted label bytes,
        // not the original key's byte boundaries. Partial tags also need an
        // actual pass reduction because they preserve the old child geometry.
        if (!complete || prefix == raw) && (bits + 7) / 8 >= prefix_passes {
            return None;
        }
        Some(Self {
            mask: U::from_u128(selected),
            bits,
            remaining: if selected == raw {
                0
            } else {
                selected.trailing_zeros() as usize
            },
        })
    }

    pub(super) fn conservative(variation: U, cap: usize, budget: usize) -> Option<Self> {
        let selection = Self::new(variation, cap, budget)?;
        let raw = variation & U::mask(cap);
        let baseline = if selection.remaining == 0 {
            raw
        } else {
            raw >> (U::BITS - raw.leading_zeros() as usize).saturating_sub(budget)
        };
        let original_passes = (0..U::BITS / 8)
            .filter(|&byte| baseline.byte(byte) != 0)
            .count();
        // One removed pass did not amortize denser bucket writes and gathering
        // in the independent float cohorts. Require two saved physical passes;
        // full labels compare against the entire original key remainder.
        if (selection.bits + 7) / 8 + 1 >= original_passes {
            return None;
        }
        Some(selection)
    }

    // Preserve the earlier over-partitioning policy for diagnostic controls.
    pub(super) fn aggressive(variation: U, cap: usize, budget: usize) -> Option<Self> {
        let variation = variation & U::mask(cap);
        let q = variation.count_ones() as usize;
        let width = U::BITS - variation.leading_zeros() as usize;
        // Dense coordinates cannot remove a hole or improve the frontier.
        if q == 0 || q == width {
            return None;
        }
        let bits = q.min(budget).min(32);
        if bits == 0 {
            return None;
        }
        let raw = variation.to_u128();
        let mut selected = 0u128;
        let mut count = 0;
        for bit in (0..width).rev() {
            if raw & (1u128 << bit) != 0 {
                selected |= 1u128 << bit;
                count += 1;
                if count == bits {
                    break;
                }
            }
        }
        let active_bytes = (0..U::BITS / 8)
            .filter(|&byte| (selected >> (8 * byte)) & 255 != 0)
            .count();
        if active_bytes <= (bits + 7) / 8 {
            return None;
        }
        Some(Self {
            mask: U::from_u128(selected),
            bits,
            remaining: if bits == q {
                0
            } else {
                selected.trailing_zeros() as usize
            },
        })
    }
}

pub(super) struct Packed<U, L, I = hakmem::isa::Native, const CONTEXT: bool = false> {
    mask: U,
    bits: usize,
    remaining: usize,
    cpu: I,
    _label: PhantomData<L>,
}

impl<U: NativeKey, L: NativeKey, I: hakmem::isa::Isa, const CONTEXT: bool>
    Packed<U, L, I, CONTEXT>
{
    pub(super) fn with_isa(selection: Selection<U>, cpu: I) -> Self {
        assert!(selection.bits <= L::BITS && selection.bits <= 32);
        Self {
            mask: selection.mask,
            bits: selection.bits,
            remaining: selection.remaining,
            cpu,
            _label: PhantomData,
        }
    }

    #[inline(always)]
    fn gather(&self, key: U) -> u32 {
        key.pack_bits(self.mask, self.cpu)
    }

    pub(super) fn variation(&self) -> L {
        L::from_u128((1u128 << self.bits) - 1)
    }
}

#[cfg(test)]
impl<U: NativeKey, L: NativeKey> Packed<U, L> {
    fn new(selection: Selection<U>) -> Self {
        Self::with_isa(selection, hakmem::isa::Native)
    }
}

impl<U: NativeKey, L: NativeKey, I: hakmem::isa::Isa, const CONTEXT: bool> Classifier<U>
    for Packed<U, L, I, CONTEXT>
{
    type Label = L;
    const USE_COMPACT_RADIX: bool = true;
    #[inline]
    fn label(&self, key: U) -> L {
        L::from_u128(self.gather(key) as u128)
    }
    fn remaining(&self, _: U) -> usize {
        self.remaining
    }
    #[inline(always)]
    fn in_context<R>(&self, kernel: impl FnOnce() -> R) -> R {
        if CONTEXT {
            self.cpu.run(|_| kernel())
        } else {
            kernel()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_and_partial_projection_follow_original_key_coordinates() {
        let mut state = 42u128;
        for q in [
            1usize, 7, 8, 9, 15, 16, 17, 23, 24, 25, 31, 32, 33, 63, 64, 65, 127, 128,
        ] {
            let positions: Vec<_> = (0..q)
                .map(|j| if q == 1 { 127 } else { j * 127 / (q - 1) })
                .collect();
            let mask = positions.iter().fold(0, |mask, &p| mask | (1u128 << p));
            for budget in [8usize, 16, 24, 32] {
                let Some(selection) = Selection::new(mask, 128, budget) else {
                    continue;
                };
                let chosen = &positions[q - selection.bits..];
                assert_eq!(
                    selection.remaining,
                    if selection.bits == q { 0 } else { chosen[0] }
                );
                let projection = Packed::<_, u32>::new(selection);
                let mut keys = Vec::new();
                for _ in 0..1024 {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    let key = state & mask;
                    let expected = chosen
                        .iter()
                        .enumerate()
                        .fold(0, |x, (j, &p)| x | ((((key >> p) & 1) as u32) << j));
                    assert_eq!(projection.label(key), expected, "q={q} budget={budget}");
                    assert_eq!(projection.label(key | !mask), expected);
                    keys.push(key);
                }
                keys.sort();
                for pair in keys.windows(2) {
                    assert!(projection.label(pair[0]) <= projection.label(pair[1]));
                    if projection.label(pair[0]) == projection.label(pair[1]) {
                        assert_eq!(
                            pair[0] >> projection.remaining,
                            pair[1] >> projection.remaining
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn portable_narrow_gather_and_exhaustive_byte_subsets() {
        for mask in 0u16..=u16::MAX {
            if mask.count_ones() > 8 {
                continue;
            }
            let projection = Packed::<_, u8>::new(Selection {
                mask,
                bits: mask.count_ones() as usize,
                remaining: 0,
            });
            let positions: Vec<_> = (0..16).filter(|bit| mask & (1 << bit) != 0).collect();
            for label in 0..1usize << positions.len() {
                let key = positions.iter().enumerate().fold(0u16, |key, (j, &p)| {
                    key | ((((label >> j) & 1) as u16) << p)
                });
                assert_eq!(projection.label(key), label as u8);
            }
        }
    }
}

#[cfg(test)]
mod bounded_tests {
    use super::*;
    #[test]
    fn selection_requires_useful_original_label_work() {
        let scalar = Selection::<u32>::new((1 << 18) | 1, 32, 24);
        assert_eq!(scalar.unwrap().bits, 2);
        let mask = (0..32).fold(0u128, |v, j| v | (1u128 << (j * 127 / 31)));
        let partial = Selection::new(mask, 128, 24);
        assert!(partial.unwrap().remaining > 0);
        let wide = [0, 18, 36, 54, 72, 90, 108, 126]
            .iter()
            .fold(0u128, |mask, bit| mask | (1u128 << bit));
        let full = Selection::new(wide, 128, 24).unwrap();
        assert_eq!(full.remaining, 0);
        assert_eq!(full.bits, 8);
        assert!(Selection::new(0xffu128 << 60, 128, 24).is_none());
    }
    #[test]
    fn partial_tags_keep_the_old_partition_and_never_add_a_root_pass() {
        assert!(Selection::<u32>::aggressive(0xff01ffff, 32, 24).is_some());
        assert!(Selection::<u32>::new(0xff01ffff, 32, 24).is_none());
        // A complete three-byte tag must not replace a one-byte root whose
        // already-ordered suffix could otherwise return without moving again.
        assert!(Selection::<u128>::new((0xfu128 << 124) | ((1 << 20) - 1), 128, 24).is_none());
        let mut state = 42u128;
        for budget in [8usize, 16, 24, 32] {
            for _ in 0..10000 {
                state = state.wrapping_mul(0xda942042e4dd58b5).wrapping_add(1);
                let variation = state ^ (state >> 17);
                let Some(s) = Selection::<u128>::new(variation, 128, budget) else {
                    continue;
                };
                let width = 128 - variation.leading_zeros() as usize;
                let low = width.saturating_sub(budget);
                let prefix = if low == 0 {
                    variation
                } else {
                    variation & !((1u128 << low) - 1)
                };
                let passes = (0..16)
                    .filter(|&b| ((prefix >> low) >> (8 * b)) & 255 != 0)
                    .count();
                assert!((s.bits + 7) / 8 <= passes);
                if s.remaining > 0 {
                    assert_eq!(s.mask, prefix);
                } else {
                    assert_eq!(s.mask, variation);
                }
            }
        }
    }
}

#[cfg(test)]
mod dependency_tests {
    use super::*;
    use hakmem::isa::Isa;

    fn check<I: Isa>(cpu: I) {
        let masks = [
            0u128,
            1,
            1 << 127,
            (1 << 63) | (1 << 64),
            u32::MAX as u128,
            (u32::MAX as u128) << 96,
            (0..24).fold(0, |m, j| m | (1 << (j * 127 / 23))),
        ];
        let mut state = 42u128;
        for mask in masks {
            let projection = Packed::<_, u32, _>::with_isa(
                Selection {
                    mask,
                    bits: mask.count_ones() as usize,
                    remaining: 0,
                },
                cpu,
            );
            for _ in 0..1024 {
                state = state.wrapping_mul(0xda942042e4dd58b5).wrapping_add(1);
                let mut expected = 0u32;
                let mut rank = 0;
                for bit in 0..128 {
                    if mask & (1 << bit) != 0 {
                        expected |= (((state >> bit) & 1) as u32) << rank;
                        rank += 1;
                    }
                }
                assert_eq!(
                    projection.label(state),
                    expected,
                    "backend={cpu:?} mask={mask:x}"
                );
                assert_eq!(projection.label(state | !mask), expected);
            }
        }
    }

    #[test]
    fn library_dispatch_preserves_projection_across_every_available_backend() {
        for level in hakmem::isa::available() {
            hakmem::dispatch!(level, |cpu| check(cpu));
        }
    }
}

#[cfg(test)]
mod conservative_tests {
    use super::*;
    #[test]
    fn admission_reserves_more_than_one_removed_record_pass() {
        let mask =
            |q: usize, high: usize| (0..q).fold(0u128, |m, b| m | (1 << (b * high / (q - 1))));
        let short = mask(16, 22);
        assert!(Selection::new(short, 128, 24).is_some());
        assert!(Selection::conservative(short, 128, 24).is_none());
        for variation in [mask(16, 51), mask(16, 127), mask(8, 31)] {
            let selection = Selection::conservative(variation, 128, 24).unwrap();
            assert_eq!(selection.remaining, 0);
            assert_eq!(selection.bits, variation.count_ones() as usize);
        }
        assert!(Selection::conservative(0xffffu128, 128, 24).is_none());
    }
}
