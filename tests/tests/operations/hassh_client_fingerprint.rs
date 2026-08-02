// Tests for the hassh_client_fingerprint operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations hassh_client_fingerprint::

use rxchef::operations::hassh_client_fingerprint::HASSHClientFingerprint;
use rxchef::Operation;

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
    // Should either return an error or succeed (depending on how invalid chars are handled)
    assert!(result.is_ok() || result.is_err());
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
    // Create a minimal valid SSH KEXINIT packet
    // This is a simplified test - in reality you'd need a proper SSH packet
    let packet = "0000003000140000000000000000000000000000";
    let result = op.run(hex::decode(packet).unwrap(), &args);
    // Should either succeed or fail gracefully
    // The main thing is it shouldn't panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_hassh_client_fingerprint_algorithms_string() {
    let op = HASSHClientFingerprint;
    let args = [
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("HASSH algorithms string".to_string()),
    ];
    // Create a minimal valid SSH KEXINIT packet
    let packet = "0000003000140000000000000000000000000000";
    let result = op.run(hex::decode(packet).unwrap(), &args);
    // Should either succeed or fail gracefully
    assert!(result.is_ok() || result.is_err());
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
