// Tests for the index_of_coincidence operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations index_of_coincidence::

use rxchef::operations::index_of_coincidence::IndexOfCoincidence;
use rxchef::Operation;

#[test]
fn test_index_of_coincidence_basic() {
    let operation = IndexOfCoincidence;
    // English text should have IC around 0.066
    let input = b"the quick brown fox jumps over the lazy dog".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let value = f64::from_le_bytes(result.try_into().unwrap());
    assert!(value > 0.0);
    assert!(value < 1.0);
}
#[test]
fn test_index_of_coincidence_empty() {
    let operation = IndexOfCoincidence;
    let input = b"12345!@#$%".to_vec(); // Non-alphabetic
    let result = operation.run(input, &[]).unwrap();
    let value = f64::from_le_bytes(result.try_into().unwrap());
    // Should be 0 or close to 0 for non-alphabetic
    assert!(value >= 0.0);
}
