// Tests for the md6 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations md6::

use rxchef::operations::md6::MD6;
use rxchef::Operation;

#[test]
fn test_md6_placeholder() {
    let op = MD6;
    let input = b"test".to_vec();
    let result = op.run(input, &[]);
    assert!(result.is_err());
}
