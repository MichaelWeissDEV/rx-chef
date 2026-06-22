// Tests for the fletcher32_checksum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations fletcher32_checksum::

use rxchef::operations::fletcher32_checksum::Fletcher32Checksum;
use rxchef::Operation;

#[test]
fn test_fletcher32() {
    let op = Fletcher32Checksum;
    assert_eq!(
        String::from_utf8(op.run(b"abcde".to_vec(), &[]).unwrap()).unwrap(),
        "f04fc729"
    );
    assert_eq!(
        String::from_utf8(op.run(b"abcdefgh".to_vec(), &[]).unwrap()).unwrap(),
        "ebe19591"
    );
}
