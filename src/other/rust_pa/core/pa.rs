//! Independent Pa ablations. Policies change node preparation or its classifier;
//! all policies use the same distribution workspace and recursive ownership.
use super::prefix_band::PrefixBand;
use super::*;

#[derive(Clone, Copy, Debug)]
pub enum PaExperiment {
    Reference,
    Terminal,
    Ordered,
    Lean,
    Skew,
    SkewThreeWay,
    Both,
    BothThreeWay,
    CoreThreeWay,
    ProbeThreeWay,
    PrefixBand,
    PrefixBandChild,
    TerminalStableLeaf,
    PrefixBandStableLeaf,
    PrefixBandNested,
    OrderedTiled,
    TerminalLocal,
    PrefixBandLocal,
    Unified,
    PrefixBandCached,
    UnifiedCached,
    UnifiedSampled,
    UnifiedReduced,
    UnifiedCandidate,
    UnifiedReuse,
    UnifiedPrepared,
    UnifiedGroups,
    UnifiedIdentity,
    UnifiedPacked,
    UnifiedPackedContext,
    UnifiedPackedAggressive,
    UnifiedStrict,
}

#[derive(Clone, Copy, PartialEq)]
enum SparseCompletion {
    Recursive,
    Identity,
    Packed,
    PackedContext,
    PackedAggressive,
    Conservative,
}

#[derive(Clone, Copy)]
struct Policy {
    terminal: bool,
    ordered: bool,
    skew: bool,
    three_way: bool,
    probe: bool,
    band: bool,
    band_root: bool,
    nested_band: bool,
    stable_leaf: bool,
    tiled_order: bool,
    local_distribution: bool,
    cached_distribution: bool,
    sampled_distribution: bool,
    reduced_radix: bool,
    prepared_distribution: bool,
    grouped_scatter: bool,
    local_workspace_threshold: usize,
    block: usize,
    sparse: SparseCompletion,
}

pub fn sort_pa_experiment<T: Radixable<K>, K: RadixKey>(
    arr: &mut [T],
    experiment: PaExperiment,
    block: usize,
) -> Work
where
    Converted<T, K>: NativeKey,
{
    // Existing experiment callers retain the initial screen's parameter.
    sort_pa_with_threshold(arr, experiment, block, 1024 * 1024)
}

