// Tests for the bit_shift_left operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bit_shift_left::

use rxchef::operation::ArgValue;
use rxchef::operations::bit_shift_left::BitShiftLeft;
use rxchef::Operation;

#[test]
fn test_bit_shift_left_basic() {
    let op = BitShiftLeft;
    let input = vec![0b01010101, 0b10101010, 0b11111111, 0b00000000];
    let args = [ArgValue::Num(1.0)];
    let result = op.run(input.clone(), &args).unwrap();
    // 0b01010101 << 1 = 0b10101010
    // 0b10101010 << 1 = 0b01010100 (with overflow masked)
    // 0b11111111 << 1 = 0b11111110
    // 0b00000000 << 1 = 0b00000000
    assert_eq!(result, vec![0b10101010, 0b01010100, 0b11111110, 0b00000000]);
}
#[test]
fn test_bit_shift_left_shift_2() {
    let op = BitShiftLeft;
    let input = vec![0b00001111];
    let args = [ArgValue::Num(2.0)];
    let result = op.run(input.clone(), &args).unwrap();
    // 0b00001111 << 2 = 0b00111100
    assert_eq!(result, vec![0b00111100]);
}
#[test]
fn test_bit_shift_left_shift_3() {
    let op = BitShiftLeft;
    let input = b"Hello".to_vec();
    let args = [ArgValue::Num(3.0)];
    let result = op.run(input.clone(), &args).unwrap();
    // H = 0x48 = 0b01001000 << 3 = 0b01000000 = 64
    // e = 0x65 = 0b01100101 << 3 = 0b00101000 = 40
    // l = 0x6c = 0b01101100 << 3 = 0b01100000 = 96
    // l = 0x6c = 0b01101100 << 3 = 0b01100000 = 96
    // o = 0x6f = 0b01101111 << 3 = 888 & 0xff = 120
    assert_eq!(result, vec![64, 40, 96, 96, 120]);
}
#[test]
fn test_bit_shift_left_default_amount() {
    let op = BitShiftLeft;
    let input = vec![0b00000001];
    let result = op.run(input.clone(), &[]).unwrap();
    // 0b00000001 << 1 = 0b00000010
    assert_eq!(result, vec![0b00000010]);
}
