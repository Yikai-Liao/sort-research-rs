//! Untimed requested-heap accounting for the existing sort adapters.
//! Input, generation, reference output, and validation are outside the interval.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cmp::Ordering;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};

use rand::{seq::SliceRandom, SeedableRng};
use sort_research_rs::{other, stable, unstable};
use sort_test_tools::{ffi_types::F128, patterns, Sort};

struct CountingAllocator;
static LIVE: AtomicUsize = AtomicUsize::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static BASE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);
static REQUESTED: AtomicUsize = AtomicUsize::new(0);

fn added(bytes: usize) {
    let live = LIVE.fetch_add(bytes, Relaxed) + bytes;
    if ACTIVE.load(Relaxed) {
        PEAK.fetch_max(live.saturating_sub(BASE.load(Relaxed)), Relaxed);
    }
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            added(layout.size());
            if ACTIVE.load(Relaxed) {
                ALLOCS.fetch_add(1, Relaxed);
                REQUESTED.fetch_add(layout.size(), Relaxed);
            }
        }
        ptr
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc_zeroed(layout);
        if !ptr.is_null() {
            added(layout.size());
            if ACTIVE.load(Relaxed) {
                ALLOCS.fetch_add(1, Relaxed);
                REQUESTED.fetch_add(layout.size(), Relaxed);
            }
        }
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Relaxed);
        System.dealloc(ptr, layout);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let result = System.realloc(ptr, layout, size);
        if !result.is_null() {
            if size >= layout.size() {
                added(size - layout.size());
            } else {
                LIVE.fetch_sub(layout.size() - size, Relaxed);
            }
            if ACTIVE.load(Relaxed) {
                REALLOCS.fetch_add(1, Relaxed);
                REQUESTED.fetch_add(size, Relaxed);
            }
        }
        result
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

struct Measurement {
    peak: usize,
    allocs: usize,
    reallocs: usize,
    requested: usize,
    retained: usize,
}

fn measure<T: Ord>(input: &mut [T], sort: impl FnOnce(&mut [T])) -> Measurement {
    // This executable has one thread and no Criterion runtime. LIVE includes
    // earlier allocations, so subtract the exact pre-call baseline rather than
    // resetting a counter while allocations from outside the interval exist.
    BASE.store(LIVE.load(Relaxed), Relaxed);
    PEAK.store(0, Relaxed);
    ALLOCS.store(0, Relaxed);
    REALLOCS.store(0, Relaxed);
    REQUESTED.store(0, Relaxed);
    ACTIVE.store(true, Relaxed);
    sort(std::hint::black_box(input));
    ACTIVE.store(false, Relaxed);
    Measurement {
        peak: PEAK.load(Relaxed),
        allocs: ALLOCS.load(Relaxed),
        reallocs: REALLOCS.load(Relaxed),
        requested: REQUESTED.load(Relaxed),
        retained: LIVE
            .load(Relaxed)
            .checked_sub(BASE.load(Relaxed))
            .expect("sort freed an allocation that predates measurement"),
    }
}

fn check_accounting() {
    // Validate exclusion of a live input-like allocation and realloc accounting.
    let mut outside = Vec::<u8>::with_capacity(4096);
    std::hint::black_box(&mut outside);
    BASE.store(LIVE.load(Relaxed), Relaxed);
    PEAK.store(0, Relaxed);
    ACTIVE.store(true, Relaxed);
    let mut inside = Vec::<u8>::with_capacity(1024);
    inside.resize(1024, 1);
    inside.reserve_exact(1024);
    std::hint::black_box(&inside);
    let observed = PEAK.load(Relaxed);
    drop(inside);
    ACTIVE.store(false, Relaxed);
    assert_eq!(
        observed, 2048,
        "allocator accounting includes only the new buffer"
    );
    assert_eq!(LIVE.load(Relaxed), BASE.load(Relaxed));
    drop(outside);
}

