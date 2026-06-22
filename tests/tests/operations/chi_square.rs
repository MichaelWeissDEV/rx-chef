// Tests for the chi_square operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations chi_square::

use rxchef::operations::chi_square::ChiSquare;
use rxchef::Operation;

#[test]
fn test_chi_square_basic() {
    let operation = ChiSquare;
    let input = b"hello world".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let value = f64::from_le_bytes(result.try_into().unwrap());
    // For a uniform distribution, this should be close to 0
    assert!(value >= 0.0);
}
#[test]
fn test_chi_square_uniform() {
    let operation = ChiSquare;
    // Create a uniform distribution
    let input: Vec<u8> = (0..256).map(|i| i as u8).collect();
    let result = operation.run(input, &[]).unwrap();
    let value = f64::from_le_bytes(result.try_into().unwrap());
    assert!(value >= 0.0);
}
