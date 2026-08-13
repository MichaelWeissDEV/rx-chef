// Tests for the pgp_encrypt operation.
#[cfg(feature = "pgp")]
use rxchef::{
    operation::ArgValue,
    operations::{
        generate_pgp_key_pair::GeneratePGPKeyPair, pgp_decrypt::PGPDecrypt,
        pgp_decrypt_and_verify::PGPDecryptAndVerify, pgp_encrypt::PGPEncrypt,
        pgp_encrypt_and_sign::PGPEncryptAndSign, pgp_verify::PGPVerify,
    },
    Operation,
};

#[test]
#[cfg(feature = "pgp")]
fn test_pgp_operations_roundtrip() {
    let keys = GeneratePGPKeyPair
        .run(
            Vec::new(),
            &[
                ArgValue::Str("ECC-256".into()),
                ArgValue::Str("secret".into()),
                ArgValue::Str("rxchef test".into()),
                ArgValue::Str("test@example.com".into()),
            ],
        )
        .unwrap();
    let keys: serde_json::Value = serde_json::from_slice(&keys).unwrap();
    let public = keys["publicKey"].as_str().unwrap();
    let private = keys["privateKey"].as_str().unwrap();
    let plaintext = b"OpenPGP operation round trip";

    let encrypted = PGPEncrypt
        .run(plaintext.to_vec(), &[ArgValue::Str(public.into())])
        .unwrap();
    let decrypted = PGPDecrypt
        .run(
            encrypted,
            &[
                ArgValue::Str(private.into()),
                ArgValue::Str("secret".into()),
            ],
        )
        .unwrap();
    assert_eq!(decrypted, plaintext);

    let signed = rxchef::operations::pgp::sign(plaintext, private, "secret").unwrap();
    let verified = PGPVerify
        .run(signed, &[ArgValue::Str(public.into())])
        .unwrap();
    assert_eq!(verified, plaintext);

    let encrypted_signed = PGPEncryptAndSign
        .run(
            plaintext.to_vec(),
            &[
                ArgValue::Str(private.into()),
                ArgValue::Str("secret".into()),
                ArgValue::Str(public.into()),
            ],
        )
        .unwrap();
    let decrypted_verified = PGPDecryptAndVerify
        .run(
            encrypted_signed,
            &[
                ArgValue::Str(public.into()),
                ArgValue::Str(private.into()),
                ArgValue::Str("secret".into()),
            ],
        )
        .unwrap();
    assert_eq!(decrypted_verified, plaintext);
    for operation in [
        PGPEncrypt.is_broken(),
        PGPDecrypt.is_broken(),
        PGPVerify.is_broken(),
        PGPEncryptAndSign.is_broken(),
        PGPDecryptAndVerify.is_broken(),
    ] {
        assert!(!operation);
    }
}
