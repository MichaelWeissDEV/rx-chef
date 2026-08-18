// Tests for the url_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations url_encode::
//
// Expected values follow RFC 3986 percent-encoding: unreserved characters
// (ALPHA / DIGIT / "-" / "." / "_" / "~") are never encoded, and an encoded
// octet is "%" followed by the octet's hexadecimal value.

use rxchef::operation::ArgValue;
use rxchef::operations::url_encode::URLEncode;
use rxchef::Operation;

fn encode(input: &str, encode_all: bool) -> String {
    let op = URLEncode;
    let args = [ArgValue::Bool(encode_all)];
    String::from_utf8(op.run(input.as_bytes().to_vec(), &args).unwrap()).unwrap()
}

#[test]
fn test_url_encode_empty_input() {
    assert_eq!(encode("", false), "");
    assert_eq!(encode("", true), "");
}

#[test]
fn test_url_encode_default_leaves_unreserved_characters_alone() {
    // RFC 3986 section 2.3 unreserved set, which encodeURI also preserves.
    let unreserved = "abcXYZ019-._~";
    assert_eq!(encode(unreserved, false), unreserved);
}

#[test]
fn test_url_encode_all_special_chars_also_encodes_the_unreserved_set() {
    // Deliberate: upstream CyberChef's `encodeAllChars` post-processes
    // encodeURIComponent to additionally escape -._~!'()*, leaving only
    // alphanumerics untouched. RFC 3986 permits this (encoded and decoded
    // unreserved characters are equivalent), so it is not a defect.
    assert_eq!(encode("abcXYZ019", true), "abcXYZ019");
    assert_eq!(encode("-._~", true), "%2D%2E%5F%7E");
}

#[test]
fn test_url_encode_space_becomes_percent_20() {
    assert_eq!(encode("Hello World", false), "Hello%20World");
}

#[test]
fn test_url_encode_default_preserves_reserved_delimiters() {
    // With "encode all special chars" disabled, URI delimiters survive so the
    // result can still be spliced into a URL.
    assert_eq!(encode("a!/?&=", false), "a!/?&=");
}

#[test]
fn test_url_encode_all_special_chars_encodes_delimiters() {
    assert_eq!(encode("!/?&=", true), "%21%2F%3F%26%3D");
}

#[test]
fn test_url_encode_uses_uppercase_hex_digits() {
    // RFC 3986 section 2.1 and ECMA-262's Encode abstract operation both
    // require uppercase hexadecimal digits in percent-escapes.
    let encoded = encode("\u{00ff}", false);
    assert_eq!(encoded, "%C3%BF");
    assert!(!encoded.chars().any(|c| c.is_ascii_lowercase()));
}

#[test]
fn test_url_encode_utf8_is_encoded_per_octet() {
    // "é" is U+00E9, encoded UTF-8 as 0xC3 0xA9 -> two percent-escapes.
    assert_eq!(encode("é", false), "%C3%A9");
    // "€" is U+20AC, encoded UTF-8 as 0xE2 0x82 0xAC.
    assert_eq!(encode("€", false), "%E2%82%AC");
}

#[test]
fn test_url_encode_percent_sign_is_itself_escaped() {
    assert_eq!(encode("100%", false), "100%25");
}

#[test]
fn test_url_encode_control_characters() {
    assert_eq!(encode("a\nb", false), "a%0Ab");
    assert_eq!(encode("a\tb", false), "a%09b");
}
