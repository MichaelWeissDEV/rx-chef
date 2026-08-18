// Tests for the to_snake_case operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_snake_case::

use rxchef::operation::ArgValue;
use rxchef::operations::to_snake_case::ToSnakeCase;
use rxchef::Operation;

fn run(input: &str, smart: bool) -> String {
    let op = ToSnakeCase;
    let args = [ArgValue::Bool(smart)];
    let result = op.run(input.as_bytes().to_vec(), &args).unwrap();
    String::from_utf8(result).unwrap()
}

#[test]
fn test_camel_case_input() {
    assert_eq!(run("fooBar", false), "foo_bar");
}

#[test]
fn test_simple_words() {
    assert_eq!(run("hello world", false), "hello_world");
}

#[test]
fn test_pascal_case_input() {
    assert_eq!(run("FooBarBaz", false), "foo_bar_baz");
}

#[test]
fn test_empty_input() {
    assert_eq!(run("", false), "");
}

#[test]
fn test_default_args_matches_non_smart_mode() {
    let op = ToSnakeCase;
    let result = op.run(b"fooBar".to_vec(), &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "foo_bar");
}

#[test]
fn test_smart_mode_leaves_quoted_strings_untouched() {
    let result = run(r#""helloWorld" fooBarBaz"#, true);
    assert_eq!(result, r#""helloWorld" foo_bar_baz"#);
}
