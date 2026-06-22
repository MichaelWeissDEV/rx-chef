// Tests for the sub operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sub::

use rxchef::operation::ArgValue;
use rxchef::operations::sub::SUB;
use rxchef::Operation;

#[test]
fn test_sub_basic_hex_key() {
    // (0x48 - 0x01) % 256 = 0x47  etc.
    let op = SUB;
    let input = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
    let args = [ArgValue::Str("0x01".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, vec![0x47, 0x64, 0x6b, 0x6b, 0x6e]);
}
#[test]
fn test_sub_wraps() {
    // (0x00 - 0x01 + 256) % 256 = 255
    let op = SUB;
    let args = [ArgValue::Str("01".to_string())];
    let result = op.run(vec![0x00], &args).unwrap();
    assert_eq!(result, vec![0xff]);
}
#[test]
fn test_sub_empty_key_passthrough() {
    let op = SUB;
    let input = b"hello".to_vec();
    let args = [ArgValue::Str("".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(result, input);
}
#[test]
fn test_sub_null_preserving() {
    let op = SUB;
    // byte 0x00 should be preserved; 0x05 - 0x03 = 0x02
    let input = vec![0x00, 0x05];
    let args = [
        ArgValue::Str("03".to_string()),
        ArgValue::Str("Standard".to_string()),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, vec![0x00, 0x02]);
}
#[test]
fn test_sub_key_cycles() {
    // key = [0x01, 0x02], input = [0x03, 0x04, 0x05]
    // 0x03-0x01=0x02, 0x04-0x02=0x02, 0x05-0x01=0x04
    let op = SUB;
    let args = [ArgValue::Str("0x0102".to_string())];
    let result = op.run(vec![0x03, 0x04, 0x05], &args).unwrap();
    assert_eq!(result, vec![0x02, 0x02, 0x04]);
}
