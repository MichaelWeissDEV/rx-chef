// Tests for the x_salsa20 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations x_salsa20::

use rxchef::operation::ArgValue;
use rxchef::operations::x_salsa20::XSalsa20Op;
use rxchef::Operation;

#[test]
fn test_xsalsa20_pinned_reference_vector_first_block() {
    // Pinned CyberChef 11.4.0 reference vector at commit
    // 2e048b0290854781db61e20638dca62978379032, independently implemented in JS.
    let result = XSalsa20Op
        .run(
            vec![0; 64],
            &[
                ArgValue::Str(
                    "hex:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
                        .to_string(),
                ),
                ArgValue::Str(
                    "hex:000102030405060708090a0b0c0d0e0f1011121314151617".to_string(),
                ),
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
            "7cb660afdd9ec6468f57dd6d2433f93428fd82cd7386c5471a24d8ad2a525b6e",
            "5eff384fc7caa210bb3c8f3e688f4a9752a546df8c253fef17a2679455c7a1e1"
        )
    );
}

#[test]
fn test_xsalsa20_encryption() {
    let op = XSalsa20Op;
    // Key: 32 bytes of 0x01
    let key = vec![0x01; 32];
    // Nonce: 24 bytes of 0x02
    let nonce = vec![0x02; 24];
    let input = b"Hello World!".to_vec();
    let args = [
        ArgValue::Str(format!("hex:{}", hex::encode(&key))),
        ArgValue::Str(format!("hex:{}", hex::encode(&nonce))),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    assert_ne!(result, input);
    // Decrypt
    let decrypted = op.run(result, &args).unwrap();
    assert_eq!(decrypted, input);
}

#[test]
fn test_xsalsa_reduced_round_variants_round_trip_and_differ() {
    let op = XSalsa20Op;
    let input = b"reduced round message".to_vec();
    let mut ciphertexts = Vec::new();
    for rounds in ["8", "12", "20"] {
        let args = [
            ArgValue::Str(format!("hex:{}", hex::encode([0x11; 32]))),
            ArgValue::Str(format!("hex:{}", hex::encode([0x22; 24]))),
            ArgValue::Num(3.0),
            ArgValue::Str(rounds.to_string()),
            ArgValue::Str("Raw".to_string()),
            ArgValue::Str("Raw".to_string()),
        ];
        let encrypted = op.run(input.clone(), &args).unwrap();
        assert_eq!(op.run(encrypted.clone(), &args).unwrap(), input);
        ciphertexts.push(encrypted);
    }
    assert_ne!(ciphertexts[0], ciphertexts[1]);
    assert_ne!(ciphertexts[1], ciphertexts[2]);
}
#[test]
fn test_xsalsa20_invalid_key_length() {
    let op = XSalsa20Op;
    let input = b"test".to_vec();
    let args = [
        ArgValue::Str("01020304".to_string()), // Too short
        ArgValue::Str("010203040506070809101112131415161718192021222324".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_xsalsa20_invalid_nonce_length() {
    let op = XSalsa20Op;
    let input = b"test".to_vec();
    let args = [
        ArgValue::Str(
            "0102030405060708091011121314151617181920212223242526272829303132".to_string(),
        ),
        ArgValue::Str("0102".to_string()), // Too short
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
