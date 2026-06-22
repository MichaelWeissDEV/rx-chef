// Tests for the parse_tlv operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_tlv::

use rxchef::operation::ArgValue;
use rxchef::operations::parse_tlv::ParseTLV;
use rxchef::Operation;
use serde_json::json;

#[test]
fn test_parse_tlv_simple() {
    let op = ParseTLV;
    let input = vec![0x01, 0x02, 0xAA, 0xBB]; // key: 01, len: 02, value: AA BB
    let args = [
        ArgValue::Num(1.0),
        ArgValue::Num(1.0),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    let json: serde_json::Value = serde_json::from_str(&result_str).unwrap();
    assert_eq!(json[0]["key"], json!([1]));
    assert_eq!(json[0]["length"], 2);
    assert_eq!(json[0]["value"], json!([170, 187]));
}
