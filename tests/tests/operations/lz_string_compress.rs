// Tests for the lz_string_compress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lz_string_compress::
//
// Expected values were observed from CyberChef 11.0.0 @ 0bb5472e50e1, which
// exposes three formats (default/UTF16/Base64). rx-chef names the first
// "Standard" and additionally offers EncodedURIComponent, an lz-string
// function upstream does not expose.
//
// The operation previously implemented only the Standard format: the bit
// packer hardcoded 16 bits per output character, so the other three were
// declared in the schema and rejected.

use rxchef::runtime::{self, RuntimeError};

fn compress(input: &[u8], format: &str) -> Result<Vec<u8>, RuntimeError> {
    runtime::run_operation("LZString Compress", input.to_vec(), &[format.to_string()])
}

fn compress_text(input: &str, format: &str) -> String {
    String::from_utf8(compress(input.as_bytes(), format).expect("compression must succeed"))
        .unwrap()
}

fn decompress_text(input: &str, format: &str) -> String {
    let out = runtime::run_operation(
        "LZString Decompress",
        input.as_bytes().to_vec(),
        &[format.to_string()],
    )
    .expect("decompression must succeed");
    String::from_utf8(out).unwrap()
}

#[test]
fn test_lz_string_compress_standard_matches_upstream() {
    assert_eq!(compress_text("hello hello hello", "Standard"), "օ〶惶J፲退");
}

#[test]
fn test_lz_string_compress_base64_matches_upstream() {
    assert_eq!(
        compress_text("hello hello hello", "Base64"),
        "BYUwNmD2AEoTcpA="
    );
    assert_eq!(compress_text("a", "Base64"), "IZA=");
}

#[test]
fn test_lz_string_compress_utf16_matches_upstream() {
    assert_eq!(compress_text("hello hello hello", "UTF16"), "ˢ䰭䰾怤傻䩠 ");
}

#[test]
fn test_lz_string_compress_empty_input_emits_the_end_marker() {
    // Upstream returns early only for a null input; an empty string still
    // emits the end-of-stream marker. Returning nothing here was a divergence.
    assert_eq!(compress_text("", "Standard"), "䀀");
    assert_eq!(compress_text("", "UTF16"), "† ");
    assert_eq!(compress_text("", "Base64"), "Q===");
}

#[test]
fn test_lz_string_compress_base64_output_length_is_a_multiple_of_four() {
    for sample in [
        "",
        "a",
        "ab",
        "hello",
        "hello hello hello",
        &"x".repeat(500),
    ] {
        let out = compress_text(sample, "Base64");
        assert_eq!(
            out.len() % 4,
            0,
            "Base64 output must be padded to a multiple of four: {out:?}"
        );
    }
}

#[test]
fn test_lz_string_compress_utf16_output_ends_with_a_space() {
    // compressToUTF16 terminates the stream with a single space.
    for sample in ["", "a", "hello hello hello"] {
        assert!(
            compress_text(sample, "UTF16").ends_with(' '),
            "UTF16 output must end with a space for {sample:?}"
        );
    }
}

#[test]
fn test_lz_string_compress_handles_unicode() {
    assert_eq!(compress_text("日本語テキスト", "UTF16"), "唔梉暯ᔸㆦ⋴ᤳ僨দဠ ");
    assert_eq!(
        compress_text("日本語テキスト", "Base64"),
        "qemhpzR5UYYwyLUMidDIEwxA"
    );
}

#[test]
fn test_lz_string_compress_handles_non_bmp_unicode() {
    // lz-string operates on UTF-16 code units, so an astral character is two
    // units. Iterating Unicode scalar values instead truncated them: "😀"
    // (U+1F600) came back as U+F600, and upstream's own stream could not be
    // decompressed at all.
    let emoji = "😀😀😀 flag 🏳️‍🌈";
    for format in ["UTF16", "Base64", "EncodedURIComponent"] {
        let compressed = compress_text(emoji, format);
        assert_eq!(
            decompress_text(&compressed, format),
            emoji,
            "{format} lost non-BMP characters"
        );
    }
}

#[test]
fn test_lz_string_compress_standard_cannot_represent_astral_input() {
    // Inherent to the Standard format rather than a defect: it emits raw
    // UTF-16 code units, and compressing astral text produces lone surrogates.
    // JavaScript strings tolerate those; a Rust `String` is UTF-8 and cannot.
    // The other three formats map into representable ranges and are unaffected.
    let error = compress("😀".as_bytes(), "Standard")
        .expect_err("astral input cannot be represented in the Standard format");
    assert!(
        error.to_string().contains("surrogate"),
        "the error should explain why, got: {error}"
    );
}

#[test]
fn test_lz_string_compress_matches_upstream_for_non_bmp_input() {
    // Observed from CyberChef 11.0.0 @ 0bb5472e50e1.
    assert_eq!(compress_text("😀😀😀", "Base64"), "rwbgA92o");
    assert_eq!(decompress_text("rwbgA92o", "Base64"), "😀😀😀");
}

#[test]
fn test_lz_string_compress_roundtrips_in_every_format() {
    for format in ["Standard", "UTF16", "Base64", "EncodedURIComponent"] {
        for sample in ["", "a", "hello hello hello", "line\nbreak\ttab"] {
            let compressed = compress_text(sample, format);
            assert_eq!(
                decompress_text(&compressed, format),
                sample,
                "{format} failed to roundtrip {sample:?}"
            );
        }
    }
}

#[test]
fn test_lz_string_compress_encoded_uri_component_is_url_safe() {
    // The URI-safe alphabet replaces "/" and "=" with "-" and "$".
    let out = compress_text(
        "hello hello hello and some more text to compress",
        "EncodedURIComponent",
    );
    assert!(
        !out.contains('/') && !out.contains('='),
        "URI-safe output must avoid '/' and '=': {out:?}"
    );
}

#[test]
fn test_lz_string_compress_rejects_an_unknown_format() {
    let error = compress(b"hello", "NotAFormat").expect_err("unknown format must be rejected");
    assert!(
        matches!(&error, RuntimeError::InvalidArgument { name, .. } if name == "Compression Format"),
        "expected InvalidArgument naming the format, got {error:?}"
    );
}

#[test]
fn test_lz_string_compress_reads_upstream_streams_back() {
    // Interoperability in the other direction: streams produced by upstream
    // must decompress here.
    assert_eq!(
        decompress_text("BYUwNmD2AEoTcpA=", "Base64"),
        "hello hello hello"
    );
    assert_eq!(decompress_text("Q===", "Base64"), "");
    assert_eq!(decompress_text("† ", "UTF16"), "");
}