pub fn sort_pa_with_threshold<T: Radixable<K>, K: RadixKey>(
    arr: &mut [T],
    experiment: PaExperiment,
    block: usize,
    local_workspace_threshold: usize,
) -> Work
where
    Converted<T, K>: NativeKey,
{
    assert!(block > 0);
    let sparse = match experiment {
        PaExperiment::UnifiedStrict => SparseCompletion::Conservative,
        PaExperiment::UnifiedPacked => SparseCompletion::Packed,
        PaExperiment::UnifiedPackedContext => SparseCompletion::PackedContext,
        PaExperiment::UnifiedPackedAggressive => SparseCompletion::PackedAggressive,
        PaExperiment::UnifiedIdentity => SparseCompletion::Identity,
        _ => SparseCompletion::Recursive,
    };
    // Independent ablations inherit the complete frozen UnifiedGroups policy.
    let experiment = match experiment {
        PaExperiment::UnifiedIdentity
        | PaExperiment::UnifiedPacked
        | PaExperiment::UnifiedPackedContext
        | PaExperiment::UnifiedPackedAggressive
        | PaExperiment::UnifiedStrict => PaExperiment::UnifiedGroups,
        other => other,
    };
    let policy = Policy {
        sparse,
        terminal: matches!(
            experiment,
            PaExperiment::Terminal
                | PaExperiment::Lean
                | PaExperiment::Both
                | PaExperiment::BothThreeWay
                | PaExperiment::CoreThreeWay
                | PaExperiment::ProbeThreeWay
                | PaExperiment::PrefixBand
                | PaExperiment::PrefixBandChild
                | PaExperiment::TerminalStableLeaf
                | PaExperiment::PrefixBandStableLeaf
                | PaExperiment::PrefixBandNested
                | PaExperiment::TerminalLocal
                | PaExperiment::PrefixBandLocal
                | PaExperiment::PrefixBandCached
                | PaExperiment::Unified
                | PaExperiment::UnifiedCached
                | PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
                | PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedReduced
                | PaExperiment::UnifiedCandidate
        ),
        ordered: matches!(
            experiment,
            PaExperiment::Ordered
                | PaExperiment::Lean
                | PaExperiment::Both
                | PaExperiment::BothThreeWay
                | PaExperiment::OrderedTiled
                | PaExperiment::Unified
                | PaExperiment::UnifiedCached
                | PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
                | PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedReduced
                | PaExperiment::UnifiedCandidate
        ),
        skew: matches!(
            experiment,
            PaExperiment::Skew
                | PaExperiment::Both
                | PaExperiment::SkewThreeWay
                | PaExperiment::BothThreeWay
                | PaExperiment::CoreThreeWay
                | PaExperiment::ProbeThreeWay
        ),
        three_way: matches!(
            experiment,
            PaExperiment::SkewThreeWay
                | PaExperiment::BothThreeWay
                | PaExperiment::CoreThreeWay
                | PaExperiment::ProbeThreeWay
        ),
        probe: matches!(experiment, PaExperiment::ProbeThreeWay),
        band: matches!(
            experiment,
            PaExperiment::PrefixBand
                | PaExperiment::PrefixBandChild
                | PaExperiment::PrefixBandStableLeaf
                | PaExperiment::PrefixBandNested
                | PaExperiment::PrefixBandLocal
                | PaExperiment::PrefixBandCached
                | PaExperiment::Unified
                | PaExperiment::UnifiedCached
                | PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
                | PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedReduced
                | PaExperiment::UnifiedCandidate
        ),
        band_root: matches!(
            experiment,
            PaExperiment::PrefixBand
                | PaExperiment::PrefixBandStableLeaf
                | PaExperiment::PrefixBandNested
                | PaExperiment::PrefixBandLocal
                | PaExperiment::PrefixBandCached
                | PaExperiment::Unified
                | PaExperiment::UnifiedCached
                | PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
                | PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedReduced
                | PaExperiment::UnifiedCandidate
        ),
        nested_band: matches!(
            experiment,
            PaExperiment::PrefixBandNested
                | PaExperiment::Unified
                | PaExperiment::UnifiedCached
                | PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
                | PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedReduced
                | PaExperiment::UnifiedCandidate
        ),
        stable_leaf: matches!(
            experiment,
            PaExperiment::TerminalStableLeaf
                | PaExperiment::PrefixBandStableLeaf
                | PaExperiment::Unified
                | PaExperiment::UnifiedCached
                | PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
                | PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedReduced
                | PaExperiment::UnifiedCandidate
        ),
        tiled_order: matches!(
            experiment,
            PaExperiment::OrderedTiled
                | PaExperiment::Unified
                | PaExperiment::UnifiedCached
                | PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
                | PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedReduced
                | PaExperiment::UnifiedCandidate
        ),
        local_distribution: matches!(
            experiment,
            PaExperiment::TerminalLocal
                | PaExperiment::PrefixBandLocal
                | PaExperiment::PrefixBandCached
                | PaExperiment::Unified
                | PaExperiment::UnifiedCached
                | PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
                | PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedReduced
                | PaExperiment::UnifiedCandidate
        ),
        cached_distribution: matches!(
            experiment,
            PaExperiment::PrefixBandCached
                | PaExperiment::UnifiedCached
                | PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
                | PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedReduced
                | PaExperiment::UnifiedCandidate
        ),
        sampled_distribution: matches!(
            experiment,
            PaExperiment::UnifiedSampled
                | PaExperiment::UnifiedCandidate
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
        ),
        reduced_radix: matches!(
            experiment,
            PaExperiment::UnifiedReduced | PaExperiment::UnifiedCandidate
        ),
        prepared_distribution: matches!(
            experiment,
            PaExperiment::UnifiedReuse
                | PaExperiment::UnifiedPrepared
                | PaExperiment::UnifiedGroups
        ),
        grouped_scatter: matches!(experiment, PaExperiment::UnifiedGroups),
        local_workspace_threshold,
        block,
    };
    let mut stats = Work::default();
    if arr.len() > 1 && std::mem::size_of::<T>() > 0 {
        visit::<T, K>(arr, arr[0].type_size(), policy, false, &mut stats, 0, 0);
    }
    stats
}

struct Prepared<U> {
    union: U,
    intersection: U,
    ascending: bool,
    descending: bool,
    pivot: U,
    votes: usize,
}

