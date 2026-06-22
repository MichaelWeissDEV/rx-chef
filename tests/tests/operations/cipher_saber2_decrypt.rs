// Tests for the cipher_saber2_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations cipher_saber2_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::cipher_saber2_decrypt::CipherSaber2Decrypt;
use rxchef::Operation;

/// RC4-based CipherSaber2 encrypt/decrypt (symmetric).
fn ciphersaber2_crypt(iv: &[u8], key: &[u8], rounds: usize, data: &[u8]) -> Vec<u8> {
    let combined: Vec<u8> = key.iter().chain(iv.iter()).copied().collect();
    let combined_len = combined.len();

    let mut state: Vec<u8> = (0u8..=255).collect();
    let mut j: usize = 0;

    for _ in 0..rounds {
        for k in 0usize..256 {
            j = (j + state[k] as usize + combined[k % combined_len] as usize) % 256;
            state.swap(k, j);
        }
    }

    let mut i: usize = 0;
    j = 0;
    let mut output = Vec::with_capacity(data.len());

    for &byte in data {
        i = (i + 1) % 256;
        j = (j + state[i] as usize) % 256;
        state.swap(i, j);
        let n = (state[i] as usize + state[j] as usize) % 256;
        output.push(state[n] ^ byte);
    }

    output
}

#[test]
fn test_decrypt_too_short_input() {
    let op = CipherSaber2Decrypt;
    let args = [ArgValue::Str("key".to_string()), ArgValue::Num(20.0)];
    let result = op.run(vec![0u8; 5], &args);
    assert!(result.is_err());
}
#[test]
fn test_encrypt_decrypt_roundtrip() {
    let key = b"testkey";
    let iv = [0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
    let plaintext = b"Hello, CipherSaber2!";
    let rounds = 20;
    let encrypted = ciphersaber2_crypt(&iv, key, rounds, plaintext);
    let decrypted = ciphersaber2_crypt(&iv, key, rounds, &encrypted);
    assert_eq!(decrypted, plaintext);
}
