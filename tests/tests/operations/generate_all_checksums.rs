// Tests for the generate_all_checksums operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_all_checksums::
//
// Expected values are derived from the algorithm definitions, independently of
// rx-chef:
//   Adler-32   RFC 1950 section 9 (cross-checked against zlib)
//   Fletcher   RFC 1146 (modular sums, checked against a direct computation)
//
// These tests previously asserted only that the output *mentioned* an
// algorithm name and contained the substring "00", which no checksum value
// could fail.

use rxchef::operation::ArgValue;
use rxchef::operations::generate_all_checksums::GenerateAllChecksums;
use rxchef::Operation;

fn all_checksums(input: &[u8]) -> String {
    let args = [ArgValue::Str("All".to_string()), ArgValue::Bool(true)];
    String::from_utf8(GenerateAllChecksums.run(input.to_vec(), &args).unwrap()).unwrap()
}

fn checksum_of(output: &str, algorithm: &str) -> String {
    output
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            (name.trim() == algorithm).then(|| value.trim().to_string())
        })
        .unwrap_or_else(|| panic!("no {algorithm} line in:\n{output}"))
}

#[test]
fn test_generate_all_checksums_known_values_for_hello() {
    let output = all_checksums(b"hello");
    // Adler-32: two 16-bit sums modulo 65521, high half first (RFC 1950).
    assert_eq!(checksum_of(&output, "Adler-32"), "062c0215");
    // Fletcher-8 and -16: modular sums over 15 and 255 respectively.
    assert_eq!(checksum_of(&output, "Fletcher-8"), "07");
    assert_eq!(checksum_of(&output, "Fletcher-16"), "2d16");
}

#[test]
fn test_generate_all_checksums_adler32_matches_the_rfc_1950_definition() {
    // Adler-32 of an empty message is 1: sum1 starts at 1, sum2 at 0.
    assert_eq!(checksum_of(&all_checksums(b""), "Adler-32"), "00000001");
    // "Wikipedia" is the worked example in the algorithm's documentation.
    assert_eq!(
        checksum_of(&all_checksums(b"Wikipedia"), "Adler-32"),
        "11e60398"
    );
}

#[test]
fn test_generate_all_checksums_lists_every_supported_algorithm() {
    let output = all_checksums(b"abc");
    for algorithm in [
        "Fletcher-8",
        "Fletcher-16",
        "Fletcher-32",
        "Fletcher-64",
        "Adler-32",
    ] {
        // `checksum_of` panics with the full output when a line is missing.
        let value = checksum_of(&output, algorithm);
        assert!(
            !value.is_empty() && value.chars().all(|c| c.is_ascii_hexdigit()),
            "{algorithm} produced a non-hex value: {value:?}"
        );
    }
}

#[test]
fn test_generate_all_checksums_widths_match_their_algorithm() {
    let output = all_checksums(b"abc");
    for (algorithm, hex_digits) in [
        ("Fletcher-8", 2),
        ("Fletcher-16", 4),
        ("Fletcher-32", 8),
        ("Fletcher-64", 16),
        ("Adler-32", 8),
    ] {
        assert_eq!(
            checksum_of(&output, algorithm).len(),
            hex_digits,
            "{algorithm} has the wrong width"
        );
    }
}

#[test]
fn test_generate_all_checksums_empty_input() {
    let output = all_checksums(b"");
    assert_eq!(checksum_of(&output, "Adler-32"), "00000001");
    assert_eq!(checksum_of(&output, "Fletcher-8"), "00");
}

#[test]
fn test_generate_all_checksums_detects_a_single_bit_change() {
    // A checksum that misses a one-byte difference is not doing its job.
    assert_ne!(all_checksums(b"hello"), all_checksums(b"hellp"));
}

#[test]
fn test_generate_all_checksums_without_names_emits_only_values() {
    let args = [ArgValue::Str("All".to_string()), ArgValue::Bool(false)];
    let output =
        String::from_utf8(GenerateAllChecksums.run(b"hello".to_vec(), &args).unwrap()).unwrap();
    assert!(!output.contains("Adler-32:"), "names should be suppressed");
    assert!(
        output.contains("062c0215"),
        "the Adler-32 value should still be present: {output}"
    );
}

#[test]
fn test_generate_all_checksums_is_deterministic() {
    assert_eq!(all_checksums(b"hello"), all_checksums(b"hello"));
}
