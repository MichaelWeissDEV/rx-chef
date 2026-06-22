// Tests for the to_float operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_float::

use rxchef::operation::ArgValue;
use rxchef::operations::to_float::ToFloat;
use rxchef::Operation;

#[test]
fn test_to_float_big_endian_f32() {
    let op = ToFloat;
    // 1.0f32 big endian = 3f 80 00 00
    let input = vec![0x3f, 0x80, 0x00, 0x00u8];
    let args = [
        ArgValue::Str("Big Endian".to_string()),
        ArgValue::Str("Float (4 bytes)".to_string()),
        ArgValue::Str("Space".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "1");
}
#[test]
fn test_to_float_wrong_length() {
    let op = ToFloat;
    let input = vec![0x3f, 0x80, 0x00u8]; // 3 bytes, not multiple of 4
    let args = [
        ArgValue::Str("Big Endian".to_string()),
        ArgValue::Str("Float (4 bytes)".to_string()),
        ArgValue::Str("Space".to_string()),
    ];
    assert!(op.run(input, &args).is_err());
}
