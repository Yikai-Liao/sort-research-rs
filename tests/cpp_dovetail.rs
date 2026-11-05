#![cfg(feature = "cpp_dovetail")]

use sort_test_tools::ffi_types::{Record128, Record16};
use sort_test_tools::instantiate_sort_tests;

type TestSort = sort_research_rs::other::cpp_dovetail::SortImpl;

// DovetailSort is stable, so the suite must run the stability checks as well as
// the correctness checks for every supported type.
instantiate_sort_tests!(TestSort);

fn record16s(len: usize) -> Vec<Record16> {
    (0..len)
        .map(|index| Record16::new((index % 17) as u64, (index as u64).to_le_bytes()))
        .collect()
}

fn record128s(len: usize) -> Vec<Record128> {
    (0..len)
        .map(|index| {
            let mut payload = [0u8; 120];
            payload[..8].copy_from_slice(&(index as u64).to_le_bytes());
            Record128::new((index % 17) as u64, payload)
        })
        .collect()
}

fn assert_stable_records<T: Ord + Clone + std::fmt::Debug>(
    input: &[T],
    key: impl Fn(&T) -> u64,
    ordinal: impl Fn(&T) -> u64,
) {
    let mut sorted = input.to_vec();
    sort_research_rs::other::cpp_dovetail::sort(&mut sorted);

    assert!(sorted.windows(2).all(|pair| key(&pair[0]) <= key(&pair[1])));
    for run in sorted.chunk_by(|a, b| key(a) == key(b)) {
        assert!(
            run.windows(2)
                .all(|pair| ordinal(&pair[0]) < ordinal(&pair[1])),
            "equal keys did not preserve input order: {run:?}"
        );
    }
}

#[test]
fn record16_is_stable() {
    let input = record16s(20_000);
    assert_stable_records(
        &input,
        |record| record.key,
        |record| u64::from_le_bytes(record.payload),
    );
}

#[test]
fn record128_is_stable() {
    let input = record128s(20_000);
    assert_stable_records(
        &input,
        |record| record.key,
        |record| {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&record.payload[..8]);
            u64::from_le_bytes(bytes)
        },
    );
}
