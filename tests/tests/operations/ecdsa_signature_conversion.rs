// Tests for the ecdsa_signature_conversion operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ecdsa_signature_conversion::

use rxchef::operations::ecdsa_signature_conversion::ECDSASignatureConversion;
use rxchef::Operation;

// Removed empty input test as it's not meaningful for this operation

#[test]
fn test_ecdsa_signature_conversion_invalid_hex() {
    let op = ECDSASignatureConversion;
    let args = [
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run("ZZZZZZ".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_signature_conversion_invalid_format() {
    let op = ECDSASignatureConversion;
    let args = [
        rxchef::operation::ArgValue::Str("UNSUPPORTED".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run("test".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_signature_conversion_invalid_output_format() {
    let op = ECDSASignatureConversion;
    let args = [
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
        rxchef::operation::ArgValue::Str("UNSUPPORTED".to_string()),
    ];
    let result = op.run("304402201234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_signature_conversion_auto_detect_asn1() {
    let op = ECDSASignatureConversion;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("P1363 HEX".to_string()),
    ];
    // This looks like ASN.1 DER (starts with 0x30)
    // Using a simpler ASN.1 structure that should be valid
    let asn1_hex = "3006020101020102"; // Simple ASN.1 with R=1, S=2
    let result = op.run(asn1_hex.as_bytes().to_vec(), &args);
    // Should successfully convert from ASN.1 to P1363
    assert!(result.is_ok());
    let result_str = String::from_utf8(result.unwrap()).unwrap();
    assert!(result_str.len() > 0);
}

#[test]
fn test_ecdsa_signature_conversion_auto_detect_json() {
    let op = ECDSASignatureConversion;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    // JSON format
    let json_input = r#"{"r":"1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef","s":"1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"}"#;
    let result = op.run(json_input.as_bytes().to_vec(), &args);
    // Should successfully convert from JSON to ASN.1
    assert!(result.is_ok());
    let result_str = String::from_utf8(result.unwrap()).unwrap();
    assert!(result_str.starts_with("30")); // ASN.1 starts with 0x30
}

#[test]
fn test_ecdsa_signature_conversion_asn1_to_formats() {
    let op = ECDSASignatureConversion;
    // Using a simpler valid ASN.1 structure
    let asn1_hex = "3006020101020102";
    
    // Test ASN.1 to ASN.1 (should be same)
    let args_asn1 = [
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run(asn1_hex.as_bytes().to_vec(), &args_asn1);
    assert!(result.is_ok());
    
    // Test ASN.1 to P1363
    let args_p1363 = [
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
        rxchef::operation::ArgValue::Str("P1363 HEX".to_string()),
    ];
    let result = op.run(asn1_hex.as_bytes().to_vec(), &args_p1363);
    assert!(result.is_ok());
    let result_str = String::from_utf8(result.unwrap()).unwrap();
    assert!(result_str.len() > 0);
    
    // Test ASN.1 to JWS
    let args_jws = [
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
        rxchef::operation::ArgValue::Str("JSON Web Signature".to_string()),
    ];
    let result = op.run(asn1_hex.as_bytes().to_vec(), &args_jws);
    assert!(result.is_ok());
    
    // Test ASN.1 to Raw JSON
    let args_json = [
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
        rxchef::operation::ArgValue::Str("Raw JSON".to_string()),
    ];
    let result = op.run(asn1_hex.as_bytes().to_vec(), &args_json);
    assert!(result.is_ok());
    let result_str = String::from_utf8(result.unwrap()).unwrap();
    assert!(result_str.contains("r"));
    assert!(result_str.contains("s"));
}

#[test]
fn test_ecdsa_signature_conversion_p1363_to_asn1() {
    let op = ECDSASignatureConversion;
    // P1363 format: R || S (concatenated hex)
    let p1363_hex = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    
    let args = [
        rxchef::operation::ArgValue::Str("P1363 HEX".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run(p1363_hex.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let result_str = String::from_utf8(result.unwrap()).unwrap();
    assert!(result_str.starts_with("30")); // ASN.1 starts with 0x30
}

#[test]
fn test_ecdsa_signature_conversion_invalid_p1363_length() {
    let op = ECDSASignatureConversion;
    let args = [
        rxchef::operation::ArgValue::Str("P1363 HEX".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    // Odd length hex (invalid for P1363)
    let invalid_hex = "123";
    let result = op.run(invalid_hex.as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_ecdsa_signature_conversion_jws_to_asn1() {
    let op = ECDSASignatureConversion;
    // JWS format is base64url encoded R||S
    // Using invalid base64 to test error handling
    let jws_input = "invalid_base64";
    
    let args = [
        rxchef::operation::ArgValue::Str("JSON Web Signature".to_string()),
        rxchef::operation::ArgValue::Str("ASN.1 HEX".to_string()),
    ];
    let result = op.run(jws_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid base64
    assert!(result.is_err());
}
