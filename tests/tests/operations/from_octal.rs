// Tests for the from_octal operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_octal::

use rxchef::operation::ArgValue;
use rxchef::operations::from_octal::FromOctal;
use rxchef::Operation;

#[test]
fn test_from_octal_hello() {
    let op = FromOctal;
    let input = b"110 145 154 154 157".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, b"Hello");
}
#[test]
fn test_from_octal_comma() {
    let op = FromOctal;
    let input = b"110,145,154".to_vec();
    let args = [ArgValue::Str("Comma".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, b"Hel");
}
#[test]
fn test_from_octal_empty() {
    let op = FromOctal;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_from_octal_invalid() {
    let op = FromOctal;
    let input = b"999".to_vec(); // 999 octal is 511, too large for u8
    assert!(op.run(input, &[]).is_err());
}
