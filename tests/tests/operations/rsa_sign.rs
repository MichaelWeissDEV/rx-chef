// Tests for the rsa_sign operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rsa_sign::

use rxchef::operation::ArgValue;
use rxchef::operations::rsa_sign::RSASign;
use rxchef::Operation;

// NOTE: The above is a dummy key with replaced content for size, it won't actually work as a real key.
// I should generate a small real key for testing.
#[test]
fn test_rsa_sign_invalid_key() {
    let op = RSASign;
    let input = b"message".to_vec();
    let args = [
        ArgValue::Str("invalid key".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("SHA-256".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_rsa_sign_no_key() {
    let op = RSASign;
    let input = b"message".to_vec();
    let args = [
        ArgValue::Str("-----BEGIN RSA PRIVATE KEY-----".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("SHA-256".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
