// Tests for the generate_qr_code operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_qr_code::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_qr_code::GenerateQRCodeOp;
use rxchef::Operation;

#[test]
fn test_generate_and_parse_qr_code_png() {
    let op = GenerateQRCodeOp;
    let input = b"Hello World".to_vec();
    let args = [
        ArgValue::Str("PNG".to_string()),
        ArgValue::Num(5.0),
        ArgValue::Num(4.0),
        ArgValue::Str("Medium".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(result.starts_with(b"\x89PNG\r\n\x1a\n"));

    let parser = rxchef::operations::parse_qr_code::ParseQRCode;
    assert_eq!(parser.run(result, &[]).unwrap(), b"Hello World");
}

#[test]
fn test_generate_qr_code_svg() {
    let result = GenerateQRCodeOp
        .run(
            b"Hello SVG".to_vec(),
            &[
                ArgValue::Str("SVG".to_string()),
                ArgValue::Num(3.0),
                ArgValue::Num(2.0),
                ArgValue::Str("High".to_string()),
            ],
        )
        .unwrap();
    assert!(String::from_utf8(result).unwrap().starts_with("<svg"));
}

#[test]
fn test_generate_qr_code_single_byte_minimum_rendering_boundary() {
    let args = [
        ArgValue::Str("PNG".into()),
        ArgValue::Num(1.0),
        ArgValue::Num(0.0),
        ArgValue::Str("Medium".into()),
    ];
    let image = GenerateQRCodeOp.run(b"A".to_vec(), &args).unwrap();
    let rendered = image::load_from_memory(&image).unwrap().to_luma8();
    assert_eq!(rendered.dimensions(), (21, 21));
    assert!(rendered.pixels().any(|pixel| pixel[0] == 0));
    assert!(rendered.pixels().any(|pixel| pixel[0] == 255));
}

#[test]
fn test_generate_qr_code_rejects_empty_input() {
    assert!(GenerateQRCodeOp.run(Vec::new(), &[]).is_err());
}

#[test]
fn test_generate_qr_code_rejects_unknown_format() {
    let args = [ArgValue::Str("BMP".into())];
    assert!(GenerateQRCodeOp.run(b"A".to_vec(), &args).is_err());
}
