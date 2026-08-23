// Tests for the sm2_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sm2_encrypt::

use libsm::sm2::signature::SigCtx;
use rxchef::operation::ArgValue;
use rxchef::operations::sm2_decrypt::Sm2Decrypt;
use rxchef::operations::sm2_encrypt::Sm2Encrypt;
use rxchef::Operation;

#[test]
fn test_sm2_encrypt_invalid_key() {
    let op = Sm2Encrypt;
    let args = [
        ArgValue::Str("SHORT".to_string()),
        ArgValue::Str("DEADBEEF".to_string()),
        ArgValue::Str("C1C3C2".to_string()),
        ArgValue::Str("sm2p256v1".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}
#[test]
fn test_sm2_encrypt_missing_key() {
    let op = Sm2Encrypt;
    let args = [
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("C1C3C2".to_string()),
        ArgValue::Str("sm2p256v1".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}
#[test]
fn test_sm2_roundtrip() {
    let context = SigCtx::new();
    let (public, private) = context.new_keypair().unwrap();
    let encoded = context.serialize_pubkey(&public, false).unwrap();
    let x = hex::encode(&encoded[1..33]);
    let y = hex::encode(&encoded[33..65]);
    let private = format!("{:0>64}", private.to_str_radix(16));

    for format in ["C1C3C2", "C1C2C3"] {
        let ciphertext = Sm2Encrypt
            .run(
                b"SM2 round trip".to_vec(),
                &[
                    ArgValue::Str(x.clone()),
                    ArgValue::Str(y.clone()),
                    ArgValue::Str(format.into()),
                    ArgValue::Str("sm2p256v1".into()),
                ],
            )
            .unwrap();
        let transport = String::from_utf8(ciphertext.clone()).unwrap();
        assert_eq!(transport.len(), 2 * (64 + 32 + b"SM2 round trip".len()));
        assert!(!transport.starts_with("04"));
        let plaintext = Sm2Decrypt
            .run(
                ciphertext,
                &[
                    ArgValue::Str(private.clone()),
                    ArgValue::Str(format.into()),
                    ArgValue::Str("sm2p256v1".into()),
                ],
            )
            .unwrap();
        assert_eq!(plaintext, b"SM2 round trip");
    }
}
