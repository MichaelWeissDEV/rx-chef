// Tests for the md2 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations md2::

use rxchef::operation::ArgValue;
use rxchef::operations::md2::MD2;
use rxchef::Operation;

#[test]
fn test_md2_basic() {
    let operation = MD2;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // MD2 hash of "Hello, World!"
    assert_eq!(output, "1c8f1e6a94aaa7145210bf90bb52871a");
}
#[test]
fn test_md2_empty() {
    let operation = MD2;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // MD2 hash of empty string
    assert_eq!(output, "8350e5a3e24c153df2275c9f80692773");
}
#[test]
fn test_md2_binary() {
    let operation = MD2;
    let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output.len(), 32); // 128 bits = 32 hex chars
}
#[test]
fn test_md2_rounds_validation() {
    let operation = MD2;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(-1.0)]; // Invalid - negative rounds
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
