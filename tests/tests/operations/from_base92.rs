// Tests for the from_base92 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base92::

use rxchef::operations::from_base92::FromBase92;
use rxchef::Operation;

#[test]
fn test_from_base92_empty_input() {
    let op = FromBase92;
    let args = [];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_base92_simple_decode() {
    let op = FromBase92;
    let args = [];
    // Simple Base92 encoding
    let base92_input = "!!";
    let result = op.run(base92_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_base92_pair_decode() {
    let op = FromBase92;
    let args = [];
    // Base92 with character pairs
    let base92_input = "##";
    let result = op.run(base92_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert!(!decoded.is_empty());
}

#[test]
fn test_from_base92_mixed_chars() {
    let op = FromBase92;
    let args = [];
    // Base92 with mixed characters
    let base92_input = "!a#";
    let result = op.run(base92_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_base92_invalid_char() {
    let op = FromBase92;
    let args = [];
    // Base92 with invalid character
    let base92_input = "\u{0001}"; // Control character is not a valid Base92 character
    let result = op.run(base92_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid character
    assert!(result.is_err());
}
