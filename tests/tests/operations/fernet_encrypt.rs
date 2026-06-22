// Tests for the fernet_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations fernet_encrypt::

use fernet::Fernet;
use rxchef::operation::ArgValue;
use rxchef::operations::fernet_decrypt::FernetDecrypt;
use rxchef::operations::fernet_encrypt::FernetEncrypt;
use rxchef::Operation;

#[test]
fn test_fernet_encrypt_valid() {
    let op = FernetEncrypt;
    let result = op
        .run(
            b"This is a secret message.\n".to_vec(),
            &[ArgValue::Str(
                "MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=".to_string(),
            )],
        )
        .expect("should encrypt");
    let token = String::from_utf8(result).expect("valid utf8");
    // Fernet tokens start with gAAA (version byte 0x80 in base64url)
    assert!(token.starts_with("gAAA"));
}
#[test]
fn test_fernet_encrypt_no_key() {
    let op = FernetEncrypt;
    let result = op.run(
        b"This is a secret message.\n".to_vec(),
        &[ArgValue::Str("".to_string())],
    );
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("32 url-safe base64-encoded bytes"));
}
#[test]
fn test_fernet_roundtrip() {
    let key = "MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=";
    let enc_op = FernetEncrypt;
    let dec_op = FernetDecrypt;
    let plaintext = b"This is a secret message.\n";
    let encrypted = enc_op
        .run(plaintext.to_vec(), &[ArgValue::Str(key.to_string())])
        .expect("encrypt");
    let _ = Fernet::new(key).expect("valid key");
    let decrypted = dec_op
        .run(encrypted, &[ArgValue::Str(key.to_string())])
        .expect("decrypt");
    assert_eq!(decrypted, plaintext);
}
