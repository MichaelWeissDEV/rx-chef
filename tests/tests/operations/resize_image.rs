// Tests for the resize_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations resize_image::

use rxchef::operation::ArgValue;
use rxchef::operations::resize_image::ResizeImage;
use rxchef::Operation;

// A tiny 1x1 white PNG
const TINY_PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x04, 0x00, 0x00, 0x00, 0xb5, 0x1c, 0x0c,
    0x02, 0x00, 0x00, 0x00, 0x0b, 0x49, 0x44, 0x41, 0x54, 0x78, 0xda, 0x63, 0xfc, 0xff, 0x1f, 0x00,
    0x03, 0x03, 0x02, 0x00, 0xef, 0xa2, 0xa7, 0x5b, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44,
    0xae, 0x42, 0x60, 0x82,
];
#[test]
fn test_resize_pixels() {
    let op = ResizeImage;
    let args = [
        ArgValue::Num(10.0),
        ArgValue::Num(10.0),
        ArgValue::Str("Pixels".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("Bilinear".to_string()),
    ];
    let result = op.run(TINY_PNG.to_vec(), &args).unwrap();
    let img = image::load_from_memory(&result).unwrap();
    assert_eq!(img.width(), 10);
    assert_eq!(img.height(), 10);
}
#[test]
fn test_resize_percent() {
    let op = ResizeImage;
    let args = [
        ArgValue::Num(200.0),
        ArgValue::Num(200.0),
        ArgValue::Str("Percent".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("Nearest Neighbour".to_string()),
    ];
    let result = op.run(TINY_PNG.to_vec(), &args).unwrap();
    let img = image::load_from_memory(&result).unwrap();
    assert_eq!(img.width(), 2);
    assert_eq!(img.height(), 2);
}
#[test]
fn test_resize_aspect_ratio() {
    let op = ResizeImage;
    // 1x1 image resized to 10x20 with aspect ratio should be 10x10
    let args = [
        ArgValue::Num(10.0),
        ArgValue::Num(20.0),
        ArgValue::Str("Pixels".to_string()),
        ArgValue::Bool(true),
        ArgValue::Str("Bilinear".to_string()),
    ];
    let result = op.run(TINY_PNG.to_vec(), &args).unwrap();
    let img = image::load_from_memory(&result).unwrap();
    assert_eq!(img.width(), 10);
    assert_eq!(img.height(), 10);
}
#[test]
fn test_resize_invalid_input() {
    let op = ResizeImage;
    let args = [
        ArgValue::Num(10.0),
        ArgValue::Num(10.0),
        ArgValue::Str("Pixels".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("Bilinear".to_string()),
    ];
    let result = op.run(vec![1, 2, 3, 4], &args);
    assert!(result.is_err());
}
