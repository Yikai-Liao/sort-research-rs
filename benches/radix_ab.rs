//! Standalone A/B timing harness for the radix experiments.
//!
//! Criterion is too slow for the inner loop of a tuning session, so this target
//! runs a small interleaved tournament: every variant sorts a fresh copy of the
//! same input, the variants rotate across rounds, and each cell reports the
//! minimum and median of `--rounds` samples. Setup and input cloning stay
//! outside the timed region. Each variant is checked once before timing.
//!
//! ```text
//! cargo bench --features rust_block_lsd --bench radix_ab -- \
//!     --types u64,u32 --patterns random --sizes 1000000
//! ```
//!
//! Experiment scaffolding, not part of the library build.

use std::time::{Duration, Instant};

use sort_test_tools::{patterns, Sort};

struct Config {
    types: Vec<String>,
    patterns: Vec<String>,
    sizes: Vec<usize>,
    rounds: usize,
    filter: Option<String>,
}

fn parse_config() -> Config {
    let mut cfg = Config {
        types: vec!["u64".into()],
        patterns: vec!["random".into()],
        sizes: vec![1_000_000],
        rounds: 7,
        filter: None,
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = || args.next().expect("missing value");
        match arg.as_str() {
            "--types" => cfg.types = value().split(',').map(str::to_owned).collect(),
            "--patterns" => cfg.patterns = value().split(',').map(str::to_owned).collect(),
            "--sizes" => {
                cfg.sizes = value()
                    .split(',')
                    .map(|s| s.parse().expect("bad size"))
                    .collect()
            }
            "--rounds" => cfg.rounds = value().parse().expect("bad round count"),
            "--filter" => cfg.filter = Some(value()),
            // Passed by the cargo harness for every bench target.
            "--bench" | "--nocapture" | "--test" => {}
            other => panic!("unknown argument: {other}"),
        }
    }
    cfg
}

fn base_keys(pattern: &str, len: usize) -> Vec<i32> {
    match pattern {
        "random" => patterns::random(len),
        "random_d256" => patterns::random_uniform(len, 0..256),
        "random_d16" => patterns::random_uniform(len, 0..16),
        // Skewed distributions: a long head plus a thin tail is what the MSD
        // experiments had to survive, so they stay in the default set.
        "zipf11" => patterns::random_zipf(len, 1.1),
        "zipf13" => patterns::random_zipf(len, 1.3),
        "zipf20" => patterns::random_zipf(len, 2.0),
        "p95zero" => {
            let zeros = len - len / 20;
            let mut v = vec![0i32; zeros];
            v.extend(patterns::random(len - zeros));
            v
        }
        // The library's `random_sorted` sometimes comes out fully ascending (the
        // appended tail can be larger than the sorted prefix), which would take
        // the ordered fast path and measure something else. These two are
        // ascending with a fixed fraction of adjacent pairs swapped: near-sorted,
        // guaranteed not sorted.
        "random_sorted95" => displaced(patterns::ascending(len), len / 20),
        "random_sorted99" => displaced(patterns::ascending(len), len / 100),
        "random_sorted99" => patterns::random_sorted(len, 99.0),
        // Nearly reverse ordered: descending, then 5 % of the positions swapped,
        // which is what a sort sees after a few appends and a reversal.
        "random_reverse95" => {
            let mut v = patterns::descending(len);
            let step = (len / 20).max(1);
            let mut i = 0;
            while i + step < v.len() {
                v.swap(i, i + step);
                i += 2 * step;
            }
            v
        }
        // Few distinct values spread over the whole type range: the digits are
        // sparse, which is the case a fixed digit grid handles worst.
        "sparse8" => {
            let mut rng = patterns::random(8);
            let mut v = Vec::with_capacity(len);
            for index in 0..len {
                v.push(rng[index % 8]);
            }
            v
        }
        // Eight narrow bands far apart. The band index sits in the top three
        // bits and the offset in the low sixteen, so the widening mappings (which
        // shift the pattern into the type's high half) keep them apart: within a
        // band only the low bits vary, between bands only the top three do.
        "bands8" => {
            let offsets = patterns::random_uniform(len, 0..(1 << 16));
            let mut v = Vec::with_capacity(len);
            for index in 0..len {
                let band = (index % 8) as i32;
                v.push((band << 28) | offsets[index]);
            }
            v
        }
        "ascending" => patterns::ascending(len),
        "descending" => patterns::descending(len),
        "all_equal" => patterns::all_equal(len),
        "pipe_organ" => patterns::pipe_organ(len),
        other => panic!("unknown pattern: {other}"),
    }
}

