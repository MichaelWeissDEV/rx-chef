// Tests for the sharpen_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sharpen_image::

use image::ImageFormat;
use image::{DynamicImage, Rgba, RgbaImage};
use rxchef::operation::ArgValue;
use rxchef::operations::sharpen_image::SharpenImage;
use rxchef::Operation;
use std::io::Cursor;

#[test]
fn test_sharpen_image_invalid_input() {
    let op = SharpenImage;
    let input = b"not an image".to_vec();
    let args = [ArgValue::Num(2.0), ArgValue::Num(1.0), ArgValue::Num(10.0)];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_sharpen_image_basic() {
    let op = SharpenImage;
    // Create a 2x2 black image
    let mut img = RgbaImage::new(2, 2);
    img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
    img.put_pixel(1, 1, Rgba([0, 255, 0, 255]));
    let mut input = Vec::new();
    DynamicImage::ImageRgba8(img)
        .write_to(&mut Cursor::new(&mut input), ImageFormat::Png)
        .unwrap();
    let args = [ArgValue::Num(1.0), ArgValue::Num(1.0), ArgValue::Num(0.0)];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
    let out_img = image::load_from_memory(&result).unwrap();
    assert_eq!(out_img.width(), 2);
    assert_eq!(out_img.height(), 2);
}
