// Tests for the ja3_fingerprint operation.

use rxchef::operation::ArgValue;
use rxchef::operations::ja3_fingerprint::JA3Fingerprint;
use rxchef::Operation;

#[test]
fn test_ja3_tls_1_0_reference_packet() {
    // Exact CyberChef 11.4.0 reference packet/result at commit 2e048b029085;
    // the digest is also the upstream JA3 regression vector.
    let packet = "16030100a4010000a00301543dd2dd48f517ca9a93b1e599f019fdece704a23e86c1dcac588427abbaddf200005cc014c00a0039003800880087c00fc00500350084c012c00800160013c00dc003000ac013c00900330032009a009900450044c00ec004002f009600410007c011c007c00cc002000500040015001200090014001100080006000300ff0100001b000b000403000102000a000600040018001700230000000f000101";
    let output = JA3Fingerprint
        .run(
            packet.as_bytes().to_vec(),
            &[
                ArgValue::Str("Hex".into()),
                ArgValue::Str("Hash digest".into()),
            ],
        )
        .unwrap();
    assert_eq!(output, b"503053a0c5b2bd9b9334bf7f3d3b8852");
}
