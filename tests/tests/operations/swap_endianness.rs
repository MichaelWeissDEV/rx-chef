// Tests for the swap_endianness operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations swap_endianness::

use rxchef::operation::ArgValue;
use rxchef::operations::swap_endianness::SwapEndianness;
use rxchef::Operation;

fn run_op(input: &[u8], format: &str, word_len: usize, pad: bool) -> Vec<u8> {
    let op = SwapEndianness;
    let args = [
        ArgValue::Str(format.to_string()),
        ArgValue::Num(word_len as f64),
        ArgValue::Bool(pad),
    ];
    op.run(input.to_vec(), &args).unwrap()
}
#[test]
fn test_swap_endianness_raw_4_bytes() {
    // [0x01, 0x02, 0x03, 0x04] reversed -> [0x04, 0x03, 0x02, 0x01]
    let result = run_op(&[0x01, 0x02, 0x03, 0x04], "Raw", 4, true);
    assert_eq!(result, vec![0x04, 0x03, 0x02, 0x01]);
}
#[test]
fn test_swap_endianness_raw_8_bytes_two_words() {
    // [0x01,0x02,0x03,0x04, 0x05,0x06,0x07,0x08] with word_len=4
    // -> [0x04,0x03,0x02,0x01, 0x08,0x07,0x06,0x05]
    let result = run_op(
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        "Raw",
        4,
        true,
    );
    assert_eq!(result, vec![0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05]);
}
#[test]
fn test_swap_endianness_raw_incomplete_word_with_pad() {
    // [0x01, 0x02] with word_len=4, pad=true
    // word padded to [0x01, 0x02, 0x00, 0x00], reversed -> [0x00, 0x00, 0x02, 0x01]
    let result = run_op(&[0x01, 0x02], "Raw", 4, true);
    assert_eq!(result, vec![0x00, 0x00, 0x02, 0x01]);
}
#[test]
fn test_swap_endianness_raw_incomplete_word_no_pad() {
    // [0x01, 0x02] with word_len=4, pad=false
    // word stays [0x01, 0x02], reversed -> [0x02, 0x01]
    let result = run_op(&[0x01, 0x02], "Raw", 4, false);
    assert_eq!(result, vec![0x02, 0x01]);
}
#[test]
fn test_swap_endianness_invalid_word_length() {
    let op = SwapEndianness;
    let args = [
        ArgValue::Str("Raw".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
    ];
    let result = op.run(b"test".to_vec(), &args);
    assert!(result.is_err());
}
#[test]
fn test_swap_endianness_hex_format() {
    // hex input "01020304" -> bytes [0x01,0x02,0x03,0x04] -> reversed -> "04 03 02 01"
    let result = run_op(b"01020304", "Hex", 4, true);
    let s = String::from_utf8(result).unwrap();
    assert_eq!(s, "04 03 02 01");
}
