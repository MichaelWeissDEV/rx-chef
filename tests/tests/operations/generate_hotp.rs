// Tests for the generate_hotp operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_hotp::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_hotp::GenerateHOTPOp;
use rxchef::Operation;

#[test]
fn test_generate_hotp_basic() {
    let op = GenerateHOTPOp;
    let input = b"GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ".to_vec();
    let args = [
        ArgValue::Str("Test".to_string()),
        ArgValue::Num(6.0),
        ArgValue::Num(0.0),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Password: 755224"));
}
