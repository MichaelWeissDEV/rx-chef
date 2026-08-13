// Tests for the dither_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations dither_image::

use rxchef::operations::dither_image::DitherImage;
use rxchef::Operation;

#[test]
fn test_dither_image_empty_input() {
    let op = DitherImage;
    let args = [];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}

#[test]
fn test_dither_image_with_data() {
    let op = DitherImage;
    let args = [];
    // Test with some dummy image data
    let dummy_image_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
    let result = op.run(dummy_image_data, &args);
    assert!(result.is_err());
}

#[test]
fn test_dither_image_large_input() {
    let op = DitherImage;
    let args = [];
    // Test with larger dummy data
    let dummy_image_data = vec![0xFF; 1024]; // 1KB of dummy data
    let result = op.run(dummy_image_data, &args);
    assert!(result.is_err());
}

#[test]
fn test_dither_image_produces_black_and_white_png() {
    use image::{DynamicImage, GrayImage, ImageFormat, Luma};
    use std::io::Cursor;

    let image = GrayImage::from_fn(4, 4, |x, y| Luma([((x + y) * 40) as u8]));
    let mut input = Cursor::new(Vec::new());
    DynamicImage::ImageLuma8(image)
        .write_to(&mut input, ImageFormat::Png)
        .unwrap();
    let output = DitherImage.run(input.into_inner(), &[]).unwrap();
    let decoded = image::load_from_memory(&output).unwrap().to_luma8();
    assert!(decoded.pixels().all(|pixel| matches!(pixel.0[0], 0 | 255)));
}
