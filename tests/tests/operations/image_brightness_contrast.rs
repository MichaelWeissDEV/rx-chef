// Tests for the image_brightness_contrast operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations image_brightness_contrast::

use image::DynamicImage;
use image::ImageFormat;
use rxchef::operation::ArgValue;
use rxchef::operations::image_brightness_contrast::ImageBrightnessContrast;
use rxchef::Operation;
use std::io::Cursor;

#[test]
fn test_brightness_exact_pixel_value() {
    // The documented mapping is brightness/100*255 added per channel.
    let img = image::RgbaImage::from_pixel(1, 1, image::Rgba([10, 20, 30, 255]));
    let mut input = Vec::new();
    DynamicImage::ImageRgba8(img)
        .write_to(&mut Cursor::new(&mut input), ImageFormat::Png)
        .unwrap();
    let output = ImageBrightnessContrast
        .run(input, &[ArgValue::Num(20.0), ArgValue::Num(0.0)])
        .unwrap();
    let pixels = image::load_from_memory(&output).unwrap().to_rgba8();
    let pixel = pixels.get_pixel(0, 0);
    assert_eq!(pixel.0, [61, 71, 81, 255]);
}

// Helper to create a 1x1 white PNG image
fn create_test_image() -> Vec<u8> {
    let img = DynamicImage::new_rgb8(1, 1);
    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
        .unwrap();
    buf
}
#[test]
fn test_image_brightness_contrast_no_change() {
    let op = ImageBrightnessContrast;
    let input = create_test_image();
    let args = [ArgValue::Num(0.0), ArgValue::Num(0.0)];
    let result = op.run(input.clone(), &args).unwrap();
    assert!(!result.is_empty());
}
#[test]
fn test_image_brightness_increase() {
    let op = ImageBrightnessContrast;
    let input = create_test_image();
    let args = [ArgValue::Num(50.0), ArgValue::Num(0.0)];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}
#[test]
fn test_image_contrast_increase() {
    let op = ImageBrightnessContrast;
    let input = create_test_image();
    let args = [ArgValue::Num(0.0), ArgValue::Num(50.0)];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}
#[test]
fn test_empty_input() {
    let op = ImageBrightnessContrast;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
