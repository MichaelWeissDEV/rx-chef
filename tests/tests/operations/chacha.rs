// Tests for the chacha operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations chacha::

use rxchef::operations::chacha::ChaCha;
use rxchef::Operation;

fn run(input: &str, args: &[rxchef::operation::ArgValue]) -> String {
    let op = ChaCha;
    String::from_utf8(op.run(input.as_bytes().to_vec(), args).unwrap()).unwrap()
}

#[test]
fn test_chacha_basic_encryption() {
    // Test basic ChaCha encryption with default parameters
    let args = [
        rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e0f".to_string()), // 16-byte key
        rxchef::operation::ArgValue::Str("0x0000000000000000".to_string()), // 8-byte nonce
        rxchef::operation::ArgValue::Num(0.0), // Counter
        rxchef::operation::ArgValue::Str("20".to_string()), // Rounds
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Output format
    ];
    
    // Encrypt "Hello" (hex: 48656c6c6f)
    let result = run("48656c6c6f", &args);
    assert_eq!(result.len(), 10); // 5 bytes = 10 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_chacha_256bit_key() {
    // Test with 256-bit (32-byte) key
    let args = [
        rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f".to_string()), // 32-byte key
        rxchef::operation::ArgValue::Str("0x0000000000000000".to_string()), // 8-byte nonce
        rxchef::operation::ArgValue::Num(0.0), // Counter
        rxchef::operation::ArgValue::Str("20".to_string()), // Rounds
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Output format
    ];
    
    let result = run("48656c6c6f", &args);
    assert_eq!(result.len(), 10); // 5 bytes = 10 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_chacha_12byte_nonce() {
    // Test with 12-byte nonce
    let args = [
        rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e0f".to_string()), // 16-byte key
        rxchef::operation::ArgValue::Str("0x000000000000000000000000".to_string()), // 12-byte nonce
        rxchef::operation::ArgValue::Num(0.0), // Counter
        rxchef::operation::ArgValue::Str("20".to_string()), // Rounds
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Output format
    ];
    
    let result = run("48656c6c6f", &args);
    assert_eq!(result.len(), 10); // 5 bytes = 10 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_chacha_different_rounds() {
    // Test with different round counts
    for rounds in [8, 12, 20] {
        let args = [
            rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e0f".to_string()), // 16-byte key
            rxchef::operation::ArgValue::Str("0x0000000000000000".to_string()), // 8-byte nonce
            rxchef::operation::ArgValue::Num(0.0), // Counter
            rxchef::operation::ArgValue::Str(rounds.to_string()), // Rounds
            rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
            rxchef::operation::ArgValue::Str("Hex".to_string()), // Output format
        ];
        
        let result = run("48656c6c6f", &args);
        assert_eq!(result.len(), 10); // 5 bytes = 10 hex characters
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

#[test]
fn test_chacha_raw_output() {
    // Test with raw output format
    let op = ChaCha;
    let args = [
        rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e0f".to_string()), // 16-byte key
        rxchef::operation::ArgValue::Str("0x0000000000000000".to_string()), // 8-byte nonce
        rxchef::operation::ArgValue::Num(0.0), // Counter
        rxchef::operation::ArgValue::Str("20".to_string()), // Rounds
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
        rxchef::operation::ArgValue::Str("Raw".to_string()), // Output format
    ];
    
    let result = op.run("48656c6c6f".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result.len(), 5); // Raw bytes, not hex
}

#[test]
fn test_chacha_empty_input() {
    let args = [
        rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e0f".to_string()), // 16-byte key
        rxchef::operation::ArgValue::Str("0x0000000000000000".to_string()), // 8-byte nonce
        rxchef::operation::ArgValue::Num(0.0), // Counter
        rxchef::operation::ArgValue::Str("20".to_string()), // Rounds
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Output format
    ];
    
    let result = run("", &args);
    assert_eq!(result, ""); // Empty input should produce empty output
}

#[test]
fn test_chacha_large_input() {
    // Test with input larger than 64 bytes (requires multiple blocks)
    let args = [
        rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e0f".to_string()), // 16-byte key
        rxchef::operation::ArgValue::Str("0x0000000000000000".to_string()), // 8-byte nonce
        rxchef::operation::ArgValue::Num(0.0), // Counter
        rxchef::operation::ArgValue::Str("20".to_string()), // Rounds
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Output format
    ];
    
    // 100 bytes of input (will require multiple ChaCha blocks)
    let input = "48656c6c6f".repeat(20); // "Hello" repeated 20 times
    let result = run(&input, &args);
    assert_eq!(result.len(), 200); // 100 bytes = 200 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_chacha_invalid_key_length() {
    let op = ChaCha;
    let args = [
        rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e".to_string()), // 14-byte key (invalid)
        rxchef::operation::ArgValue::Str("0x0000000000000000".to_string()), // 8-byte nonce
        rxchef::operation::ArgValue::Num(0.0), // Counter
        rxchef::operation::ArgValue::Str("20".to_string()), // Rounds
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Output format
    ];
    
    let result = op.run("48656c6c6f".as_bytes().to_vec(), &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid key length"));
}

#[test]
fn test_chacha_invalid_nonce_length() {
    let op = ChaCha;
    let args = [
        rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e0f".to_string()), // 16-byte key
        rxchef::operation::ArgValue::Str("0x000000000000".to_string()), // 6-byte nonce (invalid)
        rxchef::operation::ArgValue::Num(0.0), // Counter
        rxchef::operation::ArgValue::Str("20".to_string()), // Rounds
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Output format
    ];
    
    let result = op.run("48656c6c6f".as_bytes().to_vec(), &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid nonce length"));
}

#[test]
fn test_chacha_invalid_hex_input() {
    let op = ChaCha;
    let args = [
        rxchef::operation::ArgValue::Str("0x000102030405060708090a0b0c0d0e0f".to_string()), // 16-byte key
        rxchef::operation::ArgValue::Str("0x0000000000000000".to_string()), // 8-byte nonce
        rxchef::operation::ArgValue::Num(0.0), // Counter
        rxchef::operation::ArgValue::Str("20".to_string()), // Rounds
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Input format
        rxchef::operation::ArgValue::Str("Hex".to_string()), // Output format
    ];
    
    let result = op.run("invalid hex".as_bytes().to_vec(), &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid hex input"));
}

#[test]
fn test_chacha_nonce_counter_variation() {
    // Test that different counter values produce different output
    let key = "0x000102030405060708090a0b0c0d0e0f".to_string();
    let nonce = "0x0000000000000000".to_string();
    let input = "48656c6c6f";
    
    let result1 = {
        let args = [
            rxchef::operation::ArgValue::Str(key.clone()),
            rxchef::operation::ArgValue::Str(nonce.clone()),
            rxchef::operation::ArgValue::Num(0.0), // Counter 0
            rxchef::operation::ArgValue::Str("20".to_string()),
            rxchef::operation::ArgValue::Str("Hex".to_string()),
            rxchef::operation::ArgValue::Str("Hex".to_string()),
        ];
        run(input, &args)
    };
    
    let result2 = {
        let args = [
            rxchef::operation::ArgValue::Str(key),
            rxchef::operation::ArgValue::Str(nonce),
            rxchef::operation::ArgValue::Num(1.0), // Counter 1
            rxchef::operation::ArgValue::Str("20".to_string()),
            rxchef::operation::ArgValue::Str("Hex".to_string()),
            rxchef::operation::ArgValue::Str("Hex".to_string()),
        ];
        run(input, &args)
    };
    
    // Different counters should produce different output
    assert_ne!(result1, result2);
}
