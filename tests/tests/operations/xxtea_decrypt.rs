// Tests for the xxtea_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xxtea_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::xxtea_decrypt::XxteaDecryptOp;
use rxchef::operations::xxtea_encrypt::XxteaEncryptOp;
use rxchef::Operation;

#[test]
fn test_xxtea_roundtrip() {
    let encrypt_op = XxteaEncryptOp;
    let decrypt_op = XxteaDecryptOp;
    let input = b"Hello World".to_vec();
    let args = [ArgValue::Str("secret".to_string())];
    let encrypted = encrypt_op.run(input.clone(), &args).unwrap();
    let decrypted = decrypt_op.run(encrypted, &args).unwrap();
    assert_eq!(decrypted, input);
}
