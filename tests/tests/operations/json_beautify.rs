// Tests for the json_beautify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations json_beautify::

use rxchef::operation::ArgValue;
use rxchef::operations::json_beautify::JSONBeautify;
use rxchef::Operation;

fn run_op(input: &str, indent: &str, sort: bool) -> String {
    let op = JSONBeautify;
    let args = vec![ArgValue::Str(indent.to_string()), ArgValue::Bool(sort)];
    let result = op
        .run(input.as_bytes().to_vec(), &args)
        .expect("run failed");
    String::from_utf8(result).expect("utf8")
}
#[test]
fn test_empty_input() {
    assert_eq!(run_op("", " ", false), "");
}
#[test]
fn test_number() {
    assert_eq!(run_op("42", " ", false), "42");
}
#[test]
fn test_boolean() {
    assert_eq!(run_op("false", " ", false), "false");
}
#[test]
fn test_empty_list() {
    assert_eq!(run_op("[]", " ", false), "[]");
}
#[test]
fn test_list_space() {
    assert_eq!(run_op("[2,1]", " ", false), "[\n 2,\n 1\n]");
}
#[test]
fn test_list_tab() {
    assert_eq!(run_op("[2,1]", "\t", false), "[\n\t2,\n\t1\n]");
}
#[test]
fn test_object_space() {
    assert_eq!(
        run_op("{\"second\":2,\"first\":3}", " ", false),
        "{\n \"second\": 2,\n \"first\": 3\n}"
    );
}
#[test]
fn test_nested_tab() {
    let input =
        "[2,{\"second\":2,\"first\":3,\"beginning\":{\"j\":\"3\",\"i\":[2,3,false]}},1,2,3]";
    let expected = "[\n\t2,\n\t{\n\t\t\"second\": 2,\n\t\t\"first\": 3,\n\t\t\"beginning\": {\n\t\t\t\"j\": \"3\",\n\t\t\t\"i\": [\n\t\t\t\t2,\n\t\t\t\t3,\n\t\t\t\tfalse\n\t\t\t]\n\t\t}\n\t},\n\t1,\n\t2,\n\t3\n]";
    assert_eq!(run_op(input, "\t", false), expected);
}
#[test]
fn test_nested_tab_sorted() {
    let input =
        "[2,{\"second\":2,\"first\":3,\"beginning\":{\"j\":\"3\",\"i\":[2,3,false]}},1,2,3]";
    let expected = "[\n\t2,\n\t{\n\t\t\"beginning\": {\n\t\t\t\"i\": [\n\t\t\t\t2,\n\t\t\t\t3,\n\t\t\t\tfalse\n\t\t\t],\n\t\t\t\"j\": \"3\"\n\t\t},\n\t\t\"first\": 3,\n\t\t\"second\": 2\n\t},\n\t1,\n\t2,\n\t3\n]";
    assert_eq!(run_op(input, "\t", true), expected);
}
