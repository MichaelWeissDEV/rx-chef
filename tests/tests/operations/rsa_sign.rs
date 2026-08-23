// Tests for the rsa_sign operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rsa_sign::

use rxchef::operation::ArgValue;
use rxchef::operations::rsa_sign::RSASign;
use rxchef::Operation;

// NOTE: The fixture is intentionally invalid and exercises key rejection.
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

#[test]
fn test_rsa_sign_matches_openssl_pkcs1_v15_vector() {
    let private_key = include_str!("../../fixtures/known_answer/rsa_private_1024.pem");
    let args = [
        ArgValue::Str(private_key.to_string()),
        ArgValue::Str(String::new()),
        ArgValue::Str("SHA-256".to_string()),
    ];
    let expected = hex::decode("6b12c60f17510170b4ddc82c6c6cda55cc07c0fee1096a9b53f418a21b7c17374bd83ffa8c539e8939862f2ed1c278626d35a097732e06b78b0a9b43c103d9295eb3cada989cb734507849c73666c8d0a3e17e13a0af584c31976765bbe03ec660bc2577ba60060ccf8caf55fa9dc0b8a5531e24dbcd551e5c1e5bf7b59c6412").unwrap();

    assert_eq!(
        RSASign
            .run(b"RSA known-answer message\n".to_vec(), &args)
            .unwrap(),
        expected
    );
}
