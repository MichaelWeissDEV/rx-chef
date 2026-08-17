// Tests for the to_quoted_printable operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_quoted_printable::
//
// Expected values follow RFC 2045 section 6.7 (Quoted-Printable Content-
// Transfer-Encoding): octets outside 33-60 and 62-126 are represented as "="
// followed by two uppercase hexadecimal digits, "=" is always escaped, and
// encoded lines are at most 76 characters including the soft line break.

use rxchef::operations::to_quoted_printable::ToQuotedPrintable;
use rxchef::Operation;

fn encode(input: &[u8]) -> String {
    let op = ToQuotedPrintable;
    String::from_utf8(op.run(input.to_vec(), &[]).unwrap()).unwrap()
}

#[test]
fn test_to_quoted_printable_empty_input() {
    assert_eq!(encode(b""), "");
}

#[test]
fn test_to_quoted_printable_printable_ascii_is_literal() {
    // RFC 2045: decimal 33-60 and 62-126 may be represented as themselves.
    assert_eq!(encode(b"Hello World"), "Hello World");
}

#[test]
fn test_to_quoted_printable_escapes_the_equals_sign() {
    // RFC 2045 rule #1: "=" must always be encoded, as =3D.
    assert_eq!(encode(b"Hello=World"), "Hello=3DWorld");
}

#[test]
fn test_to_quoted_printable_encodes_non_ascii_octets() {
    // "café" in UTF-8 ends with 0xC3 0xA9.
    assert_eq!(encode("café".as_bytes()), "caf=C3=A9");
    // "€" is 0xE2 0x82 0xAC.
    assert_eq!(encode("€".as_bytes()), "=E2=82=AC");
}

#[test]
fn test_to_quoted_printable_uses_uppercase_hex_digits() {
    // RFC 2045: "the uppercase letters must be used".
    let encoded = encode(&[0xAB, 0xCD, 0xEF]);
    assert_eq!(encoded, "=AB=CD=EF");
}

#[test]
fn test_to_quoted_printable_encodes_high_and_low_boundary_octets() {
    assert_eq!(encode(&[0x00]), "=00");
    assert_eq!(encode(&[0xFF]), "=FF");
    // 0x21 ('!') and 0x7E ('~') are the inclusive edges of the literal ranges.
    assert_eq!(encode(&[0x21]), "!");
    assert_eq!(encode(&[0x7E]), "~");
    // 0x7F is outside the literal range.
    assert_eq!(encode(&[0x7F]), "=7F");
}

#[test]
fn test_to_quoted_printable_wraps_long_lines_at_76_characters() {
    // RFC 2045 rule #5: lines must not be longer than 76 characters; a soft
    // line break is a "=" as the final character of an encoded line.
    let encoded = encode(&b"A".repeat(200));
    for line in encoded.split("\r\n") {
        assert!(
            line.len() <= 76,
            "encoded line exceeds the 76 character limit: {} chars",
            line.len()
        );
    }
    assert!(
        encoded.contains("=\r\n"),
        "long input must be split with soft line breaks"
    );
    // Soft breaks are not data: removing them restores the original octets.
    assert_eq!(encoded.replace("=\r\n", ""), "A".repeat(200));
}

#[test]
fn test_to_quoted_printable_roundtrips_through_from_quoted_printable() {
    use rxchef::operations::from_quoted_printable::FromQuotedPrintable;

    // A roundtrip alone would not prove either direction correct, so this runs
    // alongside the fixed vectors above rather than instead of them.
    let original: Vec<u8> = (0u8..=255).collect();
    let encoded = encode(&original);
    let decoded = FromQuotedPrintable
        .run(encoded.into_bytes(), &[])
        .expect("decoding our own quoted-printable output must succeed");
    assert_eq!(decoded, original);
}
