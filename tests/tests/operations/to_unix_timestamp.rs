// Tests for the to_unix_timestamp operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_unix_timestamp::
//
// Reference instants are the well-known POSIX epoch anchors: 1970-01-01
// 00:00:00 UTC is 0, and 2000-01-01 00:00:00 UTC is 946684800 (30 years
// containing 7 leap days: 365*30 + 7 = 10957 days).

use rxchef::operation::{ArgValue, OperationError};
use rxchef::operations::to_unix_timestamp::ToUNIXTimestamp;
use rxchef::Operation;

fn convert(input: &str, units: &str, show_datetime: bool) -> String {
    let args = [
        ArgValue::Str(units.to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(show_datetime),
    ];
    String::from_utf8(
        ToUNIXTimestamp
            .run(input.as_bytes().to_vec(), &args)
            .unwrap(),
    )
    .unwrap()
}

#[test]
fn test_to_unix_timestamp_epoch_is_zero() {
    assert_eq!(convert("1970-01-01 00:00:00", "Seconds (s)", false), "0");
}

#[test]
fn test_to_unix_timestamp_one_second_after_the_epoch() {
    assert_eq!(convert("1970-01-01 00:00:01", "Seconds (s)", false), "1");
}

#[test]
fn test_to_unix_timestamp_year_2000() {
    // 10957 days * 86400 seconds.
    assert_eq!(
        convert("2000-01-01 00:00:00", "Seconds (s)", false),
        "946684800"
    );
}

#[test]
fn test_to_unix_timestamp_milliseconds_scale_by_1000() {
    assert_eq!(
        convert("2000-01-01 00:00:00", "Milliseconds (ms)", false),
        "946684800000"
    );
}

#[test]
fn test_to_unix_timestamp_microseconds_and_nanoseconds() {
    assert_eq!(
        convert("1970-01-01 00:00:01", "Microseconds (μs)", false),
        "1000000"
    );
    assert_eq!(
        convert("1970-01-01 00:00:01", "Nanoseconds (ns)", false),
        "1000000000"
    );
}

#[test]
fn test_to_unix_timestamp_before_the_epoch_is_negative() {
    assert_eq!(convert("1969-12-31 23:59:59", "Seconds (s)", false), "-1");
}

#[test]
fn test_to_unix_timestamp_leap_day_is_accepted() {
    // 2000 is a leap year (divisible by 400), so 2000-02-29 exists.
    assert_eq!(
        convert("2000-02-29 00:00:00", "Seconds (s)", false),
        "951782400"
    );
}

#[test]
fn test_to_unix_timestamp_show_datetime_appends_the_parsed_instant() {
    let output = convert("1970-01-01 00:00:01", "Seconds (s)", true);
    assert!(output.starts_with('1'), "unexpected output: {output}");
    assert!(
        output.contains("1970") && output.contains("UTC"),
        "expected the parsed datetime alongside the value, got: {output}"
    );
}

#[test]
fn test_to_unix_timestamp_rejects_unparseable_input() {
    let args = [
        ArgValue::Str("Seconds (s)".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let error = ToUNIXTimestamp
        .run(b"not a date at all".to_vec(), &args)
        .expect_err("unparseable input must be an error");
    assert!(
        matches!(error, OperationError::InvalidInput(_)),
        "expected InvalidInput, got {error:?}"
    );
}

#[test]
fn test_to_unix_timestamp_empty_input_produces_empty_output() {
    // Deliberate: an empty or whitespace-only buffer passes through as empty
    // rather than being reported as an unparseable date, so empty pipeline
    // stages stay empty.
    let args = [
        ArgValue::Str("Seconds (s)".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    assert_eq!(
        ToUNIXTimestamp.run(Vec::new(), &args).unwrap(),
        Vec::<u8>::new()
    );
    assert_eq!(
        ToUNIXTimestamp.run(b"   ".to_vec(), &args).unwrap(),
        Vec::<u8>::new()
    );
}
