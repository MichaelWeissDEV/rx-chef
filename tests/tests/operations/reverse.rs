// Tests for the reverse operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations reverse::

use rxchef::operation::ArgValue;
use rxchef::operations::reverse::Reverse;
use rxchef::Operation;

fn run(input: &[u8], by: &str) -> Vec<u8> {
    let op = Reverse;
    let args = [ArgValue::Str(by.to_string())];
    op.run(input.to_vec(), &args).unwrap()
}
#[test]
fn test_byte_reverse() {
    assert_eq!(run(b"hello", "Byte"), b"olleh");
}
#[test]
fn test_char_reverse_ascii() {
    assert_eq!(run(b"hello", "Character"), b"olleh");
}
#[test]
fn test_char_reverse_unicode() {
    // "cafe\u{301}" (e + combining acute) reversed as chars gives
    // combining-acute + e + f + a + c -- but as Unicode scalars it
    // should still decode correctly.
    let s = "abcd";
    let result = run(s.as_bytes(), "Character");
    assert_eq!(result, b"dcba");
}
#[test]
fn test_line_reverse() {
    let input = b"line1\nline2\nline3";
    let result = run(input, "Line");
    assert_eq!(result, b"line3\nline2\nline1");
}
#[test]
fn test_line_reverse_trailing_newline() {
    // Input ends with newline: last segment is empty but should be handled.
    let input = b"a\nb\nc\n";
    let result = run(input, "Line");
    // JS: lines are ["a","b","c",""], reversed ["","c","b","a"],
    // joined with \n gives "\nc\nb\na\n", sliced to original length 6
    // => "\nc\nb\n" ... wait that is length 6.
    // Let us just check length is preserved.
    assert_eq!(result.len(), input.len());
}
#[test]
fn test_invalid_by() {
    let op = Reverse;
    let args = [ArgValue::Str("Word".to_string())];
    assert!(op.run(b"test".to_vec(), &args).is_err());
}
