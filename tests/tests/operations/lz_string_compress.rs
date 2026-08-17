// Tests for the lz_string_compress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lz_string_compress::

use rxchef::operation::{ArgValue, OperationError};
use rxchef::operations::lz_string_compress::LZStringCompress;
use rxchef::operations::lz_string_decompress::LZStringDecompress;
use rxchef::Operation;

fn try_compress(input: &str, format: &str) -> Result<Vec<u8>, OperationError> {
    let args = [ArgValue::Str(format.to_string())];
    LZStringCompress.run(input.as_bytes().to_vec(), &args)
}

fn compress(input: &str) -> Vec<u8> {
    try_compress(input, "Standard").unwrap()
}

fn decompress(input: Vec<u8>) -> String {
    let args = [ArgValue::Str("Standard".to_string())];
    String::from_utf8(LZStringDecompress.run(input, &args).unwrap()).unwrap()
}

#[test]
fn test_lz_string_compress_empty_input() {
    assert!(compress("").is_empty());
}

#[test]
fn test_lz_string_compress_roundtrips_ascii() {
    let original = "hello hello hello";
    assert_eq!(decompress(compress(original)), original);
}

#[test]
fn test_lz_string_compress_roundtrips_single_character() {
    assert_eq!(decompress(compress("x")), "x");
}

#[test]
fn test_lz_string_compress_is_deterministic() {
    assert_eq!(compress("repeatable input"), compress("repeatable input"));
}

#[test]
fn test_lz_string_compress_default_format_matches_decompress_default() {
    // Regression: the two operations declared disjoint format vocabularies
    // ("default" vs "Standard"), so the defaults could never be paired.
    let compress_default = LZStringCompress.args_schema()[0].default_value;
    let decompress_default = LZStringDecompress.args_schema()[0].default_value;
    assert_eq!(compress_default, decompress_default);

    let compress_choices = LZStringCompress.args_schema()[0].choices;
    let decompress_choices = LZStringDecompress.args_schema()[0].choices;
    assert_eq!(compress_choices, decompress_choices);
}

#[test]
fn test_lz_string_compress_reports_unimplemented_formats() {
    // Regression: the format argument was ignored entirely, so requesting
    // Base64 returned a standard-format stream that could not be read back.
    for format in ["Base64", "UTF16", "EncodedURIComponent"] {
        let error = try_compress("hello", format)
            .expect_err("unimplemented output formats must be reported");
        assert!(
            matches!(&error, OperationError::InvalidArgument { name, .. }
                if name == "Compression Format"),
            "expected an InvalidArgument naming Compression Format for {format}, got {error:?}"
        );
    }
}

#[test]
fn test_lz_string_compress_rejects_unknown_format() {
    let error =
        try_compress("hello", "not-a-format").expect_err("unknown formats must be rejected");
    assert!(
        matches!(&error, OperationError::InvalidArgument { name, .. }
            if name == "Compression Format"),
        "expected an InvalidArgument, got {error:?}"
    );
}

#[test]
fn test_lz_string_compress_never_panics_on_surrogate_code_units() {
    // Regression: the packer pushed each 16-bit code unit through
    // `char::from_u32(..).unwrap()`, which panics for the surrogate range
    // U+D800..=U+DFFF that this bit stream can legitimately produce.
    // Whatever the outcome, it must be a Result rather than a panic.
    for input in [
        "A".repeat(2000),
        "abcabcabc".repeat(200),
        "café — €50 — naïve".repeat(50),
        (0u8..=127)
            .map(|b| b as char)
            .collect::<String>()
            .repeat(30),
    ] {
        let result = try_compress(&input, "Standard");
        if let Err(error) = &result {
            assert!(
                matches!(error, OperationError::ProcessingError(_)),
                "surrogate output must be a ProcessingError, got {error:?}"
            );
        }
        // When compression does succeed the result must round-trip.
        if let Ok(compressed) = result {
            assert_eq!(decompress(compressed), input);
        }
    }
}

#[test]
fn test_lz_string_compress_shrinks_repetitive_input_when_it_succeeds() {
    let original = "abcabcabc".repeat(200);
    if let Ok(compressed) = try_compress(&original, "Standard") {
        assert!(
            compressed.len() < original.len(),
            "compressed {} bytes is not smaller than the {} byte input",
            compressed.len(),
            original.len()
        );
    }
}