fn run_sort<T: Ord + Clone, S: Sort>(
    name: &str,
    pattern: &str,
    original: &[T],
    expected: &[T],
    full_eq: fn(&T, &T) -> bool,
) {
    let mut data = original.to_vec(); // Before the measurement interval.
    let bytes = std::mem::size_of::<T>();
    let result = measure(&mut data, S::sort);
    assert_eq!(result.retained, 0, "sort retained heap allocations");
    assert!(
        data.iter().zip(expected).all(|(a, b)| full_eq(a, b)),
        "full output mismatch: {name}/{pattern}"
    );
    #[cfg(feature = "rust_pa_work")]
    if S::name() == "rust_pa_unified_groups_stable_radix" {
        let mut diagnostic = original.to_vec();
        let w = other::rust_pa::diagnose(&mut diagnostic);
        eprintln!("PA_WORK,{name},{pattern},{},nodes={},dense_nodes={},block_nodes={},passes={},scatter_records={},leaves={},leaf_records={},max_depth={},max_path_passes={}",
            original.len(), w.nodes, w.dense_nodes, w.block_nodes, w.passes,
            w.scatter_records, w.leaves, w.leaf_records, w.max_depth, w.max_path_passes);
    }
    println!(
        "{},{},{},{},{},{},{},{},{},{},{}",
        S::name(),
        name,
        pattern,
        data.len(),
        bytes,
        data.len() * bytes,
        result.peak,
        result.allocs,
        result.reallocs,
        result.requested,
        result.retained
    );
}

fn run_case<T: Ord + Clone>(
    name: &str,
    pattern: &str,
    input: Vec<T>,
    full_eq: fn(&T, &T) -> bool,
    radsort: bool,
) {
    let mut expected = input.clone();
    expected.sort(); // Reference allocation and sorting are outside all intervals.
    run_sort::<T, stable::rust_std::SortImpl>(name, pattern, &input, &expected, full_eq);
    run_sort::<T, unstable::rust_ipnsort::SortImpl>(name, pattern, &input, &expected, full_eq);
    run_sort::<T, other::rust_block_lsd::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_pa")]
    if other::rust_pa::supports_type::<T>() {
        run_sort::<T, other::rust_pa::SortImpl>(name, pattern, &input, &expected, full_eq);
    }
    #[cfg(feature = "rust_lsd11")]
    run_sort::<T, other::rust_lsd11::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_radsort_cached")]
    run_sort::<T, other::rust_radsort_cached::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_radsort_unopt")]
    run_sort::<T, other::rust_radsort_unopt::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_basic_lsd")]
    run_sort::<T, other::rust_basic_lsd::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_basic_msd")]
    run_sort::<T, other::rust_basic_msd::SortImpl>(name, pattern, &input, &expected, full_eq);
    if radsort {
        run_sort::<T, other::rust_radsort::SortImpl>(name, pattern, &input, &expected, full_eq);
    }
}

fn input(pattern: &str, len: usize) -> Vec<i32> {
    match pattern {
        "random" => patterns::random(len),
        "random_z1" => patterns::random_zipf(len, 1.0),
        "random_d20" => patterns::random_uniform(len, 0..20),
        "random_s95" => patterns::random_sorted(len, 95.0),
        "ascending" => patterns::ascending(len),
        "descending" => patterns::descending(len),
        "all_equal" => patterns::all_equal(len),
        "random_p5" => {
            // Match benches/bench.rs: 95% zeros, 5% random, shuffled.
            let zeros = ((len as f64 / 100.0) * 95.0).round() as usize;
            let mut values: Vec<_> = std::iter::repeat(0)
                .take(zeros)
                .chain(patterns::random(len - zeros))
                .collect();
            values.shuffle(&mut rand::rngs::StdRng::seed_from_u64(
                patterns::random_init_seed(),
            ));
            values
        }
        _ => panic!("unsupported pattern: {pattern}"),
    }
}

// Match bench.rs::compress_i32(..., 65536): preserve order and duplicate
// classes while scaling the observed i32 range across the u16 key space.
fn compress_u16(values: &[i32]) -> Vec<u16> {
    let shift = |v: i32| (v as i64 + i32::MAX as i64 + 1) as u32;
    let Some(min) = values.iter().copied().map(shift).min() else {
        return Vec::new();
    };
    let max = values.iter().copied().map(shift).max().unwrap();
    if min == max {
        return vec![0; values.len()];
    }
    let scale = u16::MAX as f64 / (max - min) as f64;
    values
        .iter()
        .map(|&v| (((shift(v) - min) as f64 * scale).round() as u32) as u16)
        .collect()
}

fn extend(value: i32) -> u64 {
    // Same transform as the timing harness; includes all 32 original key bits.
    ((value as i64 + i32::MAX as i64 + 1) as u64) * i32::MAX as u64
}

