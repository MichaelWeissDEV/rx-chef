// Tests for the parse_unix_file_permissions operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_unix_file_permissions::

use rxchef::operations::parse_unix_file_permissions::ParseUNIXFilePermissions;
use rxchef::Operation;

#[test]
fn test_octal() {
    let op = ParseUNIXFilePermissions;
    let input = b"755".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Textual representation: -rwxr-xr-x"));
    assert!(result_str.contains("Octal representation:   0755"));
}
#[test]
fn test_textual() {
    let op = ParseUNIXFilePermissions;
    let input = b"drwxr-xr-x".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Textual representation: drwxr-xr-x"));
    assert!(result_str.contains("Octal representation:   0755"));
    assert!(result_str.contains("File type: Directory"));
}
