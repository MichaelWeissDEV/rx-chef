// Tests for the parse_asn1_hex_string operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_asn1_hex_string::

use rxchef::operations::parse_asn1_hex_string::ParseASN1HexString;
use rxchef::Operation;

#[test]
fn test_parse_asn1_hex_basic() {
    let op = ParseASN1HexString;
    // 30 03 02 01 05 -> SEQUENCE { INTEGER 5 }
    let input = b"3003020105".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("SEQUENCE"));
    assert!(result_str.contains("INTEGER"));
}
