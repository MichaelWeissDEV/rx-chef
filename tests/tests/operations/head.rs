// Tests for the head operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations head::

use rxchef::operation::ArgValue;
use rxchef::operations::head::Head;
use rxchef::Operation;

#[test]
fn test_head_first_3() {
    let op = Head;
    let input = b"a\nb\nc\nd\ne".to_vec();
    let result = op
        .run(
            input,
            &[ArgValue::Str("Line feed".to_string()), ArgValue::Num(3.0)],
        )
        .expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "a\nb\nc");
}
#[test]
fn test_head_negative() {
    let op = Head;
    // All but last 2
    let input = b"a\nb\nc\nd\ne".to_vec();
    let result = op
        .run(
            input,
            &[ArgValue::Str("Line feed".to_string()), ArgValue::Num(-2.0)],
        )
        .expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "a\nb\nc");
}
#[test]
fn test_head_more_than_available() {
    let op = Head;
    let input = b"a\nb".to_vec();
    let result = op
        .run(
            input,
            &[ArgValue::Str("Line feed".to_string()), ArgValue::Num(10.0)],
        )
        .expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "a\nb");
}
