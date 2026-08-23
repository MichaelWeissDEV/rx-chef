// Tests for the ja3s_fingerprint operation.

use rxchef::operation::ArgValue;
use rxchef::operations::ja3s_fingerprint::JA3SFingerprint;
use rxchef::Operation;

#[test]
fn test_ja3s_tls_1_0_reference_packet() {
    // Exact CyberChef 11.4.0 reference packet/result at commit 2e048b029085;
    // the digest is also the upstream JA3S regression vector.
    let packet = "160301003d020000390301543dd2ddedbfe33895bd6bc676a3fa6b9fe5773a6e04d5476d1af3bcbc1dcbbb00c011000011ff01000100000b00040300010200230000";
    let output = JA3SFingerprint
        .run(
            packet.as_bytes().to_vec(),
            &[
                ArgValue::Str("Hex".into()),
                ArgValue::Str("Hash digest".into()),
            ],
        )
        .unwrap();
    assert_eq!(output, b"bed95e1b525d2f41db3a6d68fac5b566");
}
