// Tests for the defang_url operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations defang_url::

use rxchef::operation::ArgValue;
use rxchef::operations::defang_url::DefangURL;
use rxchef::Operation;

#[test]
fn test_defang_url_basic() {
    let operation = DefangURL;
    let input = b"https://example.com/path".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("[.]"));
}
#[test]
fn test_defang_url_http() {
    let operation = DefangURL;
    let input = b"http://example.com".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("hxxp"));
}
#[test]
fn test_defang_url_no_slashes() {
    let operation = DefangURL;
    let input = b"https://example.com".to_vec();
    let result = operation
        .run(
            input,
            &[
                ArgValue::Bool(true),
                ArgValue::Bool(true),
                ArgValue::Bool(false),
                ArgValue::Str("Valid domains and full URLs".to_string()),
            ],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(!output.contains("[://]"));
}
