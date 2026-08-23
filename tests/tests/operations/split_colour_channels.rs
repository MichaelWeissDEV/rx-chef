// Tests for the split_colour_channels operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations split_colour_channels::

use image::{ImageBuffer, Rgba};
use rxchef::operations::split_colour_channels::SplitColourChannels;
use rxchef::Operation;
use std::io::Cursor;
use std::io::Read;

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
    let mut archive = zip::ZipArchive::new(Cursor::new(result)).unwrap();
    let expected = [
        ("red.png", [255, 0, 0, 255]),
        ("green.png", [0, 128, 0, 255]),
        ("blue.png", [0, 0, 64, 255]),
    ];
    assert_eq!(archive.len(), expected.len());
    for (name, rgba) in expected {
        let mut bytes = Vec::new();
        archive.by_name(name).unwrap().read_to_end(&mut bytes).unwrap();
        let pixels = image::load_from_memory(&bytes).unwrap().to_rgba8();
        let pixel = pixels.get_pixel(0, 0);
        assert_eq!(pixel.0, rgba, "wrong channel data in {name}");
    }
}
#[test]
fn test_split_colour_channels_empty() {
    let op = SplitColourChannels;
    let input = Vec::new();
    let result = op.run(input, &[]).unwrap();
    assert!(result.is_empty());
}
