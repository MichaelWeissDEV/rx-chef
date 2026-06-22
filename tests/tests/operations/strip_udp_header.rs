// Tests for the strip_udp_header operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations strip_udp_header::

use rxchef::operations::strip_udp_header::StripUDPHeader;
use rxchef::Operation;

#[test]
fn test_strip_udp_header_normal() {
    let op = StripUDPHeader;
    // 8 bytes header + payload
    let mut input = vec![0u8; 8];
    input.extend_from_slice(b"payload");
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, b"payload");
}
#[test]
fn test_strip_udp_header_empty_payload() {
    let op = StripUDPHeader;
    let input = vec![0u8; 8];
    let result = op.run(input, &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_strip_udp_header_too_short() {
    let op = StripUDPHeader;
    let input = vec![0u8; 7];
    let result = op.run(input, &[]);
    assert!(result.is_err());
}
#[test]
fn test_strip_udp_header_with_data() {
    let op = StripUDPHeader;
    let input = b"\x00\x35\x00\x35\x00\x0d\x00\x00hello".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, b"hello");
}
