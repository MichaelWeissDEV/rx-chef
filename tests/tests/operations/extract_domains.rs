// Tests for the extract_domains operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_domains::

use rxchef::operation::ArgValue;
use rxchef::operations::extract_domains::ExtractDomains;
use rxchef::Operation;

#[test]
fn test_extract_domains() {
    let op = ExtractDomains;
    let input = b"Visit example.com or sub.example.org for more info.".to_vec();
    let args = [
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "example.com\nsub.example.org");
}
#[test]
fn test_extract_domains_dmarc() {
    let op = ExtractDomains;
    let input = b"Check _dmarc.example.com".to_vec();
    let args = [
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "_dmarc.example.com");
}
