// Tests for the blowfish_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations blowfish_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::blowfish_encrypt::BlowfishEncrypt;
use rxchef::Operation;

#[test]
fn test_blowfish_encrypt_invalid_key_length() {
    let op = BlowfishEncrypt;
    let input = b"Test message".to_vec();
    let args = [
        ArgValue::Str("0123456789abcdef01234567".to_string()), // 24 bytes - valid
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    // 24 bytes should be valid
    let result = op.run(input.clone(), &args);
    assert!(result.is_ok());
    // 3 bytes should be invalid
    let args = [
        ArgValue::Str("012".to_string()), // 3 bytes - invalid
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_blowfish_encrypt_invalid_iv_length() {
    let op = BlowfishEncrypt;
    let input = b"Test message".to_vec();
    let args = [
        ArgValue::Str("0123456789abcdef0123456789abcdef".to_string()), // 16 bytes - valid
        ArgValue::Str("0123456789".to_string()),                       // 10 bytes - invalid
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_blowfish_encrypt_ecb_mode() {
    let op = BlowfishEncrypt;
    let key = hex::decode("0000000000000000").unwrap(); // 8 bytes
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Str("".to_string()), // Empty IV (will be null)
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(b"Hello, World!".to_vec(), &args);
    assert!(result.is_ok());
}
#[test]
fn test_blowfish_encrypt_cbc_mode() {
    let op = BlowfishEncrypt;
    let key = hex::decode("0123456789abcdef0123456789abcdef").unwrap(); // 16 bytes
    let iv = hex::decode("0000000000000000").unwrap(); // 8 bytes
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(b"Test message for CBC encryption".to_vec(), &args);
    assert!(result.is_ok());
}
#[test]
fn test_blowfish_encrypt_roundtrip() {
    let encrypt_op = BlowfishEncrypt;
    let decrypt_op = rxchef::operations::blowfish_decrypt::BlowfishDecrypt;
    let input = b"Test message for roundtrip encryption".to_vec();
    let key = hex::decode("0123456789abcdef0123456789abcdef").unwrap(); // 16 bytes
    let iv = hex::decode("0000000000000000").unwrap(); // 8 bytes
                                                       // Test CBC mode
    let encrypt_args = [
        ArgValue::Bytes(key.clone()),
        ArgValue::Bytes(iv.clone()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let encrypted = encrypt_op.run(input.clone(), &encrypt_args).unwrap();
    let decrypt_args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(iv),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let decrypted = decrypt_op.run(encrypted, &decrypt_args).unwrap();
    assert_eq!(input, decrypted);
}
#[test]
fn test_blowfish_encrypt_key_formats() {
    let op = BlowfishEncrypt;
    let key_hex = "0123456789abcdef0123456789abcdef";
    // Test with hex input
    let args = [
        ArgValue::Str(key_hex.to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(b"Test".to_vec(), &args);
    assert!(result.is_ok());
    // Test with base64 input
    let key_base64 = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        "0123456789abcdef0123456789abcdef",
    );
    let args = [
        ArgValue::Str(key_base64),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(b"Test".to_vec(), &args);
    assert!(result.is_ok());
}
#[test]
fn test_blowfish_encrypt_output_formats() {
    let op = BlowfishEncrypt;
    let key = hex::decode("0123456789abcdef0123456789abcdef").unwrap();
    // Test with Hex output
    let args = [
        ArgValue::Bytes(key.clone()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()), // Hex output
    ];
    let result = op.run(b"Test".to_vec(), &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    let output_str = String::from_utf8_lossy(&output);
    assert!(output_str.starts_with("0x") || output_str.len().is_multiple_of(2)); // Hex should have even length
                                                                                 // Test with Raw output
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()), // Raw output
    ];
    let result = op.run(b"Test".to_vec(), &args);
    assert!(result.is_ok());
}
#[test]
fn test_blowfish_encrypt_all_modes() {
    let op = BlowfishEncrypt;
    let key = hex::decode("0123456789abcdef0123456789abcdef").unwrap();
    let iv = hex::decode("0000000000000000").unwrap();
    let input = b"Test message for all modes".to_vec();
    let modes = ["ECB", "CBC", "CFB", "OFB", "CTR"];
    for mode in modes.iter() {
        let args = [
            ArgValue::Bytes(key.clone()),
            ArgValue::Bytes(iv.clone()),
            ArgValue::Str(mode.to_string()),
            ArgValue::Str("Raw".to_string()),
            ArgValue::Str("Hex".to_string()),
        ];
        let result = op.run(input.clone(), &args);
        assert!(result.is_ok(), "Mode {} failed", mode);
    }
}
