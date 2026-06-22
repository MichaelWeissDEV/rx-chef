// Tests for the to_base operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_base::

use rxchef::operation::ArgValue;
use rxchef::operations::to_base::ToBase;
use rxchef::Operation;

#[test]
fn test_to_binary() {
    let op = ToBase;
    let input = b"10".to_vec();
    let args = [ArgValue::Num(2.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "1010");
}
#[test]
fn test_to_hex() {
    let op = ToBase;
    let input = b"255".to_vec();
    let args = [ArgValue::Num(16.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "ff");
}
#[test]
fn test_to_base36() {
    let op = ToBase;
    let input = b"35".to_vec();
    let args = [ArgValue::Num(36.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "z");
}
#[test]
fn test_invalid_radix() {
    let op = ToBase;
    let input = b"10".to_vec();
    let args = [ArgValue::Num(37.0)];
    assert!(op.run(input, &args).is_err());
}
#[test]
fn test_zero() {
    let op = ToBase;
    let input = b"0".to_vec();
    let args = [ArgValue::Num(2.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "0");
}
