// Tests for the to_octal operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_octal::

use rxchef::operation::ArgValue;
use rxchef::operations::from_octal::FromOctal;
use rxchef::operations::to_octal::ToOctal;
use rxchef::Operation;

#[test]
fn test_to_octal_hello() {
    let op = ToOctal;
    let input = b"Hello".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "110 145 154 154 157");
}
#[test]
fn test_to_octal_comma_delim() {
    let op = ToOctal;
    let input = b"Hi".to_vec();
    let args = [ArgValue::Str("Comma".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "110,151");
}
#[test]
fn test_to_octal_empty() {
    let op = ToOctal;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_roundtrip() {
    let to = ToOctal;
    let from = FromOctal;
    let original = b"Hello World".to_vec();
    let encoded = to
        .run(original.clone(), &[ArgValue::Str("Space".to_string())])
        .unwrap();
    let decoded = from
        .run(encoded, &[ArgValue::Str("Space".to_string())])
        .unwrap();
    assert_eq!(decoded, original);
}
