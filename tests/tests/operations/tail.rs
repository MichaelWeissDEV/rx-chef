// Tests for the tail operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations tail::

use rxchef::operation::ArgValue;
use rxchef::operations::tail::Tail;
use rxchef::Operation;

#[test]
fn test_tail_last_3() {
    let op = Tail;
    let input = b"a\nb\nc\nd\ne".to_vec();
    let result = op
        .run(
            input,
            &[ArgValue::Str("Line feed".to_string()), ArgValue::Num(3.0)],
        )
        .expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "c\nd\ne");
}
#[test]
fn test_tail_negative() {
    let op = Tail;
    // All after line 2 (skip first 2)
    let input = b"a\nb\nc\nd\ne".to_vec();
    let result = op
        .run(
            input,
            &[ArgValue::Str("Line feed".to_string()), ArgValue::Num(-2.0)],
        )
        .expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "c\nd\ne");
}
#[test]
fn test_tail_more_than_available() {
    let op = Tail;
    let input = b"a\nb".to_vec();
    let result = op
        .run(
            input,
            &[ArgValue::Str("Line feed".to_string()), ArgValue::Num(10.0)],
        )
        .expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "a\nb");
}
