// Tests for the to_hex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_hex::

use rxchef::operation::ArgValue;
use rxchef::operations::to_hex::ToHex;
use rxchef::Operation;

#[test]
fn test_to_hex_empty() {
    let op = ToHex;
    let result = op
        .run(
            vec![],
            &[ArgValue::Str("Space".to_string()), ArgValue::Num(0.0)],
        )
        .unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_to_hex_all_bytes() {
    let op = ToHex;
    let input: Vec<u8> = (0u8..=255u8).collect();
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Num(0.0)];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    let parts: Vec<&str> = s.split(' ').collect();
    assert_eq!(parts.len(), 256);
    assert_eq!(parts[0], "00");
    assert_eq!(parts[255], "ff");
}
#[test]
fn test_to_hex_none_delim() {
    // UTF-8 bytes of " " should encode to that hex string
    let op = ToHex;
    let input = " ".as_bytes().to_vec();
    let args = [ArgValue::Str("None".to_string()), ArgValue::Num(0.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "20");
}
#[test]
fn test_to_hex_0x_with_comma() {
    let op = ToHex;
    let input = vec![0xDE_u8, 0xAD_u8, 0xBE_u8];
    let args = [
        ArgValue::Str("0x with comma".to_string()),
        ArgValue::Num(0.0),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "0xde,0xad,0xbe");
}
#[test]
fn test_to_hex_line_wrap() {
    let op = ToHex;
    let input: Vec<u8> = (0u8..4u8).collect();
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Num(2.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "00 01\n02 03");
}
