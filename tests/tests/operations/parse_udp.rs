// Tests for the parse_udp operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_udp::

use rxchef::operation::ArgValue;
use rxchef::operations::parse_udp::ParseUDP;
use rxchef::Operation;

#[test]
fn test_parse_udp_hex() {
    let op = ParseUDP;
    let input = b"04d20050000c1234deadbeef".to_vec(); // src: 1234, dst: 80, len: 12, checksum: 0x1234, data: deadbeef
    let args = [ArgValue::Str("Hex".to_string())];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    let json: serde_json::Value = serde_json::from_str(&result_str).unwrap();
    assert_eq!(json["Source port"], 1234);
    assert_eq!(json["Destination port"], 80);
    assert_eq!(json["Length"], 12);
    assert_eq!(json["Checksum"], "0x1234");
    assert_eq!(json["Data"], "0xdeadbeef");
}
