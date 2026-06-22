// Tests for the subtract operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations subtract::

use rxchef::operation::ArgValue;
use rxchef::operations::subtract::Subtract;
use rxchef::Operation;

#[test]
fn test_subtract_basic() {
    // JS doc example: "0x0a 8 .5" -> 10 - 8 - 0.5 = 1.5
    let op = Subtract;
    let result = op
        .run(b"0x0a 8 .5".to_vec(), &[ArgValue::Str("Space".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "1.5");
}
#[test]
fn test_subtract_single() {
    let op = Subtract;
    let result = op
        .run(b"5".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "5");
}
#[test]
fn test_subtract_empty() {
    let op = Subtract;
    let result = op
        .run(b"".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "NaN");
}
#[test]
fn test_subtract_two_numbers() {
    // 10 - 3 = 7
    let op = Subtract;
    let result = op
        .run(b"10\n3".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "7");
}
#[test]
fn test_subtract_negative_result() {
    // 3 - 10 = -7
    let op = Subtract;
    let result = op
        .run(b"3\n10".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "-7");
}
