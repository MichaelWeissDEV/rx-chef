// Tests for the parse_tls_record operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_tls_record::

use rxchef::operations::parse_tls_record::ParseTLSRecord;
use rxchef::Operation;
use serde_json::Value;

#[test]
fn test_parse_tls_record_handshake() {
    let op = ParseTLSRecord;
    // Simplified TLS Record: Type=22(Handshake), Version=0303, Length=0005, HandshakeType=01(ClientHello), HandshakeLen=000001, Data=00
    let input = vec![0x16, 0x03, 0x03, 0x00, 0x05, 0x01, 0x00, 0x00, 0x01, 0x00];
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    let json: Value = serde_json::from_str(&result_str).unwrap();
    assert_eq!(json[0]["type"], "handshake");
    assert_eq!(json[0]["handshakeType"], "client_hello");
}
