// Tests for the derive_pbkdf2_key operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations derive_pbkdf2_key::

use rxchef::operations::derive_pbkdf2_key::DerivePBKDF2Key;
use rxchef::Operation;

fn run(passphrase: &str, key_size: usize, iterations: usize, hash_func: &str, salt: &str) -> String {
    let op = DerivePBKDF2Key;
    let args = [
        rxchef::operation::ArgValue::Str(passphrase.to_string()),
        rxchef::operation::ArgValue::Num(key_size as f64),
        rxchef::operation::ArgValue::Num(iterations as f64),
        rxchef::operation::ArgValue::Str(hash_func.to_string()),
        rxchef::operation::ArgValue::Str(salt.to_string()),
    ];
    String::from_utf8(op.run(vec![], &args).unwrap()).unwrap()
}

#[test]
fn test_derive_pbkdf2_basic() {
    // Test basic PBKDF2 derivation
    let result = run("password", 128, 1, "SHA256", "salt");
    assert_eq!(result.len(), 32); // 128 bits = 32 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_derive_pbkdf2_different_key_sizes() {
    // Test different key sizes
    for key_size in [128, 256, 512] {
        let result = run("password", key_size, 1, "SHA256", "salt");
        assert_eq!(result.len(), key_size / 4); // key_size bits = key_size/4 hex chars
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

#[test]
fn test_derive_pbkdf2_different_iterations() {
    // Test different iteration counts
    for iterations in [1, 10, 100, 1000] {
        let result = run("password", 128, iterations, "SHA256", "salt");
        assert_eq!(result.len(), 32); // 128 bits = 32 hex characters
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

#[test]
fn test_derive_pbkdf2_different_hash_functions() {
    // Test different hash functions
    let hash_functions = ["SHA1", "SHA256", "SHA384", "SHA512", "MD5"];
    
    for hash_func in hash_functions {
        let result = run("password", 128, 1, hash_func, "salt");
        assert_eq!(result.len(), 32); // 128 bits = 32 hex characters
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

#[test]
fn test_derive_pbkdf2_empty_passphrase() {
    // Test with empty passphrase
    let result = run("", 128, 1, "SHA256", "salt");
    assert_eq!(result.len(), 32); // 128 bits = 32 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_derive_pbkdf2_empty_salt() {
    // Test with empty salt (should generate random salt)
    let result1 = run("password", 128, 1, "SHA256", "");
    let result2 = run("password", 128, 1, "SHA256", "");
    
    // Should produce different results due to random salts
    assert_ne!(result1, result2);
    assert_eq!(result1.len(), 32);
    assert_eq!(result2.len(), 32);
}

#[test]
fn test_derive_pbkdf2_hex_salt() {
    // Test with hex-encoded salt
    let result = run("password", 128, 1, "SHA256", "0xdeadbeef");
    assert_eq!(result.len(), 32); // 128 bits = 32 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_derive_pbkdf2_large_iterations() {
    // Test with large iteration count (performance test)
    let result = run("password", 128, 10000, "SHA256", "salt");
    assert_eq!(result.len(), 32); // 128 bits = 32 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_derive_pbkdf2_invalid_hash_function() {
    let op = DerivePBKDF2Key;
    let args = [
        rxchef::operation::ArgValue::Str("password".to_string()),
        rxchef::operation::ArgValue::Num(128.0),
        rxchef::operation::ArgValue::Num(1.0),
        rxchef::operation::ArgValue::Str("INVALID".to_string()),
        rxchef::operation::ArgValue::Str("salt".to_string()),
    ];
    
    let result = op.run(vec![], &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported hashing function"));
}

#[test]
fn test_derive_pbkdf2_deterministic() {
    // Test that same inputs produce same outputs
    let result1 = run("password", 128, 1, "SHA256", "salt");
    let result2 = run("password", 128, 1, "SHA256", "salt");
    
    assert_eq!(result1, result2); // Same inputs should produce same output
}

#[test]
fn test_derive_pbkdf2_different_salts() {
    // Test that different salts produce different outputs
    let result1 = run("password", 128, 1, "SHA256", "salt1");
    let result2 = run("password", 128, 1, "SHA256", "salt2");
    
    assert_ne!(result1, result2); // Different salts should produce different outputs
}

#[test]
fn test_derive_pbkdf2_large_key_size() {
    // Test with large key size
    let result = run("password", 1024, 1, "SHA512", "salt");
    assert_eq!(result.len(), 256); // 1024 bits = 256 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_derive_pbkdf2_unicode_passphrase() {
    // Test with Unicode passphrase
    let result = run("pässwörd", 128, 1, "SHA256", "salt");
    assert_eq!(result.len(), 32); // 128 bits = 32 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_derive_pbkdf2_long_passphrase() {
    // Test with long passphrase
    let long_passphrase = "a".repeat(1000);
    let result = run(&long_passphrase, 128, 1, "SHA256", "salt");
    assert_eq!(result.len(), 32); // 128 bits = 32 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}
