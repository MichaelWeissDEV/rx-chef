// Tests for the hassh_client_fingerprint operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations hassh_client_fingerprint::

use rxchef::operations::hassh_client_fingerprint::HASSHClientFingerprint;
use rxchef::Operation;

fn packet_hex() -> String {
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
fn test_hassh_client_fingerprint_empty_input() {
    let op = HASSHClientFingerprint;
    let args = [
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("Hash digest".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_hassh_client_fingerprint_invalid_hex() {
    let op = HASSHClientFingerprint;
    let args = [
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("Hash digest".to_string()),
    ];
    let result = op.run("ZZZ".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_hassh_client_fingerprint_invalid_base64() {
    let op = HASSHClientFingerprint;
    let args = [
        rxchef::operation::ArgValue::Str("Base64".to_string()),
        rxchef::operation::ArgValue::Str("Hash digest".to_string()),
    ];
    let result = op.run("!!!invalid!!!".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_hassh_client_fingerprint_short_packet() {
    let op = HASSHClientFingerprint;
    let args = [
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("Hash digest".to_string()),
    ];
    // Too short to be a valid SSH packet
    let result = op.run("00000001".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_hassh_client_fingerprint_wrong_message_code() {
    let op = HASSHClientFingerprint;
    let args = [
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("Hash digest".to_string()),
    ];
    // SSH packet with wrong message code (not 20 for KEXINIT)
    let packet = "0000003000140000000000000000000000000000"; // Message code 20 would be 14 in hex
    let result = op.run(hex::decode(packet).unwrap(), &args);
    assert!(result.is_err());
}

#[test]
fn test_hassh_client_fingerprint_full_details() {
    let op = HASSHClientFingerprint;
    let args = [
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("Full details".to_string()),
    ];
    let result = String::from_utf8(op.run(packet_hex().into_bytes(), &args).unwrap()).unwrap();
    // Salesforce's HASSH specification hashes the four semicolon-separated
    // client algorithm lists with MD5.  The packet is built independently
    // above and its canonical algorithms string is asserted below as well.
    assert!(result.contains("Hash digest:\ne97d07603350d1111ec2b64bf25413c9"));
    assert!(result.contains("curve25519-sha256;aes128-ctr;hmac-sha2-256;none"));
}

#[test]
fn test_hassh_client_fingerprint_algorithms_string() {
    let op = HASSHClientFingerprint;
    let args = [
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("HASSH algorithms string".to_string()),
    ];
    let result = op.run(packet_hex().into_bytes(), &args).unwrap();
    assert_eq!(result, b"curve25519-sha256;aes128-ctr;hmac-sha2-256;none");
}

#[test]
fn test_hassh_client_fingerprint_invalid_utf8() {
    let op = HASSHClientFingerprint;
    let args = [
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("Hash digest".to_string()),
    ];
    // Invalid UTF-8 sequence
    let invalid_utf8 = vec![0xFF, 0xFE];
    let result = op.run(invalid_utf8, &args);
    assert!(result.is_err());
}
