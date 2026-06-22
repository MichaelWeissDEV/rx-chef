// Tests for the lm_hash operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lm_hash::

use rxchef::operations::lm_hash::LMHash;
use rxchef::Operation;

#[test]
fn test_lm_hash_basic() {
    let op = LMHash;
    let input = b"password".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "e52cac67419a9a224a3b108f3fa6cb6d"
    );
}
#[test]
fn test_lm_hash_empty() {
    let op = LMHash;
    let input = b"".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "aad3b435b51404eeaad3b435b51404ee"
    );
}
#[test]
fn test_lm_hash_long() {
    let op = LMHash;
    let input = b"thisisalongpassword".to_vec();
    let result = op.run(input, &[]).unwrap();
    // LM Hash only uses first 14 characters
    // "THISISALONGPAS"
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "8a6d8380cac58f22d171de7eff6a9f31"
    );
}
