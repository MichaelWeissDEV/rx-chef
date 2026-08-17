// Tests for the register operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations register::

use rxchef::operation::ArgValue;
use rxchef::operations::register::Register;
use rxchef::Operation;

fn args(extractor: &str) -> [ArgValue; 4] {
    [
        ArgValue::Str(extractor.to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ]
}

fn run(input: &[u8], extractor: &str) -> Vec<u8> {
    Register.run(input.to_vec(), &args(extractor)).unwrap()
}

#[test]
fn test_register_passes_input_through_unchanged() {
    // Register captures values for later recipe steps to interpolate; it is
    // a no-op on the data flowing through the pipeline.
    assert_eq!(run(b"key=value", r"(\w+)=(\w+)"), b"key=value");
}

#[test]
fn test_register_empty_input() {
    assert_eq!(run(b"", r"([\s\S]*)"), b"");
}

#[test]
fn test_register_preserves_binary_input_exactly() {
    let binary: Vec<u8> = (0u8..=255).collect();
    assert_eq!(run(&binary, r"([\s\S]*)"), binary);
}

#[test]
fn test_register_does_not_depend_on_the_extractor() {
    // Whatever the pattern, the data is forwarded untouched.
    let input = b"abc123";
    assert_eq!(run(input, r"(\d+)"), input);
    assert_eq!(run(input, r"(nomatch)"), input);
    assert_eq!(run(input, r"([\s\S]*)"), input);
}

#[test]
fn test_register_is_deterministic() {
    assert_eq!(run(b"same", r"(.*)"), run(b"same", r"(.*)"));
}

#[test]
fn test_register_handles_utf8_input() {
    let input = "café €".as_bytes();
    assert_eq!(run(input, r"([\s\S]*)"), input);
}
