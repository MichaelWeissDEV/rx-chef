// Tests for the generate_pgp_key_pair operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_pgp_key_pair::

#[cfg(not(feature = "pgp"))]
use rxchef::{operations::generate_pgp_key_pair::GeneratePGPKeyPair, Operation};
#[cfg(feature = "pgp")]
use rxchef::{operation::ArgValue, operations::{generate_pgp_key_pair::GeneratePGPKeyPair, pgp_decrypt::PGPDecrypt, pgp_encrypt::PGPEncrypt}, Operation};

#[test]
#[cfg(not(feature = "pgp"))]
fn test_generate_pgp_key_pair_always_errors() {
    let op = GeneratePGPKeyPair;
    let args = [
        rxchef::operation::ArgValue::Str("RSA-2048".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
    ];
    let input = b"";
    let result = op.run(input.to_vec(), &args);
    // Should always return an error since sequoia-openpgp is not integrated
    assert!(result.is_err());
    let error = result.unwrap_err();
    if let rxchef::operation::OperationError::ProcessingError(msg) = error {
        assert!(msg.contains("--features pgp"));
    } else {
        panic!("Expected ProcessingError");
    }
}

#[test]
#[cfg(not(feature = "pgp"))]
fn test_generate_pgp_key_pair_different_args() {
    let op = GeneratePGPKeyPair;
    let args = [
        rxchef::operation::ArgValue::Str("ECC-256".to_string()),
        rxchef::operation::ArgValue::Str("password123".to_string()),
        rxchef::operation::ArgValue::Str("Test User".to_string()),
        rxchef::operation::ArgValue::Str("test@example.com".to_string()),
    ];
    let input = b"some input";
    let result = op.run(input.to_vec(), &args);
    // Should still return an error regardless of arguments
    assert!(result.is_err());
}

#[test]
#[cfg(not(feature = "pgp"))]
fn test_generate_pgp_key_pair_empty_input() {
    let op = GeneratePGPKeyPair;
    let args = [
        rxchef::operation::ArgValue::Str("RSA-4096".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
    ];
    let result = op.run(vec![], &args);
    // Should still return an error even with empty input
    assert!(result.is_err());
}

#[test]
#[cfg(feature = "pgp")]
fn test_generate_pgp_key_pair_exact_interoperability_payload() {
    let keys = GeneratePGPKeyPair.run(Vec::new(), &[
        ArgValue::Str("ECC-256".into()), ArgValue::Str(String::new()),
        ArgValue::Str("Ada Lovelace".into()), ArgValue::Str("ada@example.test".into()),
    ]).unwrap();
    let keys: serde_json::Value = serde_json::from_slice(&keys).unwrap();
    let encrypted = PGPEncrypt.run(
        b"generated PGP key proof".to_vec(),
        &[ArgValue::Str(keys["publicKey"].as_str().unwrap().into())],
    ).unwrap();
    let decrypted = PGPDecrypt.run(encrypted, &[
        ArgValue::Str(keys["privateKey"].as_str().unwrap().into()),
        ArgValue::Str(String::new()),
    ]).unwrap();
    assert_eq!(decrypted, b"generated PGP key proof");
}
