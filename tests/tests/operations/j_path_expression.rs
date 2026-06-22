// Tests for the j_path_expression operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations j_path_expression::

use rxchef::operation::ArgValue;
use rxchef::operations::j_path_expression::JPathExpression;
use rxchef::Operation;

#[test]
fn test_jpath_basic() {
    let op = JPathExpression;
    let input = b"{\"a\": {\"b\": 1}}".to_vec();
    let args = [
        ArgValue::Str("$.a.b".to_string()),
        ArgValue::Str("\\n".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "1");
}
#[test]
fn test_jpath_wildcard() {
    let op = JPathExpression;
    let input = b"{\"a\": [1, 2, 3]}".to_vec();
    let args = [
        ArgValue::Str("$.a[*]".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "1,2,3");
}
#[test]
fn test_jpath_empty_json() {
    let op = JPathExpression;
    let input = b"".to_vec();
    let args = [
        ArgValue::Str("$.a".to_string()),
        ArgValue::Str("\\n".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