// Specialize scans outside the record loop. Ordinary nodes do not pay the
// candidate comparison/branch; ordered ablation removes order reductions here.
fn prepare<T: Radixable<K>, K: RadixKey, const ORDER: bool, const CANDIDATE: bool>(
    arr: &[T],
) -> Prepared<Converted<T, K>>
where
    Converted<T, K>: NativeKey,
{
    let first = arr[0].into_key_type();
    let mut p = Prepared {
        union: first,
        intersection: first,
        ascending: true,
        descending: true,
        pivot: first,
        votes: 0,
    };
    let mut previous = first;
    for item in arr {
        let key = item.into_key_type();
        p.union |= key;
        p.intersection &= key;
        if ORDER {
            p.ascending &= previous <= key;
            p.descending &= previous >= key;
            previous = key;
        }
        if CANDIDATE {
            if p.votes == 0 {
                p.pivot = key;
                p.votes = 1;
            } else if p.pivot == key {
                p.votes += 1;
            } else {
                p.votes -= 1;
            }
        }
    }
    p
}

fn prepare_band<T: Radixable<K>, K: RadixKey, const ORDER: bool, const INNER: bool>(
    arr: &[T],
    band: &PrefixBand<Converted<T, K>>,
    inner: &PrefixBand<Converted<T, K>>,
) -> (Prepared<Converted<T, K>>, [usize; 2])
where
    Converted<T, K>: NativeKey,
{
    let first = arr[0].into_key_type();
    let mut p = Prepared {
        union: first,
        intersection: first,
        ascending: true,
        descending: true,
        pivot: first,
        votes: 0,
    };
    let mut previous = first;
    let mut populations = [0, 0];
    for item in arr {
        let key = item.into_key_type();
        p.union |= key;
        p.intersection &= key;
        if ORDER {
            p.ascending &= previous <= key;
            p.descending &= previous >= key;
            previous = key;
        }
        populations[0] += usize::from(band.contains(key));
        if INNER {
            populations[1] += usize::from(inner.contains(key));
        }
    }
    (p, populations)
}

// Return as soon as an order violation is seen. Reverse only after validating
// the whole range, and restore equal-key groups only when any equality exists.
fn ordered<T: Radixable<K>, K: RadixKey>(arr: &mut [T], stats: &mut Work) -> bool
where
    Converted<T, K>: NativeKey,
{
    work! { stats.order_records += 1; }
    let first = arr[0].into_key_type();
    let mut different = 1;
    while different < arr.len() && {
        work! { stats.order_records += 1; }
        arr[different].into_key_type() == first
    } {
        different += 1;
    }
    if different == arr.len() {
        return true;
    }
    work! { stats.order_records += 1; }
    let mut previous = arr[different].into_key_type();
    let ascending = first < previous;
    let mut equal = different > 1;
    for item in &arr[different + 1..] {
        work! { stats.order_records += 1; }
        let key = item.into_key_type();
        if (ascending && previous > key) || (!ascending && previous < key) {
            return false;
        }
        equal |= previous == key;
        previous = key;
    }
    if !ascending {
        arr.reverse();
        if equal {
            restore_equal_groups::<T, K>(arr, stats);
        }
    }
    true
}

pub(super) fn restore_equal_groups<T: Radixable<K>, K: RadixKey>(arr: &mut [T], stats: &mut Work)
where
    Converted<T, K>: NativeKey,
{
    let mut start = 0;
    while start < arr.len() {
        work! { stats.order_records += 1; }
        let key = arr[start].into_key_type();
        let mut end = start + 1;
        while end < arr.len() && {
            work! { stats.order_records += 1; }
            arr[end].into_key_type() == key
        } {
            end += 1;
        }
        arr[start..end].reverse();
        start = end;
    }
}

// Only a cost gate: correctness and actual classifier selection still use the
// complete node scan. Monotone samples avoid candidate work on ordered inputs.
fn root_majority_signal<T: Radixable<K>, K: RadixKey>(arr: &[T]) -> bool
where
    Converted<T, K>: NativeKey,
{
    let mut pivot = arr[0].into_key_type();
    let mut previous = pivot;
    let mut votes = 0;
    let mut ascending = true;
    let mut descending = true;
    for i in 0..64 {
        let key = arr[i * (arr.len() - 1) / 63].into_key_type();
        ascending &= previous <= key;
        descending &= previous >= key;
        previous = key;
        if votes == 0 {
            pivot = key;
            votes = 1;
        } else if pivot == key {
            votes += 1;
        } else {
            votes -= 1;
        }
    }
    votes > 32 && !ascending && !descending
}

