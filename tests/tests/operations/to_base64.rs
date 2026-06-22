// Tests for the to_base64 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_base64::

use rxchef::operations::to_base64::ToBase64;
use rxchef::Operation;

fn run(input: &[u8], args: &[rxchef::operation::ArgValue]) -> Vec<u8> {
    let op = ToBase64;
    op.run(input.to_vec(), args).unwrap()
}

#[test]
fn test_to_base64_empty_input() {
    let op = ToBase64;
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_to_base64_simple_string() {
    let input = "Hello World".as_bytes();
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(input, &args);
    assert_eq!(result, "SGVsbG8gV29ybGQ=".as_bytes());
}

#[test]
fn test_to_base64_binary_data() {
    let input = vec![0x00, 0x01, 0x02];
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(&input, &args);
    assert_eq!(result, "AAEC".as_bytes());
}

#[test]
fn test_to_base64_with_padding() {
    let input = "test".as_bytes(); // Should require padding
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(input, &args);
    assert_eq!(result, "dGVzdA==".as_bytes());
}

#[test]
fn test_to_base64_custom_alphabet() {
    let input = "test".as_bytes();
    // Use a truly custom alphabet that won't be expanded
    let custom_alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/";
    let args = [rxchef::operation::ArgValue::Str(custom_alphabet.to_string())];
    let result = run(input, &args);
    // Standard base64 of "test" is "dGVzdA==", with custom alphabet it should be different
    assert_ne!(result, "dGVzdA==".as_bytes());
    // Should use the custom alphabet characters
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.len() > 0);
}

#[test]
fn test_to_base64_long_input() {
    let input = "This is a longer test string that should be encoded properly".as_bytes();
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(input, &args);
    let expected = "VGhpcyBpcyBhIGxvbmdlciB0ZXN0IHN0cmluZyB0aGF0IHNob3VsZCBiZSBlbmNvZGVkIHByb3Blcmx5".as_bytes();
    assert_eq!(result, expected);
}

#[test]
fn test_to_base64_unicode_input() {
    let input = "Hello 世界".as_bytes(); // UTF-8 encoded
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(input, &args);
    assert_eq!(result, "SGVsbG8g5LiW55WM".as_bytes());
}

#[test]
fn test_to_base64_single_byte() {
    let input = vec![0x41]; // 'A'
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(&input, &args);
    assert_eq!(result, "QQ==".as_bytes());
}

#[test]
fn test_to_base64_two_bytes() {
    let input = vec![0x41, 0x42]; // 'AB'
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(&input, &args);
    assert_eq!(result, "QUI=".as_bytes());
}

#[test]
fn test_to_base64_three_bytes() {
    let input = vec![0x41, 0x42, 0x43]; // 'ABC'
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(&input, &args);
    assert_eq!(result, "QUJD".as_bytes());
}

#[test]
fn test_to_base64_default_alphabet() {
    let input = "test".as_bytes();
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(input, &args);
    assert_eq!(result, "dGVzdA==".as_bytes());
}

#[test]
fn test_to_base64_invalid_alphabet() {
    let op = ToBase64;
    let input = "test".as_bytes();
    let invalid_alphabet = "short"; // Less than 64 characters
    let args = [rxchef::operation::ArgValue::Str(invalid_alphabet.to_string())];
    let result = op.run(input.to_vec(), &args);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, rxchef::OperationError::InvalidArgument { .. }));
    }
}

#[test]
fn test_to_base64_special_characters() {
    let input = vec![0x00, 0x01, 0x7F, 0x80, 0xFF];
    let args = [rxchef::operation::ArgValue::Str("A-Za-z0-9+/=".to_string())];
    let result = run(&input, &args);
    assert_eq!(result, "AAF/gP8=".as_bytes());
}
