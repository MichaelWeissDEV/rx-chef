// Tests for the remove_null_bytes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations remove_null_bytes::

use rxchef::operations::remove_null_bytes::RemoveNullBytes;
use rxchef::Operation;

#[test]
fn test_remove_null_bytes_basic() {
    let operation = RemoveNullBytes;
    let input = vec![
        b'h', b'e', b'l', b'l', b'o', 0, b'w', b'o', b'r', b'l', b'd', 0,
    ];
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "helloworld");
}
#[test]
fn test_remove_null_bytes_no_nulls() {
    let operation = RemoveNullBytes;
    let input = b"hello world".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "hello world");
}
#[test]
fn test_remove_null_bytes_all_nulls() {
    let operation = RemoveNullBytes;
    let input = vec![0, 0, 0, 0, 0];
    let result = operation.run(input, &[]).unwrap();
    assert!(result.is_empty());
}
