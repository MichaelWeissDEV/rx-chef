// Tests for the parse_date_time operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_date_time::

use rxchef::operation::ArgValue;
use rxchef::operations::parse_date_time::ParseDateTime;
use rxchef::Operation;

#[test]
fn test_parse_date_time_basic() {
    let op = ParseDateTime;
    let input = b"25/12/2023 14:30:00".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("December"));
    assert!(out.contains("2023"));
    assert!(out.contains("Quarter: 4"));
}
#[test]
fn test_parse_date_time_leap_year() {
    let op = ParseDateTime;
    let input = b"29/02/2024 00:00:00".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("Leap year: true"));
}
#[test]
fn test_parse_date_time_invalid() {
    let op = ParseDateTime;
    let input = b"not-a-date".to_vec();
    assert!(op.run(input, &[]).is_err());
}
#[test]
fn test_parse_date_time_custom_format() {
    let op = ParseDateTime;
    let input = b"2023-06-15 09:00:00".to_vec();
    let args = vec![ArgValue::Str("%Y-%m-%d %H:%M:%S".to_string())];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("June"));
    assert!(out.contains("Quarter: 2"));
}
