// Tests for the sleep operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sleep::

use rxchef::operation::ArgValue;
use rxchef::operations::sleep::Sleep;
use rxchef::Operation;
use std::time::Duration;
use std::time::Instant;

#[test]
fn test_sleep_returns_input() {
    let op = Sleep;
    let input = b"Hello".to_vec();
    let args = [ArgValue::Num(10.0)];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
#[test]
fn test_sleep_duration() {
    let op = Sleep;
    let input = b"".to_vec();
    let args = [ArgValue::Num(100.0)];
    let start = Instant::now();
    let _ = op.run(input, &args).unwrap();
    let duration = start.elapsed();
    assert!(duration >= Duration::from_millis(100));
}
