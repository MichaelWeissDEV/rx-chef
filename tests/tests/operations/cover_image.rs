// Tests for the cover_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations cover_image::

use rxchef::operations::cover_image::CoverImage;
use rxchef::Operation;

#[test]
fn test_cover_image_empty_input() {
    let op = CoverImage;
    let input = vec![];
    let args = [
        rxchef::operation::ArgValue::Num(100.0),
        rxchef::operation::ArgValue::Num(100.0),
        rxchef::operation::ArgValue::Str("Center".to_string()),
        rxchef::operation::ArgValue::Str("Middle".to_string()),
        rxchef::operation::ArgValue::Str("Bilinear".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_cover_image_invalid_format() {
    let op = CoverImage;
    let input = b"This is not an image".to_vec();
    let args = [
        rxchef::operation::ArgValue::Num(100.0),
        rxchef::operation::ArgValue::Num(100.0),
        rxchef::operation::ArgValue::Str("Center".to_string()),
        rxchef::operation::ArgValue::Str("Middle".to_string()),
        rxchef::operation::ArgValue::Str("Bilinear".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported image format"));
}

#[test]
fn test_cover_image_basic_resize() {
    let op = CoverImage;
    
    // Create a simple test image (10x10 PNG)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(10, 10);
    // Fill with red color
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 0, 0, 255]);
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Num(50.0), // Target width
        rxchef::operation::ArgValue::Num(50.0), // Target height
        rxchef::operation::ArgValue::Str("Center".to_string()),
        rxchef::operation::ArgValue::Str("Middle".to_string()),
        rxchef::operation::ArgValue::Str("Bilinear".to_string()),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    
    // Should be valid image data
    assert!(!result.is_empty());
    
    // Load the result and verify dimensions
    let covered_img = image::load_from_memory(&result).unwrap();
    assert_eq!(covered_img.width(), 50);
    assert_eq!(covered_img.height(), 50);
}

#[test]
fn test_cover_image_different_alignments() {
    let op = CoverImage;
    
    // Create a simple test image (10x10 PNG)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 0, 0, 255]);
    }
    
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
            rxchef::operation::ArgValue::Num(50.0),
            rxchef::operation::ArgValue::Num(50.0),
            rxchef::operation::ArgValue::Str(h_align.to_string()),
            rxchef::operation::ArgValue::Str(v_align.to_string()),
            rxchef::operation::ArgValue::Str("Bilinear".to_string()),
        ];
        
        let result = op.run(img_buf.clone(), &args).unwrap();
        assert!(!result.is_empty());
        
        let covered_img = image::load_from_memory(&result).unwrap();
        assert_eq!(covered_img.width(), 50);
        assert_eq!(covered_img.height(), 50);
    }
}

#[test]
fn test_cover_image_different_algorithms() {
    let op = CoverImage;
    
    // Create a simple test image
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 0, 0, 255]);
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    // Test different resizing algorithms
    let algorithms = ["Nearest Neighbour", "Bilinear", "Bicubic", "Hermite", "Bezier"];
    
    for algorithm in algorithms {
        let args = [
            rxchef::operation::ArgValue::Num(50.0),
            rxchef::operation::ArgValue::Num(50.0),
            rxchef::operation::ArgValue::Str("Center".to_string()),
            rxchef::operation::ArgValue::Str("Middle".to_string()),
            rxchef::operation::ArgValue::Str(algorithm.to_string()),
        ];
        
        let result = op.run(img_buf.clone(), &args).unwrap();
        assert!(!result.is_empty());
        
        let covered_img = image::load_from_memory(&result).unwrap();
        assert_eq!(covered_img.width(), 50);
        assert_eq!(covered_img.height(), 50);
    }
}

#[test]
fn test_cover_image_maintain_aspect_ratio_with_crop() {
    let op = CoverImage;
    
    // Create a non-square test image (20x10 PNG)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(20, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 0, 0, 255]);
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Num(100.0), // Target width
        rxchef::operation::ArgValue::Num(100.0), // Target height
        rxchef::operation::ArgValue::Str("Center".to_string()),
        rxchef::operation::ArgValue::Str("Middle".to_string()),
        rxchef::operation::ArgValue::Str("Bilinear".to_string()),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    // The covered image should be exactly the target dimensions
    let covered_img = image::load_from_memory(&result).unwrap();
    assert_eq!(covered_img.width(), 100);
    assert_eq!(covered_img.height(), 100);
    
    // The original image should be scaled up and cropped to fit
}

#[test]
fn test_cover_image_large_target() {
    let op = CoverImage;
    
    // Create a small test image
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(2, 2);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 0, 0, 255]);
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Num(200.0), // Large target width
        rxchef::operation::ArgValue::Num(200.0), // Large target height
        rxchef::operation::ArgValue::Str("Center".to_string()),
        rxchef::operation::ArgValue::Str("Middle".to_string()),
        rxchef::operation::ArgValue::Str("Bilinear".to_string()),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    let covered_img = image::load_from_memory(&result).unwrap();
    assert_eq!(covered_img.width(), 200);
    assert_eq!(covered_img.height(), 200);
}

#[test]
fn test_cover_image_square_to_rectangle() {
    let op = CoverImage;
    
    // Create a square test image (10x10 PNG)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 0, 0, 255]);
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Num(100.0), // Target width
        rxchef::operation::ArgValue::Num(50.0),  // Target height (different aspect ratio)
        rxchef::operation::ArgValue::Str("Center".to_string()),
        rxchef::operation::ArgValue::Str("Middle".to_string()),
        rxchef::operation::ArgValue::Str("Bilinear".to_string()),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    // The covered image should be exactly the target dimensions
    let covered_img = image::load_from_memory(&result).unwrap();
    assert_eq!(covered_img.width(), 100);
    assert_eq!(covered_img.height(), 50);
}