/// Full-range keys: mixes both 32-bit halves so every bit position varies.
fn wide_keys(pattern: &str, len: usize) -> Vec<i32> {
    // "random_wide" is the full-width flavour of the plain random pattern; the
    // named distributions keep their shape and are widened by the type mapping.
    let v = if pattern == "random_wide" {
        patterns::random(len)
    } else {
        base_keys(pattern, len)
    };
    v.into_iter()
        .enumerate()
        .map(|(i, x)| {
            let lo = (x as u32).wrapping_mul(0x9e37_79b9);
            let hi = (x as u32)
                .wrapping_mul(0x85eb_ca6b)
                .rotate_left(i as u32 & 31);
            (lo ^ hi) as i32
        })
        .collect()
}

fn narrow_u32(x: i32) -> u32 {
    (x as i64 + (i32::MAX as i64 + 1)) as u32
}

fn narrow_u64(x: i32) -> u64 {
    (narrow_u32(x) as u64) * i32::MAX as u64
}

fn wide_u64(x: i32) -> u64 {
    (x as u32 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
}

/// Swaps \`swaps\` evenly spread adjacent pairs: an ordered array with that many
/// displaced elements, which is the shape "sorted, then a few edits" produces.
fn displaced(mut v: Vec<i32>, swaps: usize) -> Vec<i32> {
    let step = (v.len() / swaps.max(1)).max(2);
    let mut index = 0;
    while index + 1 < v.len() {
        v.swap(index, index + 1);
        index += step;
    }
    v
}

fn keys_u32(pattern: &str, len: usize) -> Vec<u32> {
    if pattern == "random_wide" {
        wide_keys(pattern, len)
            .into_iter()
            .map(|x| x as u32)
            .collect()
    } else {
        base_keys(pattern, len)
            .into_iter()
            .map(narrow_u32)
            .collect()
    }
}

fn keys_u64(pattern: &str, len: usize) -> Vec<u64> {
    if pattern == "random_wide" {
        wide_keys(pattern, len)
            .into_iter()
            .map(|x| wide_u64(x) ^ ((x as u64) << 32))
            .collect()
    } else {
        base_keys(pattern, len)
            .into_iter()
            .map(narrow_u64)
            .collect()
    }
}

fn keys_u128(pattern: &str, len: usize) -> Vec<u128> {
    if pattern == "random_wide" {
        // Full width: both halves hashed, every bit position varies.
        return wide_keys(pattern, len)
            .into_iter()
            .map(|x| ((wide_u64(x) as u128) << 64) | narrow_u64(x) as u128)
            .collect();
    }
    // Named patterns keep their shape and their order: the pattern lands in the
    // top 32 bits and the low half is constant. A hashed high half would turn
    // "ascending" into a random array, which is not what the row claims.
    base_keys(pattern, len)
        .into_iter()
        .map(|x| (narrow_u32(x) as u128) << 96)
        .collect()
}

/// A 16-byte record that sorts by its leading u64, like benches/bench.rs does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
struct Record16 {
    key: u64,
    _padding: [u8; 8],
}

impl Ord for Record16 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}
impl PartialOrd for Record16 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl sort_research_rs::other::radix_support::Key for Record16 {
    type Value = u64;
    fn key(&self) -> u64 {
        self.key
    }
}

/// A 128-byte record that sorts by its leading u64, like the repository's
/// u64_128b benchmark type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
struct Record128 {
    key: u64,
    _padding: [u8; 120],
}

impl Ord for Record128 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}
impl PartialOrd for Record128 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl sort_research_rs::other::radix_support::Key for Record128 {
    type Value = u64;
    fn key(&self) -> u64 {
        self.key
    }
}

