// Tests for the parse_ipv4_header operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_ipv4_header::

use rxchef::operation::ArgValue;
use rxchef::operations::parse_ipv4_header::ParseIPv4Header;
use rxchef::Operation;

/// Compute the one's complement checksum over the given bytes
fn internet_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 1 < data.len() {
        let word = ((data[i] as u32) << 8) | (data[i + 1] as u32);
        sum += word;
        i += 2;
    }
    if i < data.len() {
        sum += (data[i] as u32) << 8;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !sum as u16
}

// Minimal valid IPv4 header with no payload: version=4, IHL=5, protocol=6 (TCP)
// 45 00 00 14 00 00 40 00 40 06 [checksum] c0a80001 c0a80002
fn make_ipv4_header() -> Vec<u8> {
    let mut h = vec![
        0x45u8, 0x00, 0x00, 0x14, // version/IHL, DSCP/ECN, total length
        0x00, 0x01, // identification
        0x40, 0x00, // flags / frag offset
        0x40, 0x06, // TTL=64, protocol=TCP
        0x00, 0x00, // checksum placeholder
        0xc0, 0xa8, 0x00, 0x01, // src 192.168.0.1
        0xc0, 0xa8, 0x00, 0x02, // dst 192.168.0.2
    ];
    // Compute checksum
    let cs = internet_checksum(&h);
    h[10] = (cs >> 8) as u8;
    h[11] = cs as u8;
    h
}
#[test]
fn test_parse_ipv4_header_table() {
    let op = ParseIPv4Header;
    let header = make_ipv4_header();
    let hex_str = hex::encode(&header);
    let result = op
        .run(
            hex_str.into_bytes(),
            &[
                ArgValue::Str("Hex".to_string()),
                ArgValue::Str("Table".to_string()),
            ],
        )
        .unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("192.168.0.1"));
    assert!(out.contains("192.168.0.2"));
    assert!(out.contains("correct"));
}
#[test]
fn test_parse_ipv4_header_too_short() {
    let op = ParseIPv4Header;
    assert!(op.run(b"4500".to_vec(), &[]).is_err());
}
