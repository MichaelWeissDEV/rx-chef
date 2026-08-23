// Tests for the generate_rsa_key_pair operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_rsa_key_pair::

use rxchef::operations::generate_rsa_key_pair::GenerateRSAKeyPair;
use rxchef::operation::ArgValue;
use rxchef::operations::rsa_sign::RSASign;
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

#[test]
fn test_generate_rsa_key_pair_512_bit_boundary() {
    let args = [
        ArgValue::Str("512".to_string()),
        ArgValue::Str("PEM".to_string()),
    ];
    let output = String::from_utf8(GenerateRSAKeyPair.run(Vec::new(), &args).unwrap()).unwrap();
    assert!(output.contains("-----BEGIN RSA PUBLIC KEY-----"));
    assert!(output.contains("-----BEGIN RSA PRIVATE KEY-----"));
}

#[test]
fn test_generated_rsa_key_is_accepted_by_ring_validator() {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use ring::signature::{UnparsedPublicKey, RSA_PKCS1_2048_8192_SHA256};

    let args = [
        ArgValue::Str("2048".to_string()),
        ArgValue::Str("PEM".to_string()),
    ];
    let output = String::from_utf8(GenerateRSAKeyPair.run(Vec::new(), &args).unwrap()).unwrap();
    let public_pem = pem_block(&output, "RSA PUBLIC KEY");
    let private_pem = pem_block(&output, "RSA PRIVATE KEY");
    let message = b"independent RSA key-generation validation";
    let signature = RSASign
        .run(
            message.to_vec(),
            &[
                ArgValue::Str(private_pem),
                ArgValue::Str(String::new()),
                ArgValue::Str("SHA-256".to_string()),
            ],
        )
        .unwrap();
    let der = STANDARD
        .decode(
            public_pem
                .lines()
                .filter(|line| !line.starts_with("-----"))
                .collect::<String>(),
        )
        .unwrap();

    assert_eq!(
        UnparsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, der).verify(message, &signature),
        Ok(())
    );
}

fn pem_block(text: &str, label: &str) -> String {
    let begin = format!("-----BEGIN {label}-----");
    let end = format!("-----END {label}-----");
    let start = text.find(&begin).unwrap();
    let finish = text[start..].find(&end).unwrap() + start + end.len();
    format!("{}\n", &text[start..finish])
}
