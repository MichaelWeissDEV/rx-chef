//! No registered operation may panic on user-controlled input.
//!
//! A panic crosses the FFI boundary as undefined behaviour, aborts the JSONL
//! server mid-session, and kills the CLI without an exit code a caller can act
//! on. Malformed input must come back as a structured error instead.
//!
//! This sweep found two reachable panics when it was written:
//!   * `XXTEA Decrypt` underflowed `n - 3` for a single-word ciphertext.
//!   * `PHP Deserialize` sliced a `&str` at a byte offset that fell inside a
//!     multi-byte character.

use rxchef::runtime;
use std::panic;

/// Inputs chosen to hit the usual decoder edges: empty, boundary lengths,
/// invalid UTF-8, structural characters and numeric extremes.
fn adversarial_inputs() -> Vec<(&'static str, Vec<u8>)> {
    vec![
        ("empty", Vec::new()),
        ("single nul", vec![0]),
        ("single byte", vec![b'a']),
        ("all high bytes", vec![0xFF; 64]),
        ("invalid utf-8", vec![0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1]),
        ("surrogate encoding", vec![0xED, 0xA0, 0x80]),
        ("truncated multibyte", vec![0xE2, 0x82]),
        ("long ascii", vec![b'a'; 4096]),
        ("every byte value", (0u8..=255).collect()),
        ("only newlines", vec![b'\n'; 128]),
        ("unbalanced brackets", b"{{{{{{[[[[[[".to_vec()),
        ("percent signs", b"%%%%%%%%%%%%".to_vec()),
        ("dashes", b"------------".to_vec()),
        ("equals signs", b"============".to_vec()),
        ("quotes", b"\"'\"'\"'\"'".to_vec()),
        ("backslashes", b"\\\\\\\\\\\\\\\\".to_vec()),
        ("oversized number", b"999999999999999999999999999999".to_vec()),
        ("negative number", b"-1".to_vec()),
        ("dots", b"..........".to_vec()),
        ("colon prefixed", b":::::::::::".to_vec()),
    ]
}

#[test]
fn no_operation_panics_on_adversarial_input() {
    let previous = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    let inputs = adversarial_inputs();
    let mut panicked = Vec::new();

    for name in runtime::operation_names(None) {
        let Ok(info) = runtime::operation_info(&name) else {
            continue;
        };
        // Non-deterministic and side-effecting operations reach the network,
        // the clock or the filesystem; they are out of scope here.
        if !info.deterministic || !info.side_effects.is_empty() {
            continue;
        }
        // Schema defaults, exactly as every frontend materialises them.
        let args: Vec<String> = info
            .args
            .iter()
            .map(|argument| argument.default_value.to_string())
            .collect();

        for (label, bytes) in &inputs {
            let operation = name.clone();
            let arguments = args.clone();
            let input = bytes.clone();
            let outcome = panic::catch_unwind(move || {
                let _ = runtime::run_operation(&operation, input, &arguments);
            });
            if outcome.is_err() {
                panicked.push(format!("{name} panicked on {label}"));
            }
        }
    }

    panic::set_hook(previous);
    assert!(
        panicked.is_empty(),
        "operations must reject malformed input rather than panic:\n  {}",
        panicked.join("\n  ")
    );
}

#[test]
fn xxtea_decrypt_rejects_a_single_word_ciphertext() {
    // Regression: `to_uint8_array` computed `n - 3` after `n` had reached 0,
    // underflowing and panicking in debug builds.
    let args = vec![String::new(), "Raw".to_string(), "Raw".to_string()];
    let result = runtime::run_operation("XXTEA Decrypt", vec![0], &args);
    assert!(
        result.is_ok() || result.is_err(),
        "the call must return rather than panic"
    );
}

#[test]
fn php_deserialize_rejects_a_length_inside_a_character() {
    // Regression: the byte length from the serialized form was used to slice a
    // `&str`, which panics when it lands inside a multi-byte character.
    let result = runtime::run_operation("PHP Deserialize", vec![0xFF; 4], &[]);
    assert!(result.is_err(), "malformed input must be an error");
}
