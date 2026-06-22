// Tests for the rotate_left operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rotate_left::

use rxchef::operation::ArgValue;
use rxchef::operations::rotate_left::RotateLeft;
use rxchef::Operation;

fn run_op(input: &[u8], amount: usize, carry: bool) -> Vec<u8> {
    let op = RotateLeft;
    let args = [ArgValue::Num(amount as f64), ArgValue::Bool(carry)];
    op.run(input.to_vec(), &args).unwrap()
}
#[test]
fn test_rotate_left_nothing() {
    let result = run_op(b"", 1, false);
    assert_eq!(result, b"");
}
#[test]
fn test_rotate_left_normal() {
    // From JS test: "61 62 63 31 32 33" rotl by 1 -> "c2 c4 c6 62 64 66"
    let input = [0x61u8, 0x62, 0x63, 0x31, 0x32, 0x33];
    let result = run_op(&input, 1, false);
    assert_eq!(result, vec![0xc2, 0xc4, 0xc6, 0x62, 0x64, 0x66]);
}
#[test]
fn test_rotate_left_carry() {
    // From JS test: "61 62 63 31 32 33" rotl carry 2 -> "85 89 8c c4 c8 cd"
    let input = [0x61u8, 0x62, 0x63, 0x31, 0x32, 0x33];
    let result = run_op(&input, 2, true);
    assert_eq!(result, vec![0x85, 0x89, 0x8c, 0xc4, 0xc8, 0xcd]);
}
#[test]
fn test_rotate_left_single_byte() {
    // 0b10110001 = 0xB1 rotl 1 -> 0b01100011 = 0x63
    let result = run_op(&[0xB1], 1, false);
    assert_eq!(result, vec![0x63]);
}
#[test]
fn test_rotate_left_full_rotation() {
    // Rotating 8 times should give same result as no rotation
    let input = vec![0x61u8, 0x62, 0x63];
    let result = run_op(&input, 8, false);
    assert_eq!(result, input);
}
