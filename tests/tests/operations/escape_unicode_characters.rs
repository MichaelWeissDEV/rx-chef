// Tests for the escape_unicode_characters operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations escape_unicode_characters::

use rxchef::operation::ArgValue;
use rxchef::operations::escape_unicode_characters::EscapeUnicodeCharacters;
use rxchef::Operation;

#[test]
fn test_escape_unicode_characters() {
    let op = EscapeUnicodeCharacters;
    let args = [
        ArgValue::Str("\\u".to_string()),
        ArgValue::Bool(false),
        ArgValue::Num(4.0),
        ArgValue::Bool(true),
    ];
    let input = "".as_bytes().to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "");
}
#[test]
fn test_escape_unicode_characters_all() {
    let op = EscapeUnicodeCharacters;
    let args = [
        ArgValue::Str("\\u".to_string()),
        ArgValue::Bool(true),
        ArgValue::Num(4.0),
        ArgValue::Bool(true),
    ];
    let input = "abc".as_bytes().to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "\\u0061\\u0062\\u0063");
}
