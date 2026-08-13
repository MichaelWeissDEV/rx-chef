// Tests for the java_script_minify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations java_script_minify::

use rxchef::operations::java_script_minify::JavaScriptMinify;
use rxchef::Operation;

#[test]
fn test_minify_basic() {
    let op = JavaScriptMinify;
    let input = b"function add(a, b) {\n    return a + b;\n}".to_vec();
    let result = op.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("function add(a, b) {"));
    assert!(output.contains("return a + b;"));
}
#[test]
fn test_minify_comments() {
    let op = JavaScriptMinify;
    let input = b"// This is a comment\nfunction test() {\n    /* Multi-line\n       comment */\n    return 1;\n}".to_vec();
    let result = op.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(!output.contains("comment"));
    assert!(output.contains("function test() {"));
}
#[test]
fn test_minify_whitespace() {
    let op = JavaScriptMinify;
    let input = b"var x   =    10;   \n   var y = 20;".to_vec();
    let result = op.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "var x = 10;\nvar y = 20;");
}

#[test]
fn test_minify_preserves_comment_markers_in_literals() {
    let input = br#"
        const url = "https://example.test/a//b"; // remove me
        const template = `/* literal */`;
        const pattern = /https?:\/\/[^/]+/g;
    "#;
    let output = JavaScriptMinify.run(input.to_vec(), &[]).unwrap();
    let output = String::from_utf8(output).unwrap();
    assert!(output.contains("https://example.test/a//b"));
    assert!(output.contains("`/* literal */`"));
    assert!(output.contains(r"/https?:\/\/[^/]+/g"));
    assert!(!output.contains("remove me"));
}
