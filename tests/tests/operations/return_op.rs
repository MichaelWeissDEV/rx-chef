// Tests for the return_op operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations return_op::

use rxchef::operations::return_op::ReturnOp;
use rxchef::Operation;

#[test]
fn test_return_basic() {
    let op = ReturnOp;
    let input = b"test data".to_vec();
    let args = [];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}

#[test]
fn test_return_empty_input() {
    let op = ReturnOp;
    let input = b"".to_vec();
    let args = [];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}

#[test]
fn test_return_unicode_input() {
    let op = ReturnOp;
    let input = "Hello 世界".as_bytes().to_vec();
    let args = [];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}

#[test]
fn test_return_binary_data() {
    let op = ReturnOp;
    let input = vec![0x00, 0x01, 0xFF, 0xFE, 0xAA, 0xBB];
    let args = [];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}

#[test]
fn test_return_large_input() {
    let op = ReturnOp;
    let input = vec![b'A'; 10000]; // 10KB of 'A's
    let args = [];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
