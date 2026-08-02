// Tests for the from_bcd operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_bcd::

use rxchef::operations::from_bcd::FromBCD;
use rxchef::Operation;

#[test]
fn test_from_bcd_empty_input() {
    let op = FromBCD;
    let args = [
        rxchef::operation::ArgValue::Str("8 4 2 1".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Str("Nibbles".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_bcd_simple_8421() {
    let op = FromBCD;
    let args = [
        rxchef::operation::ArgValue::Str("8 4 2 1".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Str("Nibbles".to_string()),
    ];
    // Simple BCD encoding: "0000 0001" should decode to "01" (leading zero preserved)
    let bcd_input = "00000001";
    let result = op.run(bcd_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "01");
}

#[test]
fn test_from_bcd_packed_format() {
    let op = FromBCD;
    let args = [
        rxchef::operation::ArgValue::Str("8 4 2 1".to_string()),
        rxchef::operation::ArgValue::Bool(true), // Packed
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Str("Nibbles".to_string()),
    ];
    // Packed BCD: "00000001 00000010" should decode to "12"
    let bcd_input = "0000000100000010";
    let result = op.run(bcd_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_bcd_different_scheme() {
    let op = FromBCD;
    let args = [
        rxchef::operation::ArgValue::Str("Excess-3".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Str("Nibbles".to_string()),
    ];
    // Test with Excess-3 scheme
    let bcd_input = "00110100"; // Should represent digits in Excess-3
    let result = op.run(bcd_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_bcd_invalid_scheme() {
    let op = FromBCD;
    let args = [
        rxchef::operation::ArgValue::Str("InvalidScheme".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Str("Nibbles".to_string()),
    ];
    let bcd_input = "00000001";
    let result = op.run(bcd_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid scheme
    assert!(result.is_err());
}
