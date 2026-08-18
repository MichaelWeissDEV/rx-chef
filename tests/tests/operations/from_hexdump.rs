// Tests for the from_hexdump operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_hexdump::

use rxchef::operations::from_hexdump::FromHexdump;
use rxchef::operations::to_hexdump::ToHexdump;
use rxchef::Operation;

#[test]
fn test_empty_input() {
    let op = FromHexdump;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_roundtrip_with_to_hexdump_default_format() {
    let to_op = ToHexdump;
    let from_op = FromHexdump;
    let input = b"The quick brown fox jumps over the lazy dog!".to_vec();
    let dump = to_op.run(input.clone(), &[]).unwrap();
    let recovered = from_op.run(dump, &[]).unwrap();
    assert_eq!(recovered, input);
}

#[test]
fn test_roundtrip_with_to_hexdump_narrow_width() {
    let to_op = ToHexdump;
    let from_op = FromHexdump;
    let input: Vec<u8> = (0..64).collect();
    let args = [
        rxchef::operation::ArgValue::Num(8.0),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let dump = to_op.run(input.clone(), &args).unwrap();
    let recovered = from_op.run(dump, &[]).unwrap();
    assert_eq!(recovered, input);
}

#[test]
fn test_roundtrip_with_uppercase_hex() {
    let to_op = ToHexdump;
    let from_op = FromHexdump;
    let input = b"binary\x00\x01\xffdata".to_vec();
    let args = [
        rxchef::operation::ArgValue::Num(16.0),
        rxchef::operation::ArgValue::Bool(true), // upper case hex
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let dump = to_op.run(input.clone(), &args).unwrap();
    let recovered = from_op.run(dump, &[]).unwrap();
    assert_eq!(recovered, input);
}

#[test]
fn test_plain_offset_and_hex_line() {
    // A single, hand-written hexdump line: 8-digit offset, two-space
    // separator, space-separated hex byte pairs, pipe-delimited ASCII
    // column. This mirrors the exact shape ToHexdump produces.
    let op = FromHexdump;
    let line = "00000000  68 65 6c 6c 6f 20 77 6f 72 6c 64 21              |hello world!|";
    let result = op.run(line.as_bytes().to_vec(), &[]).unwrap();
    assert_eq!(result, b"hello world!".to_vec());
}

#[test]
fn test_non_hexdump_text_yields_no_bytes() {
    // Plain prose with no offset/hex-column structure shouldn't match the
    // hexdump line regex at all.
    let op = FromHexdump;
    let result = op
        .run(b"just some plain text, nothing hexy here".to_vec(), &[])
        .unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_hexdump_invalid_utf8() {
    let op = FromHexdump;
    let args = [];
    let result = op.run(vec![0xFF, 0xFE], &args);
    assert!(result.is_err());
}
