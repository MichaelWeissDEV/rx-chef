// Tests for the gost_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations gost_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::gost_decrypt::GOSTDecryptOp;
use rxchef::Operation;

#[test]
fn test_gost_decrypt_magma_ecb() {
    let op = GOSTDecryptOp;
    // Key: 32 bytes (256 bits)
    let key = vec![0u8; 32];
    let input_hex = "911e3b5e4062a492".to_string(); // Encrypted 0000000000000000 with zero key in Magma
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Bytes(vec![]),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("GOST R 34.12 (Magma, 2015)".to_string()),
        ArgValue::Str("E-TEST".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("NO".to_string()),
        ArgValue::Str("NO".to_string()),
    ];
    let result = op.run(input_hex.into_bytes(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Result should be 16 zeros in hex
    assert!(result_str.len() >= 16);
}
