// Tests for the template operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations template::

use rxchef::operation::ArgValue;
use rxchef::operations::template::Template;
use rxchef::Operation;
use serde_json::json;

#[test]
fn test_template_basic() {
    let op = Template;
    let input = serde_json::to_vec(&json!({"name": "World"})).unwrap();
    let args = [ArgValue::Str("Hello {{name}}!".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "Hello World!");
}
#[test]
fn test_template_nested() {
    let op = Template;
    let input = serde_json::to_vec(&json!({"user": {"name": "Alice", "id": 123}})).unwrap();
    let args = [ArgValue::Str(
        "User {{user.name}} has ID {{user.id}}".to_string(),
    )];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "User Alice has ID 123");
}
#[test]
fn test_template_missing_key() {
    let op = Template;
    let input = serde_json::to_vec(&json!({"a": 1})).unwrap();
    let args = [ArgValue::Str("Value: {{b}}".to_string())];
    let result = op.run(input, &args).unwrap();
    // {{b}} should be replaced with "null" (serde_json::Value::Null.to_string() is "null")
    // Wait, my code handles Value::Null by pushing nothing.
    // But &val["b"] returns Value::Null if not found.
    assert_eq!(String::from_utf8_lossy(&result), "Value: ");
}
#[test]
fn test_template_no_tags() {
    let op = Template;
    let input = serde_json::to_vec(&json!({})).unwrap();
    let args = [ArgValue::Str("No tags here".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "No tags here");
}
