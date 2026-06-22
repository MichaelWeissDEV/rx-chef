// Tests for the encode_text operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations encode_text::

use rxchef::operation::ArgValue;
use rxchef::operations::encode_text::EncodeText;
use rxchef::Operation;

#[test]
fn test_encode_text_utf8() {
    let op = EncodeText;
    let args = [ArgValue::Str("UTF-8 (65001)".to_string())];
    let input = "hello".as_bytes().to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(output, b"hello");
}
#[test]
fn test_encode_text_ebcdic() {
    let op = EncodeText;
    let args = [ArgValue::Str("IBM EBCDIC International (500)".to_string())];
    let input = "hello".as_bytes().to_vec();
    let output = op.run(input, &args).unwrap();
    // hello in EBCDIC 500 is 88 85 93 93 96
    assert_eq!(output, vec![0x88, 0x85, 0x93, 0x93, 0x96]);
}
