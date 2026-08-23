// Tests for the ecdsa_sign operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ecdsa_sign::

use rxchef::operations::ecdsa_sign::ECDSASign;
use rxchef::Operation;

#[test]
fn test_ecdsa_sign_rfc6979_p256_sha256_known_answer() {
    use p256::pkcs8::{EncodePrivateKey, LineEnding};

    // RFC 6979 section A.2.5: P-256, SHA-256, message "sample". Only the
    // transport encoding is generated here; the scalar and expected r/s are
    // fixed normative values.
    let scalar =
        hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721").unwrap();
    let secret = p256::SecretKey::from_slice(&scalar).unwrap();
    let private_key = secret.to_pkcs8_pem(LineEnding::LF).unwrap();
    let output = ECDSASign
        .run(
            b"sample".to_vec(),
            &[
                rxchef::operation::ArgValue::Str(private_key.to_string()),
                rxchef::operation::ArgValue::Str("SHA-256".into()),
                rxchef::operation::ArgValue::Str("ASN.1 HEX".into()),
            ],
        )
        .unwrap();
    assert_eq!(output, b"3046022100efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716022100f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8");
}

#[test]
fn test_ecdsa_sign_empty_input() {
    let op = ECDSASign;
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_sign_no_key() {
    let op = ECDSASign;
    let args = [
        rxchef::operation::ArgValue::Str("-----BEGIN EC PRIVATE KEY-----".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run("test message".as_bytes().to_vec(), &args);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Please enter a private key"));
}

#[test]
fn test_ecdsa_sign_invalid_key() {
    let op = ECDSASign;
    let args = [
        rxchef::operation::ArgValue::Str("invalid key data".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run("test message".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_sign_unsupported_format() {
    let op = ECDSASign;
    let args = [
        rxchef::operation::ArgValue::Str("-----BEGIN EC PRIVATE KEY-----".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("UNSUPPORTED FORMAT".to_string()),
    ];
    let result = op.run("test message".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_sign_simple_message() {
    // This test would need a valid PEM key to work, but we can test the error handling
    let op = ECDSASign;
    let args = [
        rxchef::operation::ArgValue::Str(
            "-----BEGIN EC PRIVATE KEY-----\ninvalid key\n-----END EC PRIVATE KEY-----".to_string(),
        ),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run("hello world".as_bytes().to_vec(), &args);
    // Should fail due to invalid key format
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_sign_different_output_formats() {
    let op = ECDSASign;

    // Test ASN.1 HEX format
    let args_asn1 = [
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run("test".as_bytes().to_vec(), &args_asn1);
    assert!(result.is_err());

    // Test P1363 HEX format
    let args_p1363 = [
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("P1363 HEX".to_string()),
    ];
    let result = op.run("test".as_bytes().to_vec(), &args_p1363);
    assert!(result.is_err());

    // Test JSON Web Signature format
    let args_jws = [
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("JSON Web Signature".to_string()),
    ];
    let result = op.run("test".as_bytes().to_vec(), &args_jws);
    assert!(result.is_err());

    // Test Raw JSON format
    let args_json = [
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("Raw JSON".to_string()),
    ];
    let result = op.run("test".as_bytes().to_vec(), &args_json);
    assert!(result.is_err());
}
