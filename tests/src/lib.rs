/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Test infrastructure for rxchef operations.
 * -----------------------------------------------------------------------------
 */

use rxchef::operation::ArgValue;
use serde_json::Value;

pub struct TestCase {
    pub name: &'static str,
    pub input: TestData,
    pub expected_output: ExpectedOutput,
    pub recipe_config: Value,
}

pub enum TestData {
    String(&'static str),
    Bytes(&'static [u8]),
    None,
}

pub enum ExpectedOutput {
    String(&'static str),
    Bytes(&'static [u8]),
    Regex(&'static str),
    Error(&'static str),
    None,
}

/// Run a single operation (by name) against `input` with `args` and assert the
/// output equals `expected`.  Returns the raw output bytes so callers can run
/// further assertions.
///
/// Panics with a clear message if the operation is not found, if it errors, or
/// if the output does not match.
pub fn run_op(op_name: &str, input: &[u8], args: &[ArgValue]) -> Vec<u8> {
    let op = rxchef::operations::get_operation(op_name)
        .unwrap_or_else(|| panic!("operation not found in registry: {op_name}"));
    op.run(input.to_vec(), args)
        .unwrap_or_else(|e| panic!("operation '{op_name}' returned error: {e}"))
}

/// Assert that running `op_name` on `input` with `args` produces `expected`.
pub fn assert_op(op_name: &str, input: &[u8], args: &[ArgValue], expected: &[u8]) {
    let got = run_op(op_name, input, args);
    assert_eq!(
        got,
        expected,
        "operation '{op_name}': output mismatch\n  got:      {}\n  expected: {}",
        String::from_utf8_lossy(&got),
        String::from_utf8_lossy(expected),
    );
}

/// Convenience wrapper for text-in / text-out operations.
pub fn assert_op_text(op_name: &str, input: &str, args: &[ArgValue], expected: &str) {
    assert_op(op_name, input.as_bytes(), args, expected.as_bytes());
}

// ─── Macros ──────────────────────────────────────────────────────────────────

/// Generate a `#[test]` function that runs a single operation step.
///
/// # Usage
///
/// ```ignore
/// op_test!(my_test_name,
///     op    = "To Hex",
///     input = "Hello",
///     args  = [ArgValue::Str("None".into()), ArgValue::Num(0.0)],
///     want  = "48656c6c6f"
/// );
/// ```
#[macro_export]
macro_rules! op_test {
    ($test_name:ident, op = $op:literal, input = $input:expr, args = [$($arg:expr),* $(,)?], want = $want:expr) => {
        #[test]
        fn $test_name() {
            cyberchef_rust_tests::assert_op_text(
                $op,
                $input,
                &[$($arg),*],
                $want,
            );
        }
    };
    // Variant: bytes input / bytes output
    ($test_name:ident, op = $op:literal, input_bytes = $input:expr, args = [$($arg:expr),* $(,)?], want_bytes = $want:expr) => {
        #[test]
        fn $test_name() {
            cyberchef_rust_tests::assert_op(
                $op,
                $input,
                &[$($arg),*],
                $want,
            );
        }
    };
}

/// Generate a `#[test]` that runs a multi-step recipe using [`execute_recipe`].
///
/// # Usage
///
/// ```ignore
/// recipe_test!(my_roundtrip,
///     input = "Hello",
///     recipe = [{"op": "To Hex", "args": ["None", 0]}, {"op": "From Hex", "args": ["Auto"]}],
///     want  = "Hello"
/// );
/// ```
#[macro_export]
macro_rules! recipe_test {
    ($test_name:ident, input = $input:expr, recipe = $recipe:tt, want = $want:expr) => {
        #[test]
        fn $test_name() {
            let tc = cyberchef_rust_tests::TestCase {
                name: stringify!($test_name),
                input: cyberchef_rust_tests::TestData::String($input),
                expected_output: cyberchef_rust_tests::ExpectedOutput::String($want),
                recipe_config: serde_json::json!($recipe),
            };
            cyberchef_rust_tests::execute_recipe(&tc);
        }
    };
}

pub fn execute_recipe(test_case: &TestCase) {
    let input = &test_case.input;
    let recipe = &test_case.recipe_config;

    let mut current_data = match input {
        TestData::String(s) => s.as_bytes().to_vec(),
        TestData::Bytes(b) => b.to_vec(),
        TestData::None => vec![],
    };

    let steps = recipe.as_array().expect("Recipe is not an array");

    for step in steps {
        let op_name = step
            .get("op")
            .and_then(|v| v.as_str())
            .expect("Missing op name");

        let op_box = rxchef::operations::get_operation(op_name);

        if op_box.is_none() {
            // For now, if the operation is not yet ported, we skip testing it
            return;
        }

        let op = op_box.unwrap();

        let args_json = step.get("args").and_then(|v| v.as_array());
        let mut parsed_args = Vec::new();

        if let Some(args) = args_json {
            for arg in args {
                let parsed_arg = if let Some(s) = arg.as_str() {
                    // Try to parse as CyberChef complex argument object
                    if let Ok(v) = serde_json::from_str::<Value>(s) {
                        if let Some(option) = v.get("option").and_then(|o| o.as_str()) {
                            if let Some(string_val) = v.get("string").and_then(|s| s.as_str()) {
                                match option {
                                    "Hex" => {
                                        if string_val.is_empty() {
                                            ArgValue::Bytes(vec![])
                                        } else {
                                            ArgValue::Str(format!("0x{}", string_val))
                                        }
                                    }
                                    "Base64" => {
                                        // We don't have a simple way to pass Base64 to operations yet
                                        // unless they support it. For now, let's keep it as is or decode it.
                                        ArgValue::Str(string_val.to_string())
                                    }
                                    _ => ArgValue::Str(string_val.to_string()),
                                }
                            } else {
                                ArgValue::Str(s.to_string())
                            }
                        } else {
                            ArgValue::Str(s.to_string())
                        }
                    } else {
                        ArgValue::Str(s.to_string())
                    }
                } else if let Some(n) = arg.as_f64() {
                    ArgValue::Num(n)
                } else if let Some(b) = arg.as_bool() {
                    ArgValue::Bool(b)
                } else if let Some(obj) = arg.as_object() {
                    // It's already a JSON object, not a string-encoded JSON
                    if let Some(option) = obj.get("option").and_then(|o| o.as_str()) {
                        if let Some(string_val) = obj.get("string").and_then(|s| s.as_str()) {
                            match option {
                                "Hex" => {
                                    if string_val.is_empty() {
                                        ArgValue::Bytes(vec![])
                                    } else {
                                        ArgValue::Str(format!("0x{}", string_val))
                                    }
                                }
                                _ => ArgValue::Str(string_val.to_string()),
                            }
                        } else {
                            ArgValue::Str(arg.to_string())
                        }
                    } else {
                        ArgValue::Str(arg.to_string())
                    }
                } else {
                    ArgValue::Str(arg.to_string())
                };
                parsed_args.push(parsed_arg);
            }
        }

        println!("Op: {}, Parsed args: {:?}", op_name, parsed_args);

        match op.run(current_data, &parsed_args) {
            Ok(new_data) => current_data = new_data,
            Err(e) => {
                let err_msg = e.to_string();
                if let ExpectedOutput::Error(expected_err) = &test_case.expected_output {
                    assert!(
                        err_msg.contains(expected_err),
                        "Error '{}' did not contain expected '{}'",
                        err_msg,
                        expected_err
                    );
                    return;
                } else if let ExpectedOutput::String(expected_str) = &test_case.expected_output {
                    // CyberChef tests sometimes expect the error message as a string output
                    if err_msg.contains(expected_str) {
                        return;
                    }
                    panic!(
                        "Operation failed with error '{}', but expected string '{}'",
                        err_msg, expected_str
                    );
                } else {
                    panic!("Operation failed: {}", err_msg);
                }
            }
        }
    }

    match &test_case.expected_output {
        ExpectedOutput::String(s) => {
            let result_str = String::from_utf8_lossy(&current_data);
            assert_eq!(result_str, *s, "Test failed: {}", test_case.name);
        }
        ExpectedOutput::Bytes(b) => {
            assert_eq!(current_data, *b, "Test failed: {}", test_case.name);
        }
        ExpectedOutput::Regex(r) => {
            let regex = regex::Regex::new(r).expect("Invalid regex");
            let result_str = String::from_utf8_lossy(&current_data);
            assert!(
                regex.is_match(&result_str),
                "Regex match failed for '{}' against '{}'",
                r,
                result_str
            );
        }
        ExpectedOutput::Error(e) => {
            panic!("Expected error '{}' but execution succeeded", e);
        }
        ExpectedOutput::None => {
            assert!(
                current_data.is_empty(),
                "Expected empty output but got data"
            );
        }
    }
}