fn keys_record128(pattern: &str, len: usize) -> Vec<Record128> {
    let f = if pattern == "random_wide" {
        wide_u64
    } else {
        narrow_u64
    };
    base_keys(pattern, len)
        .into_iter()
        .map(|x| Record128 {
            key: f(x),
            _padding: [0; 120],
        })
        .collect()
}

fn keys_record16(pattern: &str, len: usize) -> Vec<Record16> {
    let f = if pattern == "random_wide" {
        wide_u64
    } else {
        narrow_u64
    };
    base_keys(pattern, len)
        .into_iter()
        .map(|x| Record16 {
            key: f(x),
            _padding: [0; 8],
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Timing
// ---------------------------------------------------------------------------

struct Sample {
    min: Duration,
    median: Duration,
}

fn summarize(mut times: Vec<Duration>) -> Sample {
    times.sort();
    Sample {
        min: times[0],
        median: times[times.len() / 2],
    }
}

fn tournament<T: Clone>(
    input: &[T],
    variants: &[(&'static str, fn(&mut [T]))],
    rounds: usize,
) -> Vec<(&'static str, Sample)> {
    let mut samples: Vec<Vec<Duration>> = vec![Vec::with_capacity(rounds); variants.len()];
    let mut buf = input.to_vec();
    for round in 0..rounds {
        for i in 0..variants.len() {
            let idx = (round + i) % variants.len();
            buf.clone_from_slice(input);
            let start = Instant::now();
            (variants[idx].1)(&mut buf);
            samples[idx].push(start.elapsed());
        }
    }
    variants
        .iter()
        .zip(samples)
        .map(|((name, _), s)| (*name, summarize(s)))
        .collect()
}

fn run<T: Clone + Ord + std::fmt::Debug>(
    cfg: &Config,
    label: &str,
    pattern: &str,
    input: Vec<T>,
    mut variants: Vec<(&'static str, fn(&mut [T]))>,
) {
    variants.push(("std_stable", |v| v.sort()));
    variants.push(("ipnsort", |v| {
        sort_research_rs::unstable::rust_ipnsort::SortImpl::sort(v)
    }));
    if let Some(filter) = &cfg.filter {
        variants.retain(|(name, _)| name.contains(filter.as_str()));
    }

    let mut expected = input.clone();
    expected.sort();
    let mut probe = input.clone();
    for (name, sort) in &variants {
        probe.clone_from_slice(&input);
        sort(&mut probe);
        assert_eq!(probe, expected, "{name} produced the wrong order");
    }

    let results = tournament(&input, &variants, cfg.rounds);
    let best = results.iter().map(|r| r.1.min).min().unwrap();
    let bytes = std::mem::size_of::<T>();
    let ascending = input.windows(2).all(|w| w[0] <= w[1]);
    let descending = input.windows(2).all(|w| w[0] >= w[1]);
    println!(
        "\n{label} {pattern} n={} ({:.1} MiB) ascending={ascending} descending={descending} distinct={}",
        input.len(),
        (input.len() * bytes) as f64 / (1024.0 * 1024.0),
        distinct_count(&input),
    );
    println!(
        "{:<30} {:>11} {:>11} {:>8} {:>10}",
        "variant", "min", "median", "ratio", "ns/elem"
    );
    for (name, s) in &results {
        println!(
            "{:<30} {:>11} {:>11} {:>8.2} {:>10.2}",
            name,
            format!("{:.3?}", s.min),
            format!("{:.3?}", s.median),
            s.min.as_secs_f64() / best.as_secs_f64(),
            s.min.as_secs_f64() * 1e9 / input.len() as f64
        );
    }
}

fn distinct_count<T: Ord>(input: &[T]) -> usize {
    let mut seen: Vec<&T> = input.iter().collect();
    seen.sort_unstable();
    seen.dedup();
    seen.len()
}

// ---------------------------------------------------------------------------
// Variant tables
// ---------------------------------------------------------------------------

#[cfg(feature = "rust_block_lsd")]
fn block_lsd_radix<const R: usize, T: sort_research_rs::other::radix_support::Key>(s: &mut [T]) {
    sort_research_rs::other::rust_block_lsd::sort_with_radix(s, R);
}

#[cfg(feature = "rust_tessera_radix")]
fn tessera_tile<const BYTES: usize, T: sort_research_rs::other::radix_support::Key>(s: &mut [T]) {
    sort_research_rs::other::rust_tessera_radix::sort_with_tile_bytes(s, BYTES);
}

/// Leaf-policy sweep: (merge ceiling, insertion ceiling) in records.
#[cfg(feature = "rust_tessera_radix")]
fn tessera_leaf<
    const MERGE: usize,
    const INS: usize,
    T: sort_research_rs::other::radix_support::Key,
>(
    s: &mut [T],
) {
    sort_research_rs::other::rust_tessera_radix::sort_with_policy(s, 65536, MERGE, INS);
}

#[cfg(feature = "rust_msd_radix")]
fn msd_radix<T: sort_research_rs::other::radix_support::Key>(s: &mut [T]) {
    sort_research_rs::other::rust_msd_radix::sort(s);
}

macro_rules! radix_variants {
    ($ty:ty) => {{
        #[allow(unused_mut)]
        let mut v: Vec<(&'static str, fn(&mut [$ty]))> = Vec::new();
        #[cfg(feature = "rust_block_lsd")]
        {
            v.push(("block_lsd_r8", block_lsd_radix::<8, $ty>));
            v.push(("block_lsd_r11", block_lsd_radix::<11, $ty>));
        }
        #[cfg(feature = "rust_tessera_radix")]
        {
            v.push(("tessera", |s: &mut [$ty]| {
                sort_research_rs::other::rust_tessera_radix::sort(s)
            }));
            // The tile budget and the leaf crossovers are swept from here while
            // tuning; they cost a build matrix, so they stay opt-in.
            if std::env::var_os("RADIX_AB_LEAVES").is_some() {
                v.push(("tile_16k", tessera_tile::<16384, $ty>));
                v.push(("tile_24k", tessera_tile::<24576, $ty>));
                v.push(("tile_32k", tessera_tile::<32768, $ty>));
                v.push(("tile_48k", tessera_tile::<49152, $ty>));
                v.push(("tile_96k", tessera_tile::<98304, $ty>));
                v.push(("tile_128k", tessera_tile::<131072, $ty>));
                v.push(("tile_256k", tessera_tile::<262144, $ty>));
                v.push(("tile_512k", tessera_tile::<524288, $ty>));
                v.push(("leaf_count_only", tessera_leaf::<0, 0, $ty>));
                v.push(("leaf_ins48", tessera_leaf::<0, 48, $ty>));
                v.push(("leaf_merge32", tessera_leaf::<32, 48, $ty>));
                v.push(("leaf_merge128", tessera_leaf::<128, 48, $ty>));
                v.push(("leaf_merge512", tessera_leaf::<512, 48, $ty>));
                v.push(("leaf_merge2k", tessera_leaf::<2048, 48, $ty>));
                v.push(("leaf_merge_ins128", tessera_leaf::<512, 128, $ty>));
            }
        }
        #[cfg(feature = "rust_msd_radix")]
        {
            v.push(("msd_radix", msd_radix::<$ty>));
        }
        v
    }};
}

fn main() {
    let cfg = parse_config();
    for type_name in &cfg.types {
        for pattern in &cfg.patterns {
            for &len in &cfg.sizes {
                macro_rules! dispatch {
                    ($ty:ty, $make:expr, $label:expr) => {{
                        let input: Vec<$ty> = ($make)(pattern, len);
                        run(&cfg, $label, pattern, input, radix_variants!($ty));
                    }};
                }
                match type_name.as_str() {
                    "u32" => dispatch!(u32, keys_u32, "u32"),
                    "u64" => dispatch!(u64, keys_u64, "u64"),
                    "u128" => dispatch!(u128, keys_u128, "u128"),
                    "record16" => dispatch!(Record16, keys_record16, "record16"),
                    "record128" => dispatch!(Record128, keys_record128, "record128"),
                    other => panic!("unknown type {other}"),
                }
            }
        }
    }
}
