// Tests for the enigma operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations enigma::

use rxchef::operations::enigma::Enigma;
use rxchef::operation::{ArgValue, DataType};
use rxchef::Operation;

fn default_args_3() -> Vec<ArgValue> {
    vec![
        ArgValue::Str("3-rotor".into()),
        ArgValue::Str("".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("EKMFLGDQVZNTOWYHXUSPAIBRCJ<R".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("AJDKSIRUXBLHWTMCQGZNPYFVOE<F".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("BDFHJLCPRTXVZNYEIWGAKMUSQO<W".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("AY BR CU DH EQ FS GL IP JX KN MO TZ VW".into()),
        ArgValue::Str("".into()),
        ArgValue::Str("true".into()),
    ]
}

#[test]
fn enigma_encrypt_decrypt_roundtrip() {
    let op = Enigma;

    let args = default_args_3();

    let plaintext = b"HELLOWORLD".to_vec();

    let cipher = op.run(plaintext.clone(), &args).unwrap();
    let recovered = op.run(cipher, &args).unwrap();

    assert_eq!(recovered, plaintext);
}

#[test]
fn enigma_strict_removes_non_alpha() {
    let op = Enigma;

    let args = default_args_3();

    let out = op.run(b"HELLO 123!".to_vec(), &args).unwrap();

    assert!(out.iter().all(|c| c.is_ascii_uppercase()));
}

#[test]
fn enigma_non_strict_keeps_non_alpha() {
    let op = Enigma;

    let mut args = default_args_3();
    args[15] = ArgValue::Str("false".into());

    let out = op.run(b"HELLO 123!".to_vec(), &args).unwrap();

    assert!(out.contains(&b' '));
    assert!(out.contains(&b'1'));
    assert!(out.contains(&b'!'));
}

#[test]
fn enigma_invalid_rotor_returns_error() {
    let op = Enigma;

    let mut args = default_args_3();
    args[4] = ArgValue::Str("ABC".into());

    assert!(op.run(b"HELLO".to_vec(), &args).is_err());
}

#[test]
fn enigma_4_rotor_model() {
    let op = Enigma;

    let args = vec![
        ArgValue::Str("4-rotor".into()),
        ArgValue::Str("LEYJVCNIXWPBQMDRTAKZGFUHOS".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("EKMFLGDQVZNTOWYHXUSPAIBRCJ<R".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("AJDKSIRUXBLHWTMCQGZNPYFVOE<F".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("BDFHJLCPRTXVZNYEIWGAKMUSQO<W".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("A".into()),
        ArgValue::Str("AY BR CU DH EQ FS GL IP JX KN MO TZ VW".into()),
        ArgValue::Str("".into()),
        ArgValue::Str("true".into()),
    ];

    let out = op.run(b"TEST".to_vec(), &args).unwrap();

    assert_eq!(out.len(), 4);
}

#[test]
fn operation_metadata() {
    let op = Enigma;

    assert_eq!(op.name(), "Enigma");
    assert_eq!(op.module(), "Bletchley");
    assert_eq!(
        op.description(),
        "Encipher/decipher with the WW2 Enigma machine."
    );

    assert_eq!(op.input_type(), DataType::String);
    assert_eq!(op.output_type(), DataType::String);

    assert!(!op.args_schema().is_empty());
}