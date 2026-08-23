#[cfg(feature = "pgp")]
use rxchef::{operation::ArgValue, operations::pgp_decrypt::PGPDecrypt, Operation};

#[test]
#[cfg(feature = "pgp")]
fn test_pgp_decrypt_openpgpjs_known_answer() {
    let result = PGPDecrypt
        .run(
            include_bytes!("../../fixtures/openpgp/alice-encrypted-message.asc").to_vec(),
            &[
                ArgValue::Str(include_str!("../../fixtures/openpgp/alice-private.asc").into()),
                ArgValue::Str(String::new()),
            ],
        )
        .unwrap();
    assert_eq!(result, b"A common mistake that people make when trying to design something completely foolproof is to underestimate the ingenuity of complete fools.");
}
