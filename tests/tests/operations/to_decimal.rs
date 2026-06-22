// Tests for the to_decimal operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_decimal::

use rxchef::operation::ArgValue;
use rxchef::operations::to_decimal::ToDecimal;
use rxchef::Operation;

#[test]
fn test_to_decimal_hello() {
    let op = ToDecimal;
    let input = b"Hello".to_vec();
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "72 101 108 108 111");
}
#[test]
fn test_to_decimal_signed() {
    let op = ToDecimal;
    // 0xFF == 255 unsigned, -1 signed
    let input = vec![0xFF_u8, 0x80_u8];
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Bool(true)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "-1 -128");
}
#[test]
fn test_to_decimal_unsigned_high() {
    let op = ToDecimal;
    let input = vec![0xFF_u8, 0x80_u8];
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "255 128");
}
#[test]
fn test_to_decimal_empty() {
    let op = ToDecimal;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_to_decimal_comma_delim() {
    let op = ToDecimal;
    let input = b"Hi".to_vec();
    let args = [ArgValue::Str("Comma".to_string()), ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "72,105");
}
