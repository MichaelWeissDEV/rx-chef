// Tests for the atbash_cipher operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations atbash_cipher::

use rxchef::operations::atbash_cipher::AtbashCipher;
use rxchef::Operation;

#[test]
fn test_atbash_cipher_basic() {
    let operation = AtbashCipher;
    // Hello -> svool (all lowercase output)
    let input = b"Hello".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "svool");
}
#[test]
fn test_atbash_cipher_alphabet() {
    let operation = AtbashCipher;
    // abcdefghijklmnopqrstuvwxyz -> zyxwvutsrqponmlkjihgfedcba
    let input = b"abcdefghijklmnopqrstuvwxyz".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "zyxwvutsrqponmlkjihgfedcba");
}
#[test]
fn test_atbash_cipher_non_alpha() {
    let operation = AtbashCipher;
    let input = b"Hello, World! 123".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "svool, dliow! 123");
}
#[test]
fn test_atbash_cipher_empty() {
    let operation = AtbashCipher;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "");
}
