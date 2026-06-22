// Tests for the affine_cipher_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations affine_cipher_encode::

use rxchef::operation::ArgValue;
use rxchef::operations::affine_cipher_encode::AffineCipherEncode;
use rxchef::Operation;

#[test]
fn test_affine_cipher_encode_basic() {
    let op = AffineCipherEncode;
    let input = b"hello".to_vec();
    let args = [ArgValue::Num(1.0), ArgValue::Num(0.0)]; // a=1, b=0 (identity)
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "hello");
}
#[test]
fn test_affine_cipher_encode_caesar_shift() {
    let op = AffineCipherEncode;
    let input = b"abc".to_vec();
    // a=1, b=3 gives Caesar cipher with shift 3
    let args = [ArgValue::Num(1.0), ArgValue::Num(3.0)];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "def");
}
#[test]
fn test_affine_cipher_encode_uppercase() {
    let op = AffineCipherEncode;
    let input = b"ABC".to_vec();
    // a=1, b=3 gives Caesar cipher with shift 3
    let args = [ArgValue::Num(1.0), ArgValue::Num(3.0)];
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "DEF");
}
#[test]
fn test_affine_cipher_encode_non_alphabetic() {
    let op = AffineCipherEncode;
    let input = b"hello, world!".to_vec();
    let args = [ArgValue::Num(1.0), ArgValue::Num(0.0)]; // identity
    let result = op.run(input, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert_eq!(result_str, "hello, world!");
}
#[test]
fn test_affine_cipher_encode_coprime_check() {
    let op = AffineCipherEncode;
    let input = b"hello".to_vec();
    // a=2 is not coprime to 26 (gcd(2, 26) = 2)
    let args = [ArgValue::Num(2.0), ArgValue::Num(0.0)];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
