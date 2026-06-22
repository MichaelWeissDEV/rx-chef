// Tests for the has160 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations has160::

use rxchef::operation::ArgValue;
use rxchef::operations::has160::HAS160Op;
use rxchef::Operation;

#[test]
fn test_has160_placeholder() {
    let op = HAS160Op;
    let args = [ArgValue::Num(80.0)];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("HAS-160"));
    assert!(result_str.contains("80"));
}
