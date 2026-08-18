// Tests for the to_html_entity operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_html_entity::

use rxchef::operation::ArgValue;
use rxchef::operations::to_html_entity::ToHTMLEntity;
use rxchef::Operation;

fn run(input: &str, convert_all: bool, convert_to: &str) -> String {
    let op = ToHTMLEntity;
    let args = [
        ArgValue::Bool(convert_all),
        ArgValue::Str(convert_to.to_string()),
    ];
    let result = op.run(input.as_bytes().to_vec(), &args).unwrap();
    String::from_utf8(result).unwrap()
}

#[test]
fn test_default_only_converts_special_chars() {
    // Default mode (convert_all=false, Named entities): only characters that
    // have a named entity (or are outside Latin-1) get converted; ordinary
    // letters/digits/spaces pass through untouched.
    assert_eq!(
        run("<a href=\"x\">Tom & Jerry</a>", false, "Named entities"),
        "&lt;a href=&quot;x&quot;&gt;Tom &amp; Jerry&lt;/a&gt;"
    );
}

#[test]
fn test_empty_input() {
    assert_eq!(run("", false, "Named entities"), "");
}

#[test]
fn test_convert_all_named_entities() {
    // With "Convert all characters" on, every character is converted: named
    // entity if one exists, otherwise a numeric reference.
    assert_eq!(run("AB", true, "Named entities"), "&#65;&#66;");
    assert_eq!(run("&", true, "Named entities"), "&amp;");
}

#[test]
fn test_convert_all_numeric_entities() {
    assert_eq!(run("AB", true, "Numeric entities"), "&#65;&#66;");
}

#[test]
fn test_convert_all_hex_entities() {
    assert_eq!(run("AB", true, "Hex entities"), "&#x41;&#x42;");
}

#[test]
fn test_numeric_mode_only_converts_special_or_high_codepoints() {
    // 'e' with acute accent (U+00E9) has a named entity in the table, so in
    // numeric (non-"convert all") mode it becomes a numeric reference even
    // though it's a plain Latin-1 codepoint; ordinary ASCII letters that have
    // no entry (like 'a','f') pass through unchanged.
    assert_eq!(run("caf\u{e9}", false, "Numeric entities"), "caf&#233;");
}

#[test]
fn test_default_args_is_named_entities_non_convert_all() {
    let op = ToHTMLEntity;
    let result = op.run(b"<b>".to_vec(), &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "&lt;b&gt;");
}

