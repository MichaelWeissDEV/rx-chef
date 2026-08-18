// Tests for the pem_to_jwk operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations pem_to_jwk::

use rxchef::operations::pem_to_jwk::PEMToJWK;
use rxchef::Operation;

#[test]
fn test_pem_to_jwk_rsa() {
    let op = PEMToJWK;
    let input = b"-----BEGIN PUBLIC KEY-----\n\
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA5WykLKHiBAhmZh5Whocg\n\
pQQqZjdrApuRxRT21SJZx6Ce+Oz2V17/heozu5LEz63jCxW1NrBckzl/Ys8p9Leq\n\
YTu6x/LbKloTjfEWxlzXnzUSqn9JHIxmmJQzjXp9X1D99Tj+NWpRGEIiCFE7JfDh\n\
e2KnMGVDDg6kfCLokDdLo256LeQ4CEkViwY6d+at4xDlIHwvZZmG4Smk56eHhvQE\n\
3I8sSAzgoLMBamQ5m3MbiULAYtxskCpCfjFxrL6Ziaaj7HZoneF40R30KCI9ygF8\n\
vkzxLwe3t5Y4XgHL9TYQm1+BDninupIB/zTeO1ygBGA66m6zpmkmuG7d8HXIducz\n\
+wIDAQAB\n\
-----END PUBLIC KEY-----"
        .to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("\"kty\":\"RSA\""));
    assert!(result_str.contains("\"n\":"));
    assert!(result_str.contains("\"e\":"));
}
