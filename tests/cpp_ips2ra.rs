#![cfg(feature = "cpp_ips2ra")]

use sort_test_tools::ffi_types::{Record128, Record16};
use sort_test_tools::instantiate_sort_tests;

type TestSort = sort_research_rs::other::cpp_ips2ra::SortImpl;

// IPS²Ra is unstable; the suite skips stability checks for a name with that
// marker and still checks correctness, edge cases and the supported-type gate.
instantiate_sort_tests!(TestSort);

fn payload16(record: &Record16) -> u64 {
    u64::from_le_bytes(record.payload)
}

fn payload128(record: &Record128) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&record.payload[..8]);
    u64::from_le_bytes(bytes)
}

#[test]
fn record16_payloads_survive_the_c_abi() {
    let mut input: Vec<Record16> = (0..20_000)
        .map(|index| Record16::new((index % 17) as u64, (index as u64).to_le_bytes()))
        .collect();
    let mut expected_payloads: Vec<u64> = input.iter().map(payload16).collect();

    sort_research_rs::other::cpp_ips2ra::sort(&mut input);

    assert!(input.windows(2).all(|pair| pair[0].key <= pair[1].key));
    let mut payloads: Vec<u64> = input.iter().map(payload16).collect();
    payloads.sort_unstable();
    expected_payloads.sort_unstable();
    assert_eq!(payloads, expected_payloads);
}

#[test]
fn record128_payloads_survive_the_c_abi() {
    let mut input: Vec<Record128> = (0..5_000)
        .map(|index| {
            let mut payload = [0u8; 120];
            payload[..8].copy_from_slice(&(index as u64).to_le_bytes());
            Record128::new((index % 17) as u64, payload)
        })
        .collect();
    let mut expected_payloads: Vec<u64> = input.iter().map(payload128).collect();

    sort_research_rs::other::cpp_ips2ra::sort(&mut input);

    assert!(input.windows(2).all(|pair| pair[0].key <= pair[1].key));
    let mut payloads: Vec<u64> = input.iter().map(payload128).collect();
    payloads.sort_unstable();
    expected_payloads.sort_unstable();
    assert_eq!(payloads, expected_payloads);
}
