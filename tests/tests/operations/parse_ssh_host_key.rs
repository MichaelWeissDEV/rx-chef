// Tests for the parse_ssh_host_key operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_ssh_host_key::

use base64::{engine::general_purpose, Engine};
use rxchef::operation::ArgValue;
use rxchef::operations::parse_ssh_host_key::ParseSshHostKey;
use rxchef::Operation;

#[test]
fn test_parse_ssh_host_key_rsa() {
    let op = ParseSshHostKey;
    // ssh-rsa followed by length-prefixed "ssh-rsa", "exponent", "modulus"
    // "ssh-rsa" = 7 chars. len = 7.
    let mut key = vec![0, 0, 0, 7];
    key.extend_from_slice(b"ssh-rsa");
    key.extend_from_slice(&[0, 0, 0, 1, 0x01]); // exponent
    key.extend_from_slice(&[0, 0, 0, 1, 0x02]); // modulus
    let b64 = general_purpose::STANDARD.encode(&key);
    let input = format!("ssh-rsa {}", b64).into_bytes();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Key type: ssh-rsa"));
    assert!(result_str.contains("Exponent: 0x01"));
    assert!(result_str.contains("Modulus: 0x02"));
}
#[test]
fn test_parse_ssh_host_key_ed25519() {
    let op = ParseSshHostKey;
    let mut key = vec![0, 0, 0, 11];
    key.extend_from_slice(b"ssh-ed25519");
    key.extend_from_slice(&[0, 0, 0, 4, 1, 2, 3, 4]); // x
    let b64 = general_purpose::STANDARD.encode(&key);
    let input = b64.into_bytes();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Key type: ssh-ed25519"));
    assert!(result_str.contains("x: 0x01020304"));
}
#[test]
fn test_parse_ssh_host_key_hex() {
    let op = ParseSshHostKey;
    let mut key = vec![0, 0, 0, 7];
    key.extend_from_slice(b"ssh-rsa");
    key.extend_from_slice(&[0, 0, 0, 1, 0x11]);
    key.extend_from_slice(&[0, 0, 0, 1, 0x22]);
    let hex_str = hex::encode(&key);
    let args = [ArgValue::Str("Hex".to_string())];
    let result = op.run(hex_str.into_bytes(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Key type: ssh-rsa"));
    assert!(result_str.contains("Exponent: 0x11"));
    assert!(result_str.contains("Modulus: 0x22"));
}
