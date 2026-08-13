// Tests for the pub_key_from_cert operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations pub_key_from_cert::

use rxchef::operations::pub_key_from_cert::PubKeyFromCert;
use rxchef::Operation;

#[test]
fn test_pub_key_from_cert_empty_input() {
    let op = PubKeyFromCert;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_pub_key_from_cert_invalid() {
    let op = PubKeyFromCert;
    let input = b"-----BEGIN CERTIFICATE-----\nINVALID\n-----END CERTIFICATE-----".to_vec();
    let result = op.run(input, &[]);
    assert!(result.is_err());
}
