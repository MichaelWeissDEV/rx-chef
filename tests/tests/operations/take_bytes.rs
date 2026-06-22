// Tests for the take_bytes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations take_bytes::

use rxchef::operation::ArgValue;
use rxchef::operations::take_bytes::TakeBytes;
use rxchef::Operation;

fn run_op(input: &[u8], start: i64, length: i64, each_line: bool) -> Vec<u8> {
    let op = TakeBytes;
    let args = [
        ArgValue::Num(start as f64),
        ArgValue::Num(length as f64),
        ArgValue::Bool(each_line),
    ];
    op.run(input.to_vec(), &args).unwrap()
}
#[test]
fn test_take_bytes_basic() {
    // Take 5 bytes starting at 0
    let result = run_op(b"Hello, World!", 0, 5, false);
    assert_eq!(result, b"Hello");
}
#[test]
fn test_take_bytes_offset() {
    // Take 5 bytes starting at 7
    let result = run_op(b"Hello, World!", 7, 5, false);
    assert_eq!(result, b"World");
}
#[test]
fn test_take_bytes_negative_start() {
    // start=-6 from "Hello, World!" (13 bytes) => start=7
    let result = run_op(b"Hello, World!", -6, 5, false);
    assert_eq!(result, b"World");
}
#[test]
fn test_take_bytes_each_line() {
    let input = b"Hello\nWorld";
    let result = run_op(input, 0, 3, true);
    assert_eq!(result, b"Hel\nWor");
}
#[test]
fn test_take_bytes_empty() {
    let result = run_op(b"", 0, 5, false);
    assert_eq!(result, b"");
}
#[test]
fn test_take_bytes_beyond_length() {
    // Taking more than available
    let result = run_op(b"Hi", 0, 100, false);
    assert_eq!(result, b"Hi");
}
