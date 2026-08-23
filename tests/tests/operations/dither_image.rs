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
    let truncated_png = vec![0x89, 0x50, 0x4E, 0x47];
    let result = op.run(truncated_png, &args);
    assert!(result.is_err());
}

#[test]
fn test_dither_image_large_input() {
    let op = DitherImage;
    let args = [];
    let invalid_image_data = vec![0xFF; 1024];
    let result = op.run(invalid_image_data, &args);
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

#[test]
fn test_dither_preserves_exact_binary_pattern() {
    use image::{DynamicImage, GrayImage, ImageFormat, Luma};
    use std::io::Cursor;

    // Floyd-Steinberg introduces no diffusion error for values already at the
    // two quantisation endpoints, so this standard checkerboard is invariant.
    let image = GrayImage::from_fn(2, 2, |x, y| Luma([if x == y { 0 } else { 255 }]));
    let mut input = Cursor::new(Vec::new());
    DynamicImage::ImageLuma8(image)
        .write_to(&mut input, ImageFormat::Png)
        .unwrap();
    let output = DitherImage.run(input.into_inner(), &[]).unwrap();
    let decoded = image::load_from_memory(&output).unwrap().to_luma8();
    assert_eq!(decoded.as_raw(), &[0, 255, 255, 0]);
}
