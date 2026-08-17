// Tests for the pseudo_random_number_generator operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations pseudo_random_number_generator::
//
// This operation is non-deterministic by design, so these tests assert the
// shape and size of the output rather than fixed values.

use rxchef::operation::ArgValue;
use rxchef::operations::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use rxchef::Operation;

fn generate(bytes: &str, output_as: &str) -> Vec<u8> {
    let args = [
        ArgValue::Str(bytes.to_string()),
        ArgValue::Str(output_as.to_string()),
    ];
    PseudoRandomNumberGenerator.run(Vec::new(), &args).unwrap()
}

fn generate_text(bytes: &str, output_as: &str) -> String {
    String::from_utf8(generate(bytes, output_as)).unwrap()
}

#[test]
fn test_prng_hex_output_length_matches_requested_bytes() {
    // Two hexadecimal digits per byte.
    assert_eq!(generate_text("16", "Hex").trim().len(), 32);
    assert_eq!(generate_text("32", "Hex").trim().len(), 64);
    assert_eq!(generate_text("1", "Hex").trim().len(), 2);
}

#[test]
fn test_prng_hex_output_is_hexadecimal() {
    let output = generate_text("32", "Hex");
    assert!(
        output.trim().chars().all(|c| c.is_ascii_hexdigit()),
        "non-hex characters in output: {output}"
    );
}

#[test]
fn test_prng_raw_output_length_matches_requested_bytes() {
    assert_eq!(generate("16", "Raw").len(), 16);
    assert_eq!(generate("64", "Raw").len(), 64);
}

#[test]
fn test_prng_integer_output_is_a_number() {
    let output = generate_text("4", "Integer");
    assert!(
        output.trim().parse::<u128>().is_ok(),
        "integer output is not parseable: {output}"
    );
}

#[test]
fn test_prng_byte_array_output_is_a_bracketed_octet_list() {
    let output = generate_text("8", "Byte array");
    let body = output
        .trim()
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .unwrap_or_else(|| panic!("byte array output must be bracketed, got {output:?}"));
    let values: Vec<&str> = body.split(',').map(str::trim).collect();
    assert_eq!(values.len(), 8);
    for value in values {
        let octet = value
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("not a number: {value:?} in {output:?}"));
        assert!(octet <= 255, "value outside byte range: {octet}");
    }
}

#[test]
fn test_prng_zero_bytes_produces_empty_output() {
    assert_eq!(generate("0", "Raw").len(), 0);
    assert_eq!(generate_text("0", "Hex").trim(), "");
}

#[test]
fn test_prng_successive_calls_differ() {
    // Not a randomness quality test: this only catches a generator stuck at a
    // constant value. 32 bytes makes an accidental collision negligible.
    let first = generate("32", "Raw");
    let second = generate("32", "Raw");
    assert_ne!(first, second, "generator returned identical output twice");
}

#[test]
fn test_prng_ignores_its_input() {
    // The operation generates rather than transforms, so input must not
    // change the output size.
    let args = [ArgValue::Str("16".to_string()), ArgValue::Str("Raw".into())];
    let from_input = PseudoRandomNumberGenerator
        .run(b"ignored input".to_vec(), &args)
        .unwrap();
    assert_eq!(from_input.len(), 16);
}
