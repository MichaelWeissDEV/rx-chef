// Tests for the from_hex_content operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_hex_content::

use rxchef::operations::from_hex_content::FromHexContent;
use rxchef::Operation;

#[test]
fn test_from_hex_content_basic() {
    let op = FromHexContent;
    let input = b"foo|3d|bar".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, b"foo=bar");
}
#[test]
fn test_from_hex_content_with_spaces() {
    let op = FromHexContent;
    let input = b"foo|3d 41|bar".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, b"foo=Abar");
}
#[test]
fn test_from_hex_content_no_hex() {
    let op = FromHexContent;
    let input = b"plain text".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, b"plain text");
}
#[test]
fn test_from_hex_content_multiple() {
    let op = FromHexContent;
    let input = b"|48||65|llo".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, b"Hello");
}
