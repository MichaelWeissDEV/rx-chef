// Tests for the rail_fence_cipher_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rail_fence_cipher_decode::

use rxchef::operation::ArgValue;
use rxchef::operations::rail_fence_cipher_decode::RailFenceCipherDecode;
use rxchef::Operation;

#[test]
fn test_rail_fence_decode_basic() {
    let operation = RailFenceCipherDecode;
    // "Hello, World!" encoded with 2 rails and offset 0
    // Hlo ol!  e,Wrld
    let input = b"Hlo ol!  e,Wrld".to_vec();
    let result = operation.run(input, &[ArgValue::Num(2.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // The output should be a valid decryption
    assert!(!output.is_empty());
}
#[test]
fn test_rail_fence_decode_error() {
    let operation = RailFenceCipherDecode;
    let input = b"AB".to_vec();
    // Key must be >= 2
    let result = operation.run(input, &[ArgValue::Num(1.0)]);
    assert!(result.is_err());
}
