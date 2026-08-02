// Tests for the xml_minify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xml_minify::

use rxchef::operations::xml_minify::XMLMinify;
use rxchef::Operation;

#[test]
fn test_xml_minify_empty_input() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_xml_minify_simple_xml() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let input = r#"<?xml version="1.0"?>
<root>
    <child>Hello World</child>
</root>"#.as_bytes().to_vec();
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("<root>"));
    assert!(result_str.contains("<child>"));
    assert!(result_str.contains("Hello World"));
    // Should be minified (no extra whitespace)
    assert!(!result_str.contains("\n"));
}

#[test]
fn test_xml_minify_with_comments() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(false)]; // Don't preserve comments
    let input = r#"<?xml version="1.0"?>
<!-- This is a comment -->
<root>
    <child>Hello</child>
    <!-- Another comment -->
</root>"#.as_bytes().to_vec();
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Comments should be removed
    assert!(!result_str.contains("This is a comment"));
    assert!(!result_str.contains("Another comment"));
    // XML content should remain
    assert!(result_str.contains("<root>"));
    assert!(result_str.contains("<child>"));
}

#[test]
fn test_xml_minify_preserve_comments() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(true)]; // Preserve comments
    let input = r#"<?xml version="1.0"?>
<!-- This is a comment -->
<root>
    <child>Hello</child>
</root>"#.as_bytes().to_vec();
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Comments should be preserved
    assert!(result_str.contains("This is a comment"));
    // XML content should remain
    assert!(result_str.contains("<root>"));
}

#[test]
fn test_xml_minify_complex_xml() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<catalog>
    <book id="bk101">
        <author>Gambardella, Matthew</author>
        <title>XML Developer's Guide</title>
        <price>44.95</price>
    </book>
    <book id="bk102">
        <author>Ralls, Kim</author>
        <title>Midnight Rain</title>
        <price>5.95</price>
    </book>
</catalog>"#.as_bytes().to_vec();
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should contain all elements
    assert!(result_str.contains("catalog"));
    assert!(result_str.contains("book"));
    assert!(result_str.contains("author"));
    assert!(result_str.contains("title"));
    assert!(result_str.contains("price"));
    // Should be minified
    assert!(!result_str.contains("\n"));
}

#[test]
fn test_xml_minify_invalid_xml() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let invalid_xml = r#"<?xml version="1.0"?>
<root>
    <unclosed>
</root>"#.as_bytes().to_vec();
    let result = op.run(invalid_xml, &args);
    assert!(result.is_err());
}

#[test]
fn test_xml_minify_self_closing_tags() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let input = r#"<?xml version="1.0"?>
<root>
    <selfclosing/>
    <empty></empty>
</root>"#.as_bytes().to_vec();
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("selfclosing"));
    assert!(result_str.contains("empty"));
}

#[test]
fn test_xml_minify_attributes() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let input = r#"<?xml version="1.0"?>
<root>
    <element attr1="value1" attr2="value2">Content</element>
</root>"#.as_bytes().to_vec();
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("attr1=\"value1\""));
    assert!(result_str.contains("attr2=\"value2\""));
    assert!(result_str.contains("Content"));
}

#[test]
fn test_xml_minify_cdata() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let input = r#"<?xml version="1.0"?>
<root>
    <script><![CDATA[
        function test() {
            return "Hello < World &";
        }
    ]]></script>
</root>"#.as_bytes().to_vec();
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // CDATA should be preserved
    assert!(result_str.contains("CDATA"));
    assert!(result_str.contains("function test"));
}

#[test]
fn test_xml_minify_whitespace_preservation() {
    let op = XMLMinify;
    let args = [rxchef::operation::ArgValue::Bool(false)];
    let input = r#"<?xml version="1.0"?>
<root>
    <text>  Multiple   spaces  </text>
</root>"#.as_bytes().to_vec();
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Text content should be trimmed
    assert!(result_str.contains("Multiple"));
    assert!(result_str.contains("spaces"));
}
