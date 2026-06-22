// Tests for the drop_bytes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations drop_bytes::

use rxchef::operation::ArgValue;
use rxchef::operations::drop_bytes::DropBytes;
use rxchef::Operation;

#[test]
fn test_drop_bytes_basic() {
    let operation = DropBytes;
    let input = b"0123456789".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(2.0), ArgValue::Num(3.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "0156789");
}
#[test]
fn test_drop_bytes_negative_start() {
    let operation = DropBytes;
    let input = b"0123456789".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(-3.0), ArgValue::Num(2.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "01234569");
}
#[test]
fn test_drop_bytes_negative_length() {
    let operation = DropBytes;
    let input = b"0123456789".to_vec();
    let result = operation
        .run(input, &[ArgValue::Num(2.0), ArgValue::Num(-2.0)])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "23456789");
}
#[test]
fn test_drop_bytes_apply_to_lines() {
    let operation = DropBytes;
    let input = b"01234\nabcde\nXYZ".to_vec();
    let result = operation
        .run(
            input,
            &[ArgValue::Num(1.0), ArgValue::Num(2.0), ArgValue::Bool(true)],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "034\nade\nX");
}
