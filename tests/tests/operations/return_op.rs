// Tests for the return_op operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations return_op::

use rxchef::operations::return_op::ReturnOp;
use rxchef::Operation;

#[test]
fn test_return() {
    let op = ReturnOp;
    let input = b"test data".to_vec();
    let args = [];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
