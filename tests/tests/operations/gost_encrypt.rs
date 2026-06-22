// Tests for the gost_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations gost_encrypt::

use rxchef::operations::gost_encrypt::GostEncrypt;
use rxchef::Operation;

#[test]
fn test_gost_encrypt_empty_input() {
    let op = GostEncrypt;
    let args = [
        rxchef::operation::ArgValue::Bytes(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]), // 32-byte key as bytes
        rxchef::operation::ArgValue::Str("".to_string()), // Empty IV
        rxchef::operation::ArgValue::Str("Raw".to_string()),
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("GOST 28147 (1989)".to_string()),
        rxchef::operation::ArgValue::Str("E-TEST".to_string()),
        rxchef::operation::ArgValue::Str("ECB".to_string()),
        rxchef::operation::ArgValue::Str("NO".to_string()),
        rxchef::operation::ArgValue::Str("None".to_string()), // No padding for empty input
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_gost_encrypt_simple_ecb() {
    let op = GostEncrypt;
    let args = [
        rxchef::operation::ArgValue::Bytes(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]), // 32-byte key as bytes
        rxchef::operation::ArgValue::Str("".to_string()), // Empty IV
        rxchef::operation::ArgValue::Str("Raw".to_string()),
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("GOST 28147 (1989)".to_string()),
        rxchef::operation::ArgValue::Str("E-TEST".to_string()),
        rxchef::operation::ArgValue::Str("ECB".to_string()),
        rxchef::operation::ArgValue::Str("NO".to_string()),
        rxchef::operation::ArgValue::Str("None".to_string()), // No padding
    ];
    // 8-byte input (1 block) - must be exact multiple of block size
    let input = b"hello123"; // 8 bytes
    let result = op.run(input.to_vec(), &args);
    if let Err(e) = &result {
        eprintln!("Error in simple_ecb: {:?}", e);
    }
    assert!(result.is_ok());
    let encrypted = result.unwrap();
    // Should be hex encoded
    assert!(String::from_utf8_lossy(&encrypted).chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_gost_encrypt_kuznyechik() {
    let op = GostEncrypt;
    let args = [
        rxchef::operation::ArgValue::Bytes(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]), // 32-byte key as bytes
        rxchef::operation::ArgValue::Str("".to_string()), // Empty IV
        rxchef::operation::ArgValue::Str("Raw".to_string()),
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("GOST R 34.12 (Kuznyechik, 2015)".to_string()),
        rxchef::operation::ArgValue::Str("E-TEST".to_string()),
        rxchef::operation::ArgValue::Str("ECB".to_string()),
        rxchef::operation::ArgValue::Str("NO".to_string()),
        rxchef::operation::ArgValue::Str("None".to_string()), // No padding
    ];
    // 16-byte input (1 block for Kuznyechik) - Kuznyechik uses 16-byte blocks
    let input = b"hello world 1234"; // Exactly 16 bytes
    let result = op.run(input.to_vec(), &args);
    if let Err(e) = &result {
        eprintln!("Error in kuznyechik: {:?}", e);
    }
    assert!(result.is_ok());
    let encrypted = result.unwrap();
    // Should be hex encoded
    assert!(!encrypted.is_empty());
}

#[test]
fn test_gost_encrypt_invalid_key_length() {
    let op = GostEncrypt;
    let args = [
        rxchef::operation::ArgValue::Str("short".to_string()), // Too short key
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("GOST 28147 (1989)".to_string()),
        rxchef::operation::ArgValue::Str("E-TEST".to_string()),
        rxchef::operation::ArgValue::Str("ECB".to_string()),
        rxchef::operation::ArgValue::Str("NO".to_string()),
        rxchef::operation::ArgValue::Str("PKCS5".to_string()),
    ];
    let input = b"test";
    let result = op.run(input.to_vec(), &args);
    // Should fail due to invalid key length
    assert!(result.is_err());
}

#[test]
fn test_gost_encrypt_hex_input() {
    let op = GostEncrypt;
    let args = [
        rxchef::operation::ArgValue::Bytes(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]), // 32-byte key as bytes
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Hex input
        rxchef::operation::ArgValue::Str("Hex".to_string()),
        rxchef::operation::ArgValue::Str("GOST 28147 (1989)".to_string()),
        rxchef::operation::ArgValue::Str("E-TEST".to_string()),
        rxchef::operation::ArgValue::Str("ECB".to_string()),
        rxchef::operation::ArgValue::Str("NO".to_string()),
        rxchef::operation::ArgValue::Str("None".to_string()), // No padding
    ];
    // Hex input - must be exact multiple of block size (8 bytes = 16 hex chars)
    let input = "68656c6c6f313233"; // "hello123" in hex, 16 chars = 8 bytes
    let result = op.run(input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let encrypted = result.unwrap();
    assert!(!encrypted.is_empty());
}
