// Tests for the bacon_cipher_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bacon_cipher_decode::

use rxchef::operations::bacon_cipher_decode::BaconCipherDecode;
use rxchef::Operation;

#[test]
fn test_bacon_cipher_decode_basic() {
    let operation = BaconCipherDecode;
    // 00000 = A, 00001 = B, 00010 = C
    let input = b"00000 00001 00010".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "ABC");
}
#[test]
fn test_bacon_cipher_decode_empty() {
    let operation = BaconCipherDecode;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "");
}
#[test]
fn test_bacon_cipher_decode_a_b() {
    let operation = BaconCipherDecode;
    // Test with 0/1 first to verify the core logic works
    let input = b"00000 00001 00110".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "ABG");
}
