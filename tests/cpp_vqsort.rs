#![cfg(feature = "cpp_vqsort")]

use sort_research_rs::other::cpp_vqsort;

#[test]
fn u128_uses_the_complete_key() {
    let input: Vec<u128> = (0..20_000)
        .map(|index| {
            let index = index as u128;
            (index % 7) << 96 | (index % 251) << 32 | (index % 65537)
        })
        .chain([
            0,
            1,
            u64::MAX as u128,
            (1u128 << 64) - 1,
            1u128 << 64,
            (1u128 << 64) + 1,
            1u128 << 127,
            u128::MAX - 1,
            u128::MAX,
        ])
        .collect();

    let mut sorted = input.clone();
    cpp_vqsort::sort(&mut sorted);

    let mut expected = input;
    expected.sort_unstable();
    assert_eq!(sorted, expected);

    // A 64-bit-only sort would leave the high half unordered.
    assert!(sorted.windows(2).all(|pair| pair[0] <= pair[1]));
}

#[test]
fn u32_and_u64_cover_their_unsigned_ranges() {
    let mut values: Vec<u64> = (0..100_000).map(|index| index as u64).collect();
    values.extend([1u64 << 63, (1u64 << 63) - 1, u64::MAX, u64::MAX - 1]);
    cpp_vqsort::sort(&mut values);
    assert!(values.windows(2).all(|pair| pair[0] <= pair[1]));

    let mut values: Vec<u32> = (0..100_000).map(|index| index as u32).collect();
    values.extend([1u32 << 31, (1u32 << 31) - 1, u32::MAX, u32::MAX - 1]);
    cpp_vqsort::sort(&mut values);
    assert!(values.windows(2).all(|pair| pair[0] <= pair[1]));
}
