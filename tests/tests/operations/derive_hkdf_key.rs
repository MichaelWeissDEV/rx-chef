// Tests for the derive_hkdf_key operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations derive_hkdf_key::

use rxchef::operations::derive_hkdf_key::DeriveHKDFKey;
use rxchef::Operation;

fn run(input: &str, args: &[rxchef::operation::ArgValue]) -> String {
    let op = DeriveHKDFKey;
    String::from_utf8(op.run(input.as_bytes().to_vec(), args).unwrap()).unwrap()
}

#[test]
fn test_derive_hkdf_basic() {
    // Test basic HKDF derivation with default parameters
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()), // No salt
        rxchef::operation::ArgValue::Str("".to_string()), // No info
        rxchef::operation::ArgValue::Str("SHA256".to_string()), // Hash function
        rxchef::operation::ArgValue::Str("with salt".to_string()), // Extract mode
        rxchef::operation::ArgValue::Num(16.0), // Output length
    ];
    
    let result = run("secret", &args);
    assert_eq!(result.len(), 32); // 16 bytes = 32 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_derive_hkdf_with_salt() {
    let args = [
        rxchef::operation::ArgValue::Str("salt".to_string()), // Salt
        rxchef::operation::ArgValue::Str("".to_string()), // No info
        rxchef::operation::ArgValue::Str("SHA256".to_string()), // Hash function
        rxchef::operation::ArgValue::Str("with salt".to_string()), // Extract mode
        rxchef::operation::ArgValue::Num(32.0), // Output length
    ];
    
    let result = run("secret", &args);
    assert_eq!(result.len(), 64); // 32 bytes = 64 hex characters
}

#[test]
fn test_derive_hkdf_no_salt() {
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()), // No salt
        rxchef::operation::ArgValue::Str("".to_string()), // No info
        rxchef::operation::ArgValue::Str("SHA256".to_string()), // Hash function
        rxchef::operation::ArgValue::Str("no salt".to_string()), // Extract mode
        rxchef::operation::ArgValue::Num(16.0), // Output length
    ];
    
    let result = run("secret", &args);
    assert_eq!(result.len(), 32); // 16 bytes = 32 hex characters
}

#[test]
fn test_derive_hkdf_skip_mode() {
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()), // No salt
        rxchef::operation::ArgValue::Str("".to_string()), // No info
        rxchef::operation::ArgValue::Str("SHA256".to_string()), // Hash function
        rxchef::operation::ArgValue::Str("skip".to_string()), // Extract mode
        rxchef::operation::ArgValue::Num(16.0), // Output length
    ];
    
    // In skip mode, the input is treated as the PRK directly
    // This should still work but the output will be different
    let result = run("secret", &args);
    assert_eq!(result.len(), 32); // 16 bytes = 32 hex characters
}

#[test]
fn test_derive_hkdf_with_info() {
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()), // No salt
        rxchef::operation::ArgValue::Str("context info".to_string()), // Info
        rxchef::operation::ArgValue::Str("SHA256".to_string()), // Hash function
        rxchef::operation::ArgValue::Str("with salt".to_string()), // Extract mode
        rxchef::operation::ArgValue::Num(16.0), // Output length
    ];
    
    let result = run("secret", &args);
    assert_eq!(result.len(), 32); // 16 bytes = 32 hex characters
}

#[test]
fn test_derive_hkdf_different_hash_functions() {
    let hash_functions = ["SHA1", "SHA256", "SHA384", "SHA512"];
    
    for hash_func in hash_functions {
        let args = [
            rxchef::operation::ArgValue::Str("".to_string()), // No salt
            rxchef::operation::ArgValue::Str("".to_string()), // No info
            rxchef::operation::ArgValue::Str(hash_func.to_string()), // Hash function
            rxchef::operation::ArgValue::Str("with salt".to_string()), // Extract mode
            rxchef::operation::ArgValue::Num(16.0), // Output length
        ];
        
        let result = run("secret", &args);
        assert_eq!(result.len(), 32); // 16 bytes = 32 hex characters
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

#[test]
fn test_derive_hkdf_different_output_lengths() {
    let lengths = [8, 16, 32, 64];
    
    for length in lengths {
        let args = [
            rxchef::operation::ArgValue::Str("".to_string()), // No salt
            rxchef::operation::ArgValue::Str("".to_string()), // No info
            rxchef::operation::ArgValue::Str("SHA256".to_string()), // Hash function
            rxchef::operation::ArgValue::Str("with salt".to_string()), // Extract mode
            rxchef::operation::ArgValue::Num(length as f64), // Output length
        ];
        
        let result = run("secret", &args);
        assert_eq!(result.len(), length * 2); // length bytes = 2*length hex characters
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

#[test]
fn test_derive_hkdf_empty_input() {
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()), // No salt
        rxchef::operation::ArgValue::Str("".to_string()), // No info
        rxchef::operation::ArgValue::Str("SHA256".to_string()), // Hash function
        rxchef::operation::ArgValue::Str("with salt".to_string()), // Extract mode
        rxchef::operation::ArgValue::Num(16.0), // Output length
    ];
    
    let result = run("", &args);
    assert_eq!(result.len(), 32); // 16 bytes = 32 hex characters
}

#[test]
fn test_derive_hkdf_hex_salt() {
    let args = [
        rxchef::operation::ArgValue::Str("0xdeadbeef".to_string()), // Hex salt
        rxchef::operation::ArgValue::Str("".to_string()), // No info
        rxchef::operation::ArgValue::Str("SHA256".to_string()), // Hash function
        rxchef::operation::ArgValue::Str("with salt".to_string()), // Extract mode
        rxchef::operation::ArgValue::Num(16.0), // Output length
    ];
    
    let result = run("secret", &args);
    assert_eq!(result.len(), 32); // 16 bytes = 32 hex characters
}

#[test]
fn test_derive_hkdf_invalid_hash_function() {
    let op = DeriveHKDFKey;
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()), // No salt
        rxchef::operation::ArgValue::Str("".to_string()), // No info
        rxchef::operation::ArgValue::Str("INVALID".to_string()), // Invalid hash function
        rxchef::operation::ArgValue::Str("with salt".to_string()), // Extract mode
        rxchef::operation::ArgValue::Num(16.0), // Output length
    ];
    
    let result = op.run("secret".as_bytes().to_vec(), &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported hashing function"));
}

#[test]
fn test_derive_hkdf_invalid_extract_mode() {
    let op = DeriveHKDFKey;
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()), // No salt
        rxchef::operation::ArgValue::Str("".to_string()), // No info
        rxchef::operation::ArgValue::Str("SHA256".to_string()), // Hash function
        rxchef::operation::ArgValue::Str("invalid".to_string()), // Invalid extract mode
        rxchef::operation::ArgValue::Num(16.0), // Output length
    ];
    
    let result = op.run("secret".as_bytes().to_vec(), &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported extract mode"));
}
