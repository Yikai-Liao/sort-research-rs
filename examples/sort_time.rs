//! Times the shipped Tessera sort on one cell, without the benchmark harness's
//! sibling work (expected.sort, distinct_count, ipnsort), so perf stat counts
//! the sort and almost nothing else. Research tool, not part of the library.

use sort_research_rs::other::rust_tessera_radix;
use std::time::Instant;

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

macro_rules! run {
    ($ty:ty, $values:expr, $label:expr) => {{
        let values: Vec<$ty> = $values;
        let n = values.len();
        // The radix_ab harness allocates its reference array and its probe before
        // the timed buffer; `RADIX_SORT_TIME_PAD` reproduces that allocation
        // sequence to test whether it moves the buffer somewhere slower.
        let pad: usize = std::env::var("RADIX_SORT_TIME_PAD")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        let mut padding: Vec<Vec<$ty>> = (0..pad).map(|_| values.clone()).collect();
        std::hint::black_box(&mut padding);
        let mut buf = values.clone();
        let mut times = Vec::new();
        for _ in 0..7 {
            buf.clone_from_slice(&values);
            let start = Instant::now();
            rust_tessera_radix::sort(std::hint::black_box(&mut buf));
            times.push(start.elapsed().as_secs_f64() * 1e9 / n as f64);
        }
        assert!(buf.windows(2).all(|w| w[0] <= w[1]));
        println!(
            "{:<10} {:<6} n={:<9} median={:7.3} min={:7.3} ns/rec",
            $label,
            stringify!($ty),
            n,
            median(times.clone()),
            times.iter().cloned().fold(f64::MAX, f64::min),
        );
    }};
}

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .map(|s| s.parse().unwrap())
        .unwrap_or(1_000_000);
    let pattern = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "ascending".into());

    // The harness's narrowing maps, so the numbers match radix_ab.
    let u32_keys = |i: usize| -> u32 { (i as i64 + (1 << 31)) as u32 };
    let u64_keys = |i: usize| -> u64 { u32_keys(i) as u64 * i32::MAX as u64 };
    let u128_keys = |i: usize| -> u128 { (u32_keys(i) as u128) << 96 };

    let splitmix = || {
        let mut state = 0x243f_6a88_85a3_08d3u64 ^ n as u64;
        move || {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        }
    };

    match pattern.as_str() {
        "ascending" => {
            run!(u32, (0..n).map(u32_keys).collect::<Vec<_>>(), "ascending");
            run!(u64, (0..n).map(u64_keys).collect::<Vec<_>>(), "ascending");
            run!(u128, (0..n).map(u128_keys).collect::<Vec<_>>(), "ascending");
        }
        "all_equal" => {
            run!(u32, vec![7u32; n], "all_equal");
            run!(u64, vec![7u64; n], "all_equal");
            run!(u128, vec![7u128; n], "all_equal");
        }
        "random" => {
            let mut next = splitmix();
            run!(
                u32,
                (0..n).map(|_| next() as u32).collect::<Vec<_>>(),
                "random"
            );
            let mut next = splitmix();
            run!(u64, (0..n).map(|_| next()).collect::<Vec<_>>(), "random");
            let mut next = splitmix();
            run!(
                u128,
                (0..n)
                    .map(|_| (next() as u128) << 64 | next() as u128)
                    .collect::<Vec<_>>(),
                "random"
            );
        }
        other => panic!("unknown pattern {other}"),
    }
}
