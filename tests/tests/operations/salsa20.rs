// Tests for the salsa20 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations salsa20::

use rxchef::operation::ArgValue;
use rxchef::operations::salsa20::Salsa20Op;
use rxchef::Operation;

#[test]
fn test_salsa20_ecrypt_set_1_vector_0_first_block() {
    // eSTREAM/ECRYPT Salsa20 verified test set 1, vector 0. This is also
    // exercised by CyberChef 11.4.0 at commit 2e048b0290854781db61e20638dca62978379032.
    let result = Salsa20Op
        .run(
            vec![0; 64],
            &[
                ArgValue::Str(
                    "hex:8000000000000000000000000000000000000000000000000000000000000000"
                        .to_string(),
                ),
                ArgValue::Str("hex:0000000000000000".to_string()),
                ArgValue::Num(0.0),
                ArgValue::Str("20".to_string()),
                ArgValue::Str("Raw".to_string()),
                ArgValue::Str("Hex".to_string()),
            ],
        )
        .unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(),
        concat!(
            "e3be8fdd8beca2e3ea8ef9475b29a6e7003951e1097a5c38d23b7a5fad9f6844",
            "b22c97559e2723c7cbbd3fe4fc8d9a0744652a83e72a9c461876af4d7ef1a117"
        )
    );
}

#[test]
fn test_salsa20_basic() {
    let op = Salsa20Op;
    let input = b"Hello World".to_vec();
    let key = "hex:000102030405060708090a0b0c0d0e0f".to_string(); // 16 bytes
    let nonce = "hex:0011223344556677".to_string(); // 8 bytes
    let args = [
        ArgValue::Str(key.clone()),
        ArgValue::Str(nonce.clone()),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let encrypted = op.run(input.clone(), &args).unwrap();
    assert_ne!(input, encrypted);
    let decrypted = op.run(encrypted, &args).unwrap();
    assert_eq!(input, decrypted);
}
#[test]
fn test_salsa20_hex_output() {
    let op = Salsa20Op;
    let input = b"Hello".to_vec();
    let key = "hex:000102030405060708090a0b0c0d0e0f".to_string();
    let nonce = "hex:0011223344556677".to_string();
    let args = [
        ArgValue::Str(key),
        ArgValue::Str(nonce),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(result.iter().all(|&b| (b as char).is_ascii_hexdigit()));
}
#[test]
fn test_salsa20_invalid_key() {
    let op = Salsa20Op;
    let input = b"Hello".to_vec();
    let args = [
        ArgValue::Str("010203".to_string()), // Too short
        ArgValue::Str("hex:0011223344556677".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
