// Tests for the affine_cipher_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations affine_cipher_decode::

use rxchef::operation::ArgValue;
use rxchef::operations::affine_cipher_decode::AffineCipherDecode;
use rxchef::Operation;

#[test]
fn test_affine_cipher_decode_basic() {
    let op = AffineCipherDecode;
    // "hello" encoded with a=1, b=0 gives "hello"
    let input = b"hello".to_vec();
    let args = [ArgValue::Num(1.0), ArgValue::Num(0.0)];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "hello");
}
#[test]
fn test_affine_cipher_decode_caesar_shift() {
    let op = AffineCipherDecode;
    // "def" encoded with a=1, b=3 gives "abc"
    let input = b"def".to_vec();
    let args = [ArgValue::Num(1.0), ArgValue::Num(3.0)];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "abc");
}
#[test]
fn test_affine_cipher_decode_uppercase() {
    let op = AffineCipherDecode;
    // "DEF" encoded with a=1, b=3 gives "ABC"
    let input = b"DEF".to_vec();
    let args = [ArgValue::Num(1.0), ArgValue::Num(3.0)];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "ABC");
}
#[test]
fn test_affine_cipher_decode_non_alphabetic() {
    let op = AffineCipherDecode;
    // Non-alphabetic characters preserved
    let input = b"hello, world!".to_vec();
    let args = [ArgValue::Num(1.0), ArgValue::Num(0.0)];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "hello, world!");
}
#[test]
fn test_affine_cipher_decode_coprime_check() {
    let op = AffineCipherDecode;
    let input = b"hello".to_vec();
    // a=2 is not coprime to 26 (gcd(2, 26) = 2)
    let args = [ArgValue::Num(2.0), ArgValue::Num(0.0)];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
