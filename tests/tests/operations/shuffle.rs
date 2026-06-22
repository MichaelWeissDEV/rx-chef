// Tests for the shuffle operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations shuffle::

use rxchef::operation::ArgValue;
use rxchef::operations::shuffle::Shuffle;
use rxchef::Operation;

#[test]
fn test_shuffle_preserves_elements() {
    let op = Shuffle;
    let input = b"a\nb\nc\nd".to_vec();
    let args = vec![ArgValue::Str("Line feed".to_string())];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    let mut items: Vec<&str> = out.split('\n').collect();
    items.sort();
    assert_eq!(items, vec!["a", "b", "c", "d"]);
}
#[test]
fn test_shuffle_empty() {
    let op = Shuffle;
    let input = b"".to_vec();
    let args = vec![ArgValue::Str("Line feed".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, b"".to_vec());
}
#[test]
fn test_shuffle_single_element() {
    let op = Shuffle;
    let input = b"only".to_vec();
    let args = vec![ArgValue::Str("Line feed".to_string())];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "only");
}
