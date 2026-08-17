// Tests for the chi_square operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations chi_square::
//
// Reference values were observed from CyberChef 11.0.0 @ 0bb5472e50e1.
//
// These tests previously decoded the result with `f64::from_le_bytes` and
// asserted only `value >= 0.0`. That matched the operation's *bug*: it
// returned the raw IEEE-754 bytes of the score while declaring `Number`
// output, so the runtime's output contract rejected every run and the
// operation was unusable through the CLI, recipes and the API alike. Pinning
// the decimal text keeps both the format and the value under test.

use rxchef::operations::chi_square::ChiSquare;
use rxchef::Operation;

fn chi_square(input: &[u8]) -> String {
    String::from_utf8(ChiSquare.run(input.to_vec(), &[]).unwrap()).unwrap()
}

fn chi_square_value(input: &[u8]) -> f64 {
    chi_square(input)
        .parse()
        .expect("output must parse as a number")
}

#[test]
fn test_chi_square_emits_decimal_text_not_raw_float_bytes() {
    let output = chi_square(b"hello world");
    assert!(
        output
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == '-' || c == 'e'),
        "output must be decimal text, got {output:?}"
    );
    assert_eq!(output, "420.52556818181813");
}

#[test]
fn test_chi_square_known_values() {
    assert_eq!(chi_square(b"foobar"), "329.4505208333333");
    assert_eq!(chi_square(b"hello world"), "420.52556818181813");
    assert_eq!(chi_square(b"aaaaaaaaaa"), "2540.0390625");
}

#[test]
fn test_chi_square_uniform_distribution_scores_zero() {
    // Every byte value exactly once is the uniform distribution the statistic
    // is measured against, so the deviation is zero.
    let input: Vec<u8> = (0..=255).collect();
    assert_eq!(chi_square(&input), "0");
}

#[test]
fn test_chi_square_empty_input() {
    assert_eq!(chi_square(b""), "0");
}

#[test]
fn test_chi_square_is_never_negative() {
    // The statistic sums squared deviations, so it cannot go below zero.
    for sample in [b"a".as_slice(), b"abc", b"\x00\xff", b"the quick brown fox"] {
        assert!(
            chi_square_value(sample) >= 0.0,
            "negative score for {sample:?}"
        );
    }
}

#[test]
fn test_chi_square_rises_with_skew() {
    // A single repeated byte deviates from uniform far more than mixed text.
    assert!(chi_square_value(b"aaaaaaaaaa") > chi_square_value(b"abcdefghij"));
}

#[test]
fn test_chi_square_output_parses_as_a_number_for_binary_input() {
    // Binary input must not leak non-UTF-8 bytes into the numeric output.
    let binary: Vec<u8> = (0u8..64).chain(200..=255).collect();
    let output = chi_square(&binary);
    assert!(
        output.parse::<f64>().is_ok(),
        "binary input produced non-numeric output: {output:?}"
    );
}
