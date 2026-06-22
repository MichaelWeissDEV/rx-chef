// Tests for the to_hexdump operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_hexdump::

use rxchef::operation::ArgValue;
use rxchef::operations::to_hexdump::ToHexdump;
use rxchef::Operation;

#[test]
fn test_to_hexdump_hello() {
    let op = ToHexdump;
    let input = b"Hello, World!".to_vec();
    let args = [
        ArgValue::Num(16.0),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    // Should start with offset 00000000
    assert!(s.starts_with("00000000"));
    // Should contain hex and ASCII column
    assert!(s.contains("|Hello, World!|"));
}
#[test]
fn test_to_hexdump_upper_case() {
    let op = ToHexdump;
    let input = b"\xAB\xCD".to_vec();
    let args = [
        ArgValue::Num(16.0),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert!(s.contains("AB CD"));
    assert!(s.starts_with("00000000"));
}
#[test]
fn test_to_hexdump_include_final_length() {
    let op = ToHexdump;
    let input = b"AB".to_vec();
    let args = [
        ArgValue::Num(16.0),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    // Last line should be the total length "00000002"
    assert_eq!(lines.last().unwrap().trim(), "00000002");
}
#[test]
fn test_to_hexdump_non_printable() {
    let op = ToHexdump;
    let input = vec![0x00_u8, 0x41_u8, 0x7F_u8];
    let args = [
        ArgValue::Num(16.0),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    // Non-printables become '.', 'A' stays
    assert!(s.contains("|..A|") || s.contains("|.A.|"));
    // 0x00 and 0x7F are both non-printable
    assert!(s.contains("|...|") || s.contains("|.A.|"));
}
#[test]
fn test_to_hexdump_invalid_width() {
    let op = ToHexdump;
    let result = op.run(
        b"hello".to_vec(),
        &[
            ArgValue::Num(0.0),
            ArgValue::Bool(false),
            ArgValue::Bool(false),
            ArgValue::Bool(false),
        ],
    );
    assert!(result.is_err());
}
