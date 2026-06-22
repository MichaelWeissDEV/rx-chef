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
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not yet fully implemented"));
}

#[test]
fn test_dither_image_with_data() {
    let op = DitherImage;
    let args = [];
    // Test with some dummy image data
    let dummy_image_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
    let result = op.run(dummy_image_data, &args);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not yet fully implemented"));
}

#[test]
fn test_dither_image_large_input() {
    let op = DitherImage;
    let args = [];
    // Test with larger dummy data
    let dummy_image_data = vec![0xFF; 1024]; // 1KB of dummy data
    let result = op.run(dummy_image_data, &args);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not yet fully implemented"));
}
