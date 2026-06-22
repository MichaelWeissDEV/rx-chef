// Tests for the shake operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations shake::

use rxchef::operation::ArgValue;
use rxchef::operations::shake::SHAKE;
use rxchef::Operation;

#[test]
fn test_shake128_basic() {
    let operation = SHAKE;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(128.0), ArgValue::Num(32.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHAKE128 hash of "Hello, World!" (32 bytes)
    assert_eq!(output.len(), 64); // 32 bytes = 64 hex chars
}
#[test]
fn test_shake256_basic() {
    let operation = SHAKE;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(256.0), ArgValue::Num(32.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHAKE256 hash of "Hello, World!" (32 bytes)
    assert_eq!(output.len(), 64);
}
#[test]
fn test_shake_variable_size() {
    let operation = SHAKE;
    let input = b"test".to_vec();
    // Test with different sizes
    let result1 = operation
        .run(input.clone(), &[ArgValue::Num(256.0), ArgValue::Num(16.0)])
        .unwrap();
    let output1 = String::from_utf8(result1).unwrap();
    assert_eq!(output1.len(), 32); // 16 bytes = 32 hex chars
    let result2 = operation
        .run(input, &[ArgValue::Num(256.0), ArgValue::Num(64.0)])
        .unwrap();
    let output2 = String::from_utf8(result2).unwrap();
    assert_eq!(output2.len(), 128); // 64 bytes = 128 hex chars
}
#[test]
fn test_shake_empty_input() {
    let operation = SHAKE;
    let input = b"".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(256.0), ArgValue::Num(32.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output.len(), 64);
}
#[test]
fn test_shake_zero_size_error() {
    let operation = SHAKE;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(256.0), ArgValue::Num(0.0)]; // Invalid - zero size
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_shake_invalid_capacity() {
    let operation = SHAKE;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(100.0), ArgValue::Num(32.0)]; // Invalid capacity
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
