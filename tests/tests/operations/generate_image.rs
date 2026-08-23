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
    let image = image::load_from_memory(&result).unwrap().to_rgba8();
    assert_eq!(image.dimensions(), (2, 2));
    assert_eq!(image.get_pixel(0, 0).0, [0, 0, 0, 255]);
    assert_eq!(image.get_pixel(1, 0).0, [127, 127, 127, 255]);
    assert_eq!(image.get_pixel(0, 1).0, [255, 255, 255, 255]);
    assert_eq!(image.get_pixel(1, 1).0, [0, 0, 0, 255]);
}

#[test]
fn test_generate_image_empty_input_boundary() {
    assert_eq!(
        GenerateImageOp.run(Vec::new(), &[]).unwrap(),
        Vec::<u8>::new()
    );
}

#[test]
fn test_generate_image_rejects_zero_scale() {
    let args = [
        ArgValue::Str("Greyscale".into()),
        ArgValue::Num(0.0),
        ArgValue::Num(1.0),
    ];
    assert!(GenerateImageOp.run(vec![0], &args).is_err());
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
