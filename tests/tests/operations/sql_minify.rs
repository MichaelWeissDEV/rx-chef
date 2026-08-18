// Tests for the sql_minify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sql_minify::

use rxchef::operations::sql_minify::SQLMinify;
use rxchef::Operation;

#[test]
fn test_sql_minify_basic() {
    let op = SQLMinify;
    let input = b"SELECT * \n FROM table \n WHERE id = 1".to_vec();
    let result = op.run(input, &[]).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert_eq!(s, "SELECT * FROM table WHERE id = 1");
}
#[test]
fn test_sql_minify_comments() {
    let op = SQLMinify;
    let input =
        b"SELECT * -- line comment\n FROM table /* block \n comment */ WHERE id = 1".to_vec();
    let result = op.run(input, &[]).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert_eq!(s, "SELECT * FROM table WHERE id = 1");
}
#[test]
fn test_sql_minify_strings() {
    let op = SQLMinify;
    let input = b"SELECT 'string with spaces' FROM table".to_vec();
    let result = op.run(input, &[]).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert_eq!(s, "SELECT 'string with spaces' FROM table");
}
#[test]
fn test_sql_minify_empty() {
    let op = SQLMinify;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
