// Tests for the rotate_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rotate_image::

use image::DynamicImage;
use image::ImageFormat;
use rxchef::operation::ArgValue;
use rxchef::operations::rotate_image::RotateImage;
use rxchef::Operation;
use std::io::Cursor;

#[test]
fn test_rotate_90() {
    // Independent 90-degree rotation oracle for a two-pixel red/green row.
    let op = RotateImage;
    // Create a 2x1 red image
    let mut img = image::RgbImage::new(2, 1);
    img.put_pixel(0, 0, image::Rgb([255, 0, 0]));
    img.put_pixel(1, 0, image::Rgb([0, 255, 0]));
    let mut input = Vec::new();
    DynamicImage::ImageRgb8(img)
        .write_to(&mut Cursor::new(&mut input), ImageFormat::Png)
        .unwrap();
    let args = [ArgValue::Num(90.0)];
    let result = op.run(input, &args).unwrap();
    let output_img = image::load_from_memory(&result).unwrap().to_rgb8();
    assert_eq!(output_img.width(), 1);
    assert_eq!(output_img.height(), 2);
    assert_eq!(output_img.get_pixel(0, 0), &image::Rgb([255, 0, 0]));
    assert_eq!(output_img.get_pixel(0, 1), &image::Rgb([0, 255, 0]));
}
#[test]
fn test_rotate_180() {
    let op = RotateImage;
    let mut img = image::RgbImage::new(2, 1);
    img.put_pixel(0, 0, image::Rgb([255, 0, 0]));
    img.put_pixel(1, 0, image::Rgb([0, 255, 0]));
    let mut input = Vec::new();
    DynamicImage::ImageRgb8(img)
        .write_to(&mut Cursor::new(&mut input), ImageFormat::Png)
        .unwrap();
    let args = [ArgValue::Num(180.0)];
    let result = op.run(input, &args).unwrap();
    let output_img = image::load_from_memory(&result).unwrap().to_rgb8();
    assert_eq!(output_img.width(), 2);
    assert_eq!(output_img.height(), 1);
    assert_eq!(output_img.get_pixel(0, 0), &image::Rgb([0, 255, 0]));
    assert_eq!(output_img.get_pixel(1, 0), &image::Rgb([255, 0, 0]));
}

#[test]
fn test_rotate_arbitrary_angle() {
    let op = RotateImage;
    let img = image::RgbaImage::from_pixel(5, 5, image::Rgba([255, 0, 0, 255]));
    let mut input = Vec::new();
    DynamicImage::ImageRgba8(img)
        .write_to(&mut Cursor::new(&mut input), ImageFormat::Png)
        .unwrap();
    let result = op.run(input, &[ArgValue::Num(45.0)]).unwrap();
    let output = image::load_from_memory(&result).unwrap();
    assert_eq!(output.width(), 5);
    assert_eq!(output.height(), 5);
}
#[test]
fn test_invalid_input() {
    let op = RotateImage;
    let input = b"not an image".to_vec();
    let args = [ArgValue::Num(90.0)];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_empty_input() {
    let op = RotateImage;
    let input = Vec::new();
    let args = [ArgValue::Num(90.0)];
    let result = op.run(input, &args).unwrap();
    assert!(result.is_empty());
}
