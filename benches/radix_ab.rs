//! Interleaved timing of supported radix and comparison sorts.
//!
//! Each variant sorts the same input; order rotates per round. Input generation,
//! cloning and reference checks are outside the timed region.
//!
//! cargo bench --features rust_tessera_radix --bench radix_ab -- --variants tessera

use std::time::{Duration, Instant};

use sort_test_tools::ffi_types::{Record128, Record16};
use sort_test_tools::{patterns, Sort};

struct Config {
    types: Vec<String>,
    patterns: Vec<String>,
    sizes: Vec<usize>,
    rounds: usize,
    filter: Option<String>,
    /// Exact variant names to retain.
    variants: Option<Vec<String>>,
}

fn parse_config() -> Config {
    let mut cfg = Config {
        types: vec!["u64".into()],
        patterns: vec!["random".into()],
        sizes: vec![1_000_000],
        rounds: 7,
        filter: None,
        variants: None,
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
            "--variants" => cfg.variants = Some(value().split(',').map(str::to_owned).collect()),
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

        "zipf11" => patterns::random_zipf(len, 1.1),
        "zipf13" => patterns::random_zipf(len, 1.3),
        "zipf20" => patterns::random_zipf(len, 2.0),
        "p95zero" => {
            let zeros = len - len / 20;
            let mut v = vec![0i32; zeros];
            v.extend(patterns::random(len - zeros));
            v
        }
        // Adjacent swaps give a guaranteed near-sorted input.
        "random_sorted95" => displaced(patterns::ascending(len), len / 20),
        "random_sorted99" => displaced(patterns::ascending(len), len / 100),
        // Descending input with a few separated swaps.
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
        // Eight repeating keys, spread by the type mapping.
        "sparse8" => {
            let rng = patterns::random(8);
            let mut v = Vec::with_capacity(len);
            for index in 0..len {
                v.push(rng[index % 8]);
            }
            v
        }
        // Eight bands: high three bits select the band, low sixteen vary within it.
        "bands8" => {
            let offsets = patterns::random_uniform(len, 0..(1 << 16));
            let mut v = Vec::with_capacity(len);
            for index in 0..len {
                let band = (index % 8) as i32;
                v.push((band << 28) | offsets[index]);
            }
            v
        }
        // Sorted runs over disjoint ranges.
        "runs64" | "blocks_desc" => {
            let runs = 64usize.min(len.max(1));
            let run_len = len / runs;
            let mut v = Vec::with_capacity(len);
            for run in 0..runs {
                let base = (run as i64 * (i32::MAX as i64 / runs as i64)) as i32;
                let mut part: Vec<i32> = patterns::random_uniform(run_len.max(1), 0..(1 << 20))
                    .into_iter()
                    .map(|x| base.wrapping_add(x))
                    .collect();
                if pattern == "blocks_desc" {
                    part.sort_unstable_by(|a, b| b.cmp(a));
                } else {
                    part.sort_unstable();
                }
                v.extend_from_slice(&part);
            }
            v.truncate(len);
            v
        }
        // Unequal ascending/descending runs over overlapping key ranges.
        "mixed_runs" => {
            // Run layout is independent of the key generator.
            let mut state = 0x5d2b_9c11_7a63_e401u64;
            let mut below = |limit: usize| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                ((state >> 33) as usize) % limit.max(1)
            };
            let keys = patterns::random(len);
            let mut v: Vec<i32> = Vec::with_capacity(len);
            while v.len() < len {
                let run = (16 + below(2048)).min(len - v.len());
                let descending = below(2) == 0;
                let mut part: Vec<i32> = keys[v.len()..v.len() + run].to_vec();
                if descending {
                    part.sort_unstable_by(|a, b| b.cmp(a));
                } else {
                    part.sort_unstable();
                }
                v.extend_from_slice(&part);
            }
            v
        }

        // Two values spanning the base key range.
        "two_values" => {
            let rng = patterns::random(len);
            rng.into_iter()
                .map(|x| if x >= 0 { 0 } else { i32::MAX })
                .collect()
        }
        // One thousand equal-key groups in ascending order.
        "staircase" => {
            let groups = 1000usize;
            (0..len)
                .map(|index| ((index * groups / len.max(1)) as i32) << 20)
                .collect()
        }
        // Each 64-record block draws from a 64-value window.
        "random_local" => {
            let bases = patterns::random_uniform(len.div_ceil(64), 0..(1 << 20));
            let offsets = patterns::random_uniform(len, 0..64);
            (0..len)
                .map(|index| (bases[index / 64] << 10).wrapping_add(offsets[index]))
                .collect()
        }
        // The byte at bits 23–30 stays clustered under the type mappings.
        "high_byte_only" => {
            let rng = patterns::random(len);
            rng.into_iter().map(|x| (x & 0xff) << 23).collect()
        }

        "two_values_near" => {
            let rng = patterns::random(len);
            rng.into_iter().map(|x| x & 1).collect()
        }
        // Already-sorted low-cardinality keys: both shortcuts apply at once.
        "sorted_d20" => {
            let mut v = patterns::random_uniform(len, 0..20);
            v.sort_unstable();
            v
        }
        "ascending" => patterns::ascending(len),
        "descending" => patterns::descending(len),
        "all_equal" => patterns::all_equal(len),
        "pipe_organ" => patterns::pipe_organ(len),
        other => panic!("unknown pattern: {other}"),
    }
}

