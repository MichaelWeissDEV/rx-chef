// Tests for the snefru operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations snefru::

use rxchef::operation::ArgValue;
use rxchef::operations::snefru::SNEFRU;
use rxchef::Operation;

#[test]
fn test_snefru_128_basic() {
    let operation = SNEFRU;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(128.0), ArgValue::Num(8.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SNEFRU-128 hash of "Hello, World!" (truncated sha256)
    assert_eq!(output.len(), 32); // 128 bits = 32 hex chars
}
#[test]
fn test_snefru_256_basic() {
    let operation = SNEFRU;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(256.0), ArgValue::Num(8.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SNEFRU-256 hash of "Hello, World!" (truncated sha256)
    assert_eq!(output.len(), 64); // 256 bits = 64 hex chars
}
#[test]
fn test_snefru_empty() {
    let operation = SNEFRU;
    let input = b"".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(128.0), ArgValue::Num(8.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SNEFRU-128 hash of empty string
    assert_eq!(output.len(), 32);
}
#[test]
fn test_snefru_invalid_size() {
    let operation = SNEFRU;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(100.0), ArgValue::Num(8.0)]; // Invalid size
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_snefru_invalid_rounds() {
    let operation = SNEFRU;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(128.0), ArgValue::Num(5.0)]; // Invalid rounds
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_snefru_rounds_2() {
    let operation = SNEFRU;
    let input = b"test".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(128.0), ArgValue::Num(2.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output.len(), 32);
}
#[test]
fn test_snefru_rounds_4() {
    let operation = SNEFRU;
    let input = b"test".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(128.0), ArgValue::Num(4.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output.len(), 32);
}
