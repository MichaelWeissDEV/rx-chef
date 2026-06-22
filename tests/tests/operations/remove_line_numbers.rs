// Tests for the remove_line_numbers operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations remove_line_numbers::

use rxchef::operations::remove_line_numbers::RemoveLineNumbers;
use rxchef::Operation;

#[test]
fn test_remove_line_numbers() {
    let op = RemoveLineNumbers;
    let input = " 1: First line\n 2: Second line\n 3: Third line"
        .as_bytes()
        .to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(),
        " First line\n Second line\n Third line"
    );
}
#[test]
fn test_remove_line_numbers_different_format() {
    let op = RemoveLineNumbers;
    let input = "1. One\n2. Two\n3. Three".as_bytes().to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), " One\n Two\n Three");
}
#[test]
fn test_remove_line_numbers_no_numbers() {
    let op = RemoveLineNumbers;
    let input = "No numbers here\nJust text".as_bytes().to_vec();
    let result = op.run(input.clone(), &[]).unwrap();
    assert_eq!(result, input);
}
