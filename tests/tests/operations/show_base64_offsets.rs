// Tests for the show_base64_offsets operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations show_base64_offsets::

use rxchef::operation::ArgValue;
use rxchef::operations::show_base64_offsets::ShowBase64Offsets;
use rxchef::Operation;

#[test]
fn test_show_base64_offsets_basic() {
    let op = ShowBase64Offsets;
    let input = b"Hello".to_vec();
    let args = [
        ArgValue::Str("A-Za-z0-9+/=".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "SGVsbG\nhlbGxv\nIZWxsb");
}

#[test]
fn test_show_base64_offsets_matches_pinned_cyberchef() {
    let op = ShowBase64Offsets;
    let args = [
        ArgValue::Str("A-Za-z0-9+/=".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("Raw".to_string()),
    ];
    assert_eq!(op.run(b"Man".to_vec(), &args).unwrap(), b"TWFu\n1hb\nNYW");
}

#[test]
fn test_show_base64_offsets_one_byte_boundary() {
    let op = ShowBase64Offsets;
    let args = [
        ArgValue::Str("A-Za-z0-9+/=".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("Raw".to_string()),
    ];
    assert_eq!(op.run(b"M".to_vec(), &args).unwrap(), b"T\n\nN");
}

#[test]
fn test_show_base64_offsets_rejects_empty_input() {
    let op = ShowBase64Offsets;
    assert!(op.run(Vec::new(), &[]).is_err());
}
