// Tests for the a1z26_cipher_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations a1z26_cipher_encode::

use rxchef::operation::ArgValue;
use rxchef::operations::a1z26_cipher_encode::A1Z26CipherEncode;
use rxchef::Operation;

#[test]
fn test_basic_encode() {
    let op = A1Z26CipherEncode;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "8 5 12 12 15");
}
#[test]
fn test_empty_input() {
    let op = A1Z26CipherEncode;
    let input = b"".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_non_alphabet_chars_dropped() {
    let op = A1Z26CipherEncode;
    let input = b"hello123world".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "8 5 12 12 15 23 15 18 12 4");
}
#[test]
fn test_comma_delimiter() {
    let op = A1Z26CipherEncode;
    let input = b"abc".to_vec();
    let args = [ArgValue::Str("Comma".to_string())];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "1,2,3");
}
#[test]
fn test_uppercase_input() {
    let op = A1Z26CipherEncode;
    let input = b"HELLO".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "8 5 12 12 15");
}
#[test]
fn test_all_alphabet() {
    let op = A1Z26CipherEncode;
    let input = b"abcdefghijklmnopqrstuvwxyz".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(
        result_str,
        "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26"
    );
}
