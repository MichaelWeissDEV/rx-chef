// Tests for the colossus operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations colossus::

use rxchef::operations::colossus::Colossus;
use rxchef::Operation;

fn run(input: &str) -> String {
    let op = Colossus;
    String::from_utf8(op.run(input.as_bytes().to_vec(), &[]).unwrap()).unwrap()
}

#[test]
fn test_colossus_basic_input() {
    // Test with valid ITA2 characters
    let result = run("HELLO");
    assert!(result.contains("printout"));
    assert!(result.contains("counters"));
    assert!(result.contains("runcount"));
}

#[test]
fn test_colossus_empty_input() {
    let result = run("");
    assert!(result.contains("printout"));
    assert!(result.contains("counters"));
    assert!(result.contains("runcount"));
}

#[test]
fn test_colossus_valid_ita2_characters() {
    // Test with various valid ITA2 characters
    let valid_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ34589+-./ ";
    let result = run(valid_chars);
    assert!(result.contains("printout"));
    assert!(result.contains("counters"));
    assert!(result.contains("runcount"));
}

#[test]
fn test_colossus_invalid_characters() {
    let op = Colossus;
    // Test with invalid ITA2 characters (lowercase gets converted to uppercase)
    // Use characters that are invalid even when uppercase
    let result = op.run("123@#$".as_bytes().to_vec(), &[]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid ITA2 character"));
}

#[test]
fn test_colossus_mixed_case() {
    // Test that input is converted to uppercase
    let result = run("hello");
    // Should work because it gets converted to uppercase
    assert!(result.contains("printout"));
    assert!(result.contains("counters"));
    assert!(result.contains("runcount"));
}

#[test]
fn test_colossus_special_ita2_chars() {
    // Test with special ITA2 characters
    let result = run("34589+-./");
    assert!(result.contains("printout"));
    assert!(result.contains("counters"));
    assert!(result.contains("runcount"));
}

#[test]
fn test_colossus_long_input() {
    // Test with longer input
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".repeat(10);
    let result = run(&input);
    assert!(result.contains("printout"));
    assert!(result.contains("counters"));
    assert!(result.contains("runcount"));
}

#[test]
fn test_colossus_json_structure() {
    // Test that the output is valid JSON with expected structure
    let result = run("HELLO");
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed.is_object());
    assert!(parsed.get("printout").is_some());
    assert!(parsed.get("counters").is_some());
    assert!(parsed.get("runcount").is_some());
    assert_eq!(parsed["runcount"], 1);
}

#[test]
fn test_colossus_spaces_and_punctuation() {
    // Test with spaces and punctuation
    let result = run("HELLO WORLD.");
    assert!(result.contains("printout"));
    assert!(result.contains("counters"));
    assert!(result.contains("runcount"));
}

#[test]
fn test_colossus_numeric_chars() {
    // Test with numeric characters valid in ITA2
    let result = run("34589");
    assert!(result.contains("printout"));
    assert!(result.contains("counters"));
    assert!(result.contains("runcount"));
}
