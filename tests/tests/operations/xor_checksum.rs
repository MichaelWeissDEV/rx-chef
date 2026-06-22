// Tests for the xor_checksum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xor_checksum::

use rxchef::operation::ArgValue;
use rxchef::operations::xor_checksum::XORChecksum;
use rxchef::Operation;

// Test vectors from CyberChef/tests/operations/tests/XORChecksum.mjs
fn run_str(input: &[u8], blocksize: usize) -> String {
    let op = XORChecksum;
    let args = [ArgValue::Num(blocksize as f64)];
    String::from_utf8(op.run(input.to_vec(), &args).unwrap()).unwrap()
}
const BASIC: &[u8] = b"The ships hung in the sky in much the same way that bricks don't.";
// UTF-8 bytes for the Georgian string used in the JS test
// "nu panik'as" in Georgian Unicode
fn utf8_str() -> Vec<u8> {
    "\u{10dc}\u{10e3} \u{10de}\u{10d0}\u{10dc}\u{10d8}\u{10d9}\u{10d0}\u{10e1}"
        .as_bytes()
        .to_vec()
}
fn all_bytes() -> Vec<u8> {
    (0u8..=255).collect()
}
// ---- blocksize 1 ----
#[test]
fn test_bs1_empty() {
    assert_eq!(run_str(b"", 1), "00");
}
#[test]
fn test_bs1_basic_string() {
    assert_eq!(run_str(BASIC, 1), "08");
}
#[test]
fn test_bs1_utf8() {
    assert_eq!(run_str(&utf8_str(), 1), "df");
}
#[test]
fn test_bs1_all_bytes() {
    assert_eq!(run_str(&all_bytes(), 1), "00");
}
// ---- blocksize 4 ----
#[test]
fn test_bs4_empty() {
    assert_eq!(run_str(b"", 4), "00000000");
}
#[test]
fn test_bs4_basic_string() {
    assert_eq!(run_str(BASIC, 4), "4918421b");
}
#[test]
fn test_bs4_utf8() {
    assert_eq!(run_str(&utf8_str(), 4), "83a424dc");
}
#[test]
fn test_bs4_all_bytes() {
    assert_eq!(run_str(&all_bytes(), 4), "00000000");
}
