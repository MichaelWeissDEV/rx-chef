// Tests for the to_lower_case operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_lower_case::

use rxchef::operations::to_lower_case::ToLowerCase;
use rxchef::Operation;

fn run(input: &str) -> String {
    let op = ToLowerCase;
    String::from_utf8(op.run(input.as_bytes().to_vec(), &[]).unwrap()).unwrap()
}
#[test]
fn test_all_upper() {
    assert_eq!(run("HELLO WORLD"), "hello world");
}
#[test]
fn test_mixed() {
    assert_eq!(run("HeLLo WoRlD"), "hello world");
}
#[test]
fn test_already_lower() {
    assert_eq!(run("hello world"), "hello world");
}
#[test]
fn test_digits_and_punctuation() {
    assert_eq!(run("ABC 123 !@#"), "abc 123 !@#");
}
#[test]
fn test_empty() {
    assert_eq!(run(""), "");
}
