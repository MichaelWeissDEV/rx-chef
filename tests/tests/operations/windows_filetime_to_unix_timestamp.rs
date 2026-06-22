// Tests for the windows_filetime_to_unix_timestamp operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations windows_filetime_to_unix_timestamp::

use rxchef::operation::ArgValue;
use rxchef::operations::windows_filetime_to_unix_timestamp::WindowsFiletimeToUnixTimestampOp;
use rxchef::Operation;

#[test]
fn test_windows_filetime_to_unix_seconds() {
    let op = WindowsFiletimeToUnixTimestampOp;
    let input = b"131512416000000000".to_vec(); // 2017-09-30T10:40:00Z
    let args = [
        ArgValue::Str("Seconds (s)".to_string()),
        ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "1506768000");
}
#[test]
fn test_windows_filetime_to_unix_ms() {
    let op = WindowsFiletimeToUnixTimestampOp;
    let input = b"131512416000000000".to_vec();
    let args = [
        ArgValue::Str("Milliseconds (ms)".to_string()),
        ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "1506768000000");
}
#[test]
fn test_windows_filetime_hex_be() {
    let op = WindowsFiletimeToUnixTimestampOp;
    let input = b"01d33b24f3310000".to_vec(); // 131513844000000000
    let args = [
        ArgValue::Str("Seconds (s)".to_string()),
        ArgValue::Str("Hex (big endian)".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "1506910800");
}
#[test]
fn test_windows_filetime_hex_le() {
    let op = WindowsFiletimeToUnixTimestampOp;
    let input = b"000031f3243bd301".to_vec(); // LE of 01d33b24f3310000
    let args = [
        ArgValue::Str("Seconds (s)".to_string()),
        ArgValue::Str("Hex (little endian)".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "1506910800");
}
