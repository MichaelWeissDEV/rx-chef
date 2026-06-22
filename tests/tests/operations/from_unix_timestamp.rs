// Tests for the from_unix_timestamp operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_unix_timestamp::

use rxchef::operations::from_unix_timestamp::FromUNIXTimestamp;
use rxchef::Operation;

#[test]
fn test_from_unix_timestamp_empty_input() {
    let op = FromUNIXTimestamp;
    let args = [
        rxchef::operation::ArgValue::Str("Seconds (s)".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_unix_timestamp_seconds() {
    let op = FromUNIXTimestamp;
    let args = [
        rxchef::operation::ArgValue::Str("Seconds (s)".to_string()),
    ];
    // Unix timestamp for 2023-01-01 00:00:00 UTC
    let timestamp_input = "1672531200";
    let result = op.run(timestamp_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    let date_str = String::from_utf8_lossy(&decoded);
    assert!(date_str.contains("2023"));
    assert!(date_str.contains("January"));
}

#[test]
fn test_from_unix_timestamp_milliseconds() {
    let op = FromUNIXTimestamp;
    let args = [
        rxchef::operation::ArgValue::Str("Milliseconds (ms)".to_string()),
    ];
    // Unix timestamp in milliseconds for 2023-01-01 00:00:00.000 UTC
    let timestamp_input = "1672531200000";
    let result = op.run(timestamp_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    let date_str = String::from_utf8_lossy(&decoded);
    assert!(date_str.contains("2023"));
    assert!(date_str.contains("January"));
}

#[test]
fn test_from_unix_timestamp_with_decimal() {
    let op = FromUNIXTimestamp;
    let args = [
        rxchef::operation::ArgValue::Str("Seconds (s)".to_string()),
    ];
    // Unix timestamp with decimal seconds
    let timestamp_input = "1672531200.5";
    let result = op.run(timestamp_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    let date_str = String::from_utf8_lossy(&decoded);
    assert!(date_str.contains("2023"));
}

#[test]
fn test_from_unix_timestamp_invalid_units() {
    let op = FromUNIXTimestamp;
    let args = [
        rxchef::operation::ArgValue::Str("InvalidUnits".to_string()),
    ];
    let timestamp_input = "1672531200";
    let result = op.run(timestamp_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid units
    assert!(result.is_err());
}
