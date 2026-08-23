// Tests for the rc6_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rc6_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::rc6_encrypt::RC6Encrypt;
use rxchef::Operation;

fn ietf_args(word_size: f64) -> [ArgValue; 8] {
    [
        ArgValue::Str("hex:00010203".into()),
        ArgValue::Str(String::new()),
        ArgValue::Str("ECB".into()),
        ArgValue::Str("Hex".into()),
        ArgValue::Str("Hex".into()),
        ArgValue::Str("NO".into()),
        ArgValue::Num(word_size),
        ArgValue::Num(12.0),
    ]
}

#[test]
fn test_rc6_8_12_4_ietf_vector() {
    // draft-krovetz-rc6-rc5-vectors-00, RC6-8/12/4. Word size 8 is
    // also the operation's minimum accepted word-size boundary.
    let output = RC6Encrypt
        .run(b"00010203".to_vec(), &ietf_args(8.0))
        .unwrap();
    assert_eq!(output, b"aefc4612");
}

#[test]
fn test_rc6_encrypt_rejects_word_size_below_ietf_minimum() {
    let error = RC6Encrypt
        .run(b"00010203".to_vec(), &ietf_args(7.0))
        .unwrap_err();
    assert!(matches!(
        error,
        rxchef::operation::OperationError::InvalidInput(_)
    ));
}

#[test]
fn test_rc6_encrypt_basic() {
    let op = RC6Encrypt;
    let input = b"hello world".to_vec();
    let args = [
        ArgValue::Str("secret".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("PKCS5".to_string()),
        ArgValue::Str("32".to_string()),
        ArgValue::Str("20".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}
