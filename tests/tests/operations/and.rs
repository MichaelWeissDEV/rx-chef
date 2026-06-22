// Tests for the and operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations and::

use rxchef::operation::ArgValue;
use rxchef::operations::and::AndOp;
use rxchef::Operation;

#[test]
fn test_and_with_hex_key() {
    let op = AndOp;
    let input = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
    let args = [ArgValue::Str("0x41".to_string())]; // Key = 0x41 = 65
    let result = op.run(input.clone(), &args).unwrap();
    // 0x48 & 0x41 = 0x40
    // 0x65 & 0x41 = 0x41
    // 0x6c & 0x41 = 0x40
    // 0x6c & 0x41 = 0x40
    // 0x6f & 0x41 = 0x41
    assert_eq!(result, vec![0x40, 0x41, 0x40, 0x40, 0x41]);
}
#[test]
fn test_and_with_decimal_key() {
    let op = AndOp;
    let input = vec![100, 200];
    let args = [ArgValue::Str("50".to_string())]; // Key = 50
    let result = op.run(input, &args).unwrap();
    // 100 & 50 = 32
    // 200 & 50 = 0
    assert_eq!(result, vec![32, 0]);
}
#[test]
fn test_and_with_string_key() {
    let op = AndOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("a".to_string())]; // Key = 'a' = 97
    let result = op.run(input.clone(), &args).unwrap();
    // 104 & 97 = 96
    // 101 & 97 = 97
    // 108 & 97 = 96
    // 108 & 97 = 96
    // 111 & 97 = 97
    assert_eq!(result, vec![96, 97, 96, 96, 97]);
}
#[test]
fn test_and_empty_key() {
    let op = AndOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
