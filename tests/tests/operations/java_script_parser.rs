// Tests for the java_script_parser operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations java_script_parser::

use rxchef::operations::java_script_parser::JavaScriptParser;
use rxchef::Operation;

#[test]
fn test_parser_placeholder() {
    let op = JavaScriptParser;
    let input = b"var x = 1;".to_vec();
    let result = op.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("Program"));
    assert!(output.contains("not fully implemented"));
}
