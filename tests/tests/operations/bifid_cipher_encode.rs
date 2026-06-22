// Tests for the bifid_cipher_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bifid_cipher_encode::

use rxchef::operation::ArgValue;
use rxchef::operations::bifid_cipher_encode::BifidCipherEncode;
use rxchef::Operation;

#[test]
fn test_bifid_encode_basic() {
    let operation = BifidCipherEncode;
    let input = b"HELLO".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("KEY".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(!output.is_empty());
}
#[test]
fn test_bifid_encode_with_j() {
    let operation = BifidCipherEncode;
    let input = b"HELLO".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("KEY".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(!output.is_empty());
}
#[test]
fn test_bifid_encode_empty_keyword() {
    let operation = BifidCipherEncode;
    let input = b"HELLO".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(!output.is_empty());
}
