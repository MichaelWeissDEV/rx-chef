// Tests for the vigenere_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations vigenere_encode::

use rxchef::operation::ArgValue;
use rxchef::operations::vigenere_encode::VigenereEncodeOp;
use rxchef::Operation;

#[test]
fn test_vigenere_encode_basic() {
    let op = VigenereEncodeOp;
    let input = b"The quick brown fox jumps over the lazy dog".to_vec();
    let args = [ArgValue::Str("key".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&result),
        "Dlc aygmo zbsux jmh nswtq yzcb xfo pyjc byk"
    );
}
#[test]
fn test_vigenere_encode_mixed_case() {
    let op = VigenereEncodeOp;
    let input = b"Hello World!".to_vec();
    let args = [ArgValue::Str("SECRET".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "Zincs Pgvnu!");
}
#[test]
fn test_vigenere_encode_no_key() {
    let op = VigenereEncodeOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("".to_string())];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_vigenere_encode_invalid_key() {
    let op = VigenereEncodeOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("key123".to_string())];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
