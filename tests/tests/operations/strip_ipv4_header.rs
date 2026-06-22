// Tests for the strip_ipv4_header operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations strip_ipv4_header::

use rxchef::operations::strip_ipv4_header::StripIPv4Header;
use rxchef::Operation;

#[test]
fn test_strip_ipv4_header_minimum() {
    let op = StripIPv4Header;
    // IHL = 5 (20-byte header), payload = [0xde, 0xad]
    let mut input = vec![0x45u8]; // version=4, IHL=5
    input.extend_from_slice(&[0u8; 19]); // rest of header
    input.extend_from_slice(&[0xde, 0xad]); // payload
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, vec![0xde, 0xad]);
}
#[test]
fn test_strip_ipv4_header_too_short() {
    let op = StripIPv4Header;
    let input = vec![0x45u8; 10]; // only 10 bytes, need at least 20
    assert!(op.run(input, &[]).is_err());
}
#[test]
fn test_strip_ipv4_header_with_options() {
    let op = StripIPv4Header;
    // IHL = 6 (24-byte header), payload = [0xff]
    let mut input = vec![0x46u8]; // version=4, IHL=6
    input.extend_from_slice(&[0u8; 23]); // rest of header (23 more bytes = 24 total)
    input.push(0xff); // payload
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, vec![0xff]);
}