fn visit<T: Radixable<K>, K: RadixKey>(
    arr: &mut [T],
    cap: usize,
    policy: Policy,
    heavy: bool,
    stats: &mut Work,
    depth: usize,
    path: usize,
) where
    Converted<T, K>: NativeKey,
{
    if arr.len() < 2 || cap == 0 {
        return;
    }
    work! { stats.max_depth = stats.max_depth.max(depth); }
    if arr.len() <= 128 {
        work! { stats.leaves += 1; stats.leaf_records += arr.len(); stats.leaf_lengths[arr.len()] += 1; }
        if policy.stable_leaf {
            arr.sort_by_key(|item| item.into_key_type());
        } else {
            super::super::insertion_sort_key::<T, K>(arr);
        }
        return;
    }
    if policy.ordered {
        let complete = if policy.tiled_order {
            super::ordered_tiled::ordered::<T, K>(arr, stats)
        } else {
            ordered::<T, K>(arr, stats)
        };
        if complete {
            return;
        }
    }
    let bytes = (usize::BITS as usize - (arr.len() - 1).leading_zeros() as usize + 7) / 8;
    let band = if policy.band && (heavy || (policy.band_root && depth == 0)) {
        super::prefix_band::sample::<T, K>(arr, cap, bytes * 8, stats)
    } else {
        None
    };
    let candidate = policy.skew
        && cap > 8
        && (heavy || (policy.probe && depth == 0 && root_majority_signal::<T, K>(arr)));
    let inner = band.as_ref().and_then(|band| {
        if policy.nested_band {
            band.terminal_inner(arr.len())
        } else {
            None
        }
    });
    let mut populations = [0, 0];
    let p = if let Some(ref band) = band {
        let inner_band = inner.as_ref().unwrap_or(band);
        let (p, counts) = match (policy.ordered, inner.is_some()) {
            (false, false) => prepare_band::<T, K, true, false>(arr, band, inner_band),
            (false, true) => prepare_band::<T, K, true, true>(arr, band, inner_band),
            (true, false) => prepare_band::<T, K, false, false>(arr, band, inner_band),
            (true, true) => prepare_band::<T, K, false, true>(arr, band, inner_band),
        };
        populations = counts;
        p
    } else {
        match (policy.ordered, candidate) {
            (false, false) => prepare::<T, K, true, false>(arr),
            (false, true) => prepare::<T, K, true, true>(arr),
            (true, false) => prepare::<T, K, false, false>(arr),
            (true, true) => prepare::<T, K, false, true>(arr),
        }
    };
    work! { stats.prepare_records += arr.len(); if !policy.ordered { stats.order_records += arr.len(); } if candidate || band.is_some() { stats.candidate_records += arr.len(); } }
    if !policy.ordered {
        if p.ascending {
            return;
        }
        if p.descending {
            arr.reverse();
            restore_equal_groups::<T, K>(arr, stats);
            return;
        }
    }
    let width =
        (Converted::<T, K>::BITS - (p.union ^ p.intersection).leading_zeros() as usize).min(cap);
    if width == 0 {
        return;
    }
    let coarse = width.min(8 * bytes);
    work! { stats.nodes += 1; stats.node_records += arr.len(); }
    let use_skew = candidate && p.votes > arr.len() / 2 && width > 8;
    let approved_band = inner
        .and_then(|band| band.approve(width, coarse, populations[1], arr.len()))
        .map(|band| (band, populations[1], true))
        .or_else(|| {
            band.and_then(|band| band.approve(width, coarse, populations[0], arr.len()))
                .map(|band| (band, populations[0], false))
        });
    let packing = if matches!(
        policy.sparse,
        SparseCompletion::Packed
            | SparseCompletion::PackedContext
            | SparseCompletion::PackedAggressive
            | SparseCompletion::Conservative
    ) && approved_band.is_none()
    {
        super::packed_projection::hardware_gather().and_then(|cpu| {
            let selection = if policy.sparse == SparseCompletion::Conservative {
                super::packed_projection::Selection::conservative(
                    p.union ^ p.intersection,
                    cap,
                    8 * bytes,
                )
            } else if policy.sparse == SparseCompletion::PackedAggressive {
                super::packed_projection::Selection::aggressive(
                    p.union ^ p.intersection,
                    cap,
                    8 * bytes,
                )
            } else {
                super::packed_projection::Selection::new(p.union ^ p.intersection, cap, 8 * bytes)
            };
            selection.map(|selection| (selection, cpu))
        })
    } else {
        None
    };
    if let Some((band, population, terminal_band)) = approved_band {
        work! { stats.band_nodes += 1; stats.band_records += population; stats.terminal_band_nodes += usize::from(terminal_band); }
        partition::<T, K, _>(
            arr,
            &band,
            Some(Converted::<T, K>::from_u128(3)),
            policy,
            false,
            stats,
            depth,
            path,
        );
    } else if let Some((selection, cpu)) = packing {
        let bits = selection.bits;
        // Keep the projection classifier and workspace outside the visit frame.
        if bits <= 8 {
            partition_packed::<T, K, u8>(arr, selection, policy, stats, depth, path, cpu);
        } else if bits <= 16 {
            partition_packed::<T, K, u16>(arr, selection, policy, stats, depth, path, cpu);
        } else {
            partition_packed::<T, K, u32>(arr, selection, policy, stats, depth, path, cpu);
        }
    } else if policy.sparse == SparseCompletion::Identity
        && sparse_byte_projection(p.union ^ p.intersection, cap)
    {
        work! { stats.identity_nodes += 1; }
        partition::<T, K, _>(
            arr,
            &Identity,
            Some(p.union ^ p.intersection),
            policy,
            true,
            stats,
            depth,
            path,
        );
    } else if policy.terminal && coarse == width && !(policy.probe && use_skew) {
        partition::<T, K, _>(
            arr,
            &Identity,
            Some(p.union ^ p.intersection),
            policy,
            true,
            stats,
            depth,
            path,
        );
    } else if use_skew {
        // Each surviving BM vote is an occurrence of the final candidate.
        // votes > m/2 certifies a complete-key majority without a verify pass.
        // It deliberately misses some weaker majorities; correctness of the
        // ordered classifier does not depend on the majority certificate.
        if policy.three_way {
            work! { stats.equal_nodes += 1; }
            let classifier = EqualSplit {
                pivot: p.pivot & Converted::<T, K>::mask(width),
                mask: Converted::<T, K>::mask(width),
                width,
            };
            partition::<T, K, _>(
                arr,
                &classifier,
                Some(Converted::<T, K>::from_u128(3)),
                policy,
                false,
                stats,
                depth,
                path,
            );
        } else {
            work! { stats.frontier_nodes += 1; }
            let classifier = Frontier::new(p.pivot, width, 7);
            partition::<T, K, _>(arr, &classifier, None, policy, false, stats, depth, path);
        }
    } else {
        let classifier = Prefix {
            low: width - coarse,
            mask: Converted::<T, K>::mask(coarse),
        };
        partition::<T, K, _>(
            arr,
            &classifier,
            Some(((p.union ^ p.intersection) >> classifier.low) & classifier.mask),
            policy,
            false,
            stats,
            depth,
            path,
        );
    }
}

