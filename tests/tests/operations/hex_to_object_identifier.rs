// Tests for the hex_to_object_identifier operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations hex_to_object_identifier::

use rxchef::operations::hex_to_object_identifier::HexToObjectIdentifier;
use rxchef::Operation;

#[test]
fn test_hex_to_oid_sha256_with_rsa() {
    let op = HexToObjectIdentifier;
    let result = op.run(b"2a864886f70d01010b".to_vec(), &[]).unwrap();
    let oid = String::from_utf8(result).unwrap();
    assert_eq!(oid, "1.2.840.113549.1.1.11");
}
#[test]
fn test_hex_to_oid_simple() {
    let op = HexToObjectIdentifier;
    let result = op.run(b"2a03".to_vec(), &[]).unwrap();
    let oid = String::from_utf8(result).unwrap();
    assert_eq!(oid, "1.2.3");
}
#[test]
fn test_hex_to_oid_zero() {
    let op = HexToObjectIdentifier;
    let result = op.run(b"00".to_vec(), &[]).unwrap();
    let oid = String::from_utf8(result).unwrap();
    assert_eq!(oid, "0.0");
}
#[test]
fn test_hex_to_oid_invalid() {
    let op = HexToObjectIdentifier;
    assert!(op.run(b"zz".to_vec(), &[]).is_err());
}
#[test]
fn test_hex_to_oid_whitespace() {
    let op = HexToObjectIdentifier;
    let result = op.run(b"2a 03".to_vec(), &[]).unwrap();
    let oid = String::from_utf8(result).unwrap();
    assert_eq!(oid, "1.2.3");
}
