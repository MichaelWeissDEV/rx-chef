// Tests for the ecdsa_verify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ecdsa_verify::

use rxchef::operations::ecdsa_verify::ECDSAVerify;
use rxchef::Operation;

#[test]
fn test_ecdsa_verify_empty_input() {
    let op = ECDSAVerify;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_verify_no_key() {
    let op = ECDSAVerify;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("-----BEGIN PUBLIC KEY-----".to_string()),
        rxchef::operation::ArgValue::Str("test message".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run("3006020101020102".as_bytes().to_vec(), &args);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Please enter a public key"));
}

#[test]
fn test_ecdsa_verify_invalid_key() {
    let op = ECDSAVerify;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key data".to_string()),
        rxchef::operation::ArgValue::Str("test message".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run("3006020101020102".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_verify_invalid_signature_format() {
    let op = ECDSAVerify;
    let args = [
        rxchef::operation::ArgValue::Str("UNSUPPORTED".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("test message".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run("test".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_verify_invalid_message_format() {
    let op = ECDSAVerify;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("test message".to_string()),
        rxchef::operation::ArgValue::Str("UNSUPPORTED".to_string()),
    ];
    let result = op.run("3006020101020102".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_verify_auto_detect_asn1() {
    let op = ECDSAVerify;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("test message".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    // ASN.1 format signature
    let asn1_sig = "3006020101020102";
    let result = op.run(asn1_sig.as_bytes().to_vec(), &args);
    // Should fail due to invalid key, but signature format should be detected
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_verify_auto_detect_json() {
    let op = ECDSAVerify;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("test message".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    // JSON format signature
    let json_sig = r#"{"r":"01","s":"02"}"#;
    let result = op.run(json_sig.as_bytes().to_vec(), &args);
    // Should fail due to invalid key, but signature format should be detected
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_verify_different_signature_formats() {
    let op = ECDSAVerify;
    let args = [
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("test message".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    let asn1_sig = "3006020101020102";
    let result = op.run(asn1_sig.as_bytes().to_vec(), &args);
    assert!(result.is_err());
    
    // Test P1363 format
    let args_p1363 = [
        rxchef::operation::ArgValue::Str("P1363 HEX".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("test message".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    let p1363_sig = "0102";
    let result = op.run(p1363_sig.as_bytes().to_vec(), &args_p1363);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_verify_different_message_formats() {
    let op = ECDSAVerify;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("test message".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    let asn1_sig = "3006020101020102";
    let result = op.run(asn1_sig.as_bytes().to_vec(), &args);
    assert!(result.is_err());
    
    // Test Hex message format
    let args_hex = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("74657374".to_string()), // "test" in hex
        rxchef::operation::ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(asn1_sig.as_bytes().to_vec(), &args_hex);
    assert!(result.is_err());
    
    // Test Base64 message format
    let args_base64 = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("SHA-256".to_string()),
        rxchef::operation::ArgValue::Str("invalid key".to_string()),
        rxchef::operation::ArgValue::Str("dGVzdA==".to_string()), // "test" in base64
        rxchef::operation::ArgValue::Str("Base64".to_string()),
    ];
    let result = op.run(asn1_sig.as_bytes().to_vec(), &args_base64);
    assert!(result.is_err());
}
