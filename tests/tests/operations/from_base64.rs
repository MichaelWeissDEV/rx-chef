// Tests for the from_base64 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base64::

use rxchef::operations::from_base64::FromBase64;
use rxchef::Operation;

#[test]
fn test_from_base64_empty_input() {
    let op = FromBase64;
    let args = [
        rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_base64_standard_alphabet() {
    let op = FromBase64;
    let args = [
        rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
    ];
    // "SGVsbG8=" is "Hello" in Base64
    let base64_input = "SGVsbG8=";
    let result = op.run(base64_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(decoded, b"Hello");
}

#[test]
fn test_from_base64_url_safe_alphabet() {
    let op = FromBase64;
    let args = [
        rxchef::operation::ArgValue::Str("A-Za-z0-9-_".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
    ];
    // URL-safe Base64
    let base64_input = "SGVsbG8-";
    let result = op.run(base64_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_base64_remove_non_alphabet_chars() {
    let op = FromBase64;
    let args = [
        rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string()),
        rxchef::operation::ArgValue::Bool(true), // Remove non-alphabet chars
        rxchef::operation::ArgValue::Bool(false),
    ];
    // Base64 with invalid characters
    let base64_input = "SG!Vsb@G8=";
    let result = op.run(base64_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_base64_invalid_alphabet_length() {
    let op = FromBase64;
    let args = [
        rxchef::operation::ArgValue::Str("ABC".to_string()), // Too short
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let base64_input = "SGVsbG8=";
    let result = op.run(base64_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid alphabet length
    assert!(result.is_err());
}
