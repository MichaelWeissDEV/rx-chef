// Tests for the parse_tcp operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_tcp::

use rxchef::operations::parse_tcp::ParseTcp;
use rxchef::Operation;
use serde_json::Value;

#[test]
fn test_parse_tcp_basic() {
    let op = ParseTcp;
    let mut data = vec![0; 20];
    data[0..2].copy_from_slice(&80u16.to_be_bytes()); // src port
    data[2..4].copy_from_slice(&443u16.to_be_bytes()); // dst port
    data[12] = 0x50; // offset = 5 (20 bytes)
    let hex_str = hex::encode(&data);
    let result = op.run(hex_str.into_bytes(), &[]).unwrap();
    let val: Value = serde_json::from_slice(&result).unwrap();
    assert_eq!(val["Source port"], 80);
    assert_eq!(val["Destination port"], 443);
    assert!(val["Data offset"].as_str().unwrap().contains("5"));
}
#[test]
fn test_parse_tcp_with_data() {
    let op = ParseTcp;
    let mut data = vec![0; 20];
    data[12] = 0x50;
    data.extend_from_slice(b"hello");
    let hex_str = hex::encode(&data);
    let result = op.run(hex_str.into_bytes(), &[]).unwrap();
    let val: Value = serde_json::from_slice(&result).unwrap();
    assert_eq!(val["Data"], "0x68656c6c6f");
}
#[test]
fn test_parse_tcp_with_options() {
    let op = ParseTcp;
    let mut data = vec![0; 20];
    data[12] = 0x60; // offset = 6 (24 bytes)
    data.extend_from_slice(&[2, 4, 0x05, 0xb4]); // MSS = 1460
    let hex_str = hex::encode(&data);
    let result = op.run(hex_str.into_bytes(), &[]).unwrap();
    let val: Value = serde_json::from_slice(&result).unwrap();
    assert_eq!(val["Options"]["Maximum Segment Size"]["Value"], 1460);
}
