// Compares the vendored block LSD backend against a templated translation of the
// reference (clausecker/radsort @ f69e816, radixsort_permuted.c) on several
// record shapes. Both sides derive the same block size from n and the record
// size, generate the same input, and time only the sort.
//
// The C++ side lives in ../cxx/driver.cpp and mirrors this file.
#![allow(incomplete_features, internal_features, stable_features)]
#![feature(specialization, sized_type_properties)]

mod backend {
    pub mod key {
        pub use sort_research_rs::other::rust_block_lsd::{Key, Unsigned};
    }

    #[path = "/root/code/sort-research-rs/src/other/rust_block_lsd/core.rs"]
    pub mod core;
}

use backend::key::{Key, Unsigned as _};
use std::time::Instant;

const SEED: u64 = 0xdeadbeefc0febabe;

#[inline]
fn xs(a: u64) -> u64 {
    let mut a = a;
    a ^= a << 13;
    a ^= a >> 7;
    a ^= a << 17;
    a
}

/// A record with a radix key and an opaque payload, mirroring prs::Record.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Record<K, const PAYLOAD: usize> {
    key: K,
    payload: [u8; PAYLOAD],
}

impl<K: Key, const PAYLOAD: usize> Key for Record<K, PAYLOAD> {
    type Value = K::Value;
    #[inline]
    fn key(&self) -> Self::Value {
        self.key.key()
    }
}

impl<K: Key, const PAYLOAD: usize> PartialOrd for Record<K, PAYLOAD>
where
    K: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Key + Ord, const PAYLOAD: usize> Ord for Record<K, PAYLOAD> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

/// Input generation and checksum for one record shape, mirroring the C++ driver.
trait Shape: Key + Copy + Ord + 'static {
    const NAME: &'static str;
    fn generate(a: &mut u64) -> Self;
    fn checksum(&self) -> u64;
}

/// Consumes ceil(size_of::<T>()/8) xorshift words, little endian.
fn next_value<T: FromWords>(a: &mut u64) -> T {
    let mut bytes = 0u128;
    let mut i = 0;
    while i < std::mem::size_of::<T>() {
        bytes |= (*a as u128) << (i * 8);
        *a = xs(*a);
        i += 8;
    }
    T::from_words(bytes)
}

trait FromWords: Copy {
    fn from_words(v: u128) -> Self;
}

macro_rules! from_words {
    ($($ty:ty),*) => {$(
        impl FromWords for $ty {
            #[inline]
            fn from_words(v: u128) -> Self {
                v as Self
            }
        }
    )*};
}
from_words!(u32, u64, u128);

fn fill_payload<const N: usize>(payload: &mut [u8; N], a: &mut u64) {
    let mut i = 0;
    while i < N {
        let word = *a;
        *a = xs(*a);
        let n = (N - i).min(8);
        payload[i..i + n].copy_from_slice(&word.to_le_bytes()[..n]);
        i += 8;
    }
}

macro_rules! shape_primitive {
    ($($ty:ty),*) => {$(
        impl Shape for $ty {
            const NAME: &'static str = stringify!($ty);
            #[inline]
            fn generate(a: &mut u64) -> Self {
                next_value::<Self>(a)
            }
            #[inline]
            fn checksum(&self) -> u64 {
                *self as u64
            }
        }
    )*};
}
shape_primitive!(u32, u64, u128);

impl<K, const PAYLOAD: usize> Shape for Record<K, PAYLOAD>
where
    K: Key + FromWords + Ord + Copy + 'static,
{
    const NAME: &'static str = "record";
    #[inline]
    fn generate(a: &mut u64) -> Self {
        let key = next_value::<K>(a);
        let mut payload = [0u8; PAYLOAD];
        fill_payload(&mut payload, a);
        Self { key, payload }
    }
    #[inline]
    fn checksum(&self) -> u64 {
        (self.key.key().to_u128() as u64).wrapping_add(
            self.payload
                .iter()
                .fold(0u64, |acc, b| acc.wrapping_add(*b as u64)),
        )
    }
}

fn run<S: Shape>(label: &str, n: usize, reps: usize) {
    let mut data = vec![S::generate(&mut SEED.clone()); n];
    let mut times = Vec::with_capacity(reps);
    let mut sum = 0u64;

    for _ in 0..reps {
        let mut a = SEED;
        sum = 0;
        for elem in data.iter_mut() {
            *elem = S::generate(&mut a);
            sum = sum.wrapping_add(elem.checksum());
        }
        let before: u64 = data
            .iter()
            .fold(0u64, |acc, e| acc.wrapping_add(e.checksum()));

        let start = Instant::now();
        backend::core::sort(&mut data, 8);
        times.push(start.elapsed().as_nanos() as f64);

        assert!(data.windows(2).all(|w| w[0].key() <= w[1].key()), "not sorted");
        let after: u64 = data
            .iter()
            .fold(0u64, |acc, e| acc.wrapping_add(e.checksum()));
        assert_eq!(after, before, "checksum mismatch");
    }

    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = times[times.len() / 2];
    let per_elem = median / n as f64;
    println!("{label},{n},{median:.0},{per_elem:.3},{sum}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let which = args.get(1).map(|v| v.as_str()).unwrap_or("pair");
    let n: usize = args.get(2).map(|v| v.parse().unwrap()).unwrap_or(1_000_000);
    let reps: usize = args.get(3).map(|v| v.parse().unwrap()).unwrap_or(7);

    type Pair = Record<u32, 4>;
    type Rec32 = Record<u64, 24>;
    type Rec128 = Record<u64, 120>;

    match which {
        "pair" => run::<Pair>("pair", n, reps),
        "u64" => run::<u64>("u64", n, reps),
        "u128" => run::<u128>("u128", n, reps),
        "rec32" => run::<Rec32>("rec32", n, reps),
        "rec128" => run::<Rec128>("rec128", n, reps),
        other => panic!("unknown type {other}"),
    }
}
