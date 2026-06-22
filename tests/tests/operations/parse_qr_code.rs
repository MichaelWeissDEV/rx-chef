// Tests for the parse_qr_code operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_qr_code::

use rxchef::operations::parse_qr_code::ParseQRCode;
use rxchef::Operation;

#[test]
fn test_invalid_image() {
    let op = ParseQRCode;
    let result = op.run(b"not an image".to_vec(), &[]);
    assert!(result.is_err());
}
#[test]
fn test_empty_input() {
    let op = ParseQRCode;
    let result = op.run(vec![], &[]);
    assert!(result.is_err());
}
