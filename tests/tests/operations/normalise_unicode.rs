// Tests for the normalise_unicode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations normalise_unicode::

use rxchef::operation::ArgValue;
use rxchef::operations::normalise_unicode::NormaliseUnicode;
use rxchef::Operation;

#[test]
fn test_normalise_unicode_nfc() {
    let op = NormaliseUnicode;
    // '' can be represented as U+00E9 (NFC) or U+0065 U+0301 (NFD)
    let input = "e\u{0301}".as_bytes().to_vec();
    let args = [ArgValue::Str("NFC".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "\u{00E9}");
}
#[test]
fn test_normalise_unicode_nfd() {
    let op = NormaliseUnicode;
    let input = "\u{00E9}".as_bytes().to_vec();
    let args = [ArgValue::Str("NFD".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "e\u{0301}");
}
