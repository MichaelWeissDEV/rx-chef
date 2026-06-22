// Tests for the take_nth_bytes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations take_nth_bytes::

use rxchef::operation::ArgValue;
use rxchef::operations::take_nth_bytes::TakeNthBytes;
use rxchef::Operation;

fn run_op(input: &[u8], n: usize, start: usize, each_line: bool) -> Vec<u8> {
    let op = TakeNthBytes;
    let args = [
        ArgValue::Num(n as f64),
        ArgValue::Num(start as f64),
        ArgValue::Bool(each_line),
    ];
    op.run(input.to_vec(), &args).unwrap()
}
#[test]
fn test_take_nth_nothing() {
    let result = run_op(b"", 4, 0, false);
    assert_eq!(result, b"");
}
#[test]
fn test_take_nth_basic_single_line() {
    // "0123456789" take every 4 starting at 0 -> '0','4','8' -> "048"
    let result = run_op(b"0123456789", 4, 0, false);
    assert_eq!(result, b"048");
}
#[test]
fn test_take_nth_complex_single_line() {
    // "0123456789" take every 4 starting at 5 -> '5','9' -> "59"
    let result = run_op(b"0123456789", 4, 5, false);
    assert_eq!(result, b"59");
}
#[test]
fn test_take_nth_basic_multi_line_no_apply() {
    // "01234\n56789" take every 4 at 0, not applied to each line
    // indices: 0='0',1='1',2='2',3='3',4='4',5='\n',6='5',7='6',8='7',9='8',10='9'
    // offset=0, hits: i=0,4,8 -> '0','4','7'
    let result = run_op(b"01234\n56789", 4, 0, false);
    assert_eq!(result, b"047");
}
#[test]
fn test_take_nth_basic_multi_line_apply() {
    // "01234\n56789" take every 4 at 0, applied to each line
    // Line1 "01234": hits 0,4 -> "04"
    // Line2 "56789": hits 0,4 -> "59"
    let result = run_op(b"01234\n56789", 4, 0, true);
    assert_eq!(result, b"04\n59");
}
#[test]
fn test_take_nth_complex_multi_line_apply() {
    // "012345\n6789ab" take every 4 at 5, applied to each line
    // Line1 "012345": start=5, i=5 -> i-offset=5 >= start=5, (5-5)%4=0 -> '5'
    // Line2 "6789ab": start=5, i=5 -> 'b'
    let result = run_op(b"012345\n6789ab", 4, 5, true);
    assert_eq!(result, b"5\nb");
}
#[test]
fn test_take_nth_invalid_n() {
    let op = TakeNthBytes;
    let args = [
        ArgValue::Num(0.0),
        ArgValue::Num(0.0),
        ArgValue::Bool(false),
    ];
    let result = op.run(b"test".to_vec(), &args);
    assert!(result.is_err());
}
#[test]
fn test_take_nth_invalid_start() {
    let op = TakeNthBytes;
    let args = [
        ArgValue::Num(4.0),
        ArgValue::Num(-1.0),
        ArgValue::Bool(false),
    ];
    let result = op.run(b"test".to_vec(), &args);
    assert!(result.is_err());
}
