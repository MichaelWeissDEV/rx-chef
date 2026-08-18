// Tests for the extract_mac_addresses operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_mac_addresses::

use rxchef::operation::ArgValue;
use rxchef::operations::extract_mac_addresses::ExtractMACAddresses;
use rxchef::Operation;

#[test]
fn test_extract_mac_addresses() {
    let op = ExtractMACAddresses;
    let input = b"My MAC is AA:BB:CC:DD:EE:FF and another is 11-22-33-44-55-66".to_vec();
    let args = [
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "AA:BB:CC:DD:EE:FF\n11-22-33-44-55-66");
}
#[test]
fn test_extract_mac_addresses_unique_sort() {
    let op = ExtractMACAddresses;
    let input = b"AA:BB:CC:DD:EE:FF AA:BB:CC:DD:EE:FF 11-22-33-44-55-66".to_vec();
    let args = [
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "11-22-33-44-55-66\nAA:BB:CC:DD:EE:FF");
}
