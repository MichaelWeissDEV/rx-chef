// Tests for the crc_checksum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations crc_checksum::

use rxchef::operation::ArgValue;
use rxchef::operations::crc_checksum::CrcChecksum;
use rxchef::Operation;

fn run_with_algo(algo: &str, data: &[u8]) -> String {
    let op = CrcChecksum;
    let args = [ArgValue::Str(algo.to_string())];
    let result = op.run(data.to_vec(), &args).unwrap();
    String::from_utf8(result).unwrap()
}
#[test]
fn test_crc32_basic() {
    // CRC-32 of "123456789" is 0xcbf43926
    let result = run_with_algo("CRC-32", b"123456789");
    assert_eq!(result, "cbf43926");
}
#[test]
fn test_crc16_arc() {
    // CRC-16/ARC of "123456789" is 0xbb3d
    let result = run_with_algo("CRC-16", b"123456789");
    assert_eq!(result, "bb3d");
}
#[test]
fn test_crc8_smbus() {
    // CRC-8/SMBUS of "123456789" is 0xf4
    let result = run_with_algo("CRC-8", b"123456789");
    assert_eq!(result, "f4");
}
#[test]
fn test_unknown_algorithm() {
    let op = CrcChecksum;
    let args = [ArgValue::Str("CRC-99/FAKE".to_string())];
    let result = op.run(b"test".to_vec(), &args);
    assert!(result.is_err());
}
#[test]
fn test_default_algorithm() {
    let op = CrcChecksum;
    let result = op.run(b"123456789".to_vec(), &[]).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert_eq!(s, "cbf43926");
}
