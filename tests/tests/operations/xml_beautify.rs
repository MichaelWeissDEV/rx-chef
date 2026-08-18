// Tests for the xml_beautify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xml_beautify::

use rxchef::operation::ArgValue;
use rxchef::operations::xml_beautify::XMLBeautify;
use rxchef::Operation;

fn beautify(input: &str, indent: &str) -> String {
    let args = [ArgValue::Str(indent.to_string())];
    String::from_utf8(XMLBeautify.run(input.as_bytes().to_vec(), &args).unwrap()).unwrap()
}

#[test]
fn test_xml_beautify_empty_input() {
    assert_eq!(beautify("", "\\t"), "");
}

#[test]
fn test_xml_beautify_nests_child_elements() {
    assert_eq!(beautify("<a><b>c</b></a>", "\\t"), "<a>\n\t<b>c</b>\n</a>");
}

#[test]
fn test_xml_beautify_respects_a_custom_indent_string() {
    assert_eq!(beautify("<a><b>c</b></a>", "  "), "<a>\n  <b>c</b>\n</a>");
}

#[test]
fn test_xml_beautify_indents_each_nesting_level() {
    assert_eq!(
        beautify("<a><b><c>d</c></b></a>", "  "),
        "<a>\n  <b>\n    <c>d</c>\n  </b>\n</a>"
    );
}

#[test]
fn test_xml_beautify_keeps_leaf_elements_on_one_line() {
    assert_eq!(beautify("<a>text</a>", "\\t"), "<a>text</a>");
}

#[test]
fn test_xml_beautify_preserves_attributes() {
    let output = beautify("<a id=\"1\"><b x=\"y\">c</b></a>", "  ");
    assert!(output.contains("id=\"1\""), "attributes lost: {output}");
    assert!(output.contains("x=\"y\""), "attributes lost: {output}");
}

#[test]
fn test_xml_beautify_handles_sibling_elements() {
    assert_eq!(
        beautify("<r><a>1</a><b>2</b></r>", "  "),
        "<r>\n  <a>1</a>\n  <b>2</b>\n</r>"
    );
}

#[test]
fn test_xml_beautify_unclosed_tags_do_not_panic() {
    // Documented divergence: the beautifier is a formatter, not a validator,
    // so unbalanced input is reformatted on a best-effort basis rather than
    // rejected. It must still terminate without panicking.
    let output = beautify("<a><b>", "\\t");
    assert!(output.contains("<a>"), "unexpected output: {output}");
    assert!(output.contains("<b>"), "unexpected output: {output}");
}