#[repr(C)]
#[derive(Clone)]
struct Record<const N: usize> {
    key: u64,
    identity: u64,
    payload: [u8; N],
}
impl<const N: usize> PartialEq for Record<N> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<const N: usize> Eq for Record<N> {}
impl<const N: usize> PartialOrd for Record<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<const N: usize> Ord for Record<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
impl<const N: usize> other::rust_block_lsd::Key for Record<N> {
    type Value = u64;
    fn key(&self) -> u64 {
        self.key
    }
}

fn records<const N: usize>(values: &[i32]) -> Vec<Record<N>> {
    values
        .iter()
        .enumerate()
        .map(|(i, v)| Record {
            key: extend(*v),
            identity: i as u64,
            payload: [i as u8; N],
        })
        .collect()
}

fn run_copy_records<const N: usize>(name: &str, pattern: &str, values: &[i32]) {
    use sort_test_tools::ffi_types::RadixRecord;
    let input: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(i, &v)| RadixRecord::<N>::new(extend(v), i))
        .collect();
    let mut expected = input.clone();
    expected.sort();
    let full_eq = |a: &RadixRecord<N>, b: &RadixRecord<N>| {
        a.key == b.key && a.identity == b.identity && a.payload == b.payload
    };
    macro_rules! run {
        ($algorithm:path) => {
            run_sort::<_, $algorithm>(name, pattern, &input, &expected, full_eq);
        };
    }
    run!(stable::rust_std::SortImpl);
    run!(other::rust_block_lsd::SortImpl);
    #[cfg(feature = "rust_pa")]
    run!(other::rust_pa::SortImpl);
    #[cfg(feature = "rust_basic_lsd")]
    run!(other::rust_basic_lsd::SortImpl);
    #[cfg(feature = "rust_basic_msd")]
    run!(other::rust_basic_msd::SortImpl);
    #[cfg(feature = "rust_lsd11")]
    run!(other::rust_lsd11::SortImpl);
    #[cfg(feature = "rust_radsort_cached")]
    run!(other::rust_radsort_cached::SortImpl);
    #[cfg(feature = "rust_radsort_unopt")]
    run!(other::rust_radsort_unopt::SortImpl);
}

fn run_records<const N: usize>(name: &str, pattern: &str, values: &[i32]) {
    let input = records::<N>(values);
    let mut expected = input.clone();
    expected.sort();
    let full_eq = |a: &Record<N>, b: &Record<N>| {
        a.key == b.key && a.identity == b.identity && a.payload == b.payload
    };
    run_sort::<_, stable::rust_std::SortImpl>(name, pattern, &input, &expected, full_eq);
    run_sort::<_, other::rust_block_lsd::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_lsd11")]
    run_sort::<_, other::rust_lsd11::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_radsort_cached")]
    run_sort::<_, other::rust_radsort_cached::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_radsort_unopt")]
    run_sort::<_, other::rust_radsort_unopt::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_basic_lsd")]
    run_sort::<_, other::rust_basic_lsd::SortImpl>(name, pattern, &input, &expected, full_eq);
    #[cfg(feature = "rust_basic_msd")]
    run_sort::<_, other::rust_basic_msd::SortImpl>(name, pattern, &input, &expected, full_eq);
    // The existing radsort adapter accepts scalar records only. Its crate also
    // exposes stable sort_by_key, giving an actual full-buffer LSD baseline for
    // these payload records without changing the existing adapter's contract.
    let mut data = input.clone();
    let m = measure(&mut data, |data| radsort::sort_by_key(data, |v| v.key));
    assert_eq!(m.retained, 0);
    assert!(data.iter().zip(&expected).all(|(a, b)| full_eq(a, b)));
    println!(
        "rust_radsort_by_key,{name},{pattern},{},{},{},{},{},{},{},{}",
        data.len(),
        std::mem::size_of::<Record<N>>(),
        data.len() * std::mem::size_of::<Record<N>>(),
        m.peak,
        m.allocs,
        m.reallocs,
        m.requested,
        m.retained
    );
    // An unstable sort has a different ordering guarantee. Validate its full
    // record set by identity after measurement, rather than requiring stability.
    let mut data = input.clone();
    let m = measure(&mut data, unstable::rust_ipnsort::sort);
    assert_eq!(m.retained, 0);
    assert!(data.windows(2).all(|w| w[0].key <= w[1].key));
    data.sort_unstable_by_key(|v| v.identity);
    assert!(data.iter().zip(&input).all(|(a, b)| full_eq(a, b)));
    println!(
        "rust_ipnsort_unstable,{name},{pattern},{},{},{},{},{},{},{},{}",
        data.len(),
        std::mem::size_of::<Record<N>>(),
        data.len() * std::mem::size_of::<Record<N>>(),
        m.peak,
        m.allocs,
        m.reallocs,
        m.requested,
        m.retained
    );
}

