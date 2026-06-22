// Tests for the json_to_yaml operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations json_to_yaml::

use rxchef::operations::json_to_yaml::JSONToYAML;
use rxchef::Operation;

#[test]
fn test_json_to_yaml_basic() {
    let op = JSONToYAML;
    let input = b"{\"foo\": \"bar\", \"baz\": 1}".to_vec();
    let result = op.run(input, &[]).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert!(s.contains("foo: bar"));
    assert!(s.contains("baz: 1"));
}
#[test]
fn test_json_to_yaml_empty() {
    let op = JSONToYAML;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
