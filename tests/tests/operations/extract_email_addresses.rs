// Tests for the extract_email_addresses operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_email_addresses::

use rxchef::operation::ArgValue;
use rxchef::operations::extract_email_addresses::ExtractEmailAddresses;
use rxchef::Operation;

#[test]
fn test_extract_email_addresses() {
    let op = ExtractEmailAddresses;
    let input = b"Contact us at support@example.com or sales@example.org".to_vec();
    let args = [
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "support@example.com\nsales@example.org");
}
