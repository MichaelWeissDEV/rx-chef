// Tests for the xkcd_random_number operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xkcd_random_number::

use rxchef::operations::xkcd_random_number::XkcdRandomNumberOp;
use rxchef::Operation;

#[test]
fn test_xkcd_random_number() {
    let op = XkcdRandomNumberOp;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, b"4");
}
