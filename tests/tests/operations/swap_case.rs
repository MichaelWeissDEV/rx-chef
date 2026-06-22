// Tests for the swap_case operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations swap_case::

use rxchef::operations::swap_case::SwapCase;
use rxchef::Operation;

#[test]
fn test_swap_case_basic() {
    let operation = SwapCase;
    let input = b"Hello World".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "hELLO wORLD");
}
#[test]
fn test_swap_case_all_upper() {
    let operation = SwapCase;
    let input = b"HELLO".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "hello");
}
#[test]
fn test_swap_case_all_lower() {
    let operation = SwapCase;
    let input = b"hello".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "HELLO");
}
#[test]
fn test_swap_case_mixed() {
    let operation = SwapCase;
    let input = b"Hello123World!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "hELLO123wORLD!");
}