// Only cross-byte masks can save a radix pass. Low population count bounds
// the full label at one byte; it is not an estimate of Shannon entropy.
fn sparse_byte_projection<U: NativeKey>(variation: U, cap: usize) -> bool {
    let variation = variation & U::mask(cap);
    variation.count_ones() <= 8
        && (0..U::BITS / 8)
            .filter(|&byte| variation.byte(byte) != 0)
            .take(2)
            .count()
            == 2
}

// A majority certificate guarantees each unresolved side is smaller than half
// this node. Side intervals need not share a new binary prefix, so preserve the
// conservative bit cap; the equal group is complete and never recursed into.
struct EqualSplit<U> {
    pivot: U,
    mask: U,
    width: usize,
}
impl<U: NativeKey> Classifier<U> for EqualSplit<U> {
    type Label = U;
    fn label(&self, key: U) -> U {
        U::from_u128(match (key & self.mask).cmp(&self.pivot) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Greater => 2,
        })
    }
    fn remaining(&self, key: U) -> usize {
        if key & self.mask == self.pivot {
            0
        } else {
            self.width
        }
    }
}

fn partition<T: Radixable<K>, K: RadixKey, C: Classifier<Converted<T, K>>>(
    arr: &mut [T],
    classifier: &C,
    label_mask: Option<C::Label>,
    policy: Policy,
    terminal: bool,
    stats: &mut Work,
    depth: usize,
    path: usize,
) where
    Converted<T, K>: NativeKey,
{
    #[cfg(feature = "rust_pa_work")]
    let label_evals = std::cell::Cell::new(0usize);
    let label = super::labeler::Classified::<T, K, C> {
        classifier,
        marker: std::marker::PhantomData,
        #[cfg(feature = "rust_pa_work")]
        evaluations: &label_evals,
    };
    // distribute owns and releases all scratch before the first recursive call.
    let prepared_mask = if policy.prepared_distribution {
        label_mask
    } else {
        None
    };
    let passes = if let Some(mask) = prepared_mask {
        if C::USE_COMPACT_RADIX {
            super::local_distribution::distribute_prepared_packed(
                arr,
                &label,
                policy.block,
                policy.local_workspace_threshold,
                mask,
                stats,
            )
        } else if policy.grouped_scatter {
            super::local_distribution::distribute_prepared_grouped(
                arr,
                &label,
                policy.block,
                policy.local_workspace_threshold,
                mask,
                stats,
            )
        } else if policy.sampled_distribution {
            super::local_distribution::distribute_prepared_sampled(
                arr,
                &label,
                policy.block,
                policy.local_workspace_threshold,
                mask,
                stats,
            )
        } else {
            super::local_distribution::distribute_prepared(
                arr,
                &label,
                policy.block,
                policy.local_workspace_threshold,
                mask,
                stats,
            )
        }
    } else if policy.sampled_distribution && policy.reduced_radix {
        super::local_distribution::distribute_sampled_reduced(
            arr,
            &label,
            policy.block,
            policy.local_workspace_threshold,
            stats,
        )
    } else if policy.sampled_distribution {
        super::local_distribution::distribute_sampled(
            arr,
            &label,
            policy.block,
            policy.local_workspace_threshold,
            stats,
        )
    } else if policy.reduced_radix {
        super::local_distribution::distribute_reduced(
            arr,
            &label,
            policy.block,
            policy.local_workspace_threshold,
            stats,
        )
    } else if policy.cached_distribution {
        super::local_distribution::distribute_cached(
            arr,
            &label,
            policy.block,
            policy.local_workspace_threshold,
            stats,
        )
    } else if policy.local_distribution {
        super::local_distribution::distribute_local(arr, &label, policy.block, stats)
    } else {
        distribute(arr, &label, policy.block, stats)
    };
    work! { stats.max_path_passes = stats.max_path_passes.max(path + passes); }
    if !terminal {
        let parent_len = arr.len();
        let mut start = 0;
        while start < arr.len() {
            let (end, remaining) = label.in_context(|| {
                let current = label.classify(&arr[start]);
                let remaining = classifier.remaining(arr[start].into_key_type());
                let mut end = start + 1;
                while end < arr.len() && label.classify(&arr[end]) == current {
                    end += 1;
                }
                (end, remaining)
            });
            work! { stats.runs_records += end - start; }
            if end - start > 1 && remaining > 0 {
                work! { stats.unresolved_records += end - start; }
                visit::<T, K>(
                    &mut arr[start..end],
                    remaining,
                    policy,
                    end - start > parent_len / 2,
                    stats,
                    depth + 1,
                    path + passes,
                );
            }
            start = end;
        }
    }
    work! { stats.label_evals += label_evals.get(); }
}

