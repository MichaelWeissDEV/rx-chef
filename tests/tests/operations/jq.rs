// Tests for the jq operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations jq::

use rxchef::operation::ArgValue;
use rxchef::operations::jq::Jq;
use rxchef::Operation;

#[test]
fn test_jq_basic() {
    let op = Jq;
    let input = b"{\"a\": 1, \"b\": 2}".to_vec();
    let args = [ArgValue::Str(".a".to_string()), ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "1");
}
#[test]
fn test_jq_raw() {
    let op = Jq;
    let input = b"{\"a\": \"hello\", \"b\": 2}".to_vec();
    let args = [ArgValue::Str(".a".to_string()), ArgValue::Bool(true)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "hello");
}
