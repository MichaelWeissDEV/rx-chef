// Tests for the comment operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations comment::

use rxchef::operation::ArgValue;
use rxchef::operations::comment::CommentOp;
use rxchef::Operation;

#[test]
fn test_comment_pass_through() {
    let op = CommentOp;
    let input = b"test input".to_vec();
    let args: [ArgValue; 0] = [];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
#[test]
fn test_comment_with_text() {
    let op = CommentOp;
    let input = b"some data".to_vec();
    let args = [ArgValue::Str("This is a comment".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
