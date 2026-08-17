// Tests for the from_base62 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base62::

use rxchef::operations::from_base62::FromBase62;
use rxchef::Operation;

#[test]
fn test_from_base62_empty_input() {
    let op = FromBase62;
    let args = [rxchef::operation::ArgValue::Str("0-9A-Za-z".to_string())];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_base62_simple_decode() {
    let op = FromBase62;
    let args = [rxchef::operation::ArgValue::Str("0-9A-Za-z".to_string())];
    // "a" is index 36, which is the single byte 0x24 ('$').
    // Value cross-checked against CyberChef 11.0.0.
    let base62_input = "a";
    let decoded = op.run(base62_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, b"$");
}

#[test]
fn test_from_base62_number_decode() {
    let op = FromBase62;
    let args = [rxchef::operation::ArgValue::Str("0-9A-Za-z".to_string())];
    // 1*62^2 + 2*62 + 3 = 3971 = 0x0F83.
    // Value cross-checked against CyberChef 11.0.0.
    let base62_input = "123";
    let decoded = op.run(base62_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, [0x0f, 0x83]);
}

#[test]
fn test_from_base62_mixed_alphabet() {
    let op = FromBase62;
    let args = [rxchef::operation::ArgValue::Str("0-9A-Za-z".to_string())];
    // Value cross-checked against CyberChef 11.0.0.
    let base62_input = "aBc123";
    let decoded = op.run(base62_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, [0x07, 0xb8, 0x09, 0x34, 0x83]);
}

#[test]
fn test_from_base62_invalid_alphabet_length() {
    let op = FromBase62;
    let args = [
        rxchef::operation::ArgValue::Str("ABC".to_string()), // Too short
    ];
    let base62_input = "a";
    let result = op.run(base62_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid alphabet length
    assert!(result.is_err());
}
