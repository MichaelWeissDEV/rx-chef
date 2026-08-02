// Tests for the from_html_entity operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_html_entity::

use rxchef::operations::from_html_entity::FromHTMLEntity;
use rxchef::Operation;

#[test]
fn test_from_html_entity_empty_input() {
    let op = FromHTMLEntity;
    let args = [];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_html_entity_simple_named() {
    let op = FromHTMLEntity;
    let args = [];
    // Simple named entity: &lt; -> <
    let html_input = "&lt;hello&gt;";
    let result = op.run(html_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "<hello>");
}

#[test]
fn test_from_html_entity_numeric() {
    let op = FromHTMLEntity;
    let args = [];
    // Numeric entity: &#72;&#101;&#108;&#108;&#111; -> Hello
    let html_input = "&#72;&#101;&#108;&#108;&#111;";
    let result = op.run(html_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_html_entity_mixed() {
    let op = FromHTMLEntity;
    let args = [];
    // Mixed entities: &quot;Hello&quot; &amp; &apos;World&apos;
    let html_input = "&quot;Hello&quot; &amp; &apos;World&apos;";
    let result = op.run(html_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "\"Hello\" & 'World'");
}

#[test]
fn test_from_html_entity_no_entities() {
    let op = FromHTMLEntity;
    let args = [];
    // Plain text with no entities
    let html_input = "Hello World";
    let result = op.run(html_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello World");
}
