// Tests for the rail_fence_cipher_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rail_fence_cipher_encode::

use rxchef::operation::ArgValue;
use rxchef::operations::rail_fence_cipher_encode::RailFenceCipherEncode;
use rxchef::Operation;

#[test]
fn test_rail_fence_encode_basic() {
    let operation = RailFenceCipherEncode;
    let input = b"HELLO".to_vec();
    let result = operation.run(input, &[ArgValue::Num(2.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(!output.is_empty());
}
#[test]
fn test_rail_fence_encode_error() {
    let operation = RailFenceCipherEncode;
    let input = b"AB".to_vec();
    // Key must be >= 2
    let result = operation.run(input, &[ArgValue::Num(1.0)]);
    assert!(result.is_err());
}
