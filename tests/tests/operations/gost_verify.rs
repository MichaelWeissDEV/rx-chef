// Tests for the gost_verify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations gost_verify::

use rxchef::operation::ArgValue;
use rxchef::operations::gost_verify::GOSTVerifyOp;
use rxchef::Operation;

#[test]
fn test_gost_verify_placeholder() {
    let op = GOSTVerifyOp;
    let args = [
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("GOST R 34.12 (Magma, 2015)".to_string()),
        ArgValue::Str("E-TEST".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("GOST Verify"));
}
