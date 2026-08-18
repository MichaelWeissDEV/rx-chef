// Tests for the css_beautify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations css_beautify::

use rxchef::operation::ArgValue;
use rxchef::operations::css_beautify::CssBeautify;
use rxchef::Operation;

#[test]
fn test_css_beautify_basic() {
    let op = CssBeautify;
    let input = b"body{color:red;margin:0;}".to_vec();
    let args = [ArgValue::Str("    ".to_string())];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert!(s.contains("{\n"));
    assert!(s.contains("}\n") || s.ends_with('}'));
    // Beautifier inserts a space after ':', so red → "color: red;"
    assert!(s.contains("    color: red;"));
}
#[test]
fn test_css_beautify_empty() {
    let op = CssBeautify;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty() || String::from_utf8(result).unwrap().trim().is_empty());
}
#[test]
fn test_css_beautify_nested() {
    let op = CssBeautify;
    let input = b"@media screen{body{color:blue;}}".to_vec();
    let args = [ArgValue::Str("  ".to_string())];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    // Should have nested indentation
    assert!(s.contains("  body"));
}
#[test]
fn test_css_beautify_default_indent() {
    let op = CssBeautify;
    let input = b"p{font-size:12px;}".to_vec();
    let result = op.run(input, &[]).unwrap();
    let s = String::from_utf8(result).unwrap();
    // Beautifier inserts a space after ':', so font-size:12px → "font-size: 12px;"
    assert!(s.contains("font-size: 12px;"));
}
