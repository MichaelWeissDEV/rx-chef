// Tests for the index_of_coincidence operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations index_of_coincidence::
//
// Reference values were observed from CyberChef 11.0.0 @ 0bb5472e50e1.
//
// Like `chi_square`, these tests previously decoded the result with
// `f64::from_le_bytes` and asserted only a loose range. That matched the
// operation's bug — it returned raw IEEE-754 bytes while declaring `Number`
// output, so the runtime rejected every run against the output contract.

use rxchef::operations::index_of_coincidence::IndexOfCoincidence;
use rxchef::Operation;

fn ioc(input: &[u8]) -> String {
    String::from_utf8(IndexOfCoincidence.run(input.to_vec(), &[]).unwrap()).unwrap()
}

fn ioc_value(input: &[u8]) -> f64 {
    ioc(input).parse().expect("output must parse as a number")
}

#[test]
fn test_index_of_coincidence_emits_decimal_text_not_raw_float_bytes() {
    let output = ioc(b"hello world");
    assert_eq!(output, "0.08888888888888889");
    assert!(
        output.parse::<f64>().is_ok(),
        "output must be decimal text, got {output:?}"
    );
}

#[test]
fn test_index_of_coincidence_known_values() {
    assert_eq!(ioc(b"foobar"), "0.06666666666666667");
    assert_eq!(ioc(b"hello world"), "0.08888888888888889");
}

#[test]
fn test_index_of_coincidence_all_identical_letters_is_one() {
    // Every pair coincides, so the index reaches its maximum of 1.
    assert_eq!(ioc(b"aaaaaaaaaa"), "1");
}

#[test]
fn test_index_of_coincidence_all_distinct_letters_is_zero() {
    // No pair coincides, so the index is 0.
    assert_eq!(ioc(b"abcdefghij"), "0");
}

#[test]
fn test_index_of_coincidence_empty_input() {
    // The implementation floors the density at 2 to avoid dividing by zero.
    assert_eq!(ioc(b""), "0");
}

#[test]
fn test_index_of_coincidence_single_character() {
    assert_eq!(ioc(b"a"), "0");
}

#[test]
fn test_index_of_coincidence_stays_within_zero_and_one() {
    for sample in [
        b"a".as_slice(),
        b"attack at dawn",
        b"the quick brown fox jumps over the lazy dog",
        b"zzzzzzzzzzzzzzzz",
    ] {
        let value = ioc_value(sample);
        assert!(
            (0.0..=1.0).contains(&value),
            "index outside [0,1] for {sample:?}: {value}"
        );
    }
}

#[test]
fn test_index_of_coincidence_is_higher_for_natural_language_than_uniform_text() {
    // English has a markedly higher coincidence rate than evenly spread text,
    // which is what makes this statistic useful for classical cryptanalysis.
    let english =
        ioc_value(b"the quick brown fox jumps over the lazy dog and then some more english text");
    let spread = ioc_value(b"abcdefghijklmnopqrstuvwxyz");
    assert!(
        english > spread,
        "English {english} should exceed evenly spread text {spread}"
    );
}

