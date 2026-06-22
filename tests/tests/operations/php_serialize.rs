// Tests for the php_serialize operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations php_serialize::

use rxchef::operations::php_serialize::PHPSerialize;
use rxchef::Operation;

#[test]
fn test_php_serialize_basic() {
    let op = PHPSerialize;
    let input = b"[5,\"abc\",true]".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "a:3:{i:0;i:5;i:1;s:3:\"abc\";i:2;b:1;}"
    );
}
#[test]
fn test_php_serialize_object() {
    let op = PHPSerialize;
    let input = b"{\"a\": 10, \"b\": \"test\"}".to_vec();
    let result = op.run(input, &[]).unwrap();
    // JSON objects are unordered in Rust's Value::Object (BTreeMap by default if using preserve_order feature, but usually just Map)
    // Actually serde_json::Map is a BTreeMap by default or can be enabled to be indexmap.
    // Let's check both possibilities or just one if we know it.
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("s:1:\"a\";i:10;"));
    assert!(result_str.contains("s:1:\"b\";s:4:\"test\";"));
    assert!(result_str.starts_with("a:2:{"));
}
#[test]
fn test_php_serialize_null() {
    let op = PHPSerialize;
    let input = b"null".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "N;");
}
