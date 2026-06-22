// Tests for the extract_urls operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_urls::

use rxchef::operation::ArgValue;
use rxchef::operations::extract_urls::ExtractURLs;
use rxchef::Operation;

#[test]
fn test_extract_urls() {
    let op = ExtractURLs;
    let input = b"Check out https://google.com and http://example.com/path?query=1".to_vec();
    let args = [
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "https://google.com\nhttp://example.com/path?query=1");
}
