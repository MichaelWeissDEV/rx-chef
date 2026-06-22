// Tests for the text_integer_converter operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations text_integer_converter::

use rxchef::operation::ArgValue;
use rxchef::operations::text_integer_converter::TextIntegerConverter;
use rxchef::Operation;

#[test]
fn test_quoted_string_to_decimal() {
    let op = TextIntegerConverter;
    let result = op
        .run(b"\"ABC\"".to_vec(), &[ArgValue::Str("Decimal".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "4276803");
}
#[test]
fn test_quoted_string_to_hex() {
    let op = TextIntegerConverter;
    let result = op
        .run(
            b"\"ABC\"".to_vec(),
            &[ArgValue::Str("Hexadecimal".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "0x414243");
}
#[test]
fn test_decimal_to_string() {
    let op = TextIntegerConverter;
    let result = op
        .run(b"4276803".to_vec(), &[ArgValue::Str("String".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "ABC");
}
#[test]
fn test_hex_to_string() {
    let op = TextIntegerConverter;
    let result = op
        .run(
            b"0x48656C6C6F".to_vec(),
            &[ArgValue::Str("String".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Hello");
}
#[test]
fn test_unquoted_text_to_decimal() {
    let op = TextIntegerConverter;
    let result = op
        .run(b"Hi".to_vec(), &[ArgValue::Str("Decimal".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "18537");
}
#[test]
fn test_large_number_to_string() {
    let op = TextIntegerConverter;
    let result = op
        .run(
            b"113091951015816448506195587157728348242683688608116".to_vec(),
            &[ArgValue::Str("String".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Mary had a little cat");
}
#[test]
fn test_hex_to_decimal() {
    let op = TextIntegerConverter;
    let result = op
        .run(b"0xFF".to_vec(), &[ArgValue::Str("Decimal".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "255");
}
#[test]
fn test_decimal_to_hex() {
    let op = TextIntegerConverter;
    let result = op
        .run(b"255".to_vec(), &[ArgValue::Str("Hexadecimal".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "0xff");
}
#[test]
fn test_non_latin1_error() {
    let op = TextIntegerConverter;
    // A multi-byte UTF-8 character ( = U+0393, > 255)
    let input = "a\u{0393}a".as_bytes().to_vec();
    let result = op.run(input, &[ArgValue::Str("Decimal".to_string())]);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Latin-1"));
}
#[test]
fn test_single_char_to_decimal() {
    let op = TextIntegerConverter;
    let result = op
        .run(b"\"A\"".to_vec(), &[ArgValue::Str("Decimal".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "65");
}
#[test]
fn test_single_quoted_string() {
    let op = TextIntegerConverter;
    let result = op
        .run(b"'Hello'".to_vec(), &[ArgValue::Str("Decimal".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "310939249775");
}
