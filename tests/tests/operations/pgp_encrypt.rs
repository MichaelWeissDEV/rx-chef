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

#[test]
#[cfg(feature = "pgp")]
fn test_pgp_interoperates_with_pinned_openpgpjs_rsa_fixtures() {
    const PUBLIC: &str = include_str!("../../fixtures/openpgp/alice-public.asc");
    const PRIVATE: &str = include_str!("../../fixtures/openpgp/alice-private.asc");
    const ENCRYPTED: &str = include_str!("../../fixtures/openpgp/alice-encrypted-message.asc");
    const SIGNED: &str = include_str!("../../fixtures/openpgp/alice-signed-message.asc");
    const PLAINTEXT: &[u8] = b"A common mistake that people make when trying to design something completely foolproof is to underestimate the ingenuity of complete fools.";

    assert_eq!(
        PGPDecrypt
            .run(
                ENCRYPTED.as_bytes().to_vec(),
                &[ArgValue::Str(PRIVATE.into()), ArgValue::Str(String::new())],
            )
            .unwrap(),
        PLAINTEXT
    );
    assert_eq!(
        PGPVerify
            .run(SIGNED.as_bytes().to_vec(), &[ArgValue::Str(PUBLIC.into())])
            .unwrap(),
        PLAINTEXT
    );

    let encrypted = PGPEncrypt
        .run(PLAINTEXT.to_vec(), &[ArgValue::Str(PUBLIC.into())])
        .unwrap();
    assert!(String::from_utf8_lossy(&encrypted).starts_with("-----BEGIN PGP MESSAGE-----"));
    assert_eq!(
        PGPDecrypt
            .run(
                encrypted,
                &[ArgValue::Str(PRIVATE.into()), ArgValue::Str(String::new())],
            )
            .unwrap(),
        PLAINTEXT
    );

    let encrypted_signed = PGPEncryptAndSign
        .run(
            PLAINTEXT.to_vec(),
            &[
                ArgValue::Str(PRIVATE.into()),
                ArgValue::Str(String::new()),
                ArgValue::Str(PUBLIC.into()),
            ],
        )
        .unwrap();
    assert_eq!(
        PGPDecryptAndVerify
            .run(
                encrypted_signed,
                &[
                    ArgValue::Str(PUBLIC.into()),
                    ArgValue::Str(PRIVATE.into()),
                    ArgValue::Str(String::new()),
                ],
            )
            .unwrap(),
        PLAINTEXT
    );
}

#[test]
#[cfg(feature = "pgp")]
fn test_generated_pgp_key_pair_has_requested_identity_and_interoperates() {
    let output = GeneratePGPKeyPair
        .run(
            Vec::new(),
            &[
                ArgValue::Str("ECC-256".into()),
                ArgValue::Str(String::new()),
                ArgValue::Str("Ada Lovelace".into()),
                ArgValue::Str("ada@example.test".into()),
            ],
        )
        .unwrap();
    let keys: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let public = keys["publicKey"].as_str().unwrap();
    let private = keys["privateKey"].as_str().unwrap();
    assert!(public.starts_with("-----BEGIN PGP PUBLIC KEY BLOCK-----"));
    assert!(private.starts_with("-----BEGIN PGP PRIVATE KEY BLOCK-----"));
    let encrypted = PGPEncrypt
        .run(
            b"generated-key proof".to_vec(),
            &[ArgValue::Str(public.into())],
        )
        .unwrap();
    assert_eq!(
        PGPDecrypt
            .run(
                encrypted,
                &[ArgValue::Str(private.into()), ArgValue::Str(String::new())],
            )
            .unwrap(),
        b"generated-key proof"
    );
}
