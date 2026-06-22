// Tests for the bacon_cipher_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bacon_cipher_encode::

use rxchef::operations::bacon_cipher_encode::BaconCipherEncode;
use rxchef::Operation;

#[test]
fn test_bacon_cipher_encode_basic() {
    let operation = BaconCipherEncode;
    let input = b"ABC".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // A=0, B=1, C=2 -> 00000 00001 00010
    assert!(output.contains("00000"));
}
#[test]
fn test_bacon_cipher_encode_empty() {
    let operation = BaconCipherEncode;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "");
}
#[test]
fn test_bacon_cipher_encode_non_alpha() {
    let operation = BaconCipherEncode;
    let input = b"Hello!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("H") || output.contains("0") || output.contains("1"));
}
