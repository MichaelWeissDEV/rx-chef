// Tests for the json_minify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations json_minify::

use rxchef::operations::json_minify::JSONMinify;
use rxchef::Operation;

#[test]
fn test_json_minify_basic() {
    let op = JSONMinify;
    let input = b"{\n  \"foo\": \"bar\",\n  \"baz\": 1\n}".to_vec();
    let result = op.run(input, &[]).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert_eq!(s, "{\"foo\":\"bar\",\"baz\":1}"); // serde_json might reorder keys if not using preserve_order
}
#[test]
fn test_json_minify_empty() {
    let op = JSONMinify;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
