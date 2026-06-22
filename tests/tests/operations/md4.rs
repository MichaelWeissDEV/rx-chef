// Tests for the md4 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations md4::

use rxchef::operations::md4::MD4;
use rxchef::Operation;

#[test]
fn test_md4_basic() {
    let operation = MD4;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // MD4 hash of "Hello, World!"
    assert_eq!(output, "94e3cb0fa9aa7a5ee3db74b79e915989");
}
#[test]
fn test_md4_empty() {
    let operation = MD4;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // MD4 hash of empty string
    assert_eq!(output, "31d6cfe0d16ae931b73c59d7e0c089c0");
}
#[test]
fn test_md4_binary() {
    let operation = MD4;
    let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output.len(), 32); // 128 bits = 32 hex chars
}
