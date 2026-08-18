// Tests for the convert_leet_speak operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations convert_leet_speak::

use rxchef::operation::ArgValue;
use rxchef::operations::convert_leet_speak::ConvertLeetSpeak;
use rxchef::Operation;

#[test]
fn test_convert_to_leet_basic() {
    let operation = ConvertLeetSpeak;
    let input = b"hello world".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("To Leet Speak".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "h3ll0 w0rld");
}
#[test]
fn test_convert_from_leet_basic() {
    let operation = ConvertLeetSpeak;
    let input = b"h3ll0 w0rld".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("From Leet Speak".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "h3ll0 w0rld");
}
#[test]
fn test_convert_to_leet_uppercase() {
    let operation = ConvertLeetSpeak;
    let input = b"HELLO".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("To Leet Speak".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "H3LL0");
}
