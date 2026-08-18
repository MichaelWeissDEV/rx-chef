// Tests for the url_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations url_decode::

use rxchef::operation::ArgValue;
use rxchef::operations::url_decode::URLDecode;
use rxchef::Operation;

#[test]
fn test_basic_decode() {
    let op = URLDecode;
    let result = op
        .run(b"Hello%20World".to_vec(), &[ArgValue::Bool(false)])
        .unwrap();
    assert_eq!(result, b"Hello World");
}
#[test]
fn test_plus_as_space() {
    let op = URLDecode;
    let result = op
        .run(b"Hello+World".to_vec(), &[ArgValue::Bool(true)])
        .unwrap();
    assert_eq!(result, b"Hello World");
}
#[test]
fn test_plus_not_space() {
    let op = URLDecode;
    let result = op
        .run(b"Hello+World".to_vec(), &[ArgValue::Bool(false)])
        .unwrap();
    assert_eq!(result, b"Hello+World");
}
#[test]
fn test_equals_decode() {
    let op = URLDecode;
    let result = op.run(b"%3d".to_vec(), &[]).unwrap();
    assert_eq!(result, b"=");
}
#[test]
fn test_uppercase_hex() {
    let op = URLDecode;
    let result = op.run(b"%3D".to_vec(), &[]).unwrap();
    assert_eq!(result, b"=");
}
#[test]
fn test_multiple_encoded() {
    let op = URLDecode;
    let result = op.run(b"a%3Db%26c%3Dd".to_vec(), &[]).unwrap();
    assert_eq!(result, b"a=b&c=d");
}
#[test]
fn test_invalid_percent_sequence_passes_through() {
    let op = URLDecode;
    let result = op.run(b"%zz".to_vec(), &[]).unwrap();
    assert_eq!(result, b"%zz");
}
#[test]
fn test_empty_input() {
    let op = URLDecode;
    let result = op.run(b"".to_vec(), &[]).unwrap();
    assert!(result.is_empty());
}
