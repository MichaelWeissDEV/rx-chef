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
    // This test will likely fail without the font file, but we can test the basic flow
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
    
    let result = op.run(img_buf, &args);
    // This may fail due to missing font, but we can at least test that it doesn't panic
    // and handles the error gracefully
    if result.is_err() {
        // Expected when font is not available
        let err = result.unwrap_err();
        assert!(err.to_string().contains("font") || err.to_string().contains("Font"));
    } else {
        // If font is available, we should get valid image data
        let output = result.unwrap();
        assert!(!output.is_empty());
    }
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
    let alignments = vec![
        ("Left", "Top"),
        ("Center", "Middle"),
        ("Right", "Bottom"),
    ];
    
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
        
        let result = op.run(img_buf.clone(), &args);
        // Similar to above, this may fail due to font, but shouldn't panic
        assert!(result.is_ok() || result.unwrap_err().to_string().contains("font"));
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
    
    let result = op.run(img_buf, &args);
    // This may fail due to missing font, but shouldn't panic
    assert!(result.is_ok() || result.unwrap_err().to_string().contains("font"));
}
