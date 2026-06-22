// Tests for the fletcher64_checksum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations fletcher64_checksum::

use rxchef::operations::fletcher64_checksum::Fletcher64Checksum;
use rxchef::Operation;

#[test]
fn test_fletcher64() {
    let op = Fletcher64Checksum;
    assert_eq!(
        String::from_utf8(op.run(b"abcde".to_vec(), &[]).unwrap()).unwrap(),
        "c8c6c527646362c6"
    );
    assert_eq!(
        String::from_utf8(op.run(b"abcdefgh".to_vec(), &[]).unwrap()).unwrap(),
        "312e2b28cccac8c6"
    );
}
