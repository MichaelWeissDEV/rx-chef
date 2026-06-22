// Tests for the subsection operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations subsection::

use rxchef::operation::ArgValue;
use rxchef::operations::subsection::Subsection;
use rxchef::Operation;

#[test]
fn test_subsection_passthrough() {
    let op = Subsection;
    let input = b"Hello World".to_vec();
    let args = [
        ArgValue::Str("o".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
#[test]
fn test_subsection_empty_input() {
    let op = Subsection;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_subsection_no_regex() {
    let op = Subsection;
    let input = b"data".to_vec();
    let args = [
        ArgValue::Str("".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
