// Tests for the x_path_expression operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations x_path_expression::

use rxchef::operation::ArgValue;
use rxchef::operations::x_path_expression::XPathExpression;
use rxchef::Operation;
use rxchef::OperationError;

#[test]
fn test_xpath_expression_placeholder() {
    let op = XPathExpression;
    let input = b"<root><item>test</item></root>".to_vec();
    let args = [
        ArgValue::Str("/root/item".to_string()),
        ArgValue::Str("\\n".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
    if let Err(OperationError::ProcessingError(msg)) = result {
        assert!(msg.contains("not yet fully implemented"));
    } else {
        panic!("Expected ProcessingError");
    }
}
