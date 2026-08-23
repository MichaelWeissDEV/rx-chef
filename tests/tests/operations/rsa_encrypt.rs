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

const PUBLIC_KEY: &str = include_str!("../../fixtures/known_answer/rsa_public_1024.pem");
const RAW_MESSAGE: &[u8] = b"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcde\n";
const RAW_CIPHERTEXT: &str = "8a34fbb50ae5db341bb1300a82f04131f51d28e2a84fda87e8e88b48e10f323b4f68d8e30e75ec56edbc7c60e12a26dc679316364752d970e63e81de81ee8638bf1b3d60022d2a37714daa83ff8c7fdabcd2f882c1954721a11f7407853e737a8735f625df7a21d17bc57dc6d781310d308aca40b78ec4e2fc13c3cb9bb3a428";

#[test]
fn test_rsa_raw_encrypt_matches_openssl_known_answer() {
    let args = [
        ArgValue::Str(PUBLIC_KEY.to_string()),
        ArgValue::Str("RAW".to_string()),
        ArgValue::Str("SHA-256".to_string()),
    ];
    assert_eq!(
        RSAEncrypt.run(RAW_MESSAGE.to_vec(), &args).unwrap(),
        hex::decode(RAW_CIPHERTEXT).unwrap()
    );
}

#[test]
fn test_rsa_raw_encrypt_zero_boundary_is_modulus_sized() {
    let args = [
        ArgValue::Str(PUBLIC_KEY.to_string()),
        ArgValue::Str("RAW".to_string()),
        ArgValue::Str("SHA-256".to_string()),
    ];
    assert_eq!(RSAEncrypt.run(Vec::new(), &args).unwrap(), vec![0; 128]);
}
