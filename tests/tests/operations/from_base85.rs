// Tests for the from_base85 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base85::

use rxchef::operations::from_base85::FromBase85;
use rxchef::Operation;

#[test]
fn test_from_base85_empty_input() {
    let op = FromBase85;
    let args = [
        rxchef::operation::ArgValue::Str("!-u".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Str("z".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_base85_standard_alphabet() {
    let op = FromBase85;
    let args = [
        rxchef::operation::ArgValue::Str("!-u".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Str("z".to_string()),
    ];
    // Simple Base85 encoding
    let base85_input = "BOu!r";
    let result = op.run(base85_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_base85_with_delimiters() {
    let op = FromBase85;
    let args = [
        rxchef::operation::ArgValue::Str("!-u".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Str("z".to_string()),
    ];
    // Base85 with delimiters
    let base85_input = "<~BOu!r~>";
    let result = op.run(base85_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_base85_zero_group_char() {
    let op = FromBase85;
    let args = [
        rxchef::operation::ArgValue::Str("!-u".to_string()),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Str("z".to_string()),
    ];
    // Base85 with zero group character
    let base85_input = "z";
    let result = op.run(base85_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    // Should decode to 4 zero bytes
    assert_eq!(decoded, vec![0, 0, 0, 0]);
}

#[test]
fn test_from_base85_invalid_alphabet_length() {
    let op = FromBase85;
    let args = [
        rxchef::operation::ArgValue::Str("ABC".to_string()), // Too short
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Str("z".to_string()),
    ];
    let base85_input = "BOu!r";
    let result = op.run(base85_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid alphabet length
    assert!(result.is_err());
}
