// Tests for the bcrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bcrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::bcrypt::Bcrypt;
use rxchef::Operation;

#[test]
fn test_bcrypt_basic() {
    let operation = Bcrypt;
    let input = b"password123".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Bcrypt hash starts with $2a$, $2b$, or $2y$
    assert!(output.starts_with("$2"), "Hash should start with $2");
}
#[test]
fn test_bcrypt_with_custom_rounds() {
    let operation = Bcrypt;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(12.0)];
    let result = operation.run(input, &args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.starts_with("$2"), "Hash should start with $2");
}
#[test]
fn test_bcrypt_invalid_rounds() {
    let operation = Bcrypt;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(3.0)]; // Too low
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
