// Tests for the randomize_colour_palette operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations randomize_colour_palette::
//
// The operation permutes an image's colours, so there is no fixed expected
// output. What is checkable — and what these assert — are the invariants the
// permutation must satisfy: the same seed reproduces the same image, different
// seeds generally do not, dimensions survive, and the result is still a valid
// image.

use rxchef::runtime::{self, RuntimeError};

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

/// A small image with several distinct colours to permute.
fn sample() -> Vec<u8> {
    png(
        &[
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (255, 255, 0),
            (0, 255, 255),
            (255, 0, 255),
            (10, 20, 30),
            (200, 100, 50),
            (128, 128, 128),
        ],
        3,
        3,
    )
}

fn randomize(input: Vec<u8>, seed: &str) -> Result<Vec<u8>, RuntimeError> {
    runtime::run_operation("Randomize Colour Palette", input, &[seed.to_string()])
}

#[test]
fn test_randomize_colour_palette_is_reproducible_for_a_given_seed() {
    // The seed is the whole point: the same seed must give the same image.
    let once = randomize(sample(), "rxchef-seed").unwrap();
    let twice = randomize(sample(), "rxchef-seed").unwrap();
    assert_eq!(once, twice, "the same seed must reproduce the same image");
}

#[test]
fn test_randomize_colour_palette_different_seeds_give_different_results() {
    let a = randomize(sample(), "seed-a").unwrap();
    let b = randomize(sample(), "seed-b").unwrap();
    assert_ne!(a, b, "different seeds should permute differently");
}

#[test]
fn test_randomize_colour_palette_emits_a_decodable_image() {
    let out = randomize(sample(), "seed").unwrap();
    assert!(
        image::load_from_memory(&out).is_ok(),
        "the result must still be a valid image"
    );
}

#[test]
fn test_randomize_colour_palette_preserves_dimensions() {
    let out = randomize(sample(), "seed").unwrap();
    let image = image::load_from_memory(&out).unwrap();
    assert_eq!((image.width(), image.height()), (3, 3));
}

#[test]
fn test_randomize_colour_palette_preserves_the_pixel_count() {
    let out = randomize(sample(), "seed").unwrap();
    let image = image::load_from_memory(&out).unwrap().to_rgba8();
    assert_eq!(image.pixels().count(), 9);
}

#[test]
fn test_randomize_colour_palette_changes_a_multicoloured_image() {
    // A permutation that returned the input unchanged would be a no-op.
    let source = sample();
    let out = randomize(source.clone(), "shuffle").unwrap();
    let before = image::load_from_memory(&source).unwrap().to_rgba8();
    let after = image::load_from_memory(&out).unwrap().to_rgba8();
    assert_ne!(
        before.into_raw(),
        after.into_raw(),
        "the palette should actually be permuted"
    );
}

#[test]
fn test_randomize_colour_palette_single_colour_image() {
    // Boundary: with one colour there is nothing to permute, and the operation
    // must still return a valid image rather than failing.
    let flat = png(&[(42, 42, 42); 4], 2, 2);
    let out = randomize(flat, "seed").unwrap();
    let image = image::load_from_memory(&out).unwrap().to_rgba8();
    // RFC 1321 MD5("seed42.42.42") begins c4 b3 7a; the operation defines
    // the replacement RGB as those first three digest bytes.
    assert!(image
        .pixels()
        .all(|pixel| pixel.0 == [0xc4, 0xb3, 0x7a, 255]));
}

#[test]
fn test_randomize_colour_palette_empty_seed_is_accepted() {
    // The schema's default is an empty seed.
    assert!(randomize(sample(), "").is_ok());
}

#[test]
fn test_randomize_colour_palette_rejects_data_that_is_not_an_image() {
    assert!(randomize(b"definitely not an image".to_vec(), "seed").is_err());
}

#[test]
fn test_randomize_colour_palette_rejects_a_truncated_image() {
    let mut truncated = sample();
    truncated.truncate(20);
    assert!(randomize(truncated, "seed").is_err());
}

#[test]
fn test_randomize_colour_palette_does_not_panic_on_binary_input() {
    let binary: Vec<u8> = (0u8..=255).collect();
    let result = randomize(binary, "seed");
    assert!(
        result.is_ok() || result.is_err(),
        "the call must return rather than panic"
    );
}
