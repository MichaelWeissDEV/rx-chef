// Tests for the extract_ip_addresses operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_ip_addresses::

use rxchef::operation::ArgValue;
use rxchef::operations::extract_ip_addresses::ExtractIPAddresses;
use rxchef::Operation;

#[test]
fn test_extract_ip_addresses_ipv4() {
    let op = ExtractIPAddresses;
    let input = b"IPs: 192.168.1.1 and 8.8.8.8".to_vec();
    let args = &[
        ArgValue::Bool(true),  // IPv4
        ArgValue::Bool(false), // IPv6
        ArgValue::Bool(false), // Remove local
        ArgValue::Bool(false), // Display total
        ArgValue::Bool(true),  // Sort
        ArgValue::Bool(true),  // Unique
    ];
    let result = op.run(input, args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "8.8.8.8\n192.168.1.1");
}
#[test]
fn test_extract_ip_addresses_ipv6() {
    let op = ExtractIPAddresses;
    let input = b"IPv6: 2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_vec();
    let args = &[
        ArgValue::Bool(false), // IPv4
        ArgValue::Bool(true),  // IPv6
        ArgValue::Bool(false), // Remove local
        ArgValue::Bool(false), // Display total
        ArgValue::Bool(false), // Sort
        ArgValue::Bool(false), // Unique
    ];
    let result = op.run(input, args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(
        output.to_lowercase(),
        "2001:0db8:85a3:0000:0000:8a2e:0370:7334"
    );
}
