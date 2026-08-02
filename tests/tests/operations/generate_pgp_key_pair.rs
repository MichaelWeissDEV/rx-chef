// Tests for the generate_pgp_key_pair operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_pgp_key_pair::

use rxchef::operations::generate_pgp_key_pair::GeneratePGPKeyPair;
use rxchef::Operation;

#[test]
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
        assert!(msg.contains("sequoia-openpgp"));
    } else {
        panic!("Expected ProcessingError");
    }
}

#[test]
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
