// Tests for the html_to_text operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations html_to_text::

use rxchef::operations::html_to_text::HTMLToText;
use rxchef::Operation;

#[test]
fn test_html_to_text_basic() {
    let op = HTMLToText;
    let input = b"<p>Hello &amp; World</p>".to_vec();
    let result = op.run(input, &[]).expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "Hello & World");
}
#[test]
fn test_html_to_text_entities() {
    let op = HTMLToText;
    let input = b"&lt;tag&gt;".to_vec();
    let result = op.run(input, &[]).expect("should succeed");
    assert_eq!(String::from_utf8(result).unwrap(), "<tag>");
}
#[test]
fn test_html_to_text_empty() {
    let op = HTMLToText;
    let result = op.run(b"".to_vec(), &[]).expect("should succeed");
    assert_eq!(result, b"");
}
