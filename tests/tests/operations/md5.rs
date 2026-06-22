// Tests for the md5 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations md5::

use rxchef::operations::md5::MD5;
use rxchef::Operation;

#[test]
fn test_md5_basic() {
    let operation = MD5;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // MD5 hash of "Hello, World!"
    assert_eq!(output, "65a8e27d8879283831b664bd8b7f0ad4");
}
#[test]
fn test_md5_empty() {
    let operation = MD5;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // MD5 hash of empty string
    assert_eq!(output, "d41d8cd98f00b204e9800998ecf8427e");
}
#[test]
fn test_md5_binary() {
    let operation = MD5;
    let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output.len(), 32); // 128 bits = 32 hex chars
}
