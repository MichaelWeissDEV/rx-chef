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
        ArgValue::Str("UTF-8".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Offset"));
}
