// Tests for the unescape_string operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations unescape_string::

use rxchef::operations::unescape_string::UnescapeString;
use rxchef::Operation;

#[test]
fn test_unescape_basic() {
    let op = UnescapeString;
    let input = b"Hello\\nWorld".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "Hello\nWorld");
}
#[test]
fn test_unescape_hex_unicode() {
    let op = UnescapeString;
    let input = b"\\x41\\u0042\\u{0043}".to_vec();
    let result = op.run(input, &[]).unwrap();
    // \x41 = 'A', \u0042 = 'B', \u{0043} = 'C'
    assert_eq!(String::from_utf8_lossy(&result), "ABC");
}
#[test]
fn test_unescape_octal() {
    let op = UnescapeString;
    let input = b"\\101".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "A");
}
#[test]
fn test_unescape_backslash() {
    let op = UnescapeString;
    let input = b"\\\\".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "\\");
}