static OWNED_DROPS: AtomicUsize = AtomicUsize::new(0);
#[repr(C)]
struct OwnedRecord {
    key: u64,
    identity: usize,
    text: String,
    bytes: Box<[u8]>,
    object: std::rc::Rc<dyn std::fmt::Debug>,
}
impl Drop for OwnedRecord {
    fn drop(&mut self) {
        OWNED_DROPS.fetch_add(1, Relaxed);
    }
}
impl other::rust_block_lsd::Key for OwnedRecord {
    type Value = u64;
    fn key(&self) -> u64 {
        self.key
    }
}
impl PartialEq for OwnedRecord {
    fn eq(&self, b: &Self) -> bool {
        self.key == b.key
    }
}
impl Eq for OwnedRecord {}
impl PartialOrd for OwnedRecord {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}
impl Ord for OwnedRecord {
    fn cmp(&self, b: &Self) -> Ordering {
        self.key.cmp(&b.key)
    }
}

fn owned_sort(
    name: &str,
    pattern: &str,
    values: &[i32],
    sort: impl FnOnce(&mut [OwnedRecord]),
    stable: bool,
) {
    OWNED_DROPS.store(0, Relaxed);
    let root: std::rc::Rc<dyn std::fmt::Debug> = std::rc::Rc::new(17usize);
    // All payload allocation and the expected identity permutation precede the
    // counter interval. The sorter receives a non-Clone, owning record type.
    let mut data: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(identity, &value)| OwnedRecord {
            key: extend(value),
            identity,
            text: format!("payload-{identity}"),
            bytes: vec![identity as u8; 13].into_boxed_slice(),
            object: root.clone(),
        })
        .collect();
    let mut expected: Vec<_> = (0..values.len()).collect();
    expected.sort_by_key(|&i| extend(values[i]));
    let m = measure(&mut data, sort);
    assert_eq!(m.retained, 0);
    assert_eq!(OWNED_DROPS.load(Relaxed), 0);
    assert_eq!(std::rc::Rc::strong_count(&root), values.len() + 1);
    assert!(data.windows(2).all(|w| w[0].key <= w[1].key));
    if stable {
        assert_eq!(
            data.iter().map(|r| r.identity).collect::<Vec<_>>(),
            expected
        );
    }
    data.sort_unstable_by_key(|r| r.identity);
    for (i, r) in data.iter().enumerate() {
        assert_eq!(r.identity, i);
        assert_eq!(r.text, format!("payload-{i}"));
        assert!(r.bytes.iter().all(|&v| v == i as u8));
        assert!(std::rc::Rc::ptr_eq(&r.object, &root));
    }
    let size = std::mem::size_of::<OwnedRecord>();
    println!(
        "{name},owned,{pattern},{},{},{},{},{},{},{},{}",
        values.len(),
        size,
        values.len() * size,
        m.peak,
        m.allocs,
        m.reallocs,
        m.requested,
        m.retained
    );
    drop(data);
    assert_eq!(OWNED_DROPS.load(Relaxed), values.len());
    assert_eq!(std::rc::Rc::strong_count(&root), 1);
}
fn run_owned(pattern: &str, values: &[i32]) {
    owned_sort(
        "rust_std_stable",
        pattern,
        values,
        stable::rust_std::sort,
        true,
    );
    owned_sort(
        "rust_ipnsort_unstable",
        pattern,
        values,
        unstable::rust_ipnsort::sort,
        false,
    );
    owned_sort(
        "rust_block_lsd_stable_radix",
        pattern,
        values,
        other::rust_block_lsd::sort,
        true,
    );
    #[cfg(feature = "rust_lsd11")]
    owned_sort(
        "rust_lsd11_stable_radix",
        pattern,
        values,
        other::rust_lsd11::sort,
        true,
    );
    #[cfg(feature = "rust_radsort_cached")]
    owned_sort(
        "rust_radsort_cached_stable_radix",
        pattern,
        values,
        other::rust_radsort_cached::sort,
        true,
    );
    #[cfg(feature = "rust_radsort_unopt")]
    owned_sort(
        "rust_radsort_cached_unopt_stable_radix",
        pattern,
        values,
        other::rust_radsort_unopt::sort,
        true,
    );
    #[cfg(feature = "rust_basic_lsd")]
    owned_sort(
        "rust_basic_lsd_stable_radix",
        pattern,
        values,
        other::rust_basic_lsd::sort,
        true,
    );
    #[cfg(feature = "rust_basic_msd")]
    owned_sort(
        "rust_basic_msd_stable_radix",
        pattern,
        values,
        other::rust_basic_msd::sort,
        true,
    );
    owned_sort(
        "rust_radsort_by_key",
        pattern,
        values,
        |data| radsort::sort_by_key(data, |r| r.key),
        true,
    );
}

