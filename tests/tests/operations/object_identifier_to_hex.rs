// Tests for the object_identifier_to_hex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations object_identifier_to_hex::

use rxchef::operations::object_identifier_to_hex::ObjectIdentifierToHex;
use rxchef::Operation;

#[test]
fn test_oid_rsassa_pkcs1_v1_5_sha256() {
    // 1.2.840.113549.1.1.11 -> sha256WithRSAEncryption
    let op = ObjectIdentifierToHex;
    let result = op.run(b"1.2.840.113549.1.1.11".to_vec(), &[]).unwrap();
    let hex = String::from_utf8(result).unwrap();
    assert_eq!(hex, "2a864886f70d01010b");
}
#[test]
fn test_oid_simple() {
    // 1.2.3 -> 2a 03
    let op = ObjectIdentifierToHex;
    let result = op.run(b"1.2.3".to_vec(), &[]).unwrap();
    let hex = String::from_utf8(result).unwrap();
    assert_eq!(hex, "2a03");
}
#[test]
fn test_oid_invalid() {
    let op = ObjectIdentifierToHex;
    assert!(op.run(b"invalid".to_vec(), &[]).is_err());
}
#[test]
fn test_oid_zero() {
    // 0.0 -> 00
    let op = ObjectIdentifierToHex;
    let result = op.run(b"0.0".to_vec(), &[]).unwrap();
    let hex = String::from_utf8(result).unwrap();
    assert_eq!(hex, "00");
}
