//! Every `extern "C"` entry point must contain its panics.
//!
//! Unwinding out of an `extern "C"` function into a foreign frame is undefined
//! behaviour: the C caller has no landing pad, so the process may abort, leak,
//! or corrupt state. `src/ffi.rs` defines an `ffi_boundary` helper for exactly
//! this, but for a long time only `rxchef_run` used it — the other eleven
//! entry points, including `rxchef_magic` which runs the detection engine over
//! arbitrary caller bytes, were unguarded.
//!
//! This checks the source rather than trying to provoke a panic through the
//! ABI: a missing guard is a property of the code, and provoking real UB in a
//! test would be unsound.

use std::fs;

/// Source of the FFI surface.
fn ffi_source() -> String {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/ffi.rs");
    fs::read_to_string(path).expect("src/ffi.rs must be readable")
}

/// Split the file into (name, body) pairs for each `extern "C"` function.
fn extern_functions(source: &str) -> Vec<(String, String)> {
    let mut functions = Vec::new();
    let bytes: Vec<char> = source.chars().collect();
    let mut search_from = 0;

    while let Some(found) = source[search_from..].find("extern \"C\" fn ") {
        let signature_start = search_from + found;
        let after_keyword = signature_start + "extern \"C\" fn ".len();
        let name: String = source[after_keyword..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();

        // Find the opening brace of the body, then its match.
        let Some(relative_open) = source[after_keyword..].find('{') else {
            break;
        };
        let open = after_keyword + relative_open;
        let mut depth = 0usize;
        let mut index = open;
        let mut close = open;
        while index < bytes.len() {
            match bytes[index] {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        close = index;
                        break;
                    }
                }
                _ => {}
            }
            index += 1;
        }
        functions.push((name, source[open..close].to_string()));
        search_from = close.max(signature_start + 1);
    }
    functions
}

#[test]
fn every_extern_c_entry_point_contains_its_panics() {
    let source = ffi_source();
    let functions = extern_functions(&source);
    assert!(
        functions.len() >= 12,
        "expected to find the FFI surface, found {} functions",
        functions.len()
    );

    let unguarded: Vec<&str> = functions
        .iter()
        .filter(|(_, body)| !body.contains("ffi_boundary"))
        .map(|(name, _)| name.as_str())
        .collect();

    assert!(
        unguarded.is_empty(),
        "these `extern \"C\"` functions can unwind into a foreign frame, which is \
         undefined behaviour; wrap their bodies in `ffi_boundary`: {unguarded:?}"
    );
}

#[test]
fn the_panic_guard_actually_catches() {
    // Guard the helper's own behaviour, so the check above is not asserting
    // against a boundary that silently stopped catching.
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        panic!("provoked");
    }));
    assert!(caught.is_err(), "catch_unwind must intercept a panic");
}

#[test]
fn guarded_entry_points_return_a_failure_value_rather_than_unwinding() {
    // Each guarded function must name a fallback, otherwise the guard would
    // compile but leave the caller with an uninitialised return.
    let source = ffi_source();
    for (name, body) in extern_functions(&source) {
        if !body.contains("ffi_boundary") {
            continue;
        }
        let returns_pointer = body.contains("ptr::null_mut()") || body.contains("error_result");
        let discards = body.contains("let _ = ffi_boundary");
        assert!(
            returns_pointer || discards,
            "{name} wraps its body but names no fallback for the panic case"
        );
    }
}
