// Tests for the translate_date_time_format operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations translate_date_time_format::

use rxchef::operation::ArgValue;
use rxchef::operations::translate_date_time_format::TranslateDateTimeFormat;
use rxchef::Operation;

#[test]
fn test_basic_conversion() {
    let op = TranslateDateTimeFormat;
    let input = b"01/06/2023 12:00:00".to_vec();
    let args = [
        ArgValue::Str("%d/%m/%Y %H:%M:%S".to_string()),
        ArgValue::Str("UTC".to_string()),
        ArgValue::Str("%Y-%m-%d %H:%M:%S".to_string()),
        ArgValue::Str("UTC".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "2023-06-01 12:00:00");
}
#[test]
fn test_day_name_format() {
    let op = TranslateDateTimeFormat;
    let input = b"2023-06-01".to_vec();
    let args = [
        ArgValue::Str("%Y-%m-%d".to_string()),
        ArgValue::Str("UTC".to_string()),
        ArgValue::Str("%d/%m/%Y".to_string()),
        ArgValue::Str("UTC".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "01/06/2023");
}
#[test]
fn test_empty_input() {
    let op = TranslateDateTimeFormat;
    let result = op.run(b"".to_vec(), &[]).unwrap();
    assert_eq!(result, b"Invalid format.");
}
#[test]
fn test_invalid_format() {
    let op = TranslateDateTimeFormat;
    let input = b"not-a-date".to_vec();
    let args = [
        ArgValue::Str("%d/%m/%Y".to_string()),
        ArgValue::Str("UTC".to_string()),
        ArgValue::Str("%Y".to_string()),
        ArgValue::Str("UTC".to_string()),
    ];
    assert!(op.run(input, &args).is_err());
}
