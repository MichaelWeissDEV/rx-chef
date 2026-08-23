#[cfg(feature = "pgp")]
use rxchef::{operation::ArgValue, operations::pgp_verify::PGPVerify, Operation};

#[test]
#[cfg(feature = "pgp")]
fn test_pgp_verify_openpgpjs_known_answer() {
    let result = PGPVerify.run(
        include_bytes!("../../fixtures/openpgp/alice-signed-message.asc").to_vec(),
        &[ArgValue::Str(include_str!("../../fixtures/openpgp/alice-public.asc").into())],
    ).unwrap();
    assert_eq!(result, b"A common mistake that people make when trying to design something completely foolproof is to underestimate the ingenuity of complete fools.");
}
