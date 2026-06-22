// Tests for the convert_to_nato_alphabet operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations convert_to_nato_alphabet::

use rxchef::operations::convert_to_nato_alphabet::ConvertToNATOAlphabet;
use rxchef::Operation;

#[test]
fn test_convert_to_nato_basic() {
    let operation = ConvertToNATOAlphabet;
    let input = b"A B C".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("Alfa"));
    assert!(output.contains("Bravo"));
    assert!(output.contains("Charlie"));
}
#[test]
fn test_convert_to_nato_numbers() {
    let operation = ConvertToNATOAlphabet;
    let input = b"123".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("One"));
    assert!(output.contains("Two"));
    assert!(output.contains("Three"));
}
#[test]
fn test_convert_to_nato_unknown_char() {
    let operation = ConvertToNATOAlphabet;
    let input = b"Hello!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // Non-matching chars should be preserved
    assert!(output.contains("!"));
}
