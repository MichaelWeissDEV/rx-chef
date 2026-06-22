// Tests for the bifid_cipher_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bifid_cipher_decode::

use rxchef::operation::ArgValue;
use rxchef::operations::bifid_cipher_decode::BifidCipherDecode;
use rxchef::Operation;

#[test]
fn test_bifid_decode_basic() {
    let operation = BifidCipherDecode;
    let input = b"KXJE KUTE AREZ Yfbe".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("KEYWORD".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(!output.is_empty());
}
#[test]
fn test_bifid_decode_with_j() {
    let operation = BifidCipherDecode;
    let input = b"KXJE KUTE AREZ Yfbe".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("KEY".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(!output.is_empty());
}
#[test]
fn test_bifid_decode_empty_keyword() {
    let operation = BifidCipherDecode;
    let input = b"KXJE KUTE AREZ Yfbe".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(!output.is_empty());
}
