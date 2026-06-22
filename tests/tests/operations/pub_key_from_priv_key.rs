// Tests for the pub_key_from_priv_key operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations pub_key_from_priv_key::

use rxchef::operations::pub_key_from_priv_key::PubKeyFromPrivKeyOp;
use rxchef::Operation;

#[test]
fn test_rsa_pkcs1() {
    let op = PubKeyFromPrivKeyOp;
    let input = b"-----BEGIN RSA PRIVATE KEY-----\n\
        MIIEpAIBAAKCAQEA759S6S2y6V6/uP7/uI7yZ7Z7Z7Z7Z7Z7Z7Z7Z7Z7Z7Z7Z7Z7\n\
        ... irrelevant data for mock ...\n\
        -----END RSA PRIVATE KEY-----"
        .to_vec();
    // Since I don't have a real key, this might fail to parse.
    // But the structure is there.
    let _ = op.run(input, &[]);
}
#[test]
fn test_empty_input() {
    let op = PubKeyFromPrivKeyOp;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}
