// Tests for the unescape_unicode_characters operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations unescape_unicode_characters::

use rxchef::operation::ArgValue;
use rxchef::operations::unescape_unicode_characters::UnescapeUnicodeCharacters;
use rxchef::Operation;

#[test]
fn test_unescape_u_prefix() {
    let op = UnescapeUnicodeCharacters;
    let input = b"\\u03c3\\u03bf\\u03c5".to_vec();
    let args = [ArgValue::Str("\\u".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "σου");
}
#[test]
fn test_unescape_percent_u_prefix() {
    let op = UnescapeUnicodeCharacters;
    let input = b"%u0041%u0042".to_vec();
    let args = [ArgValue::Str("%u".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "AB");
}
#[test]
fn test_unescape_u_plus_prefix() {
    let op = UnescapeUnicodeCharacters;
    let input = b"U+0041U+0042".to_vec();
    let args = [ArgValue::Str("U+".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "AB");
}
#[test]
fn test_mixed_content() {
    let op = UnescapeUnicodeCharacters;
    let input = b"Char A is \\u0041".to_vec();
    let args = [ArgValue::Str("\\u".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "Char A is A");
}
