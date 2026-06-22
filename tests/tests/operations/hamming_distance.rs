// Tests for the hamming_distance operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations hamming_distance::

use rxchef::operation::ArgValue;
use rxchef::operations::hamming_distance::HammingDistance;
use rxchef::Operation;

#[test]
fn test_hamming_distance_basic() {
    let operation = HammingDistance;
    let input = b"00000001\n\n00000011";
    let result = operation
        .run(input.to_vec(), &[ArgValue::Str("\\n\\n".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "1");
}
#[test]
fn test_hamming_distance_same() {
    let operation = HammingDistance;
    let input = b"hello\n\nhello";
    let result = operation
        .run(input.to_vec(), &[ArgValue::Str("\n\n".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "0");
}
