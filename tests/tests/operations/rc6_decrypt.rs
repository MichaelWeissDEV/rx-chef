// Tests for the rc6_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rc6_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::rc6_decrypt::RC6Decrypt;
use rxchef::Operation;

#[test]
fn test_rc6_8_12_4_ietf_vector_decrypt() {
    // Inverse direction of draft-krovetz-rc6-rc5-vectors-00 RC6-8/12/4;
    // ciphertext is fixed rather than obtained from RC6Encrypt.
    let output = RC6Decrypt
        .run(
            b"aefc4612".to_vec(),
            &[
                ArgValue::Str("hex:00010203".into()),
                ArgValue::Str(String::new()),
                ArgValue::Str("ECB".into()),
                ArgValue::Str("Hex".into()),
                ArgValue::Str("Hex".into()),
                ArgValue::Str("NO".into()),
                ArgValue::Num(8.0),
                ArgValue::Num(12.0),
            ],
        )
        .unwrap();
    assert_eq!(output, b"00010203");
}

#[test]
fn test_rc6_decrypt_rejects_short_ciphertext() {
    let op = RC6Decrypt;
    let input = b"00".to_vec();
    let args = [
        ArgValue::Str("secret".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("PKCS5".to_string()),
        ArgValue::Str("32".to_string()),
        ArgValue::Str("20".to_string()),
    ];
    assert!(op.run(input, &args).is_err());
}
