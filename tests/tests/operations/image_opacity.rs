// Tests for the image_opacity operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations image_opacity::
//
// Expected alpha values follow from the operation's definition — the alpha
// channel is scaled by the requested percentage — so they are computed here
// rather than copied from rx-chef's output.

use rxchef::runtime::{self, RuntimeError};

/// Encode an RGBA image so the alpha channel is under test.
fn png_rgba(pixels: &[(u8, u8, u8, u8)], width: u32, height: u32) -> Vec<u8> {
    use image::{ImageBuffer, Rgba};
    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let (r, g, b, a) = pixels[(y * width + x) as usize];
        Rgba([r, g, b, a])
    });
    let mut encoded = Vec::new();
    image::DynamicImage::ImageRgba8(buffer)
        .write_to(
            &mut std::io::Cursor::new(&mut encoded),
            image::ImageFormat::Png,
        )
        .expect("encoding the test image must succeed");
    encoded
}

fn opacity(input: Vec<u8>, percent: &str) -> Result<Vec<u8>, RuntimeError> {
    runtime::run_operation("Image Opacity", input, &[percent.to_string()])
}

fn alpha_after(start_alpha: u8, percent: &str) -> u8 {
    let out = opacity(png_rgba(&[(10, 20, 30, start_alpha)], 1, 1), percent)
        .expect("applying opacity must succeed");
    image::load_from_memory(&out)
        .expect("the operation must emit a decodable image")
        .to_rgba8()
        .get_pixel(0, 0)
        .0[3]
}

#[test]
fn test_image_opacity_full_opacity_leaves_alpha_unchanged() {
    assert_eq!(alpha_after(255, "100"), 255);
    assert_eq!(alpha_after(128, "100"), 128);
}

#[test]
fn test_image_opacity_halves_the_alpha_channel() {
    // 255 * 0.5 = 127.5, truncated to 127.
    assert_eq!(alpha_after(255, "50"), 127);
}

#[test]
fn test_image_opacity_zero_makes_the_image_fully_transparent() {
    assert_eq!(alpha_after(255, "0"), 0);
    assert_eq!(alpha_after(128, "0"), 0);
}

#[test]
fn test_image_opacity_scales_proportionally() {
    // Each step multiplies the existing alpha rather than replacing it.
    for (start, percent, expected) in [
        (200u8, "50", 100u8),
        (100, "25", 25),
        (255, "20", 51),
        (64, "50", 32),
    ] {
        assert_eq!(
            alpha_after(start, percent),
            expected,
            "alpha {start} at {percent}%"
        );
    }
}

#[test]
fn test_image_opacity_leaves_colour_channels_alone() {
    let out = opacity(png_rgba(&[(11, 22, 33, 255)], 1, 1), "50").unwrap();
    let pixel = image::load_from_memory(&out)
        .unwrap()
        .to_rgba8()
        .get_pixel(0, 0)
        .0;
    assert_eq!([pixel[0], pixel[1], pixel[2]], [11, 22, 33]);
}

#[test]
fn test_image_opacity_preserves_dimensions() {
    let source = png_rgba(
        &[
            (1, 2, 3, 255),
            (4, 5, 6, 255),
            (7, 8, 9, 255),
            (10, 11, 12, 255),
        ],
        2,
        2,
    );
    let out = opacity(source, "50").unwrap();
    let image = image::load_from_memory(&out).unwrap();
    assert_eq!((image.width(), image.height()), (2, 2));
}

#[test]
fn test_image_opacity_empty_input_passes_through() {
    // Project-wide convention: an empty buffer is returned unchanged rather
    // than reported as an undecodable image.
    assert_eq!(opacity(Vec::new(), "50").unwrap(), Vec::<u8>::new());
}

#[test]
fn test_image_opacity_rejects_data_that_is_not_an_image() {
    assert!(opacity(b"definitely not an image".to_vec(), "50").is_err());
}

#[test]
fn test_image_opacity_rejects_a_truncated_image() {
    let mut truncated = png_rgba(&[(0, 0, 0, 255)], 1, 1);
    truncated.truncate(20);
    assert!(opacity(truncated, "50").is_err());
}

#[test]
fn test_image_opacity_does_not_panic_on_binary_input() {
    let binary: Vec<u8> = (0u8..=255).collect();
    let result = opacity(binary, "50");
    assert!(
        result.is_ok() || result.is_err(),
        "the call must return rather than panic"
    );
}
