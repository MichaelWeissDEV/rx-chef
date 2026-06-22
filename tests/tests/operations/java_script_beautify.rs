// Tests for the java_script_beautify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations java_script_beautify::

use rxchef::operation::ArgValue;
use rxchef::operations::java_script_beautify::JavaScriptBeautify;
use rxchef::Operation;

#[test]
fn test_js_beautify_basic() {
    let op = JavaScriptBeautify;
    let input = b"function test(){console.log('hello');}".to_vec();
    let args = [ArgValue::Str("  ".to_string())];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    // Just verify basic formatting structure exists
    assert!(s.contains("function test()"));
    assert!(s.contains("console.log('hello');"));
    assert!(s.contains('{'));
    assert!(s.contains('}'));
}
