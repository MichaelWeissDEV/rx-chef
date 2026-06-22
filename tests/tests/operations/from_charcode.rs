// Tests for the from_charcode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_charcode::

use rxchef::operations::from_charcode::FromCharcode;
use rxchef::Operation;

#[test]
fn test_from_charcode_empty_input() {
    let op = FromCharcode;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Num(16.0),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_charcode_space_delimited() {
    let op = FromCharcode;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Num(16.0),
    ];
    // Space-delimited hex codes: 48 65 6c 6c 6f -> "Hello"
    let charcode_input = "48 65 6c 6c 6f";
    let result = op.run(charcode_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_charcode_comma_delimited() {
    let op = FromCharcode;
    let args = [
        rxchef::operation::ArgValue::Str("Comma".to_string()),
        rxchef::operation::ArgValue::Num(10.0),
    ];
    // Comma-delimited decimal codes: 72,101,108,108,111 -> "Hello"
    let charcode_input = "72,101,108,108,111";
    let result = op.run(charcode_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_charcode_no_delimiter() {
    let op = FromCharcode;
    let args = [
        rxchef::operation::ArgValue::Str("None".to_string()),
        rxchef::operation::ArgValue::Num(16.0),
    ];
    // No delimiter with long string - should parse as continuous hex and split into bytes: "48656c6c6f" -> "Hello"
    // Need to make it longer than 17 chars to trigger byte splitting
    let charcode_input = "48656c6c6f0000000000000000"; // "Hello" + padding
    let result = op.run(charcode_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    // Should start with "Hello"
    assert!(String::from_utf8_lossy(&decoded).starts_with("Hello"));
}

#[test]
fn test_from_charcode_invalid_base() {
    let op = FromCharcode;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Num(1.0), // Invalid base
    ];
    let charcode_input = "48 65";
    let result = op.run(charcode_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid base
    assert!(result.is_err());
}
