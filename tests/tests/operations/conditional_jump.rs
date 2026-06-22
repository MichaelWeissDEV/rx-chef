// Tests for the conditional_jump operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations conditional_jump::

use rxchef::operation::ArgValue;
use rxchef::operations::conditional_jump::ConditionalJump;
use rxchef::Operation;

#[test]
fn test_conditional_jump_skips_0() {
    let op = ConditionalJump;
    let input = b"should be changed".to_vec();
    let args = [
        ArgValue::Str("match".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("".to_string()),
        ArgValue::Num(0.0),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    // Since it's a passthrough, output is identical to input
    assert_eq!(result, input);
}
#[test]
fn test_conditional_jump_skips_1() {
    let op = ConditionalJump;
    let input = b"should be changed".to_vec();
    let args = [
        ArgValue::Str("should".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("skip match".to_string()),
        ArgValue::Num(10.0),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
#[test]
fn test_conditional_jump_skips_backwards() {
    let op = ConditionalJump;
    let input = b"match".to_vec();
    let args = [
        ArgValue::Str("match".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("back to the beginning".to_string()),
        ArgValue::Num(10.0),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
