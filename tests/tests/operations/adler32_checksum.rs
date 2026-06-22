// Tests for the adler32_checksum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations adler32_checksum::

use rxchef::operations::adler32_checksum::Adler32Checksum;
use rxchef::Operation;

#[test]
fn test_adler32_basic() {
    let op = Adler32Checksum;
    let input = b"123456789".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "091e01de");
}
#[test]
fn test_adler32_empty_input() {
    let op = Adler32Checksum;
    let input = b"".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Empty input: no bytes processed, a=1, b=0, result = (0 << 16) | 1 = 0x00000001
    assert_eq!(result_str, "00000001");
}
#[test]
fn test_adler32_single_byte() {
    let op = Adler32Checksum;
    let input = b"A".to_vec(); // ASCII 65
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // a = (1 + 65) % 65521 = 66
    // b = (0 + 66) % 65521 = 66
    // result = (66 << 16) | 66 = 0x00420042
    assert_eq!(result_str, "00420042");
}
