// Tests for the argon2 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations argon2::

use rxchef::operations::argon2::Argon2;
use rxchef::Operation;

#[test]
fn test_argon2_basic() {
    let operation = Argon2;
    let input = b"password123".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Should contain the argon2 prefix
    assert!(output.contains("argon2"));
}
#[test]
fn test_argon2_empty() {
    let operation = Argon2;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("argon2"));
}
