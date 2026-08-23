// Tests for the cipher_saber2_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations cipher_saber2_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::cipher_saber2_encrypt::CipherSaber2Encrypt;
use rxchef::Operation;

#[test]
fn test_encrypt_output_longer_than_input() {
    let op = CipherSaber2Encrypt;
    let input = b"Hello World".to_vec();
    let args = [ArgValue::Str("mykey".to_string()), ArgValue::Num(20.0)];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result.len(), 10 + input.len());
}

#[test]
fn test_encrypt_empty_plaintext_boundary_is_iv_only() {
    let args = [ArgValue::Str(String::new()), ArgValue::Num(20.0)];
    assert_eq!(CipherSaber2Encrypt.run(Vec::new(), &args).unwrap().len(), 10);
}

#[test]
fn test_encrypt_rejects_malformed_hex_key() {
    let args = [ArgValue::Str("0xzz".into()), ArgValue::Num(20.0)];
    assert!(CipherSaber2Encrypt.run(b"data".to_vec(), &args).is_err());
}
