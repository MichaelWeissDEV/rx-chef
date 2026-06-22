// Tests for the view_bit_plane operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations view_bit_plane::

use image::{ImageFormat, RgbaImage};
use rxchef::operation::ArgValue;
use rxchef::operations::view_bit_plane::ViewBitPlane;
use rxchef::Operation;
use std::io::Cursor;

fn create_test_image() -> Vec<u8> {
    let mut img = RgbaImage::new(2, 2);
    // Pixel 0,0: Red channel has bit 0 as 1
    img.put_pixel(0, 0, image::Rgba([1, 0, 0, 255]));
    // Pixel 1,0: Red channel has bit 0 as 0
    img.put_pixel(1, 0, image::Rgba([2, 0, 0, 255]));
    // Pixel 0,1: Red channel has bit 1 as 1
    img.put_pixel(0, 1, image::Rgba([2, 0, 0, 255]));
    // Pixel 1,1: Red channel has bit 1 as 0
    img.put_pixel(1, 1, image::Rgba([1, 0, 0, 255]));
    let mut output = Vec::new();
    let mut cursor = Cursor::new(&mut output);
    img.write_to(&mut cursor, ImageFormat::Png).unwrap();
    output
}
#[test]
fn test_view_bit_plane_bit0_red() {
    let op = ViewBitPlane;
    let input = create_test_image();
    let args = [ArgValue::Str("Red".to_string()), ArgValue::Num(0.0)];
    let result = op.run(input, &args).unwrap();
    let out_img = image::load_from_memory(&result).unwrap().into_rgba8();
    // Pixel 0,0: originally 1 (bit 0 is 1), so output should be 0 (black)
    assert_eq!(out_img.get_pixel(0, 0)[0], 0);
    // Pixel 1,0: originally 2 (bit 0 is 0), so output should be 255 (white)
    assert_eq!(out_img.get_pixel(1, 0)[0], 255);
}
#[test]
fn test_view_bit_plane_bit1_red() {
    let op = ViewBitPlane;
    let input = create_test_image();
    let args = [ArgValue::Str("Red".to_string()), ArgValue::Num(1.0)];
    let result = op.run(input, &args).unwrap();
    let out_img = image::load_from_memory(&result).unwrap().into_rgba8();
    // Pixel 0,1: originally 2 (bit 1 is 1), so output should be 0 (black)
    assert_eq!(out_img.get_pixel(0, 1)[0], 0);
    // Pixel 1,1: originally 1 (bit 1 is 0), so output should be 255 (white)
    assert_eq!(out_img.get_pixel(1, 1)[0], 255);
}
#[test]
fn test_view_bit_plane_invalid_bit() {
    let op = ViewBitPlane;
    let input = create_test_image();
    let args = [ArgValue::Str("Red".to_string()), ArgValue::Num(8.0)];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_view_bit_plane_invalid_image() {
    let op = ViewBitPlane;
    let input = vec![1, 2, 3, 4, 5];
    let args = [ArgValue::Str("Red".to_string()), ArgValue::Num(0.0)];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