fn main() {
    check_accounting();
    let args: Vec<_> = std::env::args().collect();
    let smoke = args.iter().any(|arg| arg == "--test");
    let option = |flag: &str, default: &str| {
        args.windows(2)
            .find(|p| p[0] == flag)
            .map(|p| p[1].clone())
            .unwrap_or_else(|| default.into())
    };
    let sizes: Vec<usize> = option(
        "--sizes",
        if smoke {
            "100,10000"
        } else {
            "100,10000,100000,1000000,4000000,10000000"
        },
    )
    .split(',')
    .map(|v| v.parse().unwrap())
    .collect();
    let types = option("--types", "i32,u64,f128");
    let patterns = option(
        "--patterns",
        if smoke {
            "random,all_equal"
        } else {
            "random,random_z1,random_d20,random_p5,random_s95,ascending,descending,all_equal"
        },
    );
    println!("algorithm,type,pattern,n,record_bytes,input_bytes,peak_extra_heap_bytes,alloc_calls,realloc_calls,total_requested_bytes,retained_extra_heap_bytes");
    for len in sizes {
        for pattern in patterns.split(',') {
            let values = input(pattern, len);
            for ty in types.split(',') {
                match ty {
                    "i32" => run_case(ty, pattern, values.clone(), |a, b| a == b, true),
                    "u16" => run_case(ty, pattern, compress_u16(&values), |a, b| a == b, true),
                    "u32" => run_case(
                        ty,
                        pattern,
                        values
                            .iter()
                            .map(|&v| (v as i64 + i32::MAX as i64 + 1) as u32)
                            .collect(),
                        |a, b| a == b,
                        true,
                    ),
                    "u64" => run_case(
                        ty,
                        pattern,
                        values.iter().copied().map(extend).collect(),
                        |a, b| a == b,
                        true,
                    ),
                    "f128" => run_case(
                        ty,
                        pattern,
                        values.iter().copied().map(F128::new).collect(),
                        |a, b| a == b,
                        false,
                    ),
                    "cell_i32" => run_case(
                        ty,
                        pattern,
                        values.iter().copied().map(std::cell::Cell::new).collect(),
                        |a, b| a == b,
                        false,
                    ),
                    "1k" => run_case(
                        ty,
                        pattern,
                        values
                            .iter()
                            .copied()
                            .map(sort_test_tools::ffi_types::FFIOneKibiByte::new)
                            .collect(),
                        |a, b| a == b,
                        false,
                    ),
                    "tuple_i32" => run_case(
                        ty,
                        pattern,
                        values
                            .iter()
                            .copied()
                            .enumerate()
                            .map(|(i, v)| (v, i as i32))
                            .collect(),
                        |a, b| a == b,
                        false,
                    ),
                    "copy32" => run_copy_records::<16>(ty, pattern, &values),
                    "copy128" => run_copy_records::<112>(ty, pattern, &values),
                    "owned" => run_owned(pattern, &values),
                    "record32" => run_records::<16>(ty, pattern, &values),
                    "record128" => run_records::<112>(ty, pattern, &values),
                    _ => panic!("unsupported type: {ty}"),
                }
            }
        }
    }
}
