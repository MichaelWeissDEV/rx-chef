// Tests for the strip_tcp_header operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations strip_tcp_header::

use rxchef::operations::strip_tcp_header::StripTCPHeader;
use rxchef::Operation;

#[test]
fn test_strip_tcp_header_minimum() {
    let op = StripTCPHeader;
    // Build a 20-byte TCP header with data offset = 5 (upper nibble of byte 12 = 0x50)
    let mut input = vec![0u8; 20];
    input[12] = 0x50; // data offset = 5 (5 * 4 = 20 bytes)
    input.extend_from_slice(&[0xca, 0xfe]); // payload
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, vec![0xca, 0xfe]);
}
#[test]
fn test_strip_tcp_header_too_short() {
    let op = StripTCPHeader;
    let input = vec![0u8; 10];
    assert!(op.run(input, &[]).is_err());
}
#[test]
fn test_strip_tcp_header_with_options() {
    let op = StripTCPHeader;
    // Data offset = 6 (24-byte header)
    let mut input = vec![0u8; 24];
    input[12] = 0x60; // data offset = 6 (6 * 4 = 24 bytes)
    input.push(0xbe); // payload
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, vec![0xbe]);
}
