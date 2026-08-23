// Tests for the hassh_server_fingerprint operation.

use rxchef::operation::ArgValue;
use rxchef::operations::hassh_server_fingerprint::HASSHServerFingerprint;
use rxchef::Operation;

fn symmetric_kexinit_hex() -> String {
    // Symmetric client/server lists make the expected HASSH-server canonical
    // string easy to derive independently from the Salesforce HASSH format.
    let lists = [
        "curve25519-sha256",
        "ssh-ed25519",
        "aes128-ctr",
        "aes128-ctr",
        "hmac-sha2-256",
        "hmac-sha2-256",
        "none",
        "none",
    ];
    let mut payload = vec![20];
    payload.extend_from_slice(&[0; 16]);
    for list in lists {
        payload.extend_from_slice(&(list.len() as u32).to_be_bytes());
        payload.extend_from_slice(list.as_bytes());
    }
    let padding = [0_u8; 4];
    let packet_length = 1 + payload.len() + padding.len();
    let mut packet = (packet_length as u32).to_be_bytes().to_vec();
    packet.push(padding.len() as u8);
    packet.extend_from_slice(&payload);
    packet.extend_from_slice(&padding);
    hex::encode(packet)
}

#[test]
fn test_hassh_server_canonical_string_and_digest() {
    let packet = symmetric_kexinit_hex().into_bytes();
    let canonical = HASSHServerFingerprint
        .run(
            packet.clone(),
            &[
                ArgValue::Str("Hex".into()),
                ArgValue::Str("HASSH algorithms string".into()),
            ],
        )
        .unwrap();
    assert_eq!(canonical, b"curve25519-sha256;aes128-ctr;hmac-sha2-256;none");

    let digest = HASSHServerFingerprint
        .run(
            packet,
            &[
                ArgValue::Str("Hex".into()),
                ArgValue::Str("Hash digest".into()),
            ],
        )
        .unwrap();
    assert_eq!(digest, b"e97d07603350d1111ec2b64bf25413c9");
}
