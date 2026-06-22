// Tests for the to_binary operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_binary::

use rxchef::operation::ArgValue;
use rxchef::operations::to_binary::ToBinary;
use rxchef::Operation;

#[test]
fn test_to_binary_space_delim() {
    let op = ToBinary;
    let input = b"Hi".to_vec();
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Num(8.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "01001000 01101001");
}
#[test]
fn test_to_binary_no_delim() {
    let op = ToBinary;
    let input = b"Hi".to_vec();
    let args = [ArgValue::Str("None".to_string()), ArgValue::Num(8.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "0100100001101001");
}
#[test]
fn test_to_binary_empty() {
    let op = ToBinary;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
