// Tests for the xxtea_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xxtea_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::xxtea_encrypt::XxteaEncryptOp;
use rxchef::Operation;

#[test]
fn test_xxtea_encrypt_rejects_missing_key() {
    let error = XxteaEncryptOp.run(b"x".to_vec(), &[]).unwrap_err();
    assert!(matches!(
        error,
        rxchef::operation::OperationError::InvalidArgument { ref name, .. }
            if name == "Key"
    ));
}

const CYBERCHEF_XXTEA_VECTOR: &str =
    "3db5a39db1663fc029bb630a38635b8de5bfef62192e52cc4bf83cda8ccbc701";

#[test]
fn test_xxtea_encrypt_pinned_cyberchef_vector() {
    // Observed from CyberChef 11.4.0 at commit 2e048b029085; this is the
    // upstream fixed XXTEA regression vector, not a local round-trip value.
    let output = XxteaEncryptOp
        .run(
            "ნუ პანიკას".as_bytes().to_vec(),
            &[ArgValue::Str("1234567890".into())],
        )
        .unwrap();
    assert_eq!(hex::encode(output), CYBERCHEF_XXTEA_VECTOR);
}

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
