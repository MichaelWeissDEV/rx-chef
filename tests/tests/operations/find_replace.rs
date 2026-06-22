// Tests for the find_replace operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations find_replace::

use rxchef::operation::ArgValue;
use rxchef::operations::find_replace::FindReplace;
use rxchef::Operation;

#[test]
fn test_simple_replace() {
    let op = FindReplace;
    let input = b"hello world".to_vec();
    let args = [
        ArgValue::Str("world".to_string()),
        ArgValue::Str("Simple string".to_string()),
        ArgValue::Str("Rust".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "hello Rust");
}
#[test]
fn test_regex_replace() {
    let op = FindReplace;
    let input = b"foo123bar456".to_vec();
    let args = [
        ArgValue::Str("[0-9]+".to_string()),
        ArgValue::Str("Regex".to_string()),
        ArgValue::Str("NUM".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "fooNUMbarNUM");
}
#[test]
fn test_case_insensitive_replace() {
    let op = FindReplace;
    let input = b"Hello HELLO hello".to_vec();
    let args = [
        ArgValue::Str("hello".to_string()),
        ArgValue::Str("Simple string".to_string()),
        ArgValue::Str("hi".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "hi hi hi");
}
#[test]
fn test_non_global_replace() {
    let op = FindReplace;
    let input = b"aaa".to_vec();
    let args = [
        ArgValue::Str("a".to_string()),
        ArgValue::Str("Simple string".to_string()),
        ArgValue::Str("b".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "baa");
}
