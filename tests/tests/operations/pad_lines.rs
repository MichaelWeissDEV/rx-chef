// Tests for the pad_lines operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations pad_lines::

use rxchef::operation::ArgValue;
use rxchef::operations::pad_lines::PadLines;
use rxchef::Operation;

#[test]
fn test_pad_start() {
    let op = PadLines;
    let input = b"hello\nworld".to_vec();
    let args = vec![
        ArgValue::Str("Start".to_string()),
        ArgValue::Num(3.0),
        ArgValue::Str(" ".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "   hello\n   world");
}
#[test]
fn test_pad_end() {
    let op = PadLines;
    let input = b"hi\nbye".to_vec();
    let args = vec![
        ArgValue::Str("End".to_string()),
        ArgValue::Num(2.0),
        ArgValue::Str("*".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "hi**\nbye**");
}
#[test]
fn test_pad_zero_length() {
    let op = PadLines;
    let input = b"test".to_vec();
    let args = vec![
        ArgValue::Str("Start".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Str("-".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "test");
}
#[test]
fn test_pad_multiline() {
    let op = PadLines;
    let input = b"a\nb\nc".to_vec();
    let args = vec![
        ArgValue::Str("Start".to_string()),
        ArgValue::Num(1.0),
        ArgValue::Str("X".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "Xa\nXb\nXc");
}
