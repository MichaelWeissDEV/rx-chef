// Tests for the sum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sum::

use rxchef::operation::ArgValue;
use rxchef::operations::sum::Sum;
use rxchef::Operation;

#[test]
fn test_sum_basic() {
    // JS doc example: "0x0a 8 .5" -> 10 + 8 + 0.5 = 18.5
    let op = Sum;
    let result = op
        .run(b"0x0a 8 .5".to_vec(), &[ArgValue::Str("Space".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "18.5");
}
#[test]
fn test_sum_linefeed() {
    let op = Sum;
    let result = op
        .run(
            b"1\n2\n3\n4".to_vec(),
            &[ArgValue::Str("Line feed".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "10");
}
#[test]
fn test_sum_empty() {
    let op = Sum;
    let result = op
        .run(b"".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "NaN");
}
#[test]
fn test_sum_skips_non_numbers() {
    let op = Sum;
    let result = op
        .run(b"1 hello 2".to_vec(), &[ArgValue::Str("Space".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "3");
}
