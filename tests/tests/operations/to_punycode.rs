// Tests for the to_punycode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_punycode::

use rxchef::operation::ArgValue;
use rxchef::operations::to_punycode::ToPunycode;
use rxchef::Operation;

#[test]
fn test_to_punycode_ascii_passthrough() {
    let op = ToPunycode;
    let input = b"example".to_vec();
    let args = [ArgValue::Bool(false)];
    let result = op.run(input, &args);
    assert!(result.is_ok());
}
#[test]
fn test_to_punycode_idn() {
    let op = ToPunycode;
    let input = "example.com".as_bytes().to_vec();
    let args = [ArgValue::Bool(true)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "example.com");
}
