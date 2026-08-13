// Tests for the x_path_expression operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations x_path_expression::

use rxchef::operation::ArgValue;
use rxchef::operations::x_path_expression::XPathExpression;
use rxchef::Operation;

#[test]
fn test_xpath_expression_selects_all_nodes() {
    let op = XPathExpression;
    let input = b"<root><item>one</item><item>two</item></root>".to_vec();
    let args = [
        ArgValue::Str("/root/item".to_string()),
        ArgValue::Str("\\n".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, b"one\ntwo");
}

#[test]
fn test_xpath_expression_scalar_result() {
    let result = XPathExpression
        .run(
            b"<root><item>one</item><item>two</item></root>".to_vec(),
            &[ArgValue::Str("count(/root/item)".to_string())],
        )
        .unwrap();
    assert_eq!(result, b"2");
}
