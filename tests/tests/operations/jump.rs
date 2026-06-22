// Tests for the jump operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations jump::

use rxchef::operation::ArgValue;
use rxchef::operations::jump::Jump;
use rxchef::Operation;

#[test]
fn test_jump_passthrough() {
    let op = Jump;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("label".to_string()), ArgValue::Num(10.0)];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
