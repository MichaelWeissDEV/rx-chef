// Tests for the rc4 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rc4::

use rxchef::operation::ArgValue;
use rxchef::operations::rc4::RC4;
use rxchef::Operation;

/// Perform RC4 key setup and generate keystream XOR'd with plaintext.
fn rc4_crypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    // KSA
    let mut s: [u8; 256] = [0u8; 256];
    for i in 0..256usize {
        s[i] = i as u8;
    }
    let mut j: usize = 0;
    for i in 0..256usize {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
        s.swap(i, j);
    }

    // PRGA
    let mut output = Vec::with_capacity(data.len());
    let mut i: usize = 0;
    let mut j: usize = 0;
    for &byte in data {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k = s[(s[i] as usize + s[j] as usize) % 256];
        output.push(byte ^ k);
    }
    output
}

#[test]
fn test_rc4_basic() {
    // RC4("Key", "Plaintext") = BBF316E8D940AF0AD3
    let op = RC4;
    let input = b"Plaintext".to_vec();
    let args = [
        ArgValue::Str("Key".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let hex_out = String::from_utf8(result).unwrap();
    assert_eq!(hex_out.to_uppercase(), "BBF316E8D940AF0AD3");
}
#[test]
fn test_rc4_symmetric() {
    // Encrypting twice returns original plaintext
    let key = b"secret".to_vec();
    let plaintext = b"Hello, World!".to_vec();
    let ciphertext = rc4_crypt(&key, &plaintext);
    let decrypted = rc4_crypt(&key, &ciphertext);
    assert_eq!(decrypted, plaintext);
}
#[test]
fn test_rc4_empty_key_error() {
    let op = RC4;
    let result = op.run(b"data".to_vec(), &[ArgValue::Str("".to_string())]);
    assert!(result.is_err());
}
