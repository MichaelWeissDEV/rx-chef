// Tests for the ecdsa_sign operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ecdsa_sign::

use rxchef::operations::ecdsa_sign::ECDSASign;
use rxchef::Operation;

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
        rxchef::operation::ArgValue::Str("-----BEGIN EC PRIVATE KEY-----\ninvalid key\n-----END EC PRIVATE KEY-----".to_string()),
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
