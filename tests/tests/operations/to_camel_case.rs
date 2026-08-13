// Tests for the to_camel_case operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_camel_case::

use rxchef::operation::ArgValue;
use rxchef::operations::to_camel_case::ToCamelCase;
use rxchef::Operation;

fn run(input: &str, smart: bool) -> String {
    let op = ToCamelCase;
    let args = [ArgValue::Bool(smart)];
    let result = op.run(input.as_bytes().to_vec(), &args).unwrap();
    String::from_utf8(result).unwrap()
}

#[test]
fn test_simple_words() {
    assert_eq!(run("hello world", false), "helloWorld");
}

#[test]
fn test_snake_case_input() {
    assert_eq!(run("snake_case_example", false), "snakeCaseExample");
}

#[test]
fn test_pascal_case_input() {
    assert_eq!(run("FooBar", false), "fooBar");
}

#[test]
fn test_empty_input() {
    assert_eq!(run("", false), "");
}

#[test]
fn test_default_args_matches_non_smart_mode() {
    // Empty args slice should fall back to the documented default
    // ("Attempt to be context aware" = false).
    let op = ToCamelCase;
    let result = op.run(b"hello world".to_vec(), &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "helloWorld");
}

#[test]
fn test_smart_mode_leaves_quoted_strings_untouched() {
    // In "context aware" mode, bare identifiers are camel-cased but the
    // contents of double-quoted strings are left alone.
    let result = run(r#""hello_world" foo_bar"#, true);
    assert_eq!(result, r#""hello_world" fooBar"#);
}
