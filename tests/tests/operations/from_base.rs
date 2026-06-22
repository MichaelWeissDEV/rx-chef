// Tests for the from_base operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base::

use rxchef::operation::ArgValue;
use rxchef::operations::from_base::FromBase;
use rxchef::Operation;

#[test]
fn test_from_binary() {
    let op = FromBase;
    let input = b"1010".to_vec();
    let args = [ArgValue::Num(2.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "10");
}
#[test]
fn test_from_hex() {
    let op = FromBase;
    let input = b"ff".to_vec();
    let args = [ArgValue::Num(16.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "255");
}
#[test]
fn test_from_base36() {
    let op = FromBase;
    let input = b"z".to_vec();
    let args = [ArgValue::Num(36.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "35");
}
#[test]
fn test_invalid_radix() {
    let op = FromBase;
    let input = b"10".to_vec();
    let args = [ArgValue::Num(1.0)];
    assert!(op.run(input, &args).is_err());
}
