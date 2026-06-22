// Tests for the extract_dates operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_dates::

use rxchef::operation::ArgValue;
use rxchef::operations::extract_dates::ExtractDates;
use rxchef::Operation;

#[test]
fn test_extract_dates() {
    let op = ExtractDates;
    let input = b"Today is 2023-10-27 and tomorrow is 28/10/2023. Also 10/29/2023.".to_vec();
    let args = [ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "2023-10-27\n28/10/2023\n10/29/2023");
}
