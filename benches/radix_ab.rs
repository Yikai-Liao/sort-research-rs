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

use sort_test_tools::ffi_types::{Record128, Record16};
use sort_test_tools::{patterns, Sort};

struct Config {
    types: Vec<String>,
    patterns: Vec<String>,
    sizes: Vec<usize>,
    rounds: usize,
    filter: Option<String>,
    /// Exact variant names to keep, when `--filter`'s substring match would let a
    /// second variant through (`tessera` also matches `tessera_counting`).
    variants: Option<Vec<String>>,
    /// Run the first retained variant once and return: the mode `RADIX_TRACE=1`
    /// wants, one sort so the decision log has exactly one copy of each node.
    trace_once: bool,
}

fn parse_config() -> Config {
    let mut cfg = Config {
        types: vec!["u64".into()],
        patterns: vec!["random".into()],
        sizes: vec![1_000_000],
        rounds: 7,
        filter: None,
        variants: None,
        trace_once: false,
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
            "--trace-once" => cfg.trace_once = true,
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
            let rng = patterns::random(8);
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
        // Concatenated sorted runs with disjoint key ranges: what a merge of
        // sorted pages, a k-way merge or a B-tree leaf scan looks like. The
        // ordered probe rejects it (the run boundaries invert), so it is sorted as
        // an unsorted array with a lot of local order.
        "runs64" => {
            let runs = 64usize.min(len.max(1));
            let run_len = len / runs;
            let mut v = Vec::with_capacity(len);
            for run in 0..runs {
                let base = (run as i64 * (i32::MAX as i64 / runs as i64)) as i32;
                let mut part: Vec<i32> = patterns::random_uniform(run_len.max(1), 0..(1 << 20))
                    .into_iter()
                    .map(|x| base.wrapping_add(x))
                    .collect();
                part.sort_unstable();
                v.extend_from_slice(&part);
            }
            v.truncate(len);
            v
        }
        // Ascending and descending runs, unequal lengths, in random order, over
        // overlapping key ranges: what a merge of pre-sorted sources actually looks
        // like. No probe can accept it (the first inversion is a few records in) and
        // no run is long enough to pay for a search, so every adaptive decision has
        // to be made per node from the data in front of it.
        "mixed_runs" => {
            // A local generator: the run lengths and directions are part of the
            // pattern, not of the data, so they must not depend on the key generator.
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
                // 16 to 2064 records: a run is sometimes shorter than a tile and
                // sometimes several tiles long.
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
        // The same runs, each descending: order exists but the wrong way round,
        // and the probe cannot use it either.
        "blocks_desc" => {
            let runs = 64usize.min(len.max(1));
            let run_len = len / runs;
            let mut v = Vec::with_capacity(len);
            for run in 0..runs {
                let base = (run as i64 * (i32::MAX as i64 / runs as i64)) as i32;
                let mut part: Vec<i32> = patterns::random_uniform(run_len.max(1), 0..(1 << 20))
                    .into_iter()
                    .map(|x| base.wrapping_add(x))
                    .collect();
                part.sort_unstable_by(|a, b| b.cmp(a));
                v.extend_from_slice(&part);
            }
            v.truncate(len);
            v
        }
        // Two values, half and half: the extreme of "few distinct values" and the
        // case a counting pass with two buckets should finish in one pass.
        "two_values" => {
            let rng = patterns::random(len);
            rng.into_iter()
                .map(|x| if x >= 0 { 0 } else { i32::MAX })
                .collect()
        }
        // 1000 groups of equal keys, groups in ascending order: a sorted array with
        // long duplicate runs, so the ordered probe accepts it and the duplicate
        // structure has to be cheap.
        "staircase" => {
            let groups = 1000usize;
            (0..len)
                .map(|index| ((index * groups / len.max(1)) as i32) << 20)
                .collect()
        }
        // Random keys, but each 64-record block draws from a 64-wide window, so
        // consecutive records share most of their bits: locality inside the array,
        // no global order.
        "random_local" => {
            let bases = patterns::random_uniform(len.div_ceil(64), 0..(1 << 20));
            let offsets = patterns::random_uniform(len, 0..64);
            (0..len)
                .map(|index| (bases[index / 64] << 10).wrapping_add(offsets[index]))
                .collect()
        }
        // Only eight bits vary, and they sit above every other varying bit. The
        // shift is chosen so the widening mappings (which scale the pattern into
        // the type's high half) keep the cluster intact: bits 23-30 of the i32 land
        // at bits 54-61 of a u64 and 119-126 of a u128.
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

/// SplitMix64: the generator for patterns that claim full width. Drawing the key
/// itself, rather than hashing a single i32 per record, is what makes the claim
/// true: one i32 carries 32 bits of entropy, so a u64 drawn that way has at most
/// 2^32 distinct values and produces about 1000 times more duplicates at 10M than
/// an independent 64-bit draw -- which changes what the duplicate and bucket paths
/// are asked to do.
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

/// Patterns that only make sense in the target type's own bit space: the
/// widening mappings above scale an i32 pattern by ~2^31, which turns two
/// adjacent values into a 31-bit difference. These build the key directly, so
/// "two values one bit apart" really is one varying bit.
fn type_space_keys(pattern: &str, len: usize, bits: usize) -> Option<Vec<u128>> {
    // Real columns are not uniform over the type's bit space. The float
    // patterns below generate the *converted key* of a normal / log-normal
    // column, so the u64 row is exactly what an f64 column hands the sort.
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
        // A narrow integer stored in a wide container: every varying bit sits in
        // the low 12. The named patterns go through the i32 widening, which
        // spreads them over 62 bits, so this shape was not in the corpus.
        "small12" => {
            let mut state = 0x243f_6a88_85a3_08d3u64 ^ len as u64;
            (0..len)
                .map(|_| (splitmix64(&mut state) & 0xfff) as u128)
                .collect()
        }
        // A normalized composite key: a 16-value dictionary code in the top
        // bits and a random low word.
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
        // The same logical 32-bit key in every container width: a 4-bit
        // category plus 28 random bits, placed in the type's top 32 bits. u64
        // and u128 sort the same values in the same order, so their difference
        // is key arithmetic and record width -- the cell the narrowed key view
        // (PLAN.md §3) has to win on.
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
        // One varying bit, 50/50: the cheapest possible non-trivial input.
        "type_two_near" => patterns::random(len)
            .into_iter()
            .map(|x| (x & 1) as u128)
            .collect(),
        // Two values at opposite ends of the key: same cardinality, maximal digit
        // distance, so the digit work is maximal while the comparison work is not.
        "type_two_far" => patterns::random(len)
            .into_iter()
            .map(|x| if x >= 0 { 0 } else { u128::MAX })
            .collect(),
        // Eight values one bit apart.
        "type_eight_near" => patterns::random(len)
            .into_iter()
            .map(|x| (x & 7) as u128)
            .collect(),
        // Only the top byte varies.
        // The varying byte sits in the target type's own top byte. Placing it at
        // bit 120 unconditionally made u32 and u64 rows all-zero -- the pattern
        // sorted an array of equal keys and called it "top byte varies".
        "type_top_byte" => {
            let shift = bits - 8;
            patterns::random(len)
                .into_iter()
                .map(|x| ((x & 0xff) as u128) << shift)
                .collect()
        }
        // Few values, maximal span: 8 keys spread over the whole width of the
        // type. The previous sparse8 went through the i32 widening, which puts every
        // pattern in the top 32 bits, so "8 values across 128 bits" was really "8
        // values across 32 bits" and the low 96 bits were constant.
        // Deep skew with a fixed number of distinct keys: ten values, but one of
        // them takes 10, 50, 90 or 99 % of the records. The leaf judges a window by
        // the count of non-empty buckets, which cannot tell these four apart, so
        // this is the family that says whether that heuristic picks the right
        // strategy when the distribution is heavy tailed.
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

fn keys_u16(pattern: &str, len: usize) -> Vec<u16> {
    if pattern == "random_wide" {
        return full_width_u64(len).into_iter().map(|x| x as u16).collect();
    }
    base_keys(pattern, len)
        .into_iter()
        .map(|x| (narrow_u32(x) >> 16) as u16)
        .collect()
}

fn keys_u64(pattern: &str, len: usize) -> Vec<u64> {
    if let Some(keys) = type_space_keys(pattern, len, 64) {
        return keys.into_iter().map(|key| key as u64).collect();
    }
    if pattern == "random_wide" {
        return full_width_u64(len);
    } else {
        base_keys(pattern, len)
            .into_iter()
            .map(narrow_u64)
            .collect()
    }
}

fn keys_u128(pattern: &str, len: usize) -> Vec<u128> {
    if let Some(keys) = type_space_keys(pattern, len, 128) {
        return keys;
    }
    if pattern == "random_wide" {
        // Both halves drawn independently, so the width is the entropy.
        return full_width_u128(len);
    }
    // Named patterns keep their shape and their order: the pattern lands in the
    // top 32 bits and the low half is constant. A hashed high half would turn
    // "ascending" into a random array, which is not what the row claims.
    base_keys(pattern, len)
        .into_iter()
        .map(|x| (narrow_u32(x) as u128) << 96)
        .collect()
}

// The record layouts live in sort_test_tools so the C ABI wrappers and the
// benchmark use exactly the same field order. The Key impls live in
// sort_research_rs::other::radix_support, where the trait is defined.

fn keys_record128(pattern: &str, len: usize) -> Vec<Record128> {
    // A full-width row draws the key itself; the named patterns keep their shape.
    let keys: Vec<u64> = if pattern == "random_wide" {
        full_width_u64(len)
    } else {
        base_keys(pattern, len)
            .into_iter()
            .map(narrow_u64)
            .collect()
    };
    keys.into_iter()
        .map(|key| Record128::new(key, [0; 120]))
        .collect()
}

fn keys_record16(pattern: &str, len: usize) -> Vec<Record16> {
    let keys: Vec<u64> = if pattern == "random_wide" {
        full_width_u64(len)
    } else if pattern == "window32_high" {
        // The same key as the u64 row: the payload is the only difference, so
        // u64 -> record16 prices record bytes and nothing else.
        type_space_keys(pattern, len, 64)
            .unwrap()
            .into_iter()
            .map(|key| key as u64)
            .collect()
    } else {
        base_keys(pattern, len)
            .into_iter()
            .map(narrow_u64)
            .collect()
    };
    keys.into_iter()
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

/// Appends one line per round when RADIX_RAW names a file.
///
/// A ratio cannot be recomputed from a summary, so the raw rounds are what makes a
/// result checkable later: which key, which variant, which round, how long, and
/// which commit produced it. The commit is asked of git once per process.
fn archive_raw(pattern: &str, label: &str, n: usize, name: &str, round: usize, secs: f64) {
    use std::io::Write;
    let Ok(path) = std::env::var("RADIX_RAW") else {
        return;
    };
    static SHA: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    let sha = SHA.get_or_init(|| {
        std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|s| s.trim().to_owned())
            .unwrap_or_else(|| "unknown".into())
    });
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(file, "{sha},{label},{pattern},{n},{name},{round},{secs:.9}");
    }
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
    if cfg.trace_once {
        let mut probe = input.clone();
        (variants[0].1)(&mut probe);
        return;
    }

    let mut expected = input.clone();
    expected.sort();
    let mut probe = input.clone();
    for (name, sort) in &variants {
        probe.clone_from_slice(&input);
        sort(&mut probe);
        assert_eq!(probe, expected, "{name} produced the wrong order");
    }

    let results = tournament(&input, &variants, cfg.rounds, label, pattern);
    // The median of the interleaved rounds is the reported statistic, not the
    // minimum: on a shared VM a single fast round is a lucky one, and a ratio taken
    // from two lucky rounds is not a ratio. The minimum stays in its own column
    // because it is the only number that is stable when the machine is quiet.
    let best = results.iter().map(|r| r.1.median).min().unwrap();
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
            s.median.as_secs_f64() / best.as_secs_f64(),
            s.median.as_secs_f64() * 1e9 / input.len() as f64
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

/// The flat counting leaf, with the MSD level inside the tile switched off: same
/// process, same data, so the two leaf strategies are compared directly instead of
/// across runs on a shared machine.

#[cfg(feature = "rust_tessera_radix")]
fn tessera_counting<T: sort_research_rs::other::radix_support::Key>(s: &mut [T]) {
    sort_research_rs::other::rust_tessera_radix::sort_with_leaf_plan(s);
}

/// The other half of the leaf ablation: the MSD level always, the flat plan never.

/// Tile-budget sweep: the knob that sets the auxiliary-space bound.
#[cfg(feature = "rust_tessera_radix")]
fn tessera_tile<const BYTES: usize, T: sort_research_rs::other::radix_support::Key>(s: &mut [T]) {
    use sort_research_rs::other::rust_tessera_radix::{sort_with_policy, Policy};
    sort_with_policy(
        s,
        Policy {
            tile_bytes: BYTES,
            ..Policy::default()
        },
    );
}

/// Insertion-crossover sweep, in records.
#[cfg(feature = "rust_tessera_radix")]
fn tessera_ins<const INS: usize, T: sort_research_rs::other::radix_support::Key>(s: &mut [T]) {
    use sort_research_rs::other::rust_tessera_radix::{sort_with_policy, Policy};
    sort_with_policy(
        s,
        Policy {
            insertion_max: INS,
            ..Policy::default()
        },
    );
}

#[cfg(feature = "rust_msd_radix")]
fn msd_radix<T: sort_research_rs::other::radix_support::Key>(s: &mut [T]) {
    sort_research_rs::other::rust_msd_radix::sort(s);
}

#[cfg(feature = "cpp_ips2ra")]
fn ips2ra<T: Ord>(s: &mut [T]) {
    sort_research_rs::other::cpp_ips2ra::sort(s);
}

#[cfg(feature = "cpp_dovetail")]
fn dovetail<T: Ord>(s: &mut [T]) {
    sort_research_rs::other::cpp_dovetail::sort(s);
}

#[cfg(feature = "cpp_vqsort")]
fn vqsort<T: Ord>(s: &mut [T]) {
    sort_research_rs::other::cpp_vqsort::sort(s);
}

#[cfg(feature = "cpp_intel_avx512")]
fn intel_avx512<T: Ord>(s: &mut [T]) {
    sort_research_rs::other::cpp_intel_avx512::sort(s);
}

#[cfg(feature = "rust_radsort")]
fn radsort_direct<T: Ord>(s: &mut [T]) {
    sort_research_rs::other::rust_radsort::sort(s);
}

#[cfg(all(feature = "rust_radsort", feature = "rust_block_lsd"))]
fn radsort_keyed<T>(s: &mut [T])
where
    T: sort_research_rs::other::radix_support::Key,
    T::Value: radsort::Key,
{
    sort_research_rs::other::rust_radsort::sort_keyed(s);
}

#[cfg(feature = "cpp_ips4o")]
fn ips4o<T: Ord>(s: &mut [T]) {
    sort_research_rs::unstable::cpp_ips4o::sort(s);
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
            v.push(("tessera_counting", tessera_counting::<$ty>));
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
                v.push(("leaf_ins0", tessera_ins::<0, $ty>));
                v.push(("leaf_ins24", tessera_ins::<24, $ty>));
                v.push(("leaf_ins128", tessera_ins::<128, $ty>));
            }
        }
        #[cfg(feature = "rust_msd_radix")]
        {
            v.push(("msd_radix", msd_radix::<$ty>));
        }
        #[cfg(feature = "cpp_ips2ra")]
        if sort_research_rs::other::cpp_ips2ra::supports_type::<$ty>() {
            v.push(("ips2ra", ips2ra::<$ty>));
        }
        #[cfg(feature = "cpp_dovetail")]
        if sort_research_rs::other::cpp_dovetail::supports_type::<$ty>() {
            v.push(("dovetail", dovetail::<$ty>));
        }
        #[cfg(feature = "cpp_vqsort")]
        if sort_research_rs::other::cpp_vqsort::supports_type::<$ty>() {
            v.push(("vqsort", vqsort::<$ty>));
        }
        #[cfg(feature = "cpp_intel_avx512")]
        if sort_research_rs::other::cpp_intel_avx512::supports_type::<$ty>() {
            v.push(("intel_avx512", intel_avx512::<$ty>));
        }
        #[cfg(feature = "rust_radsort")]
        if sort_research_rs::other::rust_radsort::supports_type::<$ty>() {
            v.push(("radsort", radsort_direct::<$ty>));
        }
        #[cfg(all(feature = "rust_radsort", feature = "rust_block_lsd"))]
        if !sort_research_rs::other::rust_radsort::supports_type::<$ty>()
            && <$ty as sort_research_rs::other::radix_support::Key>::SUPPORTED
        {
            v.push(("radsort_keyed", radsort_keyed::<$ty>));
        }
        #[cfg(feature = "cpp_ips4o")]
        if std::any::TypeId::of::<$ty>() == std::any::TypeId::of::<u64>() {
            v.push(("ips4o", ips4o::<$ty>));
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
