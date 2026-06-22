// Tests for the fletcher16_checksum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations fletcher16_checksum::

use rxchef::operations::fletcher16_checksum::Fletcher16Checksum;
use rxchef::Operation;

#[test]
fn test_fletcher16() {
    let op = Fletcher16Checksum;
    assert_eq!(
        String::from_utf8(op.run(b"abcde".to_vec(), &[]).unwrap()).unwrap(),
        "c8f0"
    );
    assert_eq!(
        String::from_utf8(op.run(b"abcdefgh".to_vec(), &[]).unwrap()).unwrap(),
        "0627"
    );
}
