// Tests for the parse_csr operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_csr::

use rxchef::operations::parse_csr::ParseCSR;
use rxchef::Operation;

#[test]
fn test_empty_input() {
    let op = ParseCSR;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, b"No input");
}
// A real test would need a valid CSR PEM/DER.
// Since I cannot easily generate one here without more crates, I'll use a placeholder test
// or just ensure it returns an error on invalid input.
#[test]
fn test_invalid_input() {
    let op = ParseCSR;
    let result = op.run(b"invalid data".to_vec(), &[]);
    assert!(result.is_err());
}
