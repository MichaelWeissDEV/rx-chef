// Tests for the drop_nth_bytes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations drop_nth_bytes::

use rxchef::operation::ArgValue;
use rxchef::operations::drop_nth_bytes::DropNthBytes;
use rxchef::Operation;

#[test]
fn test_drop_nth_bytes_basic() {
    let op = DropNthBytes;
    // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] drop every 4th starting at 0
    // i=0: 0-0<0? no, (0-0)%4==0? yes -> drop
    // i=1: 1-0<0? no, (1-0)%4==1? no -> keep (1)
    // i=2: 2-0<0? no, (2-0)%4==2? no -> keep (2)
    // i=3: 3-0<0? no, (3-0)%4==3? no -> keep (3)
    // i=4: 4-0<0? no, (4-0)%4==0? yes -> drop
    // i=5: 5-0<0? no, (5-0)%4==1? no -> keep (5)
    let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let args = [
        ArgValue::Num(4.0),
        ArgValue::Num(0.0),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, vec![1, 2, 3, 5, 6, 7, 9]);
}
#[test]
fn test_drop_nth_bytes_start_1() {
    let op = DropNthBytes;
    // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] drop every 4th starting at 1
    // i=0: 0-0<1? yes -> keep (0)
    // i=1: 1-0<1? no, (1-1)%4==0? yes -> drop
    // i=2: 2-0<1? no, (2-1)%4==1? no -> keep (2)
    // i=3: 3-0<1? no, (3-1)%4==2? no -> keep (3)
    // i=4: 4-0<1? no, (4-1)%4==3? no -> keep (4)
    // i=5: 5-0<1? no, (5-1)%4==0? yes -> drop
    let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let args = [
        ArgValue::Num(4.0),
        ArgValue::Num(1.0),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, vec![0, 2, 3, 4, 6, 7, 8]);
}
#[test]
fn test_drop_nth_bytes_apply_to_each_line() {
    let op = DropNthBytes;
    // Two lines: [1, 2, 3, 4, 0x0a, 5, 6, 7, 8]
    // drop every 2nd starting at 0
    let input = vec![1, 2, 3, 4, 0x0a, 5, 6, 7, 8];
    let args = [ArgValue::Num(2.0), ArgValue::Num(0.0), ArgValue::Bool(true)];
    let result = op.run(input, &args).unwrap();
    // Line 1: [1, 2, 3, 4] -> drop 1st, 3rd -> keep [2, 4]
    // Line 2: [5, 6, 7, 8] -> drop 1st, 3rd -> keep [6, 8]
    // Result: [2, 4, 0x0a, 6, 8]
    assert_eq!(result, vec![2, 4, 0x0a, 6, 8]);
}
#[test]
fn test_drop_nth_bytes_invalid_n() {
    let op = DropNthBytes;
    let input = vec![0, 1, 2, 3];
    let args = [
        ArgValue::Num(0.0),
        ArgValue::Num(0.0),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_drop_nth_bytes_invalid_start() {
    let op = DropNthBytes;
    let input = vec![0, 1, 2, 3];
    let args = [
        ArgValue::Num(4.0),
        ArgValue::Num(-1.0),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
