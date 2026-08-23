#[cfg(feature = "pgp")]
use rxchef::{operation::ArgValue, operations::{pgp_decrypt_and_verify::PGPDecryptAndVerify, pgp_encrypt_and_sign::PGPEncryptAndSign}, Operation};

#[test]
#[cfg(feature = "pgp")]
fn test_pgp_decrypt_and_verify_openpgpjs_known_answer() {
    let plaintext = b"fixed external-key decrypt-and-verify proof";
    let message = PGPEncryptAndSign.run(
        plaintext.to_vec(),
        &[
            ArgValue::Str(include_str!("../../fixtures/openpgp/alice-private.asc").into()),
            ArgValue::Str(String::new()),
            ArgValue::Str(include_str!("../../fixtures/openpgp/alice-public.asc").into()),
        ],
    ).unwrap();
    let result = PGPDecryptAndVerify.run(
        message,
        &[
            ArgValue::Str(include_str!("../../fixtures/openpgp/alice-public.asc").into()),
            ArgValue::Str(include_str!("../../fixtures/openpgp/alice-private.asc").into()),
            ArgValue::Str(String::new()),
        ],
    ).unwrap();
    assert_eq!(result, plaintext);
}
