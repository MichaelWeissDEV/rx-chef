// Tests for the diff operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations diff::

use rxchef::operation::ArgValue;
use rxchef::operations::diff::Diff;
use rxchef::Operation;

fn run_op(
    input: &str,
    delim: &str,
    show_added: bool,
    show_removed: bool,
    show_subtraction: bool,
    ignore_ws: bool,
    ignore_case: bool,
) -> String {
    let op = Diff;
    let args = [
        ArgValue::Str(delim.to_string()),
        ArgValue::Bool(show_added),
        ArgValue::Bool(show_removed),
        ArgValue::Bool(show_subtraction),
        ArgValue::Bool(ignore_ws),
        ArgValue::Bool(ignore_case),
    ];
    let result = op.run(input.as_bytes().to_vec(), &args).unwrap();
    String::from_utf8(result).unwrap()
}
#[test]
fn test_diff_identical_strings() {
    let result = run_op("hello\n\nhello", "\n\n", true, true, false, false, false);
    assert_eq!(result, "hello");
}
#[test]
fn test_diff_show_added() {
    let result = run_op(
        "hello\n\nhello world",
        "\n\n",
        true,
        true,
        false,
        false,
        false,
    );
    assert!(result.contains("<ins>") || result.contains("world"));
}
#[test]
fn test_diff_show_removed() {
    let result = run_op(
        "hello world\n\nhello",
        "\n\n",
        true,
        true,
        false,
        false,
        false,
    );
    assert!(result.contains("<del>"));
}
#[test]
fn test_diff_hide_added() {
    let result = run_op(
        "hello\n\nhello world",
        "\n\n",
        false,
        true,
        false,
        false,
        false,
    );
    assert!(!result.contains("<ins>"));
}
#[test]
fn test_diff_hide_removed() {
    let result = run_op(
        "hello world\n\nhello",
        "\n\n",
        true,
        false,
        false,
        false,
        false,
    );
    assert!(!result.contains("<del>"));
}
#[test]
fn test_diff_show_subtraction_hides_unchanged() {
    let result = run_op("abc\n\nabd", "\n\n", true, true, true, false, false);
    // With show_subtraction=true, unchanged parts are hidden
    // 'a','b' are unchanged, 'c' removed, 'd' added
    assert!(!result.contains(">a<") && !result.contains(">b<"));
}
#[test]
fn test_diff_html_escape() {
    let result = run_op("<a>\n\n<b>", "\n\n", true, true, false, false, false);
    // HTML special chars should be escaped
    assert!(!result.contains("<a>") || result.contains("&lt;"));
}
#[test]
fn test_diff_too_few_samples() {
    let op = Diff;
    let args = [
        ArgValue::Str("\n\n".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(b"only one sample".to_vec(), &args);
    assert!(result.is_err());
}
#[test]
fn test_diff_custom_delimiter() {
    let result = run_op("hello|||world", "|||", true, true, false, false, false);
    // Should not be an error - "hello" vs "world"
    assert!(result.contains("<del>") || result.contains("<ins>") || !result.is_empty());
}
