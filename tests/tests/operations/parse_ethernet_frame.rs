// Tests for the parse_ethernet_frame operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_ethernet_frame::

use rxchef::operation::ArgValue;
use rxchef::operations::parse_ethernet_frame::ParseEthernetFrame;
use rxchef::Operation;

#[test]
fn test_parse_ethernet_basic() {
    // dst=ff:ff:ff:ff:ff:ff src=aa:bb:cc:dd:ee:ff type=0800 (IPv4) payload=01020304
    let hex = "ffffffffffff aabbccddeeff 0800 01020304";
    let op = ParseEthernetFrame;
    let result = op
        .run(
            hex.as_bytes().to_vec(),
            &[
                ArgValue::Str("Hex".to_string()),
                ArgValue::Str("Text output".to_string()),
            ],
        )
        .unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("ff:ff:ff:ff:ff:ff"));
    assert!(out.contains("aa:bb:cc:dd:ee:ff"));
}
#[test]
fn test_parse_ethernet_packet_data_hex() {
    let hex = "ffffffffffff aabbccddeeff 0800 deadbeef";
    let op = ParseEthernetFrame;
    let result = op
        .run(
            hex.as_bytes().to_vec(),
            &[
                ArgValue::Str("Hex".to_string()),
                ArgValue::Str("Packet data (hex)".to_string()),
            ],
        )
        .unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "deadbeef");
}
#[test]
fn test_parse_ethernet_too_short() {
    let op = ParseEthernetFrame;
    assert!(op
        .run(b"0102".to_vec(), &[ArgValue::Str("Raw".to_string())])
        .is_err());
}
