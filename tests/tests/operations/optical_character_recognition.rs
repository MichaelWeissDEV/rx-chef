// Tests for the optical_character_recognition operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations optical_character_recognition::

#[cfg(feature = "tesseract")]
use rxchef::operation::ArgValue;
use rxchef::operations::optical_character_recognition::OpticalCharacterRecognition;
use rxchef::Operation;

#[cfg(feature = "tesseract")]
fn test_bitmap_bmp() -> Vec<u8> {
    const GLYPHS: [[u8; 7]; 4] = [
        [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ], // T
        [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ], // E
        [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ], // S
        [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ], // T
    ];
    const SCALE: usize = 20;
    const WIDTH: usize = 520;
    const HEIGHT: usize = 220;
    let row_size = (WIDTH * 3 + 3) & !3;
    let pixel_size = row_size * HEIGHT;
    let mut bmp = Vec::with_capacity(54 + pixel_size);
    bmp.extend_from_slice(b"BM");
    bmp.extend_from_slice(&((54 + pixel_size) as u32).to_le_bytes());
    bmp.extend_from_slice(&[0; 4]);
    bmp.extend_from_slice(&54u32.to_le_bytes());
    bmp.extend_from_slice(&40u32.to_le_bytes());
    bmp.extend_from_slice(&(WIDTH as i32).to_le_bytes());
    bmp.extend_from_slice(&(HEIGHT as i32).to_le_bytes());
    bmp.extend_from_slice(&1u16.to_le_bytes());
    bmp.extend_from_slice(&24u16.to_le_bytes());
    bmp.extend_from_slice(&[0; 24]);
    let mut pixels = vec![255u8; pixel_size];
    for (glyph_index, glyph) in GLYPHS.iter().enumerate() {
        let origin_x = 40 + glyph_index * 120;
        for (row, bits) in glyph.iter().enumerate() {
            for column in 0..5 {
                if bits & (1 << (4 - column)) == 0 {
                    continue;
                }
                for dy in 0..SCALE {
                    for dx in 0..SCALE {
                        let x = origin_x + column * SCALE + dx;
                        let y = 40 + row * SCALE + dy;
                        let bmp_y = HEIGHT - 1 - y;
                        let offset = bmp_y * row_size + x * 3;
                        pixels[offset..offset + 3].fill(0);
                    }
                }
            }
        }
    }
    bmp.extend_from_slice(&pixels);
    bmp
}

#[test]
fn test_ocr_is_broken() {
    let op = OpticalCharacterRecognition;
    #[cfg(not(feature = "tesseract"))]
    assert!(op.is_broken());
    #[cfg(feature = "tesseract")]
    assert!(!op.is_broken());
}
#[test]
fn test_ocr_rejects_invalid_image() {
    let op = OpticalCharacterRecognition;
    let input = vec![0u8];
    let args = vec![];
    let result = op.run(input, &args);
    assert!(result.is_err());
}

#[test]
#[cfg(feature = "tesseract")]
fn test_ocr_reads_known_bitmap_word() {
    let result = OpticalCharacterRecognition
        .run(
            test_bitmap_bmp(),
            &[ArgValue::Bool(false), ArgValue::Str("LSTM only".into())],
        )
        .unwrap();
    assert_eq!(result, b"TEST");
}