fn full_width_u32(len: usize) -> Vec<u32> {
    patterns::random(len)
        .into_iter()
        .enumerate()
        .map(|(index, key)| {
            let lo = (key as u32).wrapping_mul(0x9e37_79b9);
            let hi = (key as u32)
                .wrapping_mul(0x85eb_ca6b)
                .rotate_left(index as u32 & 31);
            lo ^ hi
        })
        .collect()
}

fn ordered_u32_key(x: i32) -> u32 {
    (x as i64 + (i32::MAX as i64 + 1)) as u32
}

fn spread_u64_key(x: i32) -> u64 {
    (ordered_u32_key(x) as u64) * i32::MAX as u64
}

/// Full-width draws use SplitMix64, rather than widening 32-bit keys.
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// `len` independent 64-bit keys, uniform over the whole width.
fn full_width_u64(len: usize) -> Vec<u64> {
    let mut state = 0x243f_6a88_85a3_08d3u64 ^ len as u64;
    (0..len).map(|_| splitmix64(&mut state)).collect()
}

/// `len` independent 128-bit keys, uniform over the whole width.
fn full_width_u128(len: usize) -> Vec<u128> {
    let mut state = 0x1319_8a2e_0370_7344u64 ^ len as u64;
    (0..len)
        .map(|_| {
            let hi = splitmix64(&mut state) as u128;
            let lo = splitmix64(&mut state) as u128;
            (hi << 64) | lo
        })
        .collect()
}

/// Swaps evenly spaced adjacent pairs.
fn displaced(mut v: Vec<i32>, swaps: usize) -> Vec<i32> {
    let step = (v.len() / swaps.max(1)).max(2);
    let mut index = 0;
    while index + 1 < v.len() {
        v.swap(index, index + 1);
        index += step;
    }
    v
}

