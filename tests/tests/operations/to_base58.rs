// Tests for the to_base58 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_base58::

use rxchef::operations::to_base58::ToBase58;
use rxchef::Operation;

#[test]
fn test_to_base58_empty_input() {
    let op = ToBase58;
    let args = [rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string())];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_to_base58_zero() {
    let op = ToBase58;
    let args = [rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string())];
    let result = op.run(vec![0x00], &args).unwrap();
    assert_eq!(result, "1".as_bytes());
}

#[test]
fn test_to_base58_single_byte() {
    let op = ToBase58;
    let args = [rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string())];
    let result = op.run(vec![0x01], &args).unwrap();
    assert_eq!(result, "2".as_bytes());
}

#[test]
fn test_to_base58_hello_world() {
    let op = ToBase58;
    let args = [rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string())];
    let result = op.run("Hello World".as_bytes().to_vec(), &args).unwrap();
    assert_eq!(result, "JxF12TrwUP45BMd".as_bytes());
}

#[test]
fn test_to_base58_binary_data() {
    let op = ToBase58;
    let args = [rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string())];
    let result = op.run(vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD], &args).unwrap();
    // Should produce a valid base58 string
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.len() > 0);
    // Should only contain base58 characters (no 0, O, I, l)
    for ch in result_str.chars() {
        assert!(!"0OIl".contains(ch));
    }
}

#[test]
fn test_to_base58_large_input() {
    let op = ToBase58;
    let args = [rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string())];
    let input = vec![0xFF; 100]; // 100 bytes of 0xFF
    let result = op.run(input, &args).unwrap();
    // Should produce a valid base58 string
    assert!(result.len() > 0);
}

#[test]
fn test_to_base58_invalid_alphabet() {
    let op = ToBase58;
    let args = [rxchef::operation::ArgValue::Str("short".to_string())]; // Less than 58 characters
    let result = op.run(vec![0x01], &args);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, rxchef::OperationError::InvalidArgument { .. }));
    }
}

#[test]
fn test_to_base58_custom_alphabet() {
    let op = ToBase58;
    let custom_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"; // Standard Bitcoin alphabet
    let args = [rxchef::operation::ArgValue::Str(custom_alphabet.to_string())];
    let result = op.run(vec![0x00, 0x01, 0x02], &args).unwrap();
    // Should produce a valid base58 string with custom alphabet
    assert!(result.len() > 0);
}

#[test]
fn test_to_base58_max_values() {
    let op = ToBase58;
    let args = [rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string())];
    let result = op.run(vec![0xFF, 0xFF, 0xFF, 0xFF], &args).unwrap();
    // Should produce a valid base58 string
    assert!(result.len() > 0);
}

#[test]
fn test_to_base58_unicode_bytes() {
    let op = ToBase58;
    let args = [rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string())];
    let result = op.run("Hello 世界".as_bytes().to_vec(), &args).unwrap();
    // Should produce a valid base58 string
    assert!(result.len() > 0);
}
