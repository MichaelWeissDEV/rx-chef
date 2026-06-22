// Tests for the add operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations add::

use rxchef::operation::ArgValue;
use rxchef::operations::add::AddOp;
use rxchef::Operation;

#[test]
fn test_add_with_hex_key() {
    let op = AddOp;
    let input = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
    let args = [ArgValue::Str("0x41".to_string())]; // Key = 0x41 = 65
    let result = op.run(input.clone(), &args).unwrap();
    // (0x48 + 0x41) % 256 = 0x89
    // (0x65 + 0x41) % 256 = 0xa6
    // etc.
    assert_eq!(result, vec![0x89, 0xa6, 0xad, 0xad, 0xb0]);
}
#[test]
fn test_add_with_decimal_key() {
    let op = AddOp;
    let input = vec![100, 200];
    let args = [ArgValue::Str("50".to_string())]; // Key = 50
    let result = op.run(input, &args).unwrap();
    // (100 + 50) % 256 = 150
    // (200 + 50) % 256 = 250
    assert_eq!(result, vec![150, 250]);
}
#[test]
fn test_add_with_string_key() {
    let op = AddOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("a".to_string())]; // Key = 'a' = 97 (UTF-8)
    let result = op.run(input.clone(), &args).unwrap();
    // (104 + 97) % 256 = 19
    // (101 + 97) % 256 = 22
    // etc.
    assert_eq!(
        result,
        vec![104 + 97, 101 + 97, 108 + 97, 108 + 97, 111 + 97]
    );
}
#[test]
fn test_add_empty_key() {
    let op = AddOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
#[test]
fn test_add_wraps_around() {
    let op = AddOp;
    let input = vec![200, 100];
    let args = [ArgValue::Str("100".to_string())]; // Key = 100
    let result = op.run(input, &args).unwrap();
    // (200 + 100) % 256 = 44
    // (100 + 100) % 256 = 200
    assert_eq!(result, vec![44, 200]);
}
#[test]
fn test_add_key_reuse() {
    let op = AddOp;
    let input = vec![1, 2, 3, 4, 5, 6];
    let args = [ArgValue::Str("10".to_string())]; // Key = 10
    let result = op.run(input, &args).unwrap();
    // Key 10 is reused: (1+10)%256, (2+10)%256, (3+10)%256, (4+10)%256, (5+10)%256, (6+10)%256
    assert_eq!(result, vec![11, 12, 13, 14, 15, 16]);
}
