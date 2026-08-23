//! Cross-operation negative tests for registry-level argument contracts.
//!
//! These cases deliberately exercise the public runtime rather than calling
//! operations with already-typed `ArgValue`s. That is where malformed CLI/API
//! values must be rejected before an operation sees them.

use rxchef::{catalog, operation::ArgKind, runtime};

#[test]
fn malformed_typed_arguments_are_rejected_by_the_public_runtime() {
    let operations = [
        "Convert co-ordinate format",
        "Extract IP addresses",
        "Extract MAC addresses",
        "Extract URLs",
        "Extract dates",
        "Extract domains",
        "Extract email addresses",
        "Extract file paths",
        "Extract hashes",
        "Generate all checksums",
        "Generate all hashes",
        "Group IP addresses",
        "LZString Decompress",
        "Pad lines",
        "RAKE",
        "ROT47 Brute Force",
        "SIGABA",
        "Sleep",
        "Sort",
        "To Binary",
        "To Camel case",
        "To Decimal",
        "To HTML Entity",
        "To Hex",
        "To Hex Content",
        "To Modhex",
        "To Octal",
        "To Snake case",
        "To Table",
        "XOR Brute Force",
    ];

    for operation in operations {
        let descriptor = catalog::describe(operation).expect("registered operation");
        let mut args: Vec<String> = descriptor
            .args
            .iter()
            .map(|argument| argument.default.clone())
            .collect();
        let invalid = descriptor.args.iter().enumerate().find_map(|(index, argument)| {
            let value = match argument.kind {
                ArgKind::Integer | ArgKind::UnsignedInteger | ArgKind::Float => "not-a-number",
                ArgKind::Boolean => "not-a-boolean",
                ArgKind::Enum => "not-a-declared-enum-choice",
                ArgKind::Regex => "(",
                _ => return None,
            };
            Some((index, value))
        });
        let (index, invalid_value) = invalid.unwrap_or_else(|| {
            panic!("{operation} must have a typed argument suitable for this contract test")
        });
        args[index] = invalid_value.to_string();

        assert!(
            runtime::run_operation(operation, b"contract input".to_vec(), &args).is_err(),
            "{operation} accepted malformed value {invalid_value:?} for argument {}",
            descriptor.args[index].name
        );
    }
}
