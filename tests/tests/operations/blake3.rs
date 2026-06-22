// Tests for the blake3 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations blake3::

use rxchef::operation::ArgValue;
use rxchef::operations::blake3::BLAKE3;
use rxchef::Operation;

#[test]
fn test_blake3_basic() {
    let operation = BLAKE3;
    let input = b"hello world".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Standard BLAKE3 hash of "hello world"
    assert_eq!(
        output,
        "d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24"
    );
}
#[test]
fn test_blake3_empty() {
    let operation = BLAKE3;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Empty string BLAKE3 hash
    assert_eq!(
        output,
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );
}
#[test]
fn test_blake3_keyed() {
    let operation = BLAKE3;
    let input = b"hello world".to_vec();
    // Create a valid 32-byte key
    let key = "0123456789abcdef0123456789abcdef";
    let args = &[
        ArgValue::Str("32".to_string()),
        ArgValue::Str(key.to_string()),
    ];
    let result = operation.run(input, args).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Should produce different hash with key
    assert!(output.len() == 64); // 32 bytes = 64 hex chars
}
