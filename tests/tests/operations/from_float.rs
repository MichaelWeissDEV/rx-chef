// Tests for the from_float operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_float::

use rxchef::operation::ArgValue;
use rxchef::operations::from_float::FromFloat;
use rxchef::Operation;

#[test]
fn test_from_float_big_endian_f32() {
    let op = FromFloat;
    let input = b"1.0".to_vec();
    let args = [
        ArgValue::Str("Big Endian".to_string()),
        ArgValue::Str("Float (4 bytes)".to_string()),
        ArgValue::Str("Space".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    // 1.0f32 big endian = 3f 80 00 00
    assert_eq!(result, vec![0x3f, 0x80, 0x00, 0x00]);
}
#[test]
fn test_from_float_little_endian_f32() {
    let op = FromFloat;
    let input = b"1.0".to_vec();
    let args = [
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Str("Float (4 bytes)".to_string()),
        ArgValue::Str("Space".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, vec![0x00, 0x00, 0x80, 0x3f]);
}
#[test]
fn test_from_float_multiple_values() {
    let op = FromFloat;
    let input = b"1.0 2.0".to_vec();
    let args = [
        ArgValue::Str("Big Endian".to_string()),
        ArgValue::Str("Float (4 bytes)".to_string()),
        ArgValue::Str("Space".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result.len(), 8);
}
