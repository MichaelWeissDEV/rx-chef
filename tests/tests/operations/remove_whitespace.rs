// Tests for the remove_whitespace operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations remove_whitespace::

use rxchef::operation::ArgValue;
use rxchef::operations::remove_whitespace::RemoveWhitespace;
use rxchef::Operation;

#[test]
fn test_remove_whitespace_basic() {
    let operation = RemoveWhitespace;
    let input = b"hello world".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "helloworld");
}
#[test]
fn test_remove_whitespace_with_newlines() {
    let operation = RemoveWhitespace;
    let input = b"line1\nline2\nline3".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "line1line2line3");
}
#[test]
fn test_remove_whitespace_preserve() {
    let operation = RemoveWhitespace;
    let input = b"hello world".to_vec();
    let result = operation
        .run(
            input,
            &[
                ArgValue::Bool(false), // Don't remove spaces
                ArgValue::Bool(false), // Don't remove CR
                ArgValue::Bool(false), // Don't remove LF
                ArgValue::Bool(false), // Don't remove tabs
                ArgValue::Bool(false), // Don't remove form feeds
                ArgValue::Bool(false), // Don't remove full stops
            ],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "hello world");
}
