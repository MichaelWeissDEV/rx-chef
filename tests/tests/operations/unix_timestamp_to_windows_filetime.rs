// Tests for the unix_timestamp_to_windows_filetime operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations unix_timestamp_to_windows_filetime::

use rxchef::operation::ArgValue;
use rxchef::operations::unix_timestamp_to_windows_filetime::UNIXTimestampToWindowsFiletime;
use rxchef::Operation;

#[test]
fn test_seconds_to_decimal() {
    let op = UNIXTimestampToWindowsFiletime;
    let input = b"1500000000".to_vec();
    let args = [
        ArgValue::Str("Seconds (s)".to_string()),
        ArgValue::Str("Decimal".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "131444736000000000");
}
#[test]
fn test_milliseconds_to_hex_be() {
    let op = UNIXTimestampToWindowsFiletime;
    let input = b"1500000000000".to_vec();
    let args = [
        ArgValue::Str("Milliseconds (ms)".to_string()),
        ArgValue::Str("Hex (big endian)".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "1d2fc4a7ce00000");
}
#[test]
fn test_nanoseconds_to_hex_le() {
    let op = UNIXTimestampToWindowsFiletime;
    let input = b"1500000000000000000".to_vec();
    let args = [
        ArgValue::Str("Nanoseconds (ns)".to_string()),
        ArgValue::Str("Hex (little endian)".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    // 1500000000000000000 / 100 = 15000000000000000
    // 15000000000000000 + 116444736000000000 = 131444736000000000
    // 131444736000000000 in hex is 1d2fa955f9a6000
    // Little endian: 00 60 9a 5f 95 a9 2f d1 (padded to even length)
    // Wait, 1d2fa955f9a6000 is 15 chars. Padded: 01d2fa955f9a6000
    // LE: 00 60 9a 5f 95 a9 d2 01
    assert_eq!(String::from_utf8_lossy(&result), "0000e07c4afcd201");
}
