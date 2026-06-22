// Tests for the to_hex_content operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_hex_content::

use rxchef::operation::ArgValue;
use rxchef::operations::to_hex_content::ToHexContent;
use rxchef::Operation;

#[test]
fn test_to_hex_content_special_chars() {
    let op = ToHexContent;
    let input = b"foo=bar".to_vec();
    let args = [
        ArgValue::Str("Only special chars".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "foo|3d|bar");
}
#[test]
fn test_to_hex_content_all_chars() {
    let op = ToHexContent;
    let input = b"AB".to_vec();
    let args = [
        ArgValue::Str("All chars".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "|4142|");
}
#[test]
fn test_to_hex_content_with_spaces() {
    let op = ToHexContent;
    let input = b"=".to_vec();
    let args = [
        ArgValue::Str("Only special chars".to_string()),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "|3d|");
}
