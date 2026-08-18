// Tests for the multiply operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations multiply::

use rxchef::operation::ArgValue;
use rxchef::operations::multiply::Multiply;
use rxchef::Operation;

#[test]
fn test_multiply_basic() {
    // JS doc example: "0x0a 8 .5" -> 10 * 8 * 0.5 = 40
    let op = Multiply;
    let result = op
        .run(b"0x0a 8 .5".to_vec(), &[ArgValue::Str("Space".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "40");
}
#[test]
fn test_multiply_single() {
    let op = Multiply;
    let result = op
        .run(b"7".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "7");
}
#[test]
fn test_multiply_empty() {
    let op = Multiply;
    let result = op
        .run(b"".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "NaN");
}
#[test]
fn test_multiply_two_numbers() {
    // 6 * 7 = 42
    let op = Multiply;
    let result = op
        .run(b"6\n7".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "42");
}
