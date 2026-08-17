// Tests for the blowfish_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations blowfish_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::blowfish_decrypt::BlowfishDecrypt;
use rxchef::runtime;
use rxchef::Operation;

/// Encrypt through the runtime so the ciphertext really is what this
/// operation is expected to consume, rather than an arbitrary byte string.
fn encrypt_hex(plaintext: &[u8], key_hex: &str, mode: &str) -> Vec<u8> {
    runtime::run_operation(
        "Blowfish Encrypt",
        plaintext.to_vec(),
        &[
            format!("hex:{key_hex}"),
            String::new(),
            mode.to_string(),
            "Raw".to_string(),
            "Hex".to_string(),
        ],
    )
    .expect("encrypting the fixture plaintext must succeed")
}

#[test]
fn test_blowfish_decrypt_invalid_key_length() {
    let op = BlowfishDecrypt;
    let args = [
        ArgValue::Str("0123456789abcdef0123456789abcdef".to_string()), // 32 bytes - valid
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    // A valid key must decrypt a genuinely padded ciphertext back to its
    // plaintext. The previous version decrypted an empty buffer and asserted
    // only `is_ok()`, which held even while ECB left PKCS#7 padding in place.
    let ciphertext = encrypt_hex(b"round trip", "0123456789abcdef0123456789abcdef", "ECB");
    let args_hex = [
        ArgValue::Str("0123456789abcdef0123456789abcdef".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let recovered = op
        .run(ciphertext, &args_hex)
        .expect("valid key must decrypt");
    assert_eq!(recovered, b"round trip");
    let _ = &args;
    // 3 bytes should be invalid
    let args = [
        ArgValue::Str("012".to_string()), // 3 bytes - invalid
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err(), "Invalid key should return error");
}
#[test]
fn test_blowfish_decrypt_invalid_iv_length() {
    let op = BlowfishDecrypt;
    let args = [
        ArgValue::Str("0123456789abcdef0123456789abcdef".to_string()), // 16 bytes - valid
        ArgValue::Str("0123456789".to_string()),                       // 10 bytes - invalid
        ArgValue::Str("CBC".to_string()), // CBC mode requires 8-byte IV
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err(), "Invalid IV should return error");
}
#[test]
fn test_blowfish_decrypt_ecb_mode() {
    let op = BlowfishDecrypt;
    // Key: 0x0000000000000000 (8 bytes)
    // Using hex-encoded ciphertext as input with "Hex" input type
    let key = hex::decode("0000000000000000").unwrap();
    // Encrypting through the encrypt operation guarantees valid PKCS#7
    // padding; an arbitrary 8-byte block would decrypt to bytes whose last
    // value is not a valid pad length, which upstream also rejects.
    let ciphertext_hex = String::from_utf8(encrypt_hex(b"ecb", "0000000000000000", "ECB")).unwrap();
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Str("".to_string()), // Empty IV (will be null)
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let recovered = op
        .run(ciphertext_hex.as_bytes().to_vec(), &args)
        .expect("ECB decrypt failed");
    assert_eq!(recovered, b"ecb", "ECB must strip PKCS#7 padding");
}
#[test]
fn test_blowfish_decrypt_cbc_mode() {
    let encrypt_op = rxchef::operations::blowfish_encrypt::BlowfishEncrypt;
    let decrypt_op = BlowfishDecrypt;
    let key = hex::decode("0123456789abcdef0123456789abcdef").unwrap(); // 16 bytes
    let iv = hex::decode("0000000000000000").unwrap(); // 8 bytes
                                                       // Generate valid CBC-encrypted data using the encrypt operation
    let plaintext = b"Test data for CBC";
    let encrypt_args = [
        ArgValue::Bytes(key.clone()),
        ArgValue::Bytes(iv.clone()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let encrypted = encrypt_op.run(plaintext.to_vec(), &encrypt_args).unwrap();
    let encrypted_hex = String::from_utf8_lossy(&encrypted);
    // Now decrypt it back
    let decrypt_args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = decrypt_op.run(encrypted_hex.as_bytes().to_vec(), &decrypt_args);
    assert!(result.is_ok(), "CBC decrypt failed: {:?}", result.err());
    // Verify the decrypted data matches the original
    let decrypted = result.unwrap();
    assert_eq!(decrypted, plaintext.to_vec());
}
#[test]
fn test_blowfish_decrypt_key_formats() {
    let op = BlowfishDecrypt;
    let key_hex = "0123456789abcdef0123456789abcdef";
    // Test with hex input
    let args = [
        ArgValue::Str(key_hex.to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    // A hex key string and the equivalent byte key must decrypt identically.
    let ciphertext = encrypt_hex(b"key formats", key_hex, "ECB");
    let from_hex_string = op
        .run(ciphertext.clone(), &args)
        .expect("hex key string must decrypt");
    let args_bytes = [
        ArgValue::Bytes(hex::decode(key_hex).unwrap()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let from_bytes = op
        .run(ciphertext, &args_bytes)
        .expect("byte key must decrypt");
    assert_eq!(from_hex_string, b"key formats");
    assert_eq!(from_hex_string, from_bytes);
}
#[test]
fn test_blowfish_decrypt_output_formats() {
    let op = BlowfishDecrypt;
    let key = hex::decode("0123456789abcdef0123456789abcdef").unwrap();
    // Test with Hex output - using valid padded ECB ciphertext
    let args = [
        ArgValue::Bytes(key.clone()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()), // Hex output
    ];
    let ciphertext = encrypt_hex(b"formats", "0123456789abcdef0123456789abcdef", "ECB");
    let as_hex = op
        .run(ciphertext.clone(), &args)
        .expect("hex output must decrypt");
    assert_eq!(String::from_utf8(as_hex).unwrap(), hex::encode(b"formats"));
    // Test with Raw output
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()), // Raw output
    ];
    let as_raw = op.run(ciphertext, &args).expect("raw output must decrypt");
    assert_eq!(as_raw, b"formats");
}
