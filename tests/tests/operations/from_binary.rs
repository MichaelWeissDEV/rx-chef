// Tests for the from_binary operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_binary::

use rxchef::operation::ArgValue;
use rxchef::operations::from_binary::FromBinary;
use rxchef::Operation;

#[test]
fn test_from_binary_space_delim() {
    let op = FromBinary;
    let input = b"01001000 01101001".to_vec();
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Num(8.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, b"Hi");
}
#[test]
fn test_from_binary_no_delim() {
    let op = FromBinary;
    let input = b"0100100001101001".to_vec();
    let args = [ArgValue::Str("None".to_string()), ArgValue::Num(8.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, b"Hi");
}
#[test]
fn test_from_binary_empty() {
    let op = FromBinary;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
