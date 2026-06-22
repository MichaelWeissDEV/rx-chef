// Tests for the tcpip_checksum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations tcpip_checksum::

use rxchef::operations::tcpip_checksum::TCPIPChecksum;
use rxchef::Operation;

#[test]
fn test_tcpip_checksum_basic() {
    let op = TCPIPChecksum;
    let input = vec![
        0x45, 0x00, 0x00, 0x3c, 0x1c, 0x46, 0x40, 0x00, 0x40, 0x06, 0x00, 0x00, 0xac, 0x10, 0x0a,
        0x63, 0xac, 0x10, 0x0a, 0x0c,
    ];
    let result = op.run(input, &[]).unwrap();
    // CyberChef: Calculates the checksum for a TCP/IP header.
    // For the above IPv4 header (with checksum set to 0), the checksum should be 0xafb4.
    assert_eq!(String::from_utf8_lossy(&result), "b1e6");
}
#[test]
fn test_tcpip_checksum_empty() {
    let op = TCPIPChecksum;
    let input = vec![];
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "ffff");
}
#[test]
fn test_tcpip_checksum_single_byte() {
    let op = TCPIPChecksum;
    let input = vec![0x01];
    let result = op.run(input, &[]).unwrap();
    // csum = 0x0100. 0xffff - 0x0100 = 0xfeff
    assert_eq!(String::from_utf8_lossy(&result), "feff");
}
#[test]
fn test_tcpip_checksum_all_zeros() {
    let op = TCPIPChecksum;
    let input = vec![0, 0, 0, 0];
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "ffff");
}
