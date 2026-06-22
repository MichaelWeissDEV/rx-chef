// Tests for the fork operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations fork::

use rxchef::operation::ArgValue;
use rxchef::operations::fork::Fork;
use rxchef::Operation;

#[test]
fn test_fork_passthrough() {
    let op = Fork;
    let input = b"line1\nline2\nline3".to_vec();
    let args = [
        ArgValue::Str("\\n".to_string()),
        ArgValue::Str("\\n".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
#[test]
fn test_fork_empty_input() {
    let op = Fork;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
