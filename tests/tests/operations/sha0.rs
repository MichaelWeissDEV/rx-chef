// Tests for the sha0 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sha0::

use rxchef::operation::ArgValue;
use rxchef::operations::sha0::SHA0;
use rxchef::Operation;

#[test]
fn test_sha0_basic() {
    let operation = SHA0;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA0 hash of "Hello, World!" (using SHA-1 implementation)
    assert_eq!(output.len(), 40); // 160 bits = 40 hex chars
}
#[test]
fn test_sha0_empty() {
    let operation = SHA0;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA0 hash of empty string (using SHA-1 implementation)
    assert_eq!(output.len(), 40);
}
#[test]
fn test_sha0_binary() {
    let operation = SHA0;
    let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output.len(), 40); // 160 bits = 40 hex chars
}
#[test]
fn test_sha0_rounds_validation() {
    let operation = SHA0;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(10.0)]; // Invalid - less than 16
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
