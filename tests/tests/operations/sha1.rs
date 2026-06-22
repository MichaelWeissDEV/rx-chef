// Tests for the sha1 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sha1::

use rxchef::operation::ArgValue;
use rxchef::operations::sha1::SHA1;
use rxchef::Operation;

#[test]
fn test_sha1_basic() {
    let operation = SHA1;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA1 hash of "Hello, World!"
    assert_eq!(output, "0a0a9f2a6772942557ab5355d76af442f8f65e01");
}
#[test]
fn test_sha1_empty() {
    let operation = SHA1;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA1 hash of empty string
    assert_eq!(output, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
}
#[test]
fn test_sha1_rounds_validation() {
    let operation = SHA1;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(10.0)]; // Invalid - less than 16
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_sha1_binary() {
    let operation = SHA1;
    let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output.len(), 40); // 160 bits = 40 hex chars
}
