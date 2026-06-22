// Tests for the a1z26_cipher_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations a1z26_cipher_decode::

use rxchef::operation::ArgValue;
use rxchef::operations::a1z26_cipher_decode::A1Z26CipherDecode;
use rxchef::Operation;

#[test]
fn test_basic_decode() {
    let op = A1Z26CipherDecode;
    let input = b"8 5 12 12 15".to_vec(); // hello
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "hello");
}
#[test]
fn test_empty_input() {
    let op = A1Z26CipherDecode;
    let input = b"".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_all_alphabet() {
    let op = A1Z26CipherDecode;
    let input = b"1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "abcdefghijklmnopqrstuvwxyz");
}
#[test]
fn test_comma_delimiter() {
    let op = A1Z26CipherDecode;
    let input = b"8,5,12,12,15".to_vec(); // hello
    let args = [ArgValue::Str("Comma".to_string())];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "hello");
}
#[test]
fn test_invalid_number_low() {
    let op = A1Z26CipherDecode;
    let input = b"0 1 2".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_invalid_number_high() {
    let op = A1Z26CipherDecode;
    let input = b"27 1 2".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_invalid_number_text() {
    let op = A1Z26CipherDecode;
    let input = b"abc 1 2".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
