// Tests for the or operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations or::

use rxchef::operation::ArgValue;
use rxchef::operations::or::OrOp;
use rxchef::Operation;

#[test]
fn test_or_with_hex_key() {
    let op = OrOp;
    let input = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
    let args = [ArgValue::Str("0x41".to_string())]; // Key = 0x41 = 65
    let result = op.run(input.clone(), &args).unwrap();
    // 0x48 | 0x41 = 0x49
    // 0x65 | 0x41 = 0x65
    // etc.
    assert_eq!(result, vec![0x49, 0x65, 0x6d, 0x6d, 0x6f]);
}
#[test]
fn test_or_with_decimal_key() {
    let op = OrOp;
    let input = vec![100, 200];
    let args = [ArgValue::Str("50".to_string())]; // Key = 50
    let result = op.run(input, &args).unwrap();
    // 100 | 50 = 118
    // 200 | 50 = 250
    assert_eq!(result, vec![118, 250]);
}
#[test]
fn test_or_with_string_key() {
    let op = OrOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("a".to_string())]; // Key = 'a' = 97
    let result = op.run(input.clone(), &args).unwrap();
    // 104 | 97 = 105
    // 101 | 97 = 101
    // 108 | 97 = 109
    // 108 | 97 = 109
    // 111 | 97 = 111
    assert_eq!(result, vec![105, 101, 109, 109, 111]);
}
#[test]
fn test_or_empty_key() {
    let op = OrOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
