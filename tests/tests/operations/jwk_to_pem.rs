// Tests for the jwk_to_pem operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations jwk_to_pem::

use rxchef::operations::jwk_to_pem::JWKToPem;
use rxchef::Operation;

#[test]
fn test_jwk_to_pem_rsa_public_reference_key() {
    // Fixed RSA-512 public JWK/SubjectPublicKeyInfo pair from CyberChef's JWK
    // regression corpus at pinned commit 2e048b029085.
    let jwk = br#"{"kty":"RSA","n":"8qvQOnph0i3M5-TpruZrsvgEXgud6Uxgq1ugYuuTqKG2oU9kVEs1wmLrwe-e3yy0ys_nS3qOrBZDYSMx2SPp-w","e":"AQAB"}"#;
    let pem = JWKToPem.run(jwk.to_vec(), &[]).unwrap();
    assert_eq!(
        String::from_utf8(pem).unwrap(),
        "-----BEGIN PUBLIC KEY-----\n\
MFwwDQYJKoZIhvcNAQEBBQADSwAwSAJBAPKr0Dp6YdItzOfk6a7ma7L4BF4LnelM\n\
YKtboGLrk6ihtqFPZFRLNcJi68Hvnt8stMrP50t6jqwWQ2EjMdkj6fsCAwEAAQ==\n\
-----END PUBLIC KEY-----\n"
    );
}
