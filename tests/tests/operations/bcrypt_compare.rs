// Tests for the bcrypt_compare operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bcrypt_compare::

use bcrypt::{hash, DEFAULT_COST};
use rxchef::operation::ArgValue;
use rxchef::operations::bcrypt_compare::BcryptCompare;
use rxchef::Operation;

#[test]
fn test_bcrypt_compare_match() {
    let operation = BcryptCompare;
    let password = "password123";
    let hash = hash(password, DEFAULT_COST).unwrap();
    let input = password.as_bytes().to_vec();
    let args = [ArgValue::Str(hash)];
    let result = operation.run(input, &args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.starts_with("Match:"));
}
#[test]
fn test_bcrypt_compare_no_match() {
    let operation = BcryptCompare;
    let hash = hash("correcthorsebatterystaple", DEFAULT_COST).unwrap();
    let input = b"wrongpassword".to_vec();
    let args = [ArgValue::Str(hash)];
    let result = operation.run(input, &args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "No match");
}
#[test]
fn test_bcrypt_compare_invalid_hash() {
    let operation = BcryptCompare;
    let input = b"test".to_vec();
    let args = [ArgValue::Str("not_a_valid_hash".to_string())];
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
