// Tests for the rotate_right operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rotate_right::

use rxchef::operation::ArgValue;
use rxchef::operations::rotate_right::RotateRight;
use rxchef::Operation;

fn run_op(input: &[u8], amount: usize, carry: bool) -> Vec<u8> {
    let op = RotateRight;
    let args = [ArgValue::Num(amount as f64), ArgValue::Bool(carry)];
    op.run(input.to_vec(), &args).unwrap()
}
#[test]
fn test_rotate_right_nothing() {
    let result = run_op(b"", 1, false);
    assert_eq!(result, b"");
}
#[test]
fn test_rotate_right_normal() {
    // From JS test: "61 62 63 31 32 33" rotr by 1 -> "b0 31 b1 98 19 99"
    let input = [0x61u8, 0x62, 0x63, 0x31, 0x32, 0x33];
    let result = run_op(&input, 1, false);
    assert_eq!(result, vec![0xb0, 0x31, 0xb1, 0x98, 0x19, 0x99]);
}
#[test]
fn test_rotate_right_carry() {
    // From JS test: "61 62 63 31 32 33" rotr carry 2 -> "d8 58 98 cc 4c 8c"
    let input = [0x61u8, 0x62, 0x63, 0x31, 0x32, 0x33];
    let result = run_op(&input, 2, true);
    assert_eq!(result, vec![0xd8, 0x58, 0x98, 0xcc, 0x4c, 0x8c]);
}
#[test]
fn test_rotate_right_single_byte() {
    // 0b10110001 = 0xB1 rotr 1 -> 0b11011000 = 0xD8
    let result = run_op(&[0xB1], 1, false);
    assert_eq!(result, vec![0xD8]);
}
#[test]
fn test_rotate_right_full_rotation() {
    // Rotating 8 times should give same result as no rotation
    let input = vec![0x61u8, 0x62, 0x63];
    let result = run_op(&input, 8, false);
    assert_eq!(result, input);
}
