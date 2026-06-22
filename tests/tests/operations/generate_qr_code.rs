// Tests for the generate_qr_code operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_qr_code::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_qr_code::GenerateQRCodeOp;
use rxchef::Operation;

#[test]
fn test_generate_qr_code_placeholder() {
    let op = GenerateQRCodeOp;
    let input = b"Hello World".to_vec();
    let args = [
        ArgValue::Str("PNG".to_string()),
        ArgValue::Num(5.0),
        ArgValue::Num(4.0),
        ArgValue::Str("Medium".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Hello World"));
    assert!(result_str.contains("PNG"));
}
