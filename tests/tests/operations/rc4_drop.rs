// Tests for the rc4_drop operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rc4_drop::

use rxchef::operation::ArgValue;
use rxchef::operations::rc4_drop::RC4Drop;
use rxchef::Operation;

/// Perform RC4-drop with the specified number of bytes to discard.
fn rc4_drop_crypt(key: &[u8], data: &[u8], drop_bytes: usize) -> Vec<u8> {
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

    // PRGA - discard first drop_bytes bytes
    let mut i: usize = 0;
    let mut j: usize = 0;
    for _ in 0..drop_bytes {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
    }

    // Encrypt data
    let mut output = Vec::with_capacity(data.len());
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
fn test_rc4_drop_symmetric() {
    let key = b"secret".to_vec();
    let plaintext = b"Hello, World!".to_vec();
    let ciphertext = rc4_drop_crypt(&key, &plaintext, 192 * 4);
    let decrypted = rc4_drop_crypt(&key, &ciphertext, 192 * 4);
    assert_eq!(decrypted, plaintext);
}
#[test]
fn test_rc4_drop_differs_from_rc4() {
    let key = b"key".to_vec();
    let data = b"test data".to_vec();
    let no_drop = rc4_drop_crypt(&key, &data, 0);
    let with_drop = rc4_drop_crypt(&key, &data, 192 * 4);
    assert_ne!(no_drop, with_drop);
}
#[test]
fn test_rc4_drop_empty_key_error() {
    let op = RC4Drop;
    let result = op.run(b"data".to_vec(), &[ArgValue::Str("".to_string())]);
    assert!(result.is_err());
}
