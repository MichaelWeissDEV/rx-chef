// Tests for the from_quoted_printable operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_quoted_printable::

use rxchef::operations::from_quoted_printable::FromQuotedPrintable;
use rxchef::Operation;

#[test]
fn test_from_quoted_printable_empty_input() {
    let op = FromQuotedPrintable;
    let args = [];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_quoted_printable_simple() {
    let op = FromQuotedPrintable;
    let args = [];
    // Simple QP encoding: "=48=65=6C=6C=6F" -> "Hello"
    let qp_input = "=48=65=6C=6C=6F";
    let result = op.run(qp_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_quoted_printable_mixed() {
    let op = FromQuotedPrintable;
    let args = [];
    // Mixed QP: "He=6C=6Co" -> "Hello"
    let qp_input = "He=6C=6Co";
    let result = op.run(qp_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_quoted_printable_soft_line_break() {
    let op = FromQuotedPrintable;
    let args = [];
    // QP with soft line breaks: "Hello=\r\nWorld" -> "HelloWorld"
    let qp_input = "Hello=\r\nWorld";
    let result = op.run(qp_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "HelloWorld");
}

#[test]
fn test_from_quoted_printable_space_encoding() {
    let op = FromQuotedPrintable;
    let args = [];
    // QP space encoding: "Hello=20World" -> "Hello World"
    let qp_input = "Hello=20World";
    let result = op.run(qp_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello World");
}
