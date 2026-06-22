// Tests for the unicode_text_format operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations unicode_text_format::

use rxchef::operation::ArgValue;
use rxchef::operations::unicode_text_format::UnicodeTextFormat;
use rxchef::Operation;

#[test]
fn test_underline() {
    let op = UnicodeTextFormat;
    let input = b"Hello".to_vec();
    let args = [ArgValue::Bool(true), ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    // H + \u0332 + e + \u0332 ...
    let expected = "H\u{0332}e\u{0332}l\u{0332}l\u{0332}o\u{0332}";
    assert_eq!(String::from_utf8_lossy(&result), expected);
}
#[test]
fn test_strikethrough() {
    let op = UnicodeTextFormat;
    let input = b"Hello".to_vec();
    let args = [ArgValue::Bool(false), ArgValue::Bool(true)];
    let result = op.run(input, &args).unwrap();
    let expected = "H\u{0336}e\u{0336}l\u{0336}l\u{0336}o\u{0336}";
    assert_eq!(String::from_utf8_lossy(&result), expected);
}
#[test]
fn test_both() {
    let op = UnicodeTextFormat;
    let input = b"A".to_vec();
    let args = [ArgValue::Bool(true), ArgValue::Bool(true)];
    let result = op.run(input, &args).unwrap();
    // strikethrough first, then underline (based on JS order)
    let expected = "A\u{0336}\u{0332}";
    assert_eq!(String::from_utf8_lossy(&result), expected);
}
#[test]
fn test_multibyte() {
    let op = UnicodeTextFormat;
    let input = "π".as_bytes().to_vec();
    let args = [ArgValue::Bool(true), ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    let expected = "π\u{0332}";
    assert_eq!(String::from_utf8_lossy(&result), expected);
}
