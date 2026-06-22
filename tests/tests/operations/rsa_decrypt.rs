// Tests for the rsa_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rsa_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::rsa_decrypt::RSADecrypt;
use rxchef::Operation;

#[test]
fn test_rsa_decrypt_basic() {
    let op = RSADecrypt;
    let input = b"hello".to_vec();
    let args = [
        ArgValue::Str("invalid key".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("RSA-OAEP".to_string()),
        ArgValue::Str("SHA-256".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
