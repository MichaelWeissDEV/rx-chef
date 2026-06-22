// Tests for the defang_ip_addresses operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations defang_ip_addresses::

use rxchef::operations::defang_ip_addresses::DefangIPAddresses;
use rxchef::Operation;

#[test]
fn test_defang_ipv4() {
    let operation = DefangIPAddresses;
    let input = b"192.168.1.1".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "192[.]168[.]1[.]1");
}
#[test]
fn test_defang_ipv4_with_cidr() {
    let operation = DefangIPAddresses;
    let input = b"192.168.1.0/24".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "192[.]168[.]1[.]0/24");
}
#[test]
fn test_defang_ipv6() {
    let operation = DefangIPAddresses;
    let input = b"2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("[:"));
}
#[test]
fn test_defang_mixed() {
    let operation = DefangIPAddresses;
    let input = b"Visit 192.168.1.1 or 2001:db8::1 for more info".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("[.]"));
    assert!(output.contains("[:"));
}
