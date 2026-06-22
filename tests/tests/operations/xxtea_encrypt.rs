// Tests for the xxtea_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xxtea_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::xxtea_encrypt::XxteaEncryptOp;
use rxchef::Operation;

#[test]
fn test_xxtea_encrypt_basic() {
    let op = XxteaEncryptOp;
    let input = b"Hello World".to_vec();
    let args = [ArgValue::Str("secret".to_string())];
    let result = op.run(input, &args).unwrap();
    // Result should be some encrypted bytes
    assert!(!result.is_empty());
    assert_ne!(result, b"Hello World");
}
#[test]
fn test_xxtea_encrypt_empty() {
    let op = XxteaEncryptOp;
    let result = op.run(vec![], &[ArgValue::Str("key".to_string())]).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}
