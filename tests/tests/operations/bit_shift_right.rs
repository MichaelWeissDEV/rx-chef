// Tests for the bit_shift_right operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bit_shift_right::

use rxchef::operation::ArgValue;
use rxchef::operations::bit_shift_right::BitShiftRight;
use rxchef::Operation;

#[test]
fn test_bit_shift_right_logical_basic() {
    let op = BitShiftRight;
    let input = vec![0b01010101, 0b10101010, 0b11111111, 0b00000000];
    let args = [
        ArgValue::Num(1.0),
        ArgValue::Str("Logical shift".to_string()),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    // 0b01010101 >> 1 = 0b00101010
    // 0b10101010 >> 1 = 0b01010101
    // 0b11111111 >> 1 = 0b01111111
    // 0b00000000 >> 1 = 0b00000000
    assert_eq!(result, vec![0b00101010, 0b01010101, 0b01111111, 0b00000000]);
}
#[test]
fn test_bit_shift_right_arithmetic_basic() {
    let op = BitShiftRight;
    let input = vec![0b11010101, 0b10101010, 0b01111111];
    let args = [
        ArgValue::Num(1.0),
        ArgValue::Str("Arithmetic shift".to_string()),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    // 0b11010101 >> 1 = 0b01101010, preserve MSB 1: 0b11101010
    // 0b10101010 >> 1 = 0b01010101, preserve MSB 1: 0b11010101
    // 0b01111111 >> 1 = 0b00111111, preserve MSB 0: 0b00111111
    assert_eq!(result, vec![0b11101010, 0b11010101, 0b00111111]);
}
#[test]
fn test_bit_shift_right_shift_2() {
    let op = BitShiftRight;
    let input = vec![0b00111100];
    let args = [
        ArgValue::Num(2.0),
        ArgValue::Str("Logical shift".to_string()),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    // 0b00111100 >> 2 = 0b00001111
    assert_eq!(result, vec![0b00001111]);
}
#[test]
fn test_bit_shift_right_default_type() {
    let op = BitShiftRight;
    let input = vec![0b01010101];
    let args = [ArgValue::Num(1.0)]; // Default is Logical shift
    let result = op.run(input.clone(), &args).unwrap();
    // 0b01010101 >> 1 = 0b00101010
    assert_eq!(result, vec![0b00101010]);
}
