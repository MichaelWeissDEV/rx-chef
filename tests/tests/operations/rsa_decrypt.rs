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

#[test]
fn test_rsa_raw_decrypt_matches_openssl_known_answer() {
    let private_key = include_str!("../../fixtures/known_answer/rsa_private_1024.pem");
    let ciphertext = hex::decode("8a34fbb50ae5db341bb1300a82f04131f51d28e2a84fda87e8e88b48e10f323b4f68d8e30e75ec56edbc7c60e12a26dc679316364752d970e63e81de81ee8638bf1b3d60022d2a37714daa83ff8c7fdabcd2f882c1954721a11f7407853e737a8735f625df7a21d17bc57dc6d781310d308aca40b78ec4e2fc13c3cb9bb3a428").unwrap();
    let args = [
        ArgValue::Str(private_key.to_string()),
        ArgValue::Str(String::new()),
        ArgValue::Str("RAW".to_string()),
        ArgValue::Str("SHA-256".to_string()),
    ];
    assert_eq!(
        RSADecrypt.run(ciphertext, &args).unwrap(),
        b"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcde\n"
    );
}
