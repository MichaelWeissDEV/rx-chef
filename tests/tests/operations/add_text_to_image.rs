// Tests for the add_text_to_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations add_text_to_image::

use rxchef::operations::add_text_to_image::AddTextToImage;
use rxchef::Operation;

#[test]
fn test_add_text_to_image_empty_input() {
    let op = AddTextToImage;
    let input = vec![];
    let args = [];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_add_text_to_image_invalid_format() {
    let op = AddTextToImage;
    let input = b"This is not an image".to_vec();
    let args = [];
    let result = op.run(input, &args);
    assert!(result.is_err());
}

#[test]
fn test_add_text_to_image_with_text() {
    let op = AddTextToImage;

    // Create a simple 1x1 PNG image
    let mut img_buf = Vec::new();
    let img = image::RgbaImage::new(100, 100);
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();

    let args = [
        rxchef::operation::ArgValue::Str("Test Text".to_string()),
        rxchef::operation::ArgValue::Str("Center".to_string()),
        rxchef::operation::ArgValue::Str("Middle".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(32.0),
        rxchef::operation::ArgValue::Num(255.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(255.0),
    ];

    let output = op.run(img_buf, &args).unwrap();
    let rendered = image::load_from_memory(&output).unwrap().to_rgba8();
    assert!(rendered.pixels().any(|pixel| pixel[3] != 0));
}

#[test]
fn test_add_text_to_image_different_alignments() {
    let op = AddTextToImage;

    // Create a simple 1x1 PNG image
    let mut img_buf = Vec::new();
    let img = image::RgbaImage::new(200, 200);
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();

    // Test different alignment combinations
    let alignments = vec![("Left", "Top"), ("Center", "Middle"), ("Right", "Bottom")];

    for (h_align, v_align) in alignments {
        let args = [
            rxchef::operation::ArgValue::Str("Align Test".to_string()),
            rxchef::operation::ArgValue::Str(h_align.to_string()),
            rxchef::operation::ArgValue::Str(v_align.to_string()),
            rxchef::operation::ArgValue::Num(0.0),
            rxchef::operation::ArgValue::Num(0.0),
            rxchef::operation::ArgValue::Num(24.0),
            rxchef::operation::ArgValue::Num(0.0),
            rxchef::operation::ArgValue::Num(0.0),
            rxchef::operation::ArgValue::Num(255.0),
            rxchef::operation::ArgValue::Num(255.0),
        ];

        let output = op.run(img_buf.clone(), &args).unwrap();
        assert_eq!(image::load_from_memory(&output).unwrap().width(), 200);
    }
}

#[test]
fn test_add_text_to_image_custom_colors() {
    let op = AddTextToImage;

    // Create a simple 1x1 PNG image
    let mut img_buf = Vec::new();
    let img = image::RgbaImage::new(100, 100);
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();

    // Test with custom RGB color (red text)
    let args = [
        rxchef::operation::ArgValue::Str("Red Text".to_string()),
        rxchef::operation::ArgValue::Str("None".to_string()),
        rxchef::operation::ArgValue::Str("None".to_string()),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Num(24.0),
        rxchef::operation::ArgValue::Num(255.0), // Red
        rxchef::operation::ArgValue::Num(0.0),   // Green
        rxchef::operation::ArgValue::Num(0.0),   // Blue
        rxchef::operation::ArgValue::Num(255.0), // Alpha
    ];

    let output = op.run(img_buf, &args).unwrap();
    let rendered = image::load_from_memory(&output).unwrap().to_rgba8();
    assert!(rendered
        .pixels()
        .any(|pixel| pixel[0] > 0 && pixel[1] == 0 && pixel[2] == 0));
}
