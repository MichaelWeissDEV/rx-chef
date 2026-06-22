// Tests for the magic operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations magic::

use rxchef::operations::magic::Magic;
use rxchef::Operation;

#[test]
fn test_magic_entropy() {
    let op = Magic;
    let input = b"aaaaa".to_vec();
    let result = op.run(input, &[]).unwrap();
    // Just check if it returns something valid
    assert!(result.len() > 0);
}
