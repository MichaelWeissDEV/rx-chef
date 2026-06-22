// Tests for the generic_code_beautify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generic_code_beautify::

use rxchef::operations::generic_code_beautify::GenericCodeBeautify;
use rxchef::Operation;

#[test]
fn test_generic_code_beautify() {
    let op = GenericCodeBeautify;
    let input = b"if(a==b){console.log(\"hello\");}else{return false;}".to_vec();
    let result = op.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("if"));
    assert!(output.contains("a == b"));
    assert!(output.contains("console.log(\"hello\");"));
    assert!(output.contains("else"));
}
