// Tests for the invert_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations invert_image::

use rxchef::operations::invert_image::InvertImage;
use rxchef::Operation;

/// Build a minimal in-memory PNG so these tests carry no binary fixtures.
fn png(pixels: &[(u8, u8, u8)], width: u32, height: u32) -> Vec<u8> {
    use image::{ImageBuffer, Rgb};

    let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let (r, g, b) = pixels[(y * width + x) as usize];
        Rgb([r, g, b])
    });
    let mut encoded = Vec::new();
    image::DynamicImage::ImageRgb8(buffer)
        .write_to(
            &mut std::io::Cursor::new(&mut encoded),
            image::ImageFormat::Png,
        )
        .expect("encoding the test image must succeed");
    encoded
}

fn invert(input: Vec<u8>) -> image::RgbaImage {
    let output = InvertImage.run(input, &[]).expect("inversion must succeed");
    image::load_from_memory(&output)
        .expect("the operation must emit a decodable image")
        .to_rgba8()
}

#[test]
fn test_invert_image_maps_each_channel_to_255_minus_value() {
    let inverted = invert(png(&[(0, 0, 0)], 1, 1));
    let pixel = inverted.get_pixel(0, 0);
    assert_eq!([pixel[0], pixel[1], pixel[2]], [255, 255, 255]);
}

#[test]
fn test_invert_image_white_becomes_black() {
    let inverted = invert(png(&[(255, 255, 255)], 1, 1));
    let pixel = inverted.get_pixel(0, 0);
    assert_eq!([pixel[0], pixel[1], pixel[2]], [0, 0, 0]);
}

#[test]
fn test_invert_image_inverts_channels_independently() {
    let inverted = invert(png(&[(255, 0, 128)], 1, 1));
    let pixel = inverted.get_pixel(0, 0);
    assert_eq!([pixel[0], pixel[1], pixel[2]], [0, 255, 127]);
}

#[test]
fn test_invert_image_preserves_dimensions() {
    let inverted = invert(png(&[(1, 2, 3), (4, 5, 6), (7, 8, 9), (10, 11, 12)], 2, 2));
    assert_eq!(inverted.dimensions(), (2, 2));
}

#[test]
fn test_invert_image_is_its_own_inverse() {
    let original = png(
        &[(10, 20, 30), (200, 100, 50), (0, 0, 0), (255, 255, 255)],
        2,
        2,
    );
    let once = InvertImage.run(original.clone(), &[]).unwrap();
    let twice = invert(once);
    let source = image::load_from_memory(&original).unwrap().to_rgba8();
    for (left, right) in source.pixels().zip(twice.pixels()) {
        assert_eq!(left[0], right[0]);
        assert_eq!(left[1], right[1]);
        assert_eq!(left[2], right[2]);
    }
}

#[test]
fn test_invert_image_rejects_data_that_is_not_an_image() {
    assert!(InvertImage
        .run(b"definitely not an image".to_vec(), &[])
        .is_err());
}

#[test]
fn test_invert_image_empty_input_produces_empty_output() {
    // Project-wide convention: an empty buffer passes through untouched
    // instead of being reported as an undecodable image.
    assert_eq!(InvertImage.run(Vec::new(), &[]).unwrap(), Vec::<u8>::new());
}

#[test]
fn test_invert_image_rejects_a_truncated_png() {
    let mut truncated = png(&[(0, 0, 0)], 1, 1);
    truncated.truncate(20);
    assert!(InvertImage.run(truncated, &[]).is_err());
}
