// Tests for the murmur_hash3 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations murmur_hash3::

use rxchef::operation::ArgValue;
use rxchef::operations::murmur_hash3::MurmurHash3;
use rxchef::Operation;

#[test]
fn test_murmurhash3_basic() {
    let op = MurmurHash3;
    let input = b"hello".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "613153351");
}
#[test]
fn test_murmurhash3_seed() {
    let op = MurmurHash3;
    let input = b"hello".to_vec();
    let args = [ArgValue::Num(123.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "1573043710");
}
#[test]
fn test_murmurhash3_signed() {
    let op = MurmurHash3;
    let input = b"The quick brown fox jumps over the lazy dog".to_vec();
    let args = [ArgValue::Num(0.0), ArgValue::Bool(true)];
    let result = op.run(input, &args).unwrap();
    // Unsigned: 1321453272
    assert_eq!(String::from_utf8(result).unwrap(), "776992547");
    let input2 = b"test".to_vec();
    let result2 = op.run(input2, &args).unwrap();
    // Unsigned: 3127628307 -> Signed: -1167338989
    assert_eq!(String::from_utf8(result2).unwrap(), "-1167338989");
}
