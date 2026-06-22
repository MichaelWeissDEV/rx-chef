// Tests for the crc32 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations crc32::

use rxchef::operations::crc32::CRC32;
use rxchef::Operation;

#[test]
fn test_crc32_basic() {
    let operation = CRC32;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // CRC32 IEEE of "Hello, World!" with reflection enabled
    assert_eq!(output, "13B53C2F");
}
#[test]
fn test_crc32_empty() {
    let operation = CRC32;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // CRC32 IEEE of empty string = FFFFFFFF
    assert_eq!(output, "FFFFFFFF");
}
#[test]
fn test_crc32_binary() {
    let operation = CRC32;
    let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.len() == 8); // 32 bits = 8 hex chars
}
