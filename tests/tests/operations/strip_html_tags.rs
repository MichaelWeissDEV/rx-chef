// Tests for the strip_html_tags operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations strip_html_tags::

use rxchef::operations::strip_html_tags::StripHTMLTags;
use rxchef::Operation;

#[test]
fn test_strip_basic_tags() {
    let op = StripHTMLTags;
    let input = b"<p>Hello <b>World</b></p>".to_vec();
    let result = op.run(input, &[]).expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "Hello World");
}
#[test]
fn test_strip_with_attributes() {
    let op = StripHTMLTags;
    let input = b"<a href=\"http://example.com\">link</a>".to_vec();
    let result = op.run(input, &[]).expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "link");
}
#[test]
fn test_strip_empty() {
    let op = StripHTMLTags;
    let result = op.run(b"".to_vec(), &[]).expect("should succeed");
    assert_eq!(result, b"");
}
