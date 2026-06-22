// Tests for the divide operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations divide::

use rxchef::operation::ArgValue;
use rxchef::operations::divide::Divide;
use rxchef::Operation;

#[test]
fn test_divide_basic() {
    // JS doc example: "0x0a 8 .5" -> 10 / 8 / 0.5 = 2.5
    let op = Divide;
    let result = op
        .run(b"0x0a 8 .5".to_vec(), &[ArgValue::Str("Space".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "2.5");
}
#[test]
fn test_divide_single() {
    let op = Divide;
    let result = op
        .run(b"10".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "10");
}
#[test]
fn test_divide_empty() {
    let op = Divide;
    let result = op
        .run(b"".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "NaN");
}
#[test]
fn test_divide_skips_non_numbers() {
    // only 10 and 2 are valid -> 10/2 = 5
    let op = Divide;
    let result = op
        .run(
            b"abc 10 foo 2".to_vec(),
            &[ArgValue::Str("Space".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "5");
}
#[test]
fn test_divide_linefeed() {
    // 100 / 4 / 5 = 5
    let op = Divide;
    let result = op
        .run(
            b"100\n4\n5".to_vec(),
            &[ArgValue::Str("Line feed".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "5");
}
