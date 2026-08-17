// Tests for the from_base32 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base32::

use rxchef::operations::from_base32::FromBase32;
use rxchef::Operation;

#[test]
fn test_from_base32_empty_input() {
    let op = FromBase32;
    let args = [
        rxchef::operation::ArgValue::Str("A-Z2-7".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_base32_simple_decode() {
    let op = FromBase32;
    let args = [
        rxchef::operation::ArgValue::Str("A-Z2-7".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // "HELLO" in Base32 (using proper encoding)
    let base32_input = "JBSWY3DP";
    let result = op.run(base32_input.as_bytes().to_vec(), &args).unwrap();
    // The actual decoded result
    assert_eq!(result, b"Hello");
}

#[test]
fn test_from_base32_with_padding() {
    let op = FromBase32;
    let args = [
        rxchef::operation::ArgValue::Str("A-Z2-7".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Base32 with padding
    let base32_input = "JBSWY3DPEB3W64TMMQ======";
    let result = op.run(base32_input.as_bytes().to_vec(), &args).unwrap();
    // The actual decoded result
    assert_eq!(result, b"Hello world");
}

#[test]
fn test_from_base32_invalid_characters() {
    let op = FromBase32;
    let args = [
        rxchef::operation::ArgValue::Str("A-Z2-7".to_string()),
        rxchef::operation::ArgValue::Bool(true), // Remove non-alphabet chars
    ];
    // Base32 with invalid characters
    let base32_input = "JB SW Y3DP!";
    let result = op.run(base32_input.as_bytes().to_vec(), &args).unwrap();
    // The actual decoded result after removing invalid chars
    assert_eq!(result, b"Hello");
}

#[test]
fn test_from_base32_different_alphabet() {
    let op = FromBase32;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-V".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // The extended-hex alphabet (RFC 4648 base32hex) maps digits before
    // letters, so "10436841" is a different byte string than it would be in
    // the standard alphabet.
    // Value cross-checked against CyberChef 11.0.0.
    let base32_input = "10436841";
    let decoded = op.run(base32_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, [0x08, 0x08, 0x33, 0x20, 0x81]);
}
