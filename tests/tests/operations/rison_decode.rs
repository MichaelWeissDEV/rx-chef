// Tests for the rison_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rison_decode::

use rxchef::operation::ArgValue;
use rxchef::operations::rison_decode::RisonDecode;
use rxchef::Operation;
use serde_json::Value;

#[test]
fn test_rison_decode_basic() {
    let op = RisonDecode;
    let input = b"(a:1,b:!t,c:!n,d:!(1,2.0,3))".to_vec();
    let args = [ArgValue::Str("Decode".to_string())];
    let result = op.run(input, &args).unwrap();
    let val: Value = serde_json::from_slice(&result).unwrap();
    assert_eq!(val["a"], serde_json::json!(1.0));
    assert_eq!(val["b"], true);
    assert!(val["c"].is_null());
    assert_eq!(val["d"][1], serde_json::json!(2.0));
}
#[test]
fn test_rison_decode_quoted_string() {
    let op = RisonDecode;
    let input = b"'hello!! world!''".to_vec();
    let args = [ArgValue::Str("Decode".to_string())];
    let result = op.run(input, &args).unwrap();
    let val: Value = serde_json::from_slice(&result).unwrap();
    assert_eq!(val, "hello! world'");
}
#[test]
fn test_rison_decode_array() {
    let op = RisonDecode;
    let input = b"!(1,2.0,3)".to_vec();
    let args = [ArgValue::Str("Decode Array".to_string())];
    let result = op.run(input, &args).unwrap();
    let val: Value = serde_json::from_slice(&result).unwrap();
    assert_eq!(val, serde_json::json!([1.0, 2.0, 3.0]));
}
