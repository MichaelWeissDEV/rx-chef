// Tests for the from_morse_code operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_morse_code::
//
// Expected values follow ITU-R M.1677-1 International Morse code and were
// cross-checked against CyberChef 11.0.0.
//
// These tests previously asserted only `result.is_ok()` with comments such as
// "test that the operation runs without panicking". They passed while the
// operation was returning an empty string for every input, because its signal
// normalisation used `str::replace("", ...)`, which in Rust inserts the
// replacement between every character. Each case below now pins the decoded
// text so that failure mode cannot return unnoticed.

use rxchef::operation::ArgValue;
use rxchef::operations::from_morse_code::FromMorseCode;
use rxchef::Operation;

fn decode(input: &str, letter: &str, word: &str) -> String {
    let args = [
        ArgValue::Str(letter.to_string()),
        ArgValue::Str(word.to_string()),
    ];
    String::from_utf8(FromMorseCode.run(input.as_bytes().to_vec(), &args).unwrap()).unwrap()
}

#[test]
fn test_from_morse_code_empty_input() {
    assert_eq!(decode("", "Space", "Line feed"), "");
}

#[test]
fn test_from_morse_code_single_letter() {
    assert_eq!(decode(".-", "Space", "Line feed"), "A");
    assert_eq!(decode("...", "Space", "Line feed"), "S");
    assert_eq!(decode("-", "Space", "Line feed"), "T");
    assert_eq!(decode(".", "Space", "Line feed"), "E");
}

#[test]
fn test_from_morse_code_word() {
    assert_eq!(
        decode(".... . .-.. .-.. ---", "Space", "Line feed"),
        "HELLO"
    );
    assert_eq!(decode("--- ...", "Space", "Line feed"), "OS");
}

#[test]
fn test_from_morse_code_word_delimiter_separates_words() {
    assert_eq!(
        decode(
            ".... . .-.. .-.. ---\n.-- --- .-. .-.. -..",
            "Space",
            "Line feed"
        ),
        "HELLO WORLD"
    );
}

#[test]
fn test_from_morse_code_digits_and_punctuation() {
    assert_eq!(
        decode(".---- ..--- ...-- ....- .....", "Space", "Line feed"),
        "12345"
    );
    assert_eq!(decode("-----", "Space", "Line feed"), "0");
}

#[test]
fn test_from_morse_code_custom_delimiters() {
    // Letter delimiter "," and word delimiter ";".
    assert_eq!(decode(".-..,.,.-..", ",", ";"), "LEL");
}

#[test]
fn test_from_morse_code_accepts_unicode_dash_and_dot_variants() {
    // ITU Morse is written with many different dash and dot characters. En
    // dash (U+2013) and middle dot (U+00B7) must decode like "-" and ".".
    assert_eq!(
        decode(
            "\u{2013}\u{2013}\u{2013} \u{00B7}\u{00B7}\u{00B7}",
            "Space",
            "Line feed"
        ),
        "OS"
    );
    // Em dash (U+2014), hyphen (U+2010) and minus sign (U+2212) too.
    assert_eq!(decode("\u{2014}\u{00B7}", "Space", "Line feed"), "N");
    assert_eq!(decode("\u{2010}\u{00B7}", "Space", "Line feed"), "N");
    assert_eq!(decode("\u{2212}\u{00B7}", "Space", "Line feed"), "N");
    // Underscore is an accepted dash spelling.
    assert_eq!(decode("_.", "Space", "Line feed"), "N");
}

#[test]
fn test_from_morse_code_accepts_written_word_forms() {
    // "dash"/"dot" spellings, case-insensitively. The canonical tokens the
    // lookup table uses contain the substring "dash", so rewriting the word
    // forms must not corrupt tokens produced from the symbol forms.
    assert_eq!(decode("dashdot", "Space", "Line feed"), "N");
    assert_eq!(decode("DASHDOT", "Space", "Line feed"), "N");
    assert_eq!(decode("DashDot", "Space", "Line feed"), "N");
}

#[test]
fn test_from_morse_code_unknown_signal_is_dropped() {
    // An unrecognised signal contributes nothing rather than failing the run.
    assert_eq!(decode("........ .-", "Space", "Line feed"), "A");
}

#[test]
fn test_from_morse_code_roundtrips_with_to_morse_code() {
    use rxchef::operations::to_morse_code::ToMorseCode;

    let encoded = ToMorseCode
        .run(
            b"HELLO WORLD".to_vec(),
            &[
                ArgValue::Str("-/.".to_string()),
                ArgValue::Str("Space".to_string()),
                ArgValue::Str("Line feed".to_string()),
            ],
        )
        .unwrap();
    let encoded = String::from_utf8(encoded).unwrap();
    assert_eq!(decode(&encoded, "Space", "Line feed"), "HELLO WORLD");
}

#[test]
fn test_from_morse_code_malformed() {
    use rxchef::operations::from_morse_code::FromMorseCode;
    use rxchef::Operation;
    let op = FromMorseCode;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
        rxchef::operation::ArgValue::Str("Line feed".to_string())
    ];
    let result = op.run(vec![0xff, 0xff], &args);
    assert!(result.is_err());
}
