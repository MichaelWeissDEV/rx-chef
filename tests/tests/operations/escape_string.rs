// Tests for the escape_string operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations escape_string::

use rxchef::operation::ArgValue;
use rxchef::operations::escape_string::EscapeString;
use rxchef::Operation;

#[test]
fn test_escape_string_basic() {
    let op = EscapeString;
    let args = [
        ArgValue::Str("Special chars".to_string()),
        ArgValue::Str("Single".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let input = "Don't stop me now".as_bytes().to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Don\\'t stop me now");
}
#[test]
fn test_escape_string_newline() {
    let op = EscapeString;
    let args = [
        ArgValue::Str("Special chars".to_string()),
        ArgValue::Str("Double".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let input = "Hello\nWorld".as_bytes().to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Hello\\nWorld");
}

#[test]
fn test_escape_string_invalid_utf8() {
    let op = EscapeString;
    // Operations that take String input should fail on invalid UTF-8
    let result = rxchef::Operation::run(&op, vec![0xFF, 0xFE], &[]);
    assert!(result.is_err());
}
