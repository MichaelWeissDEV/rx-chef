// Tests for the sql_beautify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sql_beautify::

use rxchef::operation::ArgValue;
use rxchef::operations::sql_beautify::SQLBeautify;
use rxchef::Operation;

#[test]
fn test_sql_beautify_basic() {
    let op = SQLBeautify;
    let input = b"SELECT * FROM table WHERE id = 1".to_vec();
    let args = [ArgValue::Str("\\t".to_string())];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert!(s.contains("SELECT"));
    assert!(s.contains("FROM"));
}
#[test]
fn test_sql_beautify_bind_variables() {
    let op = SQLBeautify;
    let input = b"SELECT * FROM table WHERE id = :my_id".to_vec();
    let args = [ArgValue::Str("    ".to_string())];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert!(s.contains(":my_id"));
}
#[test]
fn test_sql_beautify_empty() {
    let op = SQLBeautify;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
