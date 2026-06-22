// Tests for the rison_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rison_encode::

use rxchef::operation::ArgValue;
use rxchef::operations::rison_encode::RisonEncode;
use rxchef::Operation;

#[test]
fn test_rison_encode_basic() {
    let op = RisonEncode;
    let input = serde_json::json!({"a": 1, "b": true, "c": null, "d": [1, 2, 3]});
    let input_bytes = serde_json::to_vec(&input).unwrap();
    let args = [ArgValue::Str("Encode".to_string())];
    let result = op.run(input_bytes, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Note: Map order might be different in Rust's BTreeMap vs JSON.
    // But (a:1,b:!t,c:!n,d:!(1,2,3)) is what we expect.
    assert!(result_str.contains("a:1"));
    assert!(result_str.contains("b:!t"));
    assert!(result_str.contains("c:!n"));
    assert!(result_str.contains("d:!(1,2,3)"));
}
#[test]
fn test_rison_encode_quoted() {
    let op = RisonEncode;
    let input = serde_json::json!("hello! world'");
    let input_bytes = serde_json::to_vec(&input).unwrap();
    let args = [ArgValue::Str("Encode".to_string())];
    let result = op.run(input_bytes, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "'hello!! world!''");
}
#[test]
fn test_rison_encode_array() {
    let op = RisonEncode;
    let input = serde_json::json!([1, 2, 3]);
    let input_bytes = serde_json::to_vec(&input).unwrap();
    let args = [ArgValue::Str("Encode Array".to_string())];
    let result = op.run(input_bytes, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "!(1,2,3)");
}
