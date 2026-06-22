// Tests for the xor operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xor::

use rxchef::operation::ArgValue;
use rxchef::operations::xor::XorOp;
use rxchef::Operation;

#[test]
fn test_xor_with_hex_key() {
    let op = XorOp;
    let input = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
    let args = [ArgValue::Str("0x41".to_string())]; // Key = 0x41 = 65
    let result = op.run(input.clone(), &args).unwrap();
    // 0x48 ^ 0x41 = 0x09
    // 0x65 ^ 0x41 = 0x24
    // 0x6c ^ 0x41 = 0x2d
    // 0x6c ^ 0x41 = 0x2d
    // 0x6f ^ 0x41 = 0x2e
    assert_eq!(result, vec![0x09, 0x24, 0x2d, 0x2d, 0x2e]);
}
#[test]
fn test_xor_with_string_key() {
    let op = XorOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("a".to_string())]; // Key = 'a' = 97
    let result = op.run(input.clone(), &args).unwrap();
    // 104 ^ 97 = 9
    // 101 ^ 97 = 4
    // 108 ^ 97 = 13
    // 108 ^ 97 = 13
    // 111 ^ 97 = 14
    assert_eq!(result, vec![9, 4, 13, 13, 14]);
}
#[test]
fn test_xor_empty_key() {
    let op = XorOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
#[test]
fn test_xor_null_preserving() {
    let op = XorOp;
    let input = vec![0x00, 0x41, 0x42]; // null byte, then 'A', 'B'
    let args = [
        ArgValue::Str("0x41".to_string()),
        ArgValue::Str("Standard".to_string()),
        ArgValue::Bool(true),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    // 0x00 preserved (null byte)
    // 0x41 ^ 0x41 = 0x00 (same as key) - but 0x41 == key_byte so should be preserved
    // Wait - null_preserving means: skip if byte == 0 OR byte == key
    // So 0x41 == 0x41, it's the same as key, so should be preserved (not XORed)
    // 0x42 ^ 0x41 = 0x03
    assert_eq!(result, vec![0x00, 0x41, 0x03]);
}
