// Tests for the css_selector operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations css_selector::

use rxchef::operation::ArgValue;
use rxchef::operations::css_selector::CssSelector;
use rxchef::Operation;

#[test]
fn test_css_selector() {
    let op = CssSelector;
    let input = b"<div id=\"test\">\n<p class=\"a\">hello</p>\n<p>world</p>\n<p class=\"a\">again</p>\n</div>".to_vec();
    let args = vec![
        ArgValue::Str("#test p.a".to_string()),
        ArgValue::Str("\\n".to_string()),
    ];
    let output = op.run(input, &args).unwrap();
    let expected = b"<p class=\"a\">hello</p>\n<p class=\"a\">again</p>".to_vec();
    assert_eq!(output, expected);
}
