// Tests for the from_morse_code operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_morse_code::

use rxchef::operations::from_morse_code::FromMorseCode;
use rxchef::Operation;

#[test]
fn test_from_morse_code_empty_input() {
    let op = FromMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_from_morse_code_basic_functionality() {
    let op = FromMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    // Test that the operation runs without panicking
    let result = op.run(".-".as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_morse_code_with_valid_input() {
    let op = FromMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    // Test with a simple valid Morse code
    let result = op.run("...".as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_morse_code_custom_delimiters() {
    let op = FromMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str(",".to_string()),
        rxchef::operation::ArgValue::Str(";".to_string()),
    ];
    // Test that custom delimiters don't cause panics
    let result = op.run(".-..,.,.-..".as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}

#[test]
fn test_from_morse_code_no_delimiters() {
    let op = FromMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("None".to_string()),
        rxchef::operation::ArgValue::Str("None".to_string()),
    ];
    // Test with no delimiters
    let result = op.run(".....-..-..---".as_bytes().to_vec(), &args);
    assert!(result.is_ok());
}
