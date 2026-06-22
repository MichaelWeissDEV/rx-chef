// Tests for the syntax_highlighter operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations syntax_highlighter::

use rxchef::operation::ArgValue;
use rxchef::operations::syntax_highlighter::SyntaxHighlighter;
use rxchef::Operation;

#[test]
fn test_syntax_highlighter_basic() {
    let op = SyntaxHighlighter;
    let input = b"var a = 1;".to_vec();
    let args = [ArgValue::Str("javascript".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&result),
        "<pre><code class=\"language-javascript\">var a = 1;</code></pre>"
    );
}
#[test]
fn test_syntax_highlighter_auto() {
    let op = SyntaxHighlighter;
    let input = b"var a = 1;".to_vec();
    let args = [ArgValue::Str("auto detect".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&result),
        "<pre><code>var a = 1;</code></pre>"
    );
}
#[test]
fn test_syntax_highlighter_html_escape() {
    let op = SyntaxHighlighter;
    let input = b"<script>alert(1)</script>".to_vec();
    let args = [ArgValue::Str("html".to_string())];
    let result = op.run(input, &args).unwrap();
    assert!(String::from_utf8_lossy(&result).contains("&lt;script&gt;"));
}
