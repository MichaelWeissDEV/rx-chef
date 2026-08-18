// Tests for the from_modhex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_modhex::

use rxchef::operation::ArgValue;
use rxchef::operations::from_modhex::FromModhex;
use rxchef::Operation;

#[test]
fn test_from_modhex_basic() {
    let op = FromModhex;
    // "cb" in modhex: c=0, b=1 -> 0x01; "de" -> d=2, e=3 -> 0x23
    let input = b"cbde".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, vec![0x01, 0x23]);
}
#[test]
fn test_from_modhex_with_space() {
    let op = FromModhex;
    let input = b"cb de".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, vec![0x01, 0x23]);
}
#[test]
fn test_from_modhex_empty() {
    let op = FromModhex;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_from_modhex_invalid_char() {
    let op = FromModhex;
    let input = b"zz".to_vec();
    assert!(op.run(input, &[]).is_err());
}

// ── Auto delimiter ────────────────────────────────────────────────────────
//
// "Auto" is upstream's default and rx-chef's now too. It used to share the
// "None" arm, which strips nothing, so any delimited input was rejected as an
// invalid modhex character even though the option claimed to detect it.

use rxchef::runtime;

fn to_modhex(input: &[u8], delimiter: &str) -> Vec<u8> {
    runtime::run_operation(
        "To Modhex",
        input.to_vec(),
        &[delimiter.to_string(), "0".into()],
    )
    .expect("encoding must succeed")
}

fn from_modhex(input: Vec<u8>, delimiter: &str) -> Vec<u8> {
    runtime::run_operation("From Modhex", input, &[delimiter.to_string()])
        .expect("decoding must succeed")
}

#[test]
fn test_from_modhex_auto_accepts_every_delimiter_the_encoder_emits() {
    for delimiter in ["None", "Space", "Comma", "Semi-colon", "Colon", "Line feed"] {
        let encoded = to_modhex(b"foobar", delimiter);
        assert_eq!(
            from_modhex(encoded, "Auto"),
            b"foobar",
            "Auto failed to decode {delimiter}-delimited modhex"
        );
    }
}

#[test]
fn test_from_modhex_auto_is_the_default_and_pairs_with_the_encoder_default() {
    // Both defaults come from upstream: encode with "Space", decode with "Auto".
    let encoded = runtime::run_operation("To Modhex", b"foobar".to_vec(), &[])
        .expect("default encode must succeed");
    assert_eq!(
        String::from_utf8(encoded.clone()).unwrap(),
        "hh hv hv hd hb id"
    );
    let decoded =
        runtime::run_operation("From Modhex", encoded, &[]).expect("default decode must succeed");
    assert_eq!(decoded, b"foobar");
}

#[test]
fn test_from_modhex_auto_ignores_unrelated_separators() {
    // Anything outside the modhex alphabet is treated as separation.
    assert_eq!(
        from_modhex(b"hh-hv/hv|hd_hb.id".to_vec(), "Auto"),
        b"foobar"
    );
}

#[test]
fn test_from_modhex_none_still_requires_an_undelimited_string() {
    assert!(
        runtime::run_operation("From Modhex", b"hh hv".to_vec(), &["None".to_string()]).is_err(),
        "None must not silently strip delimiters"
    );
}
