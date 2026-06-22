// Tests for the decode_text operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations decode_text::

use base64::Engine;
use rxchef::operation::ArgValue;
use rxchef::operations::decode_text::DecodeText;
use rxchef::Operation;

#[test]
fn test_decode_text_nothing() {
    let op = DecodeText;
    let args = [ArgValue::Str("UTF-8 (65001)".to_string())];
    let input = b"".to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "");
}
#[test]
fn test_decode_text_hello() {
    let op = DecodeText;
    let args = [ArgValue::Str("UTF-8 (65001)".to_string())];
    let input = b"hello".to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "hello");
}
#[test]
fn test_decode_text_ebcdic() {
    let op = DecodeText;
    let args = [ArgValue::Str("IBM EBCDIC International (500)".to_string())];
    // Input: 88 85 93 93 96 in hex
    let input = vec![0x88, 0x85, 0x93, 0x93, 0x96];
    let output = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "hello");
}
#[test]
fn test_decode_text_utf16le() {
    let base64_str = "ZABpAHIAIAAiAGMAOgBcAHAAcgBvAGcAcgBhAG0AIABmAGkAbABlAHMAIgAgAA==";
    let input = base64::engine::general_purpose::STANDARD
        .decode(base64_str)
        .unwrap();
    let op = DecodeText;
    let args = [ArgValue::Str("UTF-16LE (1200)".to_string())];
    let output = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "dir \"c:\\program files\" "
    );
}
