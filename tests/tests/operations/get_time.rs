// Tests for the get_time operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations get_time::

use rxchef::operation::ArgValue;
use rxchef::operations::get_time::GetTime;
use rxchef::Operation;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_get_time_seconds() {
    let op = GetTime;
    let before = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let result = op
        .run(vec![], &[ArgValue::Str("Seconds (s)".to_string())])
        .expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    let ts: u64 = s.parse().expect("numeric");
    let after = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // POSIX epoch seconds must lie between independent clock samples.
    assert!((before..=after).contains(&ts));
}
#[test]
fn test_get_time_millis() {
    let op = GetTime;
    let result = op
        .run(vec![], &[ArgValue::Str("Milliseconds (ms)".to_string())])
        .expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    let ts: u64 = s.parse().expect("numeric");
    assert!(ts > 1_577_836_800_000);
}
#[test]
fn test_get_time_invalid() {
    let op = GetTime;
    let result = op.run(vec![], &[ArgValue::Str("years".to_string())]);
    assert!(result.is_err());
}
