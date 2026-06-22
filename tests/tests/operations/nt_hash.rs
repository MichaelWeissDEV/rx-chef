// Tests for the nt_hash operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations nt_hash::

use rxchef::operations::nt_hash::NTHash;
use rxchef::Operation;

#[test]
fn test_nthash() {
    let op = NTHash;
    let input = b"password".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, b"8846F7EAEE8FB117AD06BDD830B7586C");
}
#[test]
fn test_nthash_empty() {
    let op = NTHash;
    let input = b"".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, b"31D6CFE0D16AE931B73C59D7E0C089C0");
}
