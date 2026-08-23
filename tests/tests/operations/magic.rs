// Tests for the magic operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations magic::

use rxchef::operations::magic::Magic;
use rxchef::operation::ArgValue;
use rxchef::Operation;

#[test]
fn test_magic_entropy() {
    let op = Magic;
    let input = b"aaaaa".to_vec();
    let result = op.run(input, &[]).unwrap();
    // Just check if it returns something valid
    assert!(result.len() > 0);
}

#[test]
fn test_magic_decodes_base64_known_answer() {
    let result = Magic.run(b"SGVsbG8gV29ybGQ=".to_vec(), &[]).unwrap();
    let matches: serde_json::Value = serde_json::from_slice(&result).unwrap();
    assert_eq!(matches[0]["recipe"][0]["op"], "From Base64");
    assert_eq!(matches[0]["preview"], "Hello World");
    assert_eq!(matches[0]["valid_utf8"], true);
    assert_eq!(matches[0]["printable_ratio"].as_f64(), Some(1.0));
}

#[test]
fn test_magic_zero_depth_boundary_returns_no_candidates() {
    assert_eq!(
        Magic.run(b"SGVsbG8gV29ybGQ=".to_vec(), &[ArgValue::Num(0.0)]).unwrap(),
        b"[]"
    );
}

#[test]
fn test_magic_rejects_invalid_crib_regex() {
    let result = Magic.run(
        b"data".to_vec(),
        &[
            ArgValue::Num(3.0), ArgValue::Bool(false), ArgValue::Bool(false),
            ArgValue::Str("[".into()),
        ],
    );
    assert!(result.is_err());
}
