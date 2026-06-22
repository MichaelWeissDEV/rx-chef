// Tests for the generate_rsa_key_pair operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_rsa_key_pair::

use rxchef::operations::generate_rsa_key_pair::GenerateRSAKeyPair;
use rxchef::Operation;

#[test]
fn test_generate_rsa_key_pair_pem_format() {
    let op = GenerateRSAKeyPair;
    let args = [
        rxchef::operation::ArgValue::Str("2048".to_string()),
        rxchef::operation::ArgValue::Str("PEM".to_string()),
    ];
    let input = b"";
    let result = op.run(input.to_vec(), &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    let output_str = String::from_utf8_lossy(&output);
    // Should contain PEM formatted keys
    assert!(output_str.contains("-----BEGIN RSA PRIVATE KEY-----"));
    assert!(output_str.contains("-----BEGIN RSA PUBLIC KEY-----"));
}

#[test]
fn test_generate_rsa_key_pair_json_format() {
    let op = GenerateRSAKeyPair;
    let args = [
        rxchef::operation::ArgValue::Str("1024".to_string()),
        rxchef::operation::ArgValue::Str("JSON".to_string()),
    ];
    let input = b"";
    let result = op.run(input.to_vec(), &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    let output_str = String::from_utf8_lossy(&output);
    // Should contain JSON formatted keys
    assert!(output_str.contains("publicKey"));
    assert!(output_str.contains("privateKey"));
    assert!(output_str.contains("n"));
    assert!(output_str.contains("e"));
    assert!(output_str.contains("d"));
}

#[test]
fn test_generate_rsa_key_pair_der_format() {
    let op = GenerateRSAKeyPair;
    let args = [
        rxchef::operation::ArgValue::Str("2048".to_string()),
        rxchef::operation::ArgValue::Str("DER".to_string()),
    ];
    let input = b"";
    let result = op.run(input.to_vec(), &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    // Should be binary DER format
    assert!(!output.is_empty());
}

#[test]
fn test_generate_rsa_key_pair_invalid_key_length() {
    let op = GenerateRSAKeyPair;
    let args = [
        rxchef::operation::ArgValue::Str("invalid".to_string()),
        rxchef::operation::ArgValue::Str("PEM".to_string()),
    ];
    let input = b"";
    let result = op.run(input.to_vec(), &args);
    // Should fail due to invalid key length
    assert!(result.is_err());
}

#[test]
fn test_generate_rsa_key_pair_invalid_format() {
    let op = GenerateRSAKeyPair;
    let args = [
        rxchef::operation::ArgValue::Str("2048".to_string()),
        rxchef::operation::ArgValue::Str("INVALID".to_string()),
    ];
    let input = b"";
    let result = op.run(input.to_vec(), &args);
    // Should fail due to invalid format
    assert!(result.is_err());
}
