// Tests for the pem_to_hex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations pem_to_hex::

use rxchef::operation::ArgValue;
use rxchef::operations::hex_to_pem::HexToPEM;
use rxchef::operations::pem_to_hex::PEMToHex;
use rxchef::Operation;

#[test]
fn test_pem_to_hex_basic() {
    let op = PEMToHex;
    // "deadbeef" base64-encoded is "3q2+7w=="
    let pem = b"-----BEGIN CERTIFICATE-----\n3q2+7w==\n-----END CERTIFICATE-----\n".to_vec();
    let result = op.run(pem, &[]).expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    assert_eq!(s.trim(), "deadbeef");
}
#[test]
fn test_pem_to_hex_no_pem() {
    let op = PEMToHex;
    let result = op.run(b"not pem data".to_vec(), &[]);
    assert!(result.is_err());
}
#[test]
fn test_pem_to_hex_roundtrip() {
    let hex_op = HexToPEM;
    let pem_op = PEMToHex;
    let hex_input = b"0102030405".to_vec();
    let pem = hex_op
        .run(hex_input.clone(), &[ArgValue::Str("TEST".to_string())])
        .expect("hex to pem");
    let hex_out = pem_op.run(pem, &[]).expect("pem to hex");
    assert_eq!(String::from_utf8(hex_out).unwrap().trim(), "0102030405");
}
