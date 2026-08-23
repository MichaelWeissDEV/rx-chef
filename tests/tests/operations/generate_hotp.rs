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
    // RFC 4226 Appendix D, counter 0.
    assert_eq!(result_str.lines().last(), Some("Password: 755224"));
}

#[test]
fn test_generate_hotp_six_digit_minimum_boundary() {
    let output = GenerateHOTPOp
        .run(
            b"GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ".to_vec(),
            &[
                ArgValue::Str("RFC4226".into()),
                ArgValue::Num(6.0),
                ArgValue::Num(1.0),
            ],
        )
        .unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap().lines().last(),
        Some("Password: 287082")
    );
}

#[test]
fn test_generate_hotp_rejects_invalid_base32_secret() {
    let error = GenerateHOTPOp
        .run(
            b"NOT*BASE32".to_vec(),
            &[
                ArgValue::Str("x".into()),
                ArgValue::Num(6.0),
                ArgValue::Num(0.0),
            ],
        )
        .unwrap_err();
    assert!(matches!(
        error,
        rxchef::operation::OperationError::InvalidInput(_)
    ));
}
