// Tests for the generate_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_image::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_image::GenerateImageOp;
use rxchef::Operation;

#[test]
fn test_generate_image_greyscale() {
    let op = GenerateImageOp;
    let input = vec![0, 127, 255, 0];
    let args = [
        ArgValue::Str("Greyscale".to_string()),
        ArgValue::Num(1.0),
        ArgValue::Num(2.0),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
    // Should be a PNG
    assert_eq!(&result[0..8], b"\x89PNG\r\n\x1a\n");
}
#[test]
fn test_generate_image_bits() {
    let op = GenerateImageOp;
    let input = vec![0b10101010];
    let args = [
        ArgValue::Str("Bits".to_string()),
        ArgValue::Num(1.0),
        ArgValue::Num(8.0),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}
