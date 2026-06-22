// Tests for the hex_to_pem operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations hex_to_pem::

use rxchef::operation::ArgValue;
use rxchef::operations::hex_to_pem::HexToPEM;
use rxchef::Operation;

#[test]
fn test_hex_to_pem_basic() {
    let op = HexToPEM;
    let input = b"deadbeef".to_vec();
    let result = op
        .run(input, &[ArgValue::Str("CERTIFICATE".to_string())])
        .expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    assert!(s.starts_with("-----BEGIN CERTIFICATE-----\n"));
    assert!(s.ends_with("-----END CERTIFICATE-----\n"));
    assert!(s.contains("3q2+7w=="));
}
#[test]
fn test_hex_to_pem_invalid_hex() {
    let op = HexToPEM;
    let result = op.run(b"xyz".to_vec(), &[]);
    assert!(result.is_err());
}
#[test]
fn test_hex_to_pem_custom_header() {
    let op = HexToPEM;
    let input = b"0102".to_vec();
    let result = op
        .run(input, &[ArgValue::Str("RSA PRIVATE KEY".to_string())])
        .expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    assert!(s.contains("-----BEGIN RSA PRIVATE KEY-----"));
    assert!(s.contains("-----END RSA PRIVATE KEY-----"));
}
