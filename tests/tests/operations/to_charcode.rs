// Tests for the to_charcode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_charcode::

use rxchef::operation::ArgValue;
use rxchef::operations::to_charcode::ToCharcode;
use rxchef::Operation;

#[test]
fn test_to_charcode_empty() {
    let op = ToCharcode;
    let result = op
        .run(
            vec![],
            &[ArgValue::Str("Space".to_string()), ArgValue::Num(16.0)],
        )
        .unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_to_charcode_all_bytes_hex() {
    // Each byte 0x00..=0xFF should map to a two-char hex code
    let op = ToCharcode;
    let input: Vec<u8> = (0u8..=255u8).collect();
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Num(16.0)];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    let parts: Vec<&str> = s.split(' ').collect();
    assert_eq!(parts.len(), 256);
    assert_eq!(parts[0], "00");
    assert_eq!(parts[255], "ff");
}
#[test]
fn test_to_charcode_utf8_codepoints() {
    // " " codepoints base 16
    let op = ToCharcode;
    let input = " ".as_bytes().to_vec();
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Num(16.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "20");
}
#[test]
fn test_to_charcode_base10() {
    let op = ToCharcode;
    let input = b"Hello".to_vec();
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Num(10.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "72 101 108 108 111");
}
#[test]
fn test_to_charcode_invalid_base() {
    let op = ToCharcode;
    let result = op.run(
        b"a".to_vec(),
        &[ArgValue::Str("Space".to_string()), ArgValue::Num(1.0)],
    );
    assert!(result.is_err());
}
