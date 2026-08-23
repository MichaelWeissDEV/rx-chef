// Tests for the generate_ecdsa_key_pair operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_ecdsa_key_pair::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_ecdsa_key_pair::GenerateECDSAKeyPairOp;
use rxchef::Operation;

#[test]
fn test_generate_ecdsa_key_pair_p256_pem_contains_both_keys() {
    let op = GenerateECDSAKeyPairOp;
    let args = [
        ArgValue::Str("P-256".to_string()),
        ArgValue::Str("PEM".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("BEGIN PUBLIC KEY"));
    assert!(result_str.contains("BEGIN PRIVATE KEY"));
}
#[test]
fn test_generate_ecdsa_key_pair_p384() {
    let op = GenerateECDSAKeyPairOp;
    let args = [
        ArgValue::Str("P-384".to_string()),
        ArgValue::Str("DER".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}

#[test]
fn test_generated_p256_key_is_accepted_by_ring_validator() {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use ring::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_ASN1};
    use rxchef::operations::ecdsa_sign::ECDSASign;

    let args = [
        ArgValue::Str("P-256".to_string()),
        ArgValue::Str("PEM".to_string()),
    ];
    let output = String::from_utf8(GenerateECDSAKeyPairOp.run(Vec::new(), &args).unwrap()).unwrap();
    let public_pem = pem_block(&output, "PUBLIC KEY");
    let private_pem = pem_block(&output, "PRIVATE KEY");
    let message = b"independent ECDSA key-generation validation";
    let signature_hex = ECDSASign
        .run(
            message.to_vec(),
            &[
                ArgValue::Str(private_pem),
                ArgValue::Str("SHA-256".to_string()),
                ArgValue::Str("ASN.1 HEX".to_string()),
            ],
        )
        .unwrap();
    let spki = STANDARD
        .decode(
            public_pem
                .lines()
                .filter(|line| !line.starts_with("-----"))
                .collect::<String>(),
        )
        .unwrap();
    let sec1_public_key = &spki[spki.len() - 65..];
    let signature = hex::decode(signature_hex).unwrap();

    assert_eq!(
        UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, sec1_public_key)
            .verify(message, &signature),
        Ok(())
    );
}

fn pem_block(text: &str, label: &str) -> String {
    let begin = format!("-----BEGIN {label}-----");
    let end = format!("-----END {label}-----");
    let start = text.find(&begin).unwrap();
    let finish = text[start..].find(&end).unwrap() + start + end.len();
    format!("{}\n", &text[start..finish])
}
