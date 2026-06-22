// Tests for the rsa_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rsa_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::rsa_encrypt::RSAEncrypt;
use rxchef::Operation;

#[test]
fn test_rsa_encrypt_basic() {
    // Need a real public key for a proper test, but we can check the error handling
    let op = RSAEncrypt;
    let input = b"hello".to_vec();
    let args = [
        ArgValue::Str("invalid key".to_string()),
        ArgValue::Str("RSA-OAEP".to_string()),
        ArgValue::Str("SHA-256".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
