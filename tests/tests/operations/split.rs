// Tests for the split operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations split::

use rxchef::operation::ArgValue;
use rxchef::operations::split::Split;
use rxchef::Operation;

#[test]
fn test_split_comma_to_newline() {
    let op = Split;
    let input = b"a,b,c".to_vec();
    let args = vec![
        ArgValue::Str("Comma".to_string()),
        ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "a\nb\nc");
}
#[test]
fn test_split_newline_to_comma() {
    let op = Split;
    let input = b"hello\nworld".to_vec();
    let args = vec![
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Str("Comma".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "hello,world");
}
#[test]
fn test_split_no_delimiter_found() {
    let op = Split;
    let input = b"nothinghere".to_vec();
    let args = vec![
        ArgValue::Str("Comma".to_string()),
        ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "nothinghere");
}
