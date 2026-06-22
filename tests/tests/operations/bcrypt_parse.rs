// Tests for the bcrypt_parse operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bcrypt_parse::

use rxchef::operations::bcrypt_parse::BcryptParse;
use rxchef::Operation;

#[test]
fn test_bcrypt_parse_basic() {
    let operation = BcryptParse;
    // Example bcrypt hash: $2a$10$somesalt1234567890abcd$ef
    // Format: $2a$10$ + 22 char salt + remaining hash
    let input = b"$2a$10$somesalt1234567890abcd$ef".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("Rounds: 10"));
    assert!(output.contains("Salt: $somesalt1234567890abcd"));
    assert!(output.contains("Password hash: "));
}
#[test]
fn test_bcrypt_parse_invalid_format() {
    let operation = BcryptParse;
    let input = b"not_a_valid_hash".to_vec();
    let result = operation.run(input, &[]);
    assert!(result.is_err());
}
#[test]
fn test_bcrypt_parse_empty() {
    let operation = BcryptParse;
    let input = b"".to_vec();
    let result = operation.run(input, &[]);
    assert!(result.is_err());
}