#[inline(never)]
fn partition_packed<T: Radixable<K>, K: RadixKey, L: NativeKey>(
    arr: &mut [T],
    selection: super::packed_projection::Selection<Converted<T, K>>,
    policy: Policy,
    stats: &mut Work,
    depth: usize,
    path: usize,
    cpu: super::packed_projection::GatherIsa,
) where
    Converted<T, K>: NativeKey,
{
    hakmem::isa::Isa::run(cpu, |cpu| {
        if matches!(
            policy.sparse,
            SparseCompletion::PackedContext | SparseCompletion::Conservative
        ) {
            partition_packed_with_isa::<T, K, L, _, true>(
                arr, selection, policy, stats, depth, path, cpu,
            )
        } else {
            partition_packed_with_isa::<T, K, L, _, false>(
                arr, selection, policy, stats, depth, path, cpu,
            )
        }
    });
}

fn partition_packed_with_isa<
    T: Radixable<K>,
    K: RadixKey,
    L: NativeKey,
    I: hakmem::isa::Isa,
    const CONTEXT: bool,
>(
    arr: &mut [T],
    selection: super::packed_projection::Selection<Converted<T, K>>,
    policy: Policy,
    stats: &mut Work,
    depth: usize,
    path: usize,
    cpu: I,
) where
    Converted<T, K>: NativeKey,
{
    let terminal = selection.remaining == 0;
    let projection = super::packed_projection::Packed::<_, L, _, CONTEXT>::with_isa(selection, cpu);
    work! { stats.projection_nodes += 1; stats.projection_records += arr.len(); }
    partition::<T, K, _>(
        arr,
        &projection,
        Some(projection.variation()),
        policy,
        terminal,
        stats,
        depth,
        path,
    );
}
