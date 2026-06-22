// Tests for the blowfish_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations blowfish_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::blowfish_decrypt::BlowfishDecrypt;
use rxchef::Operation;

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
    // 32 bytes should be valid
    let result = op.run(vec![], &args);
    assert!(result.is_ok(), "Valid key test failed: {:?}", result.err());
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
    let ciphertext_hex = "64ec88c00b37661d"; // Known test vector (hex-encoded)
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Str("".to_string()), // Empty IV (will be null)
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(ciphertext_hex.as_bytes().to_vec(), &args);
    assert!(result.is_ok(), "ECB decrypt failed: {:?}", result.err());
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
    let result = op.run(vec![], &args);
    // Should work (even with empty input, key parsing succeeds)
    assert!(result.is_ok());
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
    // Valid 8-byte ECB ciphertext (hex-encoded)
    let result = op.run("64ec88c00b37661d".as_bytes().to_vec(), &args);
    assert!(result.is_ok(), "Hex output test failed: {:?}", result.err());
    // Test with Raw output
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()), // Raw output
    ];
    let result = op.run("64ec88c00b37661d".as_bytes().to_vec(), &args);
    assert!(result.is_ok(), "Raw output test failed: {:?}", result.err());
}
