#![cfg(feature = "cpp_intel_avx512")]

use sort_research_rs::other::cpp_intel_avx512;

#[test]
fn u64_uses_unsigned_order() {
    // The old adapter reinterpreted uint64_t as int64_t, which puts 1 << 63
    // before 0. Build a descending unsigned input so a signed comparison is
    // visibly wrong.
    let mut values: Vec<u64> = (0..200_000)
        .rev()
        .map(|index| (index as u64) << 40)
        .collect();
    values.extend([0, 1 << 63, (1 << 63) - 1, u64::MAX, u64::MAX - 1]);
    values.sort_unstable_by(|a, b| b.cmp(a));

    cpp_intel_avx512::sort(&mut values);
    assert!(values.windows(2).all(|pair| pair[0] <= pair[1]));
    assert_eq!(values[0], 0);
    assert_eq!(values[values.len() - 1], u64::MAX);
}

#[test]
fn u32_uses_unsigned_order() {
    let mut values: Vec<u32> = (0..200_000).rev().map(|index| index as u32).collect();
    values.extend([0, 1 << 31, (1 << 31) - 1, u32::MAX]);
    values.sort_unstable_by(|a, b| b.cmp(a));

    cpp_intel_avx512::sort(&mut values);
    assert!(values.windows(2).all(|pair| pair[0] <= pair[1]));
}
