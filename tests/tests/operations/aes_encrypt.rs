// Tests for the aes_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations aes_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::aes_decrypt::AesDecrypt;
use rxchef::operations::aes_encrypt::AesEncrypt;
use rxchef::Operation;

#[test]
fn test_aes_encrypt_invalid_key_length() {
    let op = AesEncrypt;
    let input = b"Hello, World!".to_vec();
    let args = [
        ArgValue::Str("01234".to_string()), // 5 bytes - invalid
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_aes_encrypt_roundtrip_128bit_key() {
    let op = AesEncrypt;
    let decrypt_op = AesDecrypt;
    let input = b"Test message for AES encryption".to_vec();
    let key = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap(); // 16 bytes
    let iv = hex::decode("00112233445566778899aabbccddeeff").unwrap(); // 16 bytes
                                                                       // Encrypt
    let args = [
        ArgValue::Bytes(key.clone()),
        ArgValue::Bytes(iv.clone()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("".to_string()),
    ];
    let encrypted = op.run(input.clone(), &args).unwrap();
    // Decrypt
    let decrypt_args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
    ];
    let decrypted = decrypt_op.run(encrypted, &decrypt_args).unwrap();
    assert_eq!(input, decrypted);
}
#[test]
fn test_aes_encrypt_roundtrip_256bit_key() {
    let op = AesEncrypt;
    let decrypt_op = AesDecrypt;
    let input = b"Another test with 256-bit key".to_vec();
    let key =
        hex::decode("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").unwrap(); // 32 bytes
                                                                                                  // Encrypt with default IV
    let args = [
        ArgValue::Bytes(key.clone()),
        ArgValue::Str("".to_string()), // Empty IV (will be null)
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("".to_string()),
    ];
    let encrypted = op.run(input.clone(), &args).unwrap();
    // Decrypt
    let decrypt_args = [
        ArgValue::Bytes(key),
        ArgValue::Str("".to_string()), // Empty IV (will be null)
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
    ];
    let decrypted = decrypt_op.run(encrypted, &decrypt_args).unwrap();
    assert_eq!(input, decrypted);
}
#[test]
fn test_aes_encrypt_no_padding() {
    let op = AesEncrypt;
    let input = b"Hello, World!12345".to_vec(); // 16 bytes exactly
    let key = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap(); // 16 bytes
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Str("".to_string()),              // IV
        ArgValue::Str("CBC/NoPadding".to_string()), // Mode with no padding
        ArgValue::Str("Raw".to_string()),           // Input
        ArgValue::Str("Hex".to_string()),           // Output
        ArgValue::Str("".to_string()),              // AAD
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}

#[test]
fn test_aes_all_advertised_modes_roundtrip() {
    let key = ArgValue::Bytes(b"0123456789abcdef".to_vec());
    let iv = ArgValue::Bytes(b"fedcba9876543210".to_vec());
    for mode in ["CFB", "OFB", "CTR", "ECB"] {
        let input = b"AES mode roundtrip with a partial final block".to_vec();
        let encrypted = AesEncrypt
            .run(
                input.clone(),
                &[
                    key.clone(),
                    iv.clone(),
                    ArgValue::Str(mode.to_string()),
                    ArgValue::Str("Raw".to_string()),
                    ArgValue::Str("Raw".to_string()),
                ],
            )
            .unwrap();
        let decrypted = AesDecrypt
            .run(
                encrypted,
                &[
                    key.clone(),
                    iv.clone(),
                    ArgValue::Str(mode.to_string()),
                    ArgValue::Str("Raw".to_string()),
                    ArgValue::Str("Raw".to_string()),
                ],
            )
            .unwrap();
        assert_eq!(decrypted, input, "failed {mode} roundtrip");
    }
}

#[test]
fn test_aes_gcm_roundtrip_with_aad() {
    let key = ArgValue::Bytes(b"0123456789abcdef".to_vec());
    let iv = ArgValue::Bytes(b"unique nonce".to_vec());
    let aad = ArgValue::Bytes(b"metadata".to_vec());
    let plaintext = b"authenticated plaintext".to_vec();
    let encrypted = AesEncrypt
        .run(
            plaintext.clone(),
            &[
                key.clone(),
                iv.clone(),
                ArgValue::Str("GCM".to_string()),
                ArgValue::Str("Raw".to_string()),
                ArgValue::Str("Raw".to_string()),
                aad.clone(),
            ],
        )
        .unwrap();
    let split = encrypted.len() - 16;
    let decrypted = AesDecrypt
        .run(
            encrypted[..split].to_vec(),
            &[
                key,
                iv,
                ArgValue::Str("GCM".to_string()),
                ArgValue::Str("Raw".to_string()),
                ArgValue::Str("Raw".to_string()),
                ArgValue::Bytes(encrypted[split..].to_vec()),
                aad,
            ],
        )
        .unwrap();
    assert_eq!(decrypted, plaintext);
}