/// Builds keys in the target type's bit space, without the base-key mapping.
fn type_space_keys(pattern: &str, len: usize, bits: usize) -> Option<Vec<u128>> {
    // Floating columns are represented by their total-order integer keys.
    if pattern == "f64_normal" || pattern == "f64_lognormal" {
        let lognormal = pattern == "f64_lognormal";
        let mut state = 0x6a09_e667_f3bc_c909u64 ^ len as u64;
        let mut out = Vec::with_capacity(len);
        while out.len() < len {
            // Box–Muller from two independent uniforms in (0, 1].
            let u1 = ((splitmix64(&mut state) >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            let u2 = ((splitmix64(&mut state) >> 11) as f64 + 0.5) / (1u64 << 53) as f64;
            let z = (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos();
            let value = if lognormal { z.exp() } else { z };
            let key = if bits == 32 {
                let bits32 = (value as f32).to_bits();
                let sign = 1u32 << 31;
                (if bits32 & sign != 0 {
                    !bits32
                } else {
                    bits32 ^ sign
                }) as u128
            } else {
                let bits64 = value.to_bits();
                let sign = 1u64 << 63;
                (if bits64 & sign != 0 {
                    !bits64
                } else {
                    bits64 ^ sign
                }) as u128
            };
            out.push(key);
        }
        return Some(out);
    }
    let keys: Vec<u128> = match pattern {
        // Only the low twelve bits vary.
        "small12" => {
            let mut state = 0x243f_6a88_85a3_08d3u64 ^ len as u64;
            (0..len)
                .map(|_| (splitmix64(&mut state) & 0xfff) as u128)
                .collect()
        }
        // A dictionary category above a random low word.
        "composite16_32" => {
            let mut state = 0xcbf2_9ce4_8422_2325u64 ^ len as u64;
            (0..len)
                .map(|_| {
                    let category = splitmix64(&mut state) & 15;
                    let low = splitmix64(&mut state);
                    let category = category as u128;
                    match bits {
                        32 => (category << 16) | (low & 0xffff) as u128,
                        64 => (category << 48) | (low & 0xffff_ffff) as u128,
                        _ => (category << 112) | (low & 0xffff_ffff) as u128,
                    }
                })
                .collect()
        }
        // The same 32-bit logical key in the top word of every container width.
        "window32_high" => {
            let mut state = 0x3c6e_f372_fe94_f82bu64 ^ len as u64;
            let shift = bits.saturating_sub(32);
            (0..len)
                .map(|_| {
                    let category = splitmix64(&mut state) & 15;
                    let low = splitmix64(&mut state) & 0x0fff_ffff;
                    (((category << 28) | low) as u128) << shift
                })
                .collect()
        }

        "type_two_near" => patterns::random(len)
            .into_iter()
            .map(|x| (x & 1) as u128)
            .collect(),

        "type_two_far" => patterns::random(len)
            .into_iter()
            .map(|x| if x >= 0 { 0 } else { u128::MAX })
            .collect(),

        "type_eight_near" => patterns::random(len)
            .into_iter()
            .map(|x| (x & 7) as u128)
            .collect(),
        // Use the target type's top byte; a fixed shift of 120 made u32/u64 all zero.
        "type_top_byte" => {
            let shift = bits - 8;
            patterns::random(len)
                .into_iter()
                .map(|x| ((x & 0xff) as u128) << shift)
                .collect()
        }
        // Ten values with a dominant key at 10, 50, 90 or 99 percent.
        "skew10" | "skew50" | "skew90" | "skew99" => {
            let share: u64 = match pattern {
                "skew10" => 10,
                "skew50" => 50,
                "skew90" => 90,
                _ => 99,
            };
            let mask = if bits == 128 {
                u128::MAX
            } else {
                (1u128 << bits) - 1
            };
            let step = mask / 10;
            let mut state = 0x1234_5678_9abc_def1u64 ^ len as u64;
            (0..len)
                .map(|_| {
                    let roll = splitmix64(&mut state) % 100;
                    let index = if roll < share {
                        0
                    } else {
                        1 + (splitmix64(&mut state) % 9)
                    };
                    (index as u128 * step) & mask
                })
                .collect()
        }
        "sparse8_full" => {
            let mut state = 0x9e37_79b9_7f4a_7c15u64 ^ len as u64;
            let mask = if bits == 128 {
                u128::MAX
            } else {
                (1u128 << bits) - 1
            };
            let picks: Vec<u128> = (0..8)
                .map(|_| {
                    let hi = splitmix64(&mut state) as u128;
                    let lo = splitmix64(&mut state) as u128;
                    ((hi << 64) | lo) & mask
                })
                .collect();
            patterns::random(len)
                .into_iter()
                .map(|x| picks[(x as usize) & 7])
                .collect()
        }
        _ => return None,
    };
    Some(keys)
}

fn keys_u32(pattern: &str, len: usize) -> Vec<u32> {
    if let Some(keys) = type_space_keys(pattern, len, 32) {
        return keys.into_iter().map(|key| key as u32).collect();
    }
    if pattern == "random_wide" {
        full_width_u32(len)
    } else {
        base_keys(pattern, len)
            .into_iter()
            .map(ordered_u32_key)
            .collect()
    }
}

fn keys_u16(pattern: &str, len: usize) -> Vec<u16> {
    if pattern == "random_wide" {
        return full_width_u64(len).into_iter().map(|x| x as u16).collect();
    }
    base_keys(pattern, len)
        .into_iter()
        .map(|x| (ordered_u32_key(x) >> 16) as u16)
        .collect()
}

fn keys_u64(pattern: &str, len: usize) -> Vec<u64> {
    if let Some(keys) = type_space_keys(pattern, len, 64) {
        return keys.into_iter().map(|key| key as u64).collect();
    }
    if pattern == "random_wide" {
        full_width_u64(len)
    } else {
        base_keys(pattern, len)
            .into_iter()
            .map(spread_u64_key)
            .collect()
    }
}

fn keys_u128(pattern: &str, len: usize) -> Vec<u128> {
    if let Some(keys) = type_space_keys(pattern, len, 128) {
        return keys;
    }
    if pattern == "random_wide" {
        return full_width_u128(len);
    }
    // Preserve base-pattern order in the high 32 bits.
    base_keys(pattern, len)
        .into_iter()
        .map(|x| (ordered_u32_key(x) as u128) << 96)
        .collect()
}

// Record layouts are shared with the C ABI wrappers.

fn keys_record128(pattern: &str, len: usize) -> Vec<Record128> {
    keys_u64(pattern, len)
        .into_iter()
        .map(|key| Record128::new(key, [0; 120]))
        .collect()
}

fn keys_record16(pattern: &str, len: usize) -> Vec<Record16> {
    keys_u64(pattern, len)
        .into_iter()
        .map(|key| Record16::new(key, [0; 8]))
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

/// Appends revision,type,pattern,n,variant,round,seconds to RADIX_RAW.
fn archive_raw(pattern: &str, label: &str, n: usize, name: &str, round: usize, secs: f64) {
    use std::io::Write;
    let Ok(path) = std::env::var("RADIX_RAW") else {
        return;
    };
    static SHA: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    let sha = SHA.get_or_init(|| {
        std::process::Command::new("git")
            .args(["describe", "--always", "--dirty", "--abbrev=40"])
            .output()
            .ok()
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|s| s.trim().to_owned())
            .unwrap_or_else(|| "unknown".into())
    });
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("open RADIX_RAW");
    writeln!(file, "{sha},{label},{pattern},{n},{name},{round},{secs:.9}")
        .expect("write RADIX_RAW");
}

fn tournament<T: Clone>(
    input: &[T],
    variants: &[(&'static str, fn(&mut [T]))],
    rounds: usize,
    label: &str,
    pattern: &str,
) -> Vec<(&'static str, Sample)> {
    let mut samples: Vec<Vec<Duration>> = vec![Vec::with_capacity(rounds); variants.len()];
    let mut buf = input.to_vec();
    for round in 0..rounds {
        for i in 0..variants.len() {
            let idx = (round + i) % variants.len();
            buf.clone_from_slice(input);
            let start = Instant::now();
            (variants[idx].1)(&mut buf);
            let elapsed = start.elapsed();
            samples[idx].push(elapsed);
            archive_raw(
                pattern,
                label,
                input.len(),
                variants[idx].0,
                round,
                elapsed.as_secs_f64(),
            );
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
    if let Some(names) = &cfg.variants {
        variants.retain(|(name, _)| names.iter().any(|wanted| wanted == name));
    }
    assert!(!variants.is_empty(), "no variant matched the filter");

    let mut expected = input.clone();
    expected.sort();
    let mut probe = input.clone();
    for (name, sort) in &variants {
        probe.clone_from_slice(&input);
        sort(&mut probe);
        assert_eq!(probe, expected, "{name} produced the wrong order");
    }

    let results = tournament(&input, &variants, cfg.rounds, label, pattern);
    // Ratios use medians of the interleaved rounds; minima are shown separately.
    let best = results.iter().map(|r| r.1.median).min().unwrap();
    let bytes = std::mem::size_of::<T>();
    let ascending = input.windows(2).all(|w| w[0] <= w[1]);
    let descending = input.windows(2).all(|w| w[0] >= w[1]);
    println!(
        "\n{label} {pattern} n={} ({:.1} MiB) ascending={ascending} descending={descending} distinct={}",
        input.len(),
        (input.len() * bytes) as f64 / (1024.0 * 1024.0),
        expected.chunk_by(|a, b| a == b).count(),
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
            s.median.as_secs_f64() / best.as_secs_f64(),
            s.median.as_secs_f64() * 1e9 / input.len() as f64
        );
    }
}

// ---------------------------------------------------------------------------
// Variant tables
// ---------------------------------------------------------------------------

macro_rules! radix_variants {
    ($ty:ty) => {{
        #[allow(unused_mut)]
        let mut v: Vec<(&'static str, fn(&mut [$ty]))> = Vec::new();
        #[cfg(feature = "rust_block_lsd")]
        {
            v.push((
                "block_lsd_r8",
                sort_research_rs::other::rust_block_lsd::sort,
            ));
        }
        #[cfg(feature = "rust_tessera_radix")]
        {
            v.push((
                "tessera",
                sort_research_rs::other::rust_tessera_radix::sort::<$ty>,
            ));
        }
        #[cfg(feature = "rust_msd_radix")]
        {
            v.push((
                "msd_radix",
                sort_research_rs::other::rust_msd_radix::sort::<$ty>,
            ));
        }
        #[cfg(feature = "cpp_ips2ra")]
        if sort_research_rs::other::cpp_ips2ra::supports_type::<$ty>() {
            v.push(("ips2ra", sort_research_rs::other::cpp_ips2ra::sort::<$ty>));
        }
        #[cfg(feature = "cpp_dovetail")]
        if sort_research_rs::other::cpp_dovetail::supports_type::<$ty>() {
            v.push((
                "dovetail",
                sort_research_rs::other::cpp_dovetail::sort::<$ty>,
            ));
        }
        #[cfg(feature = "cpp_vqsort")]
        if sort_research_rs::other::cpp_vqsort::supports_type::<$ty>() {
            v.push(("vqsort", sort_research_rs::other::cpp_vqsort::sort::<$ty>));
        }
        #[cfg(feature = "cpp_intel_avx512")]
        if sort_research_rs::other::cpp_intel_avx512::supports_type::<$ty>() {
            v.push((
                "intel_avx512",
                sort_research_rs::other::cpp_intel_avx512::sort::<$ty>,
            ));
        }
        #[cfg(feature = "rust_radsort")]
        if sort_research_rs::other::rust_radsort::supports_type::<$ty>() {
            v.push((
                "radsort",
                sort_research_rs::other::rust_radsort::sort::<$ty>,
            ));
        }
        #[cfg(all(feature = "rust_radsort", feature = "rust_block_lsd"))]
        if !sort_research_rs::other::rust_radsort::supports_type::<$ty>()
            && <$ty as sort_research_rs::other::radix_support::Key>::SUPPORTED
        {
            v.push((
                "radsort_keyed",
                sort_research_rs::other::rust_radsort::sort_keyed::<$ty>,
            ));
        }
        #[cfg(feature = "cpp_ips4o")]
        if std::any::TypeId::of::<$ty>() == std::any::TypeId::of::<u64>() {
            v.push(("ips4o", sort_research_rs::unstable::cpp_ips4o::sort::<$ty>));
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
                    "u16" => dispatch!(u16, keys_u16, "u16"),
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
