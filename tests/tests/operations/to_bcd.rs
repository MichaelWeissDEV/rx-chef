// Tests for the to_bcd operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_bcd::

use rxchef::operation::ArgValue;
use rxchef::operations::to_bcd::ToBCD;
use rxchef::Operation;

#[test]
fn test_to_bcd_default_zero() {
    // "To BCD: default 0" -> "0000"
    let op = ToBCD;
    let args = [
        ArgValue::Str("8 4 2 1".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Str("Nibbles".to_string()),
    ];
    let result = op.run(b"0".to_vec(), &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "0000");
}
#[test]
fn test_to_bcd_unpacked_nibbles() {
    // "To BCD: unpacked nibbles" for "1234567890"
    let op = ToBCD;
    let args = [
        ArgValue::Str("8 4 2 1".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Str("Nibbles".to_string()),
    ];
    let result = op.run(b"1234567890".to_vec(), &args).unwrap();
    let expected = "0000 0001 0000 0010 0000 0011 0000 0100 0000 0101 0000 0110 0000 0111 0000 1000 0000 1001 0000 0000";
    assert_eq!(String::from_utf8(result).unwrap(), expected);
}
#[test]
fn test_to_bcd_packed_signed_bytes() {
    // "To BCD: packed, signed bytes" for "1234567890"
    let op = ToBCD;
    let args = [
        ArgValue::Str("8 4 2 1".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Str("Bytes".to_string()),
    ];
    let result = op.run(b"1234567890".to_vec(), &args).unwrap();
    let expected = "00000001 00100011 01000101 01100111 10001001 00001100";
    assert_eq!(String::from_utf8(result).unwrap(), expected);
}
#[test]
fn test_to_bcd_packed_signed_nibbles_8_4_m2_m1() {
    // "To BCD: packed, signed nibbles, 8 4 -2 -1" for "-1234567890"
    let op = ToBCD;
    let args = [
        ArgValue::Str("8 4 -2 -1".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Str("Nibbles".to_string()),
    ];
    let result = op.run(b"-1234567890".to_vec(), &args).unwrap();
    let expected = "0000 0111 0110 0101 0100 1011 1010 1001 1000 1111 0000 1101";
    assert_eq!(String::from_utf8(result).unwrap(), expected);
}
