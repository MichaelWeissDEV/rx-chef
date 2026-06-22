// Tests for the to_morse_code operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_morse_code::

use rxchef::operations::to_morse_code::ToMorseCode;
use rxchef::Operation;

#[test]
fn test_to_morse_code_empty_input() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_to_morse_code_simple_letter() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("A".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, ".-".as_bytes());
}

#[test]
fn test_to_morse_code_sos() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("SOS".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "... --- ...".as_bytes());
}

#[test]
fn test_to_morse_code_word() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("HELLO".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, ".... . .-.. .-.. ---".as_bytes());
}

#[test]
fn test_to_morse_code_with_spaces() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("HELLO WORLD".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, ".... . .-.. .-.. ---
.-- --- .-. .-.. -..".as_bytes());
}

#[test]
fn test_to_morse_code_custom_format() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("X/Y".to_string()), // Use X for dash, Y for dot
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("SOS".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "YYY XXX YYY".as_bytes());
}

#[test]
fn test_to_morse_code_custom_delimiters() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str(",".to_string()), // Comma as letter delimiter
        rxchef::operation::ArgValue::Str(";".to_string()), // Semicolon as word delimiter
    ];
    let result = op.run("HELLO WORLD".as_bytes().to_vec(), &args).unwrap();
    // Should use custom delimiters
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains(",")); // Custom letter delimiter
    assert!(result_str.contains(";")); // Custom word delimiter
}

#[test]
fn test_to_morse_code_numbers() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("123".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, ".---- ..--- ...--".as_bytes());
}

#[test]
fn test_to_morse_code_punctuation() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run(".,?".as_bytes().to_vec(), &args).unwrap();
    // Punctuation should be converted to Morse code
    assert!(result.len() > 0);
}

#[test]
fn test_to_morse_code_mixed_case() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("Hello".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, ".... . .-.. .-.. ---".as_bytes());
}

#[test]
fn test_to_morse_code_non_morse_characters() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("HELLO 123 @".as_bytes().to_vec(), &args).unwrap();
    // @ should be ignored (not in Morse code)
    // Should contain HELLO and 123 but not @
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("....")); // H
    assert!(result_str.contains(".----")); // 1
    assert!(!result_str.contains("@"));
}

#[test]
fn test_to_morse_code_multiline_input() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let input = "HELLO\nWORLD".as_bytes().to_vec();
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, ".... . .-.. .-.. ---
.-- --- .-. .-.. -..".as_bytes());
}

#[test]
fn test_to_morse_code_invalid_format() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("invalid".to_string()), // Missing slash
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("A".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_to_morse_code_no_letter_delimiter() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("None".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run("HELLO".as_bytes().to_vec(), &args).unwrap();
    // No letter delimiter should concatenate Morse codes
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "......-...-..---");
}

#[test]
fn test_to_morse_code_no_word_delimiter() {
    let op = ToMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("-/.".to_string()),
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("None".to_string()),
    ];
    let result = op.run("HELLO WORLD".as_bytes().to_vec(), &args).unwrap();
    // No word delimiter should concatenate words
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("....")); // H
    assert!(result_str.contains(".--")); // W
    assert!(!result_str.contains("\n")); // No newline
}
