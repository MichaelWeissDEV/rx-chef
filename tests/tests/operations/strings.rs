// Tests for the strings operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations strings::

use rxchef::operation::ArgValue;
use rxchef::operations::strings::Strings;
use rxchef::Operation;

#[test]
fn test_strings_basic_ascii() {
    let op = Strings;
    // "Hello" is 5 printable chars surrounded by non-printable
    let mut input = vec![0x00u8, 0x01];
    input.extend_from_slice(b"Hello");
    input.push(0x00);
    input.push(0x02);
    let args = vec![
        ArgValue::Str("Single byte".to_string()),
        ArgValue::Num(4.0),
        ArgValue::Str("All printable chars (A)".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("Hello"));
}
#[test]
fn test_strings_min_length_filter() {
    let op = Strings;
    // "Hi" is too short (2 chars < 4)
    let mut input = vec![0x00u8];
    input.extend_from_slice(b"Hi");
    input.push(0x00);
    let args = vec![
        ArgValue::Str("Single byte".to_string()),
        ArgValue::Num(4.0),
        ArgValue::Str("All printable chars (A)".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(!out.contains("Hi"));
}
#[test]
fn test_strings_display_total() {
    let op = Strings;
    let input = b"Hello World test data".to_vec();
    let args = vec![
        ArgValue::Str("Single byte".to_string()),
        ArgValue::Num(4.0),
        ArgValue::Str("All printable chars (A)".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.starts_with("Total found:"));
}
