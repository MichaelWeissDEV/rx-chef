// Tests for the extract_file_paths operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_file_paths::

use rxchef::operation::ArgValue;
use rxchef::operations::extract_file_paths::ExtractFilePaths;
use rxchef::Operation;

#[test]
fn test_extract_file_paths_unix() {
    let op = ExtractFilePaths;
    let input = b"Some text /etc/passwd and /home/user/test.txt".to_vec();
    let args = &[
        ArgValue::Bool(false), // Windows
        ArgValue::Bool(true),  // UNIX
        ArgValue::Bool(false), // Display total
        ArgValue::Bool(true),  // Sort
        ArgValue::Bool(true),  // Unique
    ];
    let result = op.run(input, args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "/etc/passwd\n/home/user/test.txt");
}
#[test]
fn test_extract_file_paths_windows() {
    let op = ExtractFilePaths;
    let input = b"File at C:\\Windows\\System32\\calc.exe and D:\\data.txt".to_vec();
    let args = &[
        ArgValue::Bool(true),  // Windows
        ArgValue::Bool(false), // UNIX
        ArgValue::Bool(false), // Display total
        ArgValue::Bool(true),  // Sort
        ArgValue::Bool(true),  // Unique
    ];
    let result = op.run(input, args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "C:\\Windows\\System32\\calc.exe\nD:\\data.txt");
}
