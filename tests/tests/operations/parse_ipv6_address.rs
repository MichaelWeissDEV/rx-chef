// Tests for the parse_ipv6_address operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_ipv6_address::

use rxchef::operations::parse_ipv6_address::ParseIPv6Address;
use rxchef::Operation;

#[test]
fn test_loopback() {
    let op = ParseIPv6Address;
    let result = op.run(b"::1".to_vec(), &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("Loopback"));
}
#[test]
fn test_unspecified() {
    let op = ParseIPv6Address;
    let result = op.run(b"::".to_vec(), &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("Unspecified"));
}
#[test]
fn test_link_local() {
    let op = ParseIPv6Address;
    let result = op.run(b"fe80::1".to_vec(), &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("Link-local"));
}
#[test]
fn test_documentation() {
    let op = ParseIPv6Address;
    let result = op.run(b"2001:db8::1".to_vec(), &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("Documentation"));
}
#[test]
fn test_invalid() {
    let op = ParseIPv6Address;
    assert!(op.run(b"not-an-ipv6".to_vec(), &[]).is_err());
}
#[test]
fn test_longhand_present() {
    let op = ParseIPv6Address;
    let result = op.run(b"2001:db8::1".to_vec(), &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("Longhand:"));
    assert!(out.contains("Shorthand:"));
}
