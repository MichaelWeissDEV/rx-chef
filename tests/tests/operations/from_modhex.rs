// Tests for the from_modhex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_modhex::

use rxchef::operation::ArgValue;
use rxchef::operations::from_modhex::FromModhex;
use rxchef::Operation;

#[test]
fn test_from_modhex_basic() {
    let op = FromModhex;
    // "cb" in modhex: c=0, b=1 -> 0x01; "de" -> d=2, e=3 -> 0x23
    let input = b"cbde".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, vec![0x01, 0x23]);
}
#[test]
fn test_from_modhex_with_space() {
    let op = FromModhex;
    let input = b"cb de".to_vec();
    let args = [ArgValue::Str("Space".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, vec![0x01, 0x23]);
}
#[test]
fn test_from_modhex_empty() {
    let op = FromModhex;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_from_modhex_invalid_char() {
    let op = FromModhex;
    let input = b"zz".to_vec();
    assert!(op.run(input, &[]).is_err());
}
