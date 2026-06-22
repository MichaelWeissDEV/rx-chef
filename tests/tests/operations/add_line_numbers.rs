// Tests for the add_line_numbers operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations add_line_numbers::

use rxchef::operation::ArgValue;
use rxchef::operations::add_line_numbers::AddLineNumbers;
use rxchef::Operation;

#[test]
fn test_add_line_numbers_basic() {
    let op = AddLineNumbers;
    let input = b"hello\nworld".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "1 hello\n2 world");
}
#[test]
fn test_add_line_numbers_with_offset() {
    let op = AddLineNumbers;
    let input = b"hello\nworld".to_vec();
    let args = [ArgValue::Str("5".to_string())];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "6 hello\n7 world");
}
#[test]
fn test_add_line_numbers_single_line() {
    let op = AddLineNumbers;
    let input = b"single".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "1 single");
}
#[test]
fn test_add_line_numbers_empty_input() {
    let op = AddLineNumbers;
    let input = b"".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "");
}
#[test]
fn test_add_line_numbers_trailing_newline() {
    let op = AddLineNumbers;
    let input = b"hello\nworld\n".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "1 hello\n2 world\n");
}
