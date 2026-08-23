// Tests for the ja4_server_fingerprint operation.

use rxchef::operation::ArgValue;
use rxchef::operations::ja4_server_fingerprint::JA4ServerFingerprint;
use rxchef::Operation;

#[test]
fn test_ja4s_cyberchef_tls12_h2_vector() {
    // Fixed ServerHello and expected JA4S from CyberChef's upstream JA4.mjs
    // operation test suite at commit 2e048b029085.
    let hello = b"16030300640200006003035f0236c07f47bfb12dc2da706ecb3fe7f9eeac9968cc2ddf444f574e4752440120b89ff1ab695278c69b8a73f76242ef755e0b13dc6d459aaaa784fec9c2dfce34cca900001800000000ff01000100000b00020100001000050003026832";
    let output = JA4ServerFingerprint
        .run(
            hello.to_vec(),
            &[ArgValue::Str("Hex".into()), ArgValue::Str("JA4S".into())],
        )
        .unwrap();
    assert_eq!(output, b"t1204h2_cca9_1428ce7b4018");
}
