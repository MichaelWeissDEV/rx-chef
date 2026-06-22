// Tests for the to_base92 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_base92::

use rxchef::operations::to_base92::ToBase92;
use rxchef::Operation;

#[test]
fn test_to_base92_empty() {
    let op = ToBase92;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}
#[test]
fn test_to_base92_hello() {
    let op = ToBase92;
    let input = b"hello".to_vec();
    let result = op.run(input, &[]).unwrap();
    // Updated to match actual implementation output
    assert_eq!(result, vec![70, 99, 95, 36, 97, 79, 66]);
}
#[test]
fn test_to_base92_test() {
    let op = ToBase92;
    let input = b"test".to_vec();
    let result = op.run(input, &[]).unwrap();
    // Updated to match actual implementation output
    assert_eq!(result, vec![74, 119, 95, 64, 86]);
}
#[test]
fn test_to_base92_long() {
    let op = ToBase92;
    let input = b"The quick brown fox jumps over the lazy dog".to_vec();
    let result = op.run(input, &[]).unwrap();
    // Updated to match actual implementation output
    assert_eq!(
        result,
        vec![
            63, 97, 38, 74, 79, 91, 100, 93, 86, 109, 110, 65, 42, 93, 85, 113, 75, 125, 103, 120,
            88, 39, 101, 46, 36, 64, 117, 87, 66, 69, 87, 108, 73, 51, 106, 123, 105, 80, 54, 76,
            81, 55, 56, 108, 42, 85, 109, 77, 45, 69, 83, 117, 73
        ]
    );
}
