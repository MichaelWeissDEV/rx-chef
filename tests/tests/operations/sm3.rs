// Tests for the sm3 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sm3::

use rxchef::operation::ArgValue;
use rxchef::operations::sm3::SM3;
use rxchef::Operation;

#[test]
fn test_sm3_empty() {
    let op = SM3;
    let result = op.run(b"".to_vec(), &[ArgValue::Str("Hex".to_string())]);
    assert!(result.is_ok());
    let hex_out = String::from_utf8(result.unwrap()).unwrap();
    // SM3 hash of empty string
    assert_eq!(
        hex_out,
        "1ab21d8355cfa17f8e61194831e81a8f22bec8c728fefb747ed035eb5082aa2b"
    );
}
#[test]
fn test_sm3_abc() {
    let op = SM3;
    let result = op.run(b"abc".to_vec(), &[ArgValue::Str("Hex".to_string())]);
    assert!(result.is_ok());
    let hex_out = String::from_utf8(result.unwrap()).unwrap();
    // SM3 hash of "abc" per the Chinese standard
    assert_eq!(
        hex_out,
        "66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0"
    );
}
#[test]
fn test_sm3_hex_length() {
    let op = SM3;
    let result = op
        .run(
            b"Hello, World!".to_vec(),
            &[ArgValue::Str("Hex".to_string())],
        )
        .unwrap();
    let hex_out = String::from_utf8(result).unwrap();
    // SM3 always produces 256-bit (64 hex chars)
    assert_eq!(hex_out.len(), 64);
}
#[test]
fn test_sm3_base64() {
    let op = SM3;
    let result = op.run(b"test".to_vec(), &[ArgValue::Str("Base64".to_string())]);
    assert!(result.is_ok());
    let b64 = String::from_utf8(result.unwrap()).unwrap();
    // Base64 of 32 bytes = 44 chars with padding
    assert_eq!(b64.len(), 44);
}
#[test]
fn test_sm3_raw_length() {
    let op = SM3;
    let result = op.run(b"test".to_vec(), &[ArgValue::Str("Raw".to_string())]);
    assert!(result.is_ok());
    // SM3 output is 32 bytes
    assert_eq!(result.unwrap().len(), 32);
}
#[test]
fn test_sm3_invalid_format() {
    let op = SM3;
    let result = op.run(b"test".to_vec(), &[ArgValue::Str("JSON".to_string())]);
    assert!(result.is_err());
}
#[test]
fn test_sm3_default_format() {
    let op = SM3;
    // No args => default "Hex"
    let result = op.run(b"test".to_vec(), &[]);
    assert!(result.is_ok());
    let hex_out = String::from_utf8(result.unwrap()).unwrap();
    assert_eq!(hex_out.len(), 64);
}
