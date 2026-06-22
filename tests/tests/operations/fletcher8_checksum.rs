// Tests for the fletcher8_checksum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations fletcher8_checksum::

use rxchef::operations::fletcher8_checksum::Fletcher8Checksum;
use rxchef::Operation;

#[test]
fn test_fletcher8() {
    let op = Fletcher8Checksum;
    assert_eq!(
        String::from_utf8(op.run(b"abcde".to_vec(), &[]).unwrap()).unwrap(),
        "50"
    );
    assert_eq!(
        String::from_utf8(op.run(b"abcdefgh".to_vec(), &[]).unwrap()).unwrap(),
        "69"
    );
}
