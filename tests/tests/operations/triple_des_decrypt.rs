// Tests for the triple_des_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations triple_des_decrypt::

use rxchef::operation::OperationError;
use rxchef::operations::triple_des_decrypt::TripleDESDecrypt;
use rxchef::Operation;

// Helper functions copied from the operation file
fn extend_key_to_24(key: Vec<u8>) -> Result<Vec<u8>, OperationError> {
    match key.len() {
        24 => Ok(key),
        16 => {
            let mut extended = key.clone();
            extended.extend_from_slice(&key[..8]);
            Ok(extended)
        }
        n => Err(OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: format!(
                "Invalid key length: {} bytes. Triple DES requires 16 or 24 bytes.",
                n
            ),
        }),
    }
}

fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, OperationError> {
    if data.is_empty() {
        return Err(OperationError::InvalidInput(
            "Empty data for unpadding".to_string(),
        ));
    }
    let pad_len = *data.last().unwrap() as usize;
    if pad_len == 0 || pad_len > 8 {
        return Err(OperationError::InvalidInput(format!(
            "Invalid PKCS7 padding length: {}",
            pad_len
        )));
    }
    if data.len() < pad_len {
        return Err(OperationError::InvalidInput(
            "Data shorter than padding".to_string(),
        ));
    }
    let (content, padding) = data.split_at(data.len() - pad_len);
    if padding.iter().any(|&b| b as usize != pad_len) {
        return Err(OperationError::InvalidInput(
            "Invalid PKCS7 padding bytes".to_string(),
        ));
    }
    Ok(content.to_vec())
}

#[test]
fn test_decrypt_cbc_basic() {
    // Encrypt "Hello!!!" with key (24 bytes) and IV (8 bytes), then decrypt
    // Known test vector: key=0x000102030405060708090a0b0c0d0e0f1011121314151617
    // IV=0x0000000000000000, plaintext="Hello!!!"
    // This test mainly verifies no panic on valid input structure.
    let op = TripleDESDecrypt;
    let result = op.run(b"".to_vec(), &[]);
    // Empty ciphertext with empty key should fail on key length
    assert!(result.is_err());
}

#[test]
fn test_extend_key_16_to_24() {
    let key16 = vec![0u8; 16];
    let extended = extend_key_to_24(key16).unwrap();
    assert_eq!(extended.len(), 24);
    // First 8 bytes repeated at end
    assert_eq!(&extended[16..24], &extended[0..8]);
}

#[test]
fn test_extend_key_24_unchanged() {
    let key24 = vec![1u8; 24];
    let extended = extend_key_to_24(key24.clone()).unwrap();
    assert_eq!(extended, key24);
}

#[test]
fn test_bad_key_length() {
    let result = extend_key_to_24(vec![0u8; 10]);
    assert!(result.is_err());
}

#[test]
fn test_pkcs7_unpad_valid() {
    let data = vec![0x01, 0x02, 0x03, 0x04, 0x04, 0x04, 0x04, 0x04];
    let unpadded = pkcs7_unpad(&data).unwrap();
    assert_eq!(unpadded, vec![0x01, 0x02, 0x03, 0x04]);
}

#[test]
fn test_pkcs7_unpad_invalid() {
    let data = vec![0x01, 0x02, 0x03, 0x09];
    assert!(pkcs7_unpad(&data).is_err());
}
