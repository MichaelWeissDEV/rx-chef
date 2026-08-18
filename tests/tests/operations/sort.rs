// Tests for the sort operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sort::

use rxchef::operation::ArgValue;
use rxchef::operations::sort::Sort;
use rxchef::Operation;

#[test]
fn test_sort_alpha_case_sensitive() {
    let op = Sort;
    let input = b"banana\napple\ncherry".to_vec();
    let args = vec![
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("Alphabetical (case sensitive)".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "apple\nbanana\ncherry");
}
#[test]
fn test_sort_numeric() {
    let op = Sort;
    let input = b"10\n2\n30\n4".to_vec();
    let args = vec![
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("Numeric".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "2\n4\n10\n30");
}
#[test]
fn test_sort_reverse() {
    let op = Sort;
    let input = b"a\nb\nc".to_vec();
    let args = vec![
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Bool(true),
        ArgValue::Str("Alphabetical (case sensitive)".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "c\nb\na");
}
#[test]
fn test_sort_ip() {
    let op = Sort;
    let input = b"10.0.0.2\n10.0.0.1\n192.168.1.1".to_vec();
    let args = vec![
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("IP address".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "10.0.0.1\n10.0.0.2\n192.168.1.1");
}
