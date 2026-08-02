// Tests for the to_kebab_case operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_kebab_case::

use rxchef::operations::to_kebab_case::ToKebabCase;
use rxchef::Operation;

#[test]
fn test_to_kebab_case_empty_input() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_to_kebab_case_simple() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run("HelloWorld".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "hello-world".as_bytes());
}

#[test]
fn test_to_kebab_case_with_spaces() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run("Hello World".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "hello-world".as_bytes());
}

#[test]
fn test_to_kebab_case_mixed_case() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run("ThisIsATest".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "this-is-atest".as_bytes());
}

#[test]
fn test_to_kebab_case_with_numbers() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run("Test123String".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "test123string".as_bytes());
}

#[test]
fn test_to_kebab_case_with_special_chars() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run("Hello-World!".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "hello-world".as_bytes());
}

#[test]
fn test_to_kebab_case_already_kebab() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run("already-kebab-case".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "already-kebab-case".as_bytes());
}

#[test]
fn test_to_kebab_case_smart_mode() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(true)];
    let result = op.run("var myVariable = 42;".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "var my-variable = 42;".as_bytes());
}

#[test]
fn test_to_kebab_case_smart_with_quotes() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(true)];
    let result = op.run(r#"function myFunction() { return "hello"; }"#.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, r#"function my-function() { return "hello"; }"#.as_bytes());
}

#[test]
fn test_to_kebab_case_all_uppercase() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run("HELLOWORLD".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "helloworld".as_bytes());
}

#[test]
fn test_to_kebab_case_single_word() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run("Hello".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "hello".as_bytes());
}

#[test]
fn test_to_kebab_case_with_underscores() {
    let op = ToKebabCase;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run("hello_world_test".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "hello-world-test".as_bytes());
}
