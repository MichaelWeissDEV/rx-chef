// Tests for the file_tree operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations file_tree::

use rxchef::operation::ArgValue;
use rxchef::operations::file_tree::FileTree;
use rxchef::Operation;

#[test]
fn test_file_tree() {
    let op = FileTree;
    let input = b"home/user/docs/file1.txt\nhome/user/pics/img1.jpg\nvar/log/syslog".to_vec();
    let args = [
        ArgValue::Str("/".to_string()),
        ArgValue::Str("Line feed".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    let expected = "home\n|---user\n|   |---docs\n|   |   |---file1.txt\n|   |---pics\n|   |   |---img1.jpg\nvar\n|---log\n|   |---syslog";
    assert_eq!(out, expected);
}
