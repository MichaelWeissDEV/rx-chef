// Tests for the date_time_delta operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations date_time_delta::

use rxchef::operation::ArgValue;
use rxchef::operations::date_time_delta::DateTimeDelta;
use rxchef::Operation;

#[test]
fn test_datetime_delta_positive() {
    let op = DateTimeDelta;
    let input = b"20/02/2024 13:36:00".to_vec();
    let args = [
        ArgValue::Str("Standard date and time".to_string()),
        ArgValue::Str("%d/%m/%Y %H:%M:%S".to_string()),
        ArgValue::Str("Add".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Num(0.0),
        ArgValue::Num(1.0),
        ArgValue::Num(0.0),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "20/02/2024 13:37:00");
}
#[test]
fn test_datetime_delta_negative() {
    let op = DateTimeDelta;
    let input = b"20/02/2024 14:37:00".to_vec();
    let args = [
        ArgValue::Str("Standard date and time".to_string()),
        ArgValue::Str("%d/%m/%Y %H:%M:%S".to_string()),
        ArgValue::Str("Subtract".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Num(1.0),
        ArgValue::Num(0.0),
        ArgValue::Num(0.0),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "20/02/2024 13:37:00");
}
#[test]
fn test_datetime_delta_empty() {
    let op = DateTimeDelta;
    let input = b"".to_vec();
    let args = [];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, b"Invalid format.");
}
#[test]
fn test_datetime_delta_invalid() {
    let op = DateTimeDelta;
    let input = b"not-a-date".to_vec();
    let args = [
        ArgValue::Str("Standard date and time".to_string()),
        ArgValue::Str("%d/%m/%Y %H:%M:%S".to_string()),
        ArgValue::Str("Add".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Num(0.0),
        ArgValue::Num(0.0),
        ArgValue::Num(0.0),
    ];
    assert!(op.run(input, &args).is_err());
}
