// Tests for the regular_expression operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations regular_expression::

use rxchef::operation::ArgValue;
use rxchef::operations::regular_expression::RegularExpressionOp;
use rxchef::Operation;

#[test]
fn test_regex_highlight() {
    let op = RegularExpressionOp;
    let input = b"Hello world".to_vec();
    let args = [
        ArgValue::Str("User defined".to_string()),
        ArgValue::Str("world".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Str("Highlight matches".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("<span class='hl2' title='Offset: 6\n'>world</span>"));
}
#[test]
fn test_regex_list() {
    let op = RegularExpressionOp;
    let input = b"abc 123 def 456".to_vec();
    let args = [
        ArgValue::Str("User defined".to_string()),
        ArgValue::Str(r"\d+".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Str("List matches".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "123\n456");
}
#[test]
fn test_regex_groups() {
    let op = RegularExpressionOp;
    let input = b"key=value".to_vec();
    let args = [
        ArgValue::Str("User defined".to_string()),
        ArgValue::Str(r"(\w+)=(\w+)".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Str("List capture groups".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "key\nvalue");
}
