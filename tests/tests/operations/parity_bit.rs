// Tests for the parity_bit operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parity_bit::

use rxchef::operation::ArgValue;
use rxchef::operations::parity_bit::ParityBit;
use rxchef::Operation;

#[test]
fn test_parity_bit_encode() {
    let operation = ParityBit;
    let input = b"\x01".to_vec(); // 00000001, one 1-bit -> odd parity
    let result = operation
        .run(input, &[ArgValue::Str("Even Parity".to_string())])
        .unwrap();
    // Even parity: add bit to make even number of 1s
    assert!(!result.is_empty());
}
#[test]
fn test_parity_bit_empty() {
    let operation = ParityBit;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    assert!(result.is_empty());
}
