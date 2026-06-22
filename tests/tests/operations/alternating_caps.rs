// Tests for the alternating_caps operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations alternating_caps::

use rxchef::operations::alternating_caps::AlternatingCaps;
use rxchef::Operation;

#[test]
fn test_alternating_caps_basic() {
    let op = AlternatingCaps;
    let input = b"Hello, world!".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "hElLo, WoRlD!");
}
#[test]
fn test_alternating_caps_empty() {
    let op = AlternatingCaps;
    let input = b"".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "");
}
#[test]
fn test_alternating_caps_no_letters() {
    let op = AlternatingCaps;
    let input = b"123!@#".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "123!@#");
}
#[test]
fn test_alternating_caps_single_letter() {
    let op = AlternatingCaps;
    let input = b"A".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "a");
}
#[test]
fn test_alternating_caps_two_letters() {
    let op = AlternatingCaps;
    let input = b"AB".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "aB");
}
