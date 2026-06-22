// Tests for the generate_lorem_ipsum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_lorem_ipsum::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_lorem_ipsum::GenerateLoremIpsum;
use rxchef::Operation;

#[test]
fn test_lorem_ipsum_paragraphs() {
    let op = GenerateLoremIpsum;
    let result = op
        .run(
            vec![],
            &[ArgValue::Num(2.0), ArgValue::Str("Paragraphs".to_string())],
        )
        .expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    assert!(s.starts_with("Lorem ipsum dolor sit amet"));
    // Two paragraphs separated by double newline
    assert!(s.contains("\n\n"));
}
#[test]
fn test_lorem_ipsum_words() {
    let op = GenerateLoremIpsum;
    let result = op
        .run(
            vec![],
            &[ArgValue::Num(5.0), ArgValue::Str("Words".to_string())],
        )
        .expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    assert!(s.starts_with("Lorem ipsum dolor sit amet"));
}
#[test]
fn test_lorem_ipsum_bytes() {
    let op = GenerateLoremIpsum;
    let result = op
        .run(
            vec![],
            &[ArgValue::Num(50.0), ArgValue::Str("Bytes".to_string())],
        )
        .expect("should succeed");
    assert_eq!(result.len(), 50);
}
#[test]
fn test_lorem_ipsum_invalid_type() {
    let op = GenerateLoremIpsum;
    let result = op.run(
        vec![],
        &[ArgValue::Num(3.0), ArgValue::Str("Pages".to_string())],
    );
    assert!(result.is_err());
}
