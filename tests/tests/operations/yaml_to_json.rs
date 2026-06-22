// Tests for the yaml_to_json operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations yaml_to_json::

use rxchef::operations::yaml_to_json::YAMLToJSON;
use rxchef::Operation;

#[test]
fn test_yaml_to_json_basic() {
    let op = YAMLToJSON;
    let input = b"foo: bar\nbaz: 1".to_vec();
    let result = op.run(input, &[]).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert!(s.contains("\"foo\": \"bar\""));
    assert!(s.contains("\"baz\": 1"));
}
#[test]
fn test_yaml_to_json_empty() {
    let op = YAMLToJSON;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
