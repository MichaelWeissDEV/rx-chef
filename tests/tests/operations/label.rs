// Tests for the label operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations label::

use rxchef::operation::ArgValue;
use rxchef::operations::label::Label;
use rxchef::Operation;

#[test]
fn test_label_passthrough() {
    let op = Label;
    let input = b"some data".to_vec();
    let args = [ArgValue::Str("test_label".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
