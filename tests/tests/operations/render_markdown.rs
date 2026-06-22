// Tests for the render_markdown operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations render_markdown::

use rxchef::operation::ArgValue;
use rxchef::operations::render_markdown::RenderMarkdown;
use rxchef::Operation;

#[test]
fn test_render_markdown_headers() {
    let op = RenderMarkdown;
    let input = b"# Header 1\n## Header 2".to_vec();
    let args = [
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("<h1>Header 1</h1>"));
    assert!(result_str.contains("<h2>Header 2</h2>"));
}
#[test]
fn test_render_markdown_formatting() {
    let op = RenderMarkdown;
    let input = b"**bold** and *italic*".to_vec();
    let args = [
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("<strong>bold</strong>"));
    assert!(result_str.contains("<em>italic</em>"));
}
#[test]
fn test_render_markdown_links() {
    let op = RenderMarkdown;
    let input = b"[Google](https://google.com)".to_vec();
    let args = [
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("<a href=\"https://google.com\"  target=\"_blank\">Google</a>"));
}
