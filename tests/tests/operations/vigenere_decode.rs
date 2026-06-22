// Tests for the vigenere_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations vigenere_decode::

use rxchef::operation::ArgValue;
use rxchef::operations::vigenere_decode::VigenereDecodeOp;
use rxchef::Operation;

#[test]
fn test_vigenere_decode_basic() {
    let op = VigenereDecodeOp;
    let input = b"Dlc aygmo zbsux jmh nswtq yzcb xfo pyjc byk".to_vec();
    let args = [ArgValue::Str("key".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&result),
        "The quick brown fox jumps over the lazy dog"
    );
}
#[test]
fn test_vigenere_decode_mixed_case() {
    let op = VigenereDecodeOp;
    let input = b"Zincs Pgvnu!".to_vec();
    let args = [ArgValue::Str("SECRET".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "Hello World!");
}
#[test]
fn test_vigenere_decode_no_key() {
    let op = VigenereDecodeOp;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("".to_string())];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
