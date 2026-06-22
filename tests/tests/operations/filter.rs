// Tests for the filter operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations filter::

use rxchef::operation::ArgValue;
use rxchef::operations::filter::Filter;
use rxchef::Operation;

#[test]
fn test_filter_lines_by_regex() {
    let op = Filter;
    let input = b"foo\nbar\nbaz\nfoo2".to_vec();
    let args = [
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Str("foo".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "foo\nfoo2");
}
#[test]
fn test_filter_inverted() {
    let op = Filter;
    let input = b"foo\nbar\nbaz".to_vec();
    let args = [
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Str("foo".to_string()),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "bar\nbaz");
}
#[test]
fn test_filter_empty_regex_passthrough() {
    let op = Filter;
    let input = b"hello\nworld".to_vec();
    let args = [
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "hello\nworld");
}
#[test]
fn test_filter_invalid_regex() {
    let op = Filter;
    let input = b"test".to_vec();
    let args = [
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Str("[invalid".to_string()),
        ArgValue::Bool(false),
    ];
    assert!(op.run(input, &args).is_err());
}
