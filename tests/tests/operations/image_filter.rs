// Tests for the image_filter operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations image_filter::
//
// Fixtures are built in memory rather than checked in, so the expected pixel
// values can be derived from the filter's defining formula instead of from
// rx-chef's own output.
//
// Sepia uses the widely published matrix (Microsoft's original sepia tone
// recipe), which is also what upstream applies:
//   r' = 0.393r + 0.769g + 0.189b
//   g' = 0.349r + 0.686g + 0.168b
//   b' = 0.272r + 0.534g + 0.131b
// Greyscale uses the image crate's luma conversion (ITU-R BT.601 luminance).

use rxchef::runtime::{self, RuntimeError};

/// Encode a one-pixel-per-entry RGB image as PNG.
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

fn filter(input: Vec<u8>, kind: &str) -> Result<Vec<u8>, RuntimeError> {
    runtime::run_operation("Image Filter", input, &[kind.to_string()])
}

fn filtered_pixel(pixels: &[(u8, u8, u8)], kind: &str) -> [u8; 4] {
    let out = filter(png(pixels, 1, 1), kind).expect("filtering must succeed");
    let image = image::load_from_memory(&out)
        .expect("the operation must emit a decodable image")
        .to_rgba8();
    image.get_pixel(0, 0).0
}

/// The published sepia matrix, computed independently of the implementation.
fn expected_sepia(r: u8, g: u8, b: u8) -> [u8; 3] {
    let (r, g, b) = (r as f32, g as f32, b as f32);
    [
        ((r * 0.393) + (g * 0.769) + (b * 0.189)).min(255.0) as u8,
        ((r * 0.349) + (g * 0.686) + (b * 0.168)).min(255.0) as u8,
        ((r * 0.272) + (g * 0.534) + (b * 0.131)).min(255.0) as u8,
    ]
}

#[test]
fn test_image_filter_greyscale_makes_all_channels_equal() {
    // A greyscale pixel has r == g == b by definition.
    for colour in [(255, 0, 0), (0, 255, 0), (0, 0, 255), (12, 200, 90)] {
        let pixel = filtered_pixel(&[colour], "Greyscale");
        assert_eq!(
            (pixel[0], pixel[1]),
            (pixel[1], pixel[2]),
            "greyscale left unequal channels for {colour:?}: {pixel:?}"
        );
    }
}

#[test]
fn test_image_filter_greyscale_preserves_black_and_white() {
    assert_eq!(filtered_pixel(&[(0, 0, 0)], "Greyscale")[0], 0);
    assert_eq!(filtered_pixel(&[(255, 255, 255)], "Greyscale")[0], 255);
}

#[test]
fn test_image_filter_greyscale_weights_green_above_red_above_blue() {
    // Luminance weighting: pure green is brighter than pure red, which is
    // brighter than pure blue.
    let red = filtered_pixel(&[(255, 0, 0)], "Greyscale")[0];
    let green = filtered_pixel(&[(0, 255, 0)], "Greyscale")[0];
    let blue = filtered_pixel(&[(0, 0, 255)], "Greyscale")[0];
    assert!(green > red, "green {green} should exceed red {red}");
    assert!(red > blue, "red {red} should exceed blue {blue}");
}

#[test]
fn test_image_filter_sepia_matches_the_published_matrix() {
    for colour in [
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (100, 150, 200),
        (0, 0, 0),
    ] {
        let pixel = filtered_pixel(&[colour], "Sepia");
        let expected = expected_sepia(colour.0, colour.1, colour.2);
        assert_eq!(
            [pixel[0], pixel[1], pixel[2]],
            expected,
            "sepia mismatch for {colour:?}"
        );
    }
}

#[test]
fn test_image_filter_sepia_clamps_at_255() {
    // White exceeds 255 in the red row (0.393+0.769+0.189 = 1.351) and must
    // saturate rather than wrap.
    let pixel = filtered_pixel(&[(255, 255, 255)], "Sepia");
    assert_eq!(pixel[0], 255);
}

#[test]
fn test_image_filter_accepts_the_grayscale_spelling() {
    assert_eq!(
        filtered_pixel(&[(10, 20, 30)], "Grayscale"),
        filtered_pixel(&[(10, 20, 30)], "Greyscale"),
        "both spellings must select the same filter"
    );
}

#[test]
fn test_image_filter_preserves_dimensions() {
    let source = png(&[(1, 2, 3), (4, 5, 6), (7, 8, 9), (10, 11, 12)], 2, 2);
    let out = filter(source, "Greyscale").unwrap();
    let image = image::load_from_memory(&out).unwrap();
    assert_eq!(image.width(), 2);
    assert_eq!(image.height(), 2);
}

#[test]
fn test_image_filter_rejects_an_unknown_filter_type() {
    // `Filter type` declares its choices, so the runtime rejects an unknown
    // value centrally before the operation runs — the same treatment every
    // other enumerated argument gets.
    let error = filter(png(&[(0, 0, 0)], 1, 1), "NotAFilter")
        .expect_err("an unknown filter must be rejected");
    assert!(
        matches!(&error, RuntimeError::InvalidArgument { name, .. } if name == "Filter type"),
        "expected InvalidArgument naming the filter, got {error:?}"
    );
}

#[test]
fn test_image_filter_rejects_data_that_is_not_an_image() {
    assert!(filter(b"definitely not an image".to_vec(), "Greyscale").is_err());
}

#[test]
fn test_image_filter_rejects_a_truncated_image() {
    let mut truncated = png(&[(0, 0, 0)], 1, 1);
    truncated.truncate(20);
    assert!(filter(truncated, "Greyscale").is_err());
}

#[test]
fn test_image_filter_does_not_panic_on_binary_input() {
    let binary: Vec<u8> = (0u8..=255).collect();
    let result = filter(binary, "Greyscale");
    assert!(
        result.is_ok() || result.is_err(),
        "the call must return rather than panic"
    );
}
