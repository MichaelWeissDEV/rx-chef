// Tests for the xxtea_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xxtea_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::xxtea_decrypt::XxteaDecryptOp;
use rxchef::operations::xxtea_encrypt::XxteaEncryptOp;
use rxchef::Operation;

#[test]
fn test_xxtea_decrypt_pinned_cyberchef_vector() {
    // Ciphertext observed from CyberChef 11.4.0 at commit 2e048b029085.
    let ciphertext =
        hex::decode("3db5a39db1663fc029bb630a38635b8de5bfef62192e52cc4bf83cda8ccbc701").unwrap();
    let output = XxteaDecryptOp
        .run(ciphertext, &[ArgValue::Str("1234567890".into())])
        .unwrap();
    assert_eq!(output, "ნუ პანიკას".as_bytes());
}

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
