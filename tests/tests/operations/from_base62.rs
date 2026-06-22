// Tests for the from_base62 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base62::

use rxchef::operations::from_base62::FromBase62;
use rxchef::Operation;

#[test]
fn test_from_base62_empty_input() {
    let op = FromBase62;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Za-z".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_base62_simple_decode() {
    let op = FromBase62;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Za-z".to_string()),
    ];
    // Simple Base62 encoding: "a" -> should decode to some bytes
    let base62_input = "a";
    let result = op.run(base62_input.as_bytes().to_vec(), &args);
    // Should successfully decode
    assert!(result.is_ok());
}

#[test]
fn test_from_base62_number_decode() {
    let op = FromBase62;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Za-z".to_string()),
    ];
    // Test with numeric input
    let base62_input = "123";
    let result = op.run(base62_input.as_bytes().to_vec(), &args);
    // Should successfully decode
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert!(!decoded.is_empty());
}

#[test]
fn test_from_base62_mixed_alphabet() {
    let op = FromBase62;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Za-z".to_string()),
    ];
    // Test with mixed alphabet characters
    let base62_input = "aBc123";
    let result = op.run(base62_input.as_bytes().to_vec(), &args);
    // Should successfully decode
    assert!(result.is_ok());
}

#[test]
fn test_from_base62_invalid_alphabet_length() {
    let op = FromBase62;
    let args = [
        rxchef::operation::ArgValue::Str("ABC".to_string()), // Too short
    ];
    let base62_input = "a";
    let result = op.run(base62_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid alphabet length
    assert!(result.is_err());
}
