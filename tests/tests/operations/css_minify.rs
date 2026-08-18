// Tests for the css_minify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations css_minify::

use rxchef::operation::ArgValue;
use rxchef::operations::css_minify::CssMinify;
use rxchef::Operation;

#[test]
fn test_css_minify_basic() {
    let op = CssMinify;
    let input = b"body {\n    color: red;\n    margin: 0;\n}\n".to_vec();
    let args = [ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert!(!s.contains('\n'));
    assert!(s.contains("color:") || s.contains("color :"));
}
#[test]
fn test_css_minify_strips_comments() {
    let op = CssMinify;
    let input = b"/* comment */ body { color: blue; }".to_vec();
    let args = [ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert!(!s.contains("comment"));
}
#[test]
fn test_css_minify_preserves_comments() {
    let op = CssMinify;
    let input = b"/* keep me */ body { color: blue; }".to_vec();
    let args = [ArgValue::Bool(true)];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert!(s.contains("keep me"));
}
#[test]
fn test_css_minify_empty() {
    let op = CssMinify;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty() || String::from_utf8(result).unwrap().trim().is_empty());
}
