// Tests for the blake2s operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations blake2s::

use rxchef::operation::ArgValue;
use rxchef::operations::blake2s::BLAKE2s;
use rxchef::Operation;

#[test]
fn test_blake2s_basic() {
    let operation = BLAKE2s;
    let input = b"hello world".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Standard BLAKE2s-256 hash of "hello world" (default)
    assert_eq!(
        output,
        "9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b"
    );
}
#[test]
fn test_blake2s_empty() {
    let operation = BLAKE2s;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Empty string BLAKE2s-256 hash (default)
    assert_eq!(
        output,
        "69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9"
    );
}
#[test]
fn test_blake2s_keyed() {
    let operation = BLAKE2s;
    let input = b"hello world".to_vec();
    let args = &[
        ArgValue::Str("256".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("mykey".to_string()),
    ];
    let result = operation.run(input, args).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Should produce different hash with key
    assert!(output.len() == 64); // 256 bits = 64 hex chars
}
