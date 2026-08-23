// Tests for the generate_totp operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_totp::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_totp::GenerateTOTP;
use rxchef::Operation;

#[test]
fn test_generate_totp_basic() {
    let op = GenerateTOTP;
    // Secret "JBSWY3DPEHPK3PXP" is "Hello!" in Base32
    let input = b"JBSWY3DPEHPK3PXP".to_vec();
    let args = [
        ArgValue::Str("TestAccount".to_string()),
        ArgValue::Num(6.0),
        ArgValue::Num(0.0),
        ArgValue::Num(30.0),
    ];
    let result = op.run(input, &args);
    assert!(result.is_ok());
    let output = String::from_utf8(result.unwrap()).unwrap();
    assert!(output.contains("URI: otpauth://totp/TestAccount?secret=JBSWY3DPEHPK3PXP"));
    assert!(output.contains("Password: "));
}

#[test]
fn test_generate_totp_eight_digit_upper_boundary() {
    let output = GenerateTOTP
        .run(
            b"GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ".to_vec(),
            &[
                ArgValue::Str("RFC6238".into()),
                ArgValue::Num(8.0),
                ArgValue::Num(0.0),
                ArgValue::Num(30.0),
            ],
        )
        .unwrap();
    let password = String::from_utf8(output)
        .unwrap()
        .lines()
        .last()
        .unwrap()
        .strip_prefix("Password: ")
        .unwrap()
        .to_string();
    assert_eq!(password.len(), 8);
    assert!(password.bytes().all(|byte| byte.is_ascii_digit()));
}

#[test]
fn test_generate_totp_rejects_zero_interval() {
    let error = GenerateTOTP
        .run(
            b"GEZDGNBVGY3TQOJQ".to_vec(),
            &[
                ArgValue::Str("x".into()),
                ArgValue::Num(6.0),
                ArgValue::Num(0.0),
                ArgValue::Num(0.0),
            ],
        )
        .unwrap_err();
    assert!(matches!(
        error,
        rxchef::operation::OperationError::InvalidArgument { ref name, .. }
            if name == "Interval (T1)"
    ));
}
