// Tests for the from_decimal operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_decimal::

use rxchef::operations::from_decimal::FromDecimal;
use rxchef::Operation;

#[test]
fn test_from_decimal_empty_input() {
    let op = FromDecimal;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_decimal_space_delimited() {
    let op = FromDecimal;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Bool(false),
    ];
    // Space-delimited decimal values: 72 101 108 108 111 -> "Hello"
    let decimal_input = "72 101 108 108 111";
    let result = op.run(decimal_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_decimal_comma_delimited() {
    let op = FromDecimal;
    let args = [
        rxchef::operation::ArgValue::Str("Comma".to_string()),
        rxchef::operation::ArgValue::Bool(false),
    ];
    // Comma-delimited decimal values: 72,101,108,108,111 -> "Hello"
    let decimal_input = "72,101,108,108,111";
    let result = op.run(decimal_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_decimal_signed_values() {
    let op = FromDecimal;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Bool(true), // Support signed values
    ];
    // Signed decimal values: 72 -1 101 -2 108 -> mixed bytes
    let decimal_input = "72 -1 101 -2 108";
    let result = op.run(decimal_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(decoded, vec![72, 255, 101, 254, 108]);
}

#[test]
fn test_from_decimal_no_delimiter() {
    let op = FromDecimal;
    let args = [
        rxchef::operation::ArgValue::Str("None".to_string()),
        rxchef::operation::ArgValue::Bool(false),
    ];
    // No delimiter - treats whole string as single value
    let decimal_input = "72";
    let result = op.run(decimal_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(decoded, vec![72]);
}
