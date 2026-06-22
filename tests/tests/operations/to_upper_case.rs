// Tests for the to_upper_case operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_upper_case::

use rxchef::operation::ArgValue;
use rxchef::operations::to_upper_case::ToUpperCase;
use rxchef::Operation;

fn run(input: &str, scope: &str) -> String {
    let op = ToUpperCase;
    let args = [ArgValue::Str(scope.to_string())];
    String::from_utf8(op.run(input.as_bytes().to_vec(), &args).unwrap()).unwrap()
}
#[test]
fn test_all() {
    assert_eq!(run("hello world", "All"), "HELLO WORLD");
}
#[test]
fn test_word() {
    assert_eq!(run("hello world", "Word"), "Hello World");
}
#[test]
fn test_sentence() {
    assert_eq!(
        run("hello world. foo bar", "Sentence"),
        "Hello world. Foo bar"
    );
}
#[test]
fn test_paragraph() {
    assert_eq!(run("hello\nworld", "Paragraph"), "Hello\nWorld");
}
#[test]
fn test_already_upper_all() {
    assert_eq!(run("HELLO", "All"), "HELLO");
}
#[test]
fn test_invalid_scope() {
    let op = ToUpperCase;
    let args = [ArgValue::Str("Bogus".to_string())];
    assert!(op.run(b"test".to_vec(), &args).is_err());
}
