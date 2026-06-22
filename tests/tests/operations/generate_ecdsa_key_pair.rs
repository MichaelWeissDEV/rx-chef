// Tests for the generate_ecdsa_key_pair operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_ecdsa_key_pair::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_ecdsa_key_pair::GenerateECDSAKeyPairOp;
use rxchef::Operation;

#[test]
fn test_generate_ecdsa_key_pair_placeholder() {
    let op = GenerateECDSAKeyPairOp;
    let args = [
        ArgValue::Str("P-256".to_string()),
        ArgValue::Str("PEM".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("BEGIN PUBLIC KEY"));
    assert!(result_str.contains("BEGIN PRIVATE KEY"));
}
#[test]
fn test_generate_ecdsa_key_pair_p384() {
    let op = GenerateECDSAKeyPairOp;
    let args = [
        ArgValue::Str("P-384".to_string()),
        ArgValue::Str("DER".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("P-384"));
    assert!(result_str.contains("DER"));
}
