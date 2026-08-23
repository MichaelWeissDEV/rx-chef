// Tests for the image_hue_saturation_lightness operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations image_hue_saturation_lightness::

use image::DynamicImage;
use image::ImageFormat;
use rxchef::operation::ArgValue;
use rxchef::operations::image_hue_saturation_lightness::ImageHueSaturationLightness;
use rxchef::Operation;
use std::io::Cursor;

#[test]
fn test_hue_rotation_red_to_green() {
    // HSL(0°,100%,50%) + 120° is exactly HSL(120°,100%,50%), pure green.
    let img = image::RgbaImage::from_pixel(1, 1, image::Rgba([255, 0, 0, 255]));
    let mut input = Vec::new();
    DynamicImage::ImageRgba8(img)
        .write_to(&mut Cursor::new(&mut input), ImageFormat::Png)
        .unwrap();
    let output = ImageHueSaturationLightness
        .run(
            input,
            &[ArgValue::Num(120.0), ArgValue::Num(0.0), ArgValue::Num(0.0)],
        )
        .unwrap();
    let pixels = image::load_from_memory(&output).unwrap().to_rgba8();
    let pixel = pixels.get_pixel(0, 0);
    assert_eq!(pixel.0, [0, 255, 0, 255]);
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
fn test_image_hsl_no_change() {
    let op = ImageHueSaturationLightness;
    let input = create_test_image();
    let args = [ArgValue::Num(0.0), ArgValue::Num(0.0), ArgValue::Num(0.0)];
    let result = op.run(input.clone(), &args).unwrap();
    assert!(!result.is_empty());
}
#[test]
fn test_image_hue_change() {
    let op = ImageHueSaturationLightness;
    let input = create_test_image();
    let args = [ArgValue::Num(180.0), ArgValue::Num(0.0), ArgValue::Num(0.0)];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}
#[test]
fn test_empty_input() {
    let op = ImageHueSaturationLightness;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
