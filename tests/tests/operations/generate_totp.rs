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
