// Tests for the unique operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations unique::

use rxchef::operation::ArgValue;
use rxchef::operations::unique::Unique;
use rxchef::Operation;

#[test]
fn test_unique_basic() {
    let op = Unique;
    let input = b"apple\nbanana\napple\ncherry".to_vec();
    let args = vec![
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "apple\nbanana\ncherry");
}
#[test]
fn test_unique_with_count() {
    let op = Unique;
    let input = b"a\na\nb".to_vec();
    let args = vec![ArgValue::Str("Line feed".to_string()), ArgValue::Bool(true)];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("2 a"));
    assert!(out.contains("1 b"));
}
#[test]
fn test_unique_no_duplicates() {
    let op = Unique;
    let input = b"x\ny\nz".to_vec();
    let args = vec![
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "x\ny\nz");
}
