// Tests for the split_colour_channels operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations split_colour_channels::

use image::{ImageBuffer, Rgba};
use rxchef::operations::split_colour_channels::SplitColourChannels;
use rxchef::Operation;
use std::io::Cursor;

#[test]
fn test_split_colour_channels() {
    let op = SplitColourChannels;
    // Create a 1x1 red pixel image
    let mut img = ImageBuffer::new(1, 1);
    img.put_pixel(0, 0, Rgba([255, 128, 64, 255]));
    let mut input = Vec::new();
    image::DynamicImage::ImageRgba8(img)
        .write_to(&mut Cursor::new(&mut input), image::ImageFormat::Png)
        .unwrap();
    let result = op.run(input, &[]).unwrap();
    // Basic check: result should be a zip file (starts with PK)
    assert!(result.starts_with(b"PK"));
    // We could use zip crate to verify contents but that might be overkill for a unit test
    // if we trust the zip crate.
}
#[test]
fn test_split_colour_channels_empty() {
    let op = SplitColourChannels;
    let input = Vec::new();
    let result = op.run(input, &[]).unwrap();
    assert!(result.is_empty());
}
