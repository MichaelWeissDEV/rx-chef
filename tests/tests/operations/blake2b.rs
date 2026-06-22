// Tests for the blake2b operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations blake2b::

use rxchef::operation::ArgValue;
use rxchef::operations::blake2b::BLAKE2b;
use rxchef::Operation;

#[test]
fn test_blake2b_basic() {
    let operation = BLAKE2b;
    let input = b"hello world".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Standard BLAKE2b-512 hash of "hello world" (default)
    assert_eq!(
        output,
        "021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0"
    );
}
#[test]
fn test_blake2b_empty() {
    let operation = BLAKE2b;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Empty string BLAKE2b-512 hash (default)
    assert_eq!(
        output,
        "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce"
    );
}
#[test]
fn test_blake2b_keyed() {
    let operation = BLAKE2b;
    let input = b"hello world".to_vec();
    let args = &[
        ArgValue::Str("512".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("mykey".to_string()),
    ];
    let result = operation.run(input, args).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Should produce different hash with key
    assert!(output.len() == 128); // 512 bits = 128 hex chars
}
