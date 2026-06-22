// Tests for the not operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations not::

use rxchef::operations::not::NOT;
use rxchef::Operation;

#[test]
fn test_not_basic() {
    let op = NOT;
    // !0x00 = 0xff, !0xff = 0x00, !0xaa = 0x55
    let result = op.run(vec![0x00, 0xff, 0xaa], &[]).unwrap();
    assert_eq!(result, vec![0xff, 0x00, 0x55]);
}
#[test]
fn test_not_empty() {
    let op = NOT;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}
#[test]
fn test_not_double_inverse() {
    // Applying NOT twice should restore the original
    let op = NOT;
    let original = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
    let once = op.run(original.clone(), &[]).unwrap();
    let twice = op.run(once, &[]).unwrap();
    assert_eq!(twice, original);
}
#[test]
fn test_not_all_bytes() {
    let op = NOT;
    let input: Vec<u8> = (0u8..=255).collect();
    let result = op.run(input.clone(), &[]).unwrap();
    for (i, &b) in input.iter().enumerate() {
        assert_eq!(result[i], !b);
    }
}
