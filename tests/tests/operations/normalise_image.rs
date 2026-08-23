// Tests for the normalise_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations normalise_image::

use rxchef::operations::normalise_image::NormaliseImage;
use rxchef::Operation;
use std::io::Cursor;

#[test]
fn test_normalise_stretches_each_channel_to_full_range() {
    let mut image = image::RgbImage::new(2, 1);
    image.put_pixel(0, 0, image::Rgb([10, 20, 30]));
    image.put_pixel(1, 0, image::Rgb([110, 220, 130]));
    let mut png = Vec::new();
    image::DynamicImage::ImageRgb8(image)
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .unwrap();
    let output = NormaliseImage.run(png, &[]).unwrap();
    let pixels = image::load_from_memory(&output).unwrap().to_rgb8();
    assert_eq!(pixels.get_pixel(0, 0).0, [0, 0, 0]);
    assert_eq!(pixels.get_pixel(1, 0).0, [255, 255, 255]);
}

#[test]
fn test_normalise_image_empty_input() {
    let op = NormaliseImage;
    let args = [];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, vec![0u8; 0]);
}

#[test]
fn test_normalise_image_invalid_format() {
    let op = NormaliseImage;
    let invalid_image = vec![0x00, 0x01, 0x02, 0x03]; // Not a valid image
    let args = [];
    let result = op.run(invalid_image, &args);
    assert!(result.is_err());
}

#[test]
fn test_normalise_image_error_handling() {
    let op = NormaliseImage;
    let invalid_data = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Invalid image data
    let args = [];

    let result = op.run(invalid_data, &args);

    // Should return an error for invalid image data
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, rxchef::OperationError::InvalidInput { .. }));
    }
}

#[test]
fn test_normalise_image_unsupported_format() {
    let op = NormaliseImage;
    let unsupported_data = vec![0x47, 0x49, 0x46, 0x38]; // GIF signature
    let args = [];

    let result = op.run(unsupported_data, &args);

    // Should return an error for unsupported format
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, rxchef::OperationError::InvalidInput { .. }));
    }
}

#[test]
fn test_normalise_image_processing_error() {
    let op = NormaliseImage;
    // Incomplete PNG header
    let incomplete_png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A];
    let args = [];

    let result = op.run(incomplete_png, &args);

    // Should return an error for incomplete image
    assert!(result.is_err());
}
