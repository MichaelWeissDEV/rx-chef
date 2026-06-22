// Tests for the to_modhex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_modhex::

use rxchef::operation::ArgValue;
use rxchef::operations::from_modhex::FromModhex;
use rxchef::operations::to_modhex::ToModhex;
use rxchef::Operation;

#[test]
fn test_to_modhex_basic() {
    let op = ToModhex;
    // 0x01 -> c=0, b=1 -> "cb"; 0x23 -> d=2, e=3 -> "de"
    let input = vec![0x01u8, 0x23];
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "cbde");
}
#[test]
fn test_to_modhex_with_space_delim() {
    let op = ToModhex;
    let input = vec![0x01u8, 0x23];
    let args = [ArgValue::Str("Space".to_string()), ArgValue::Num(0.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "cb de");
}
#[test]
fn test_roundtrip() {
    let to = ToModhex;
    let from = FromModhex;
    let original = vec![0xdeu8, 0xad, 0xbe, 0xef];
    let encoded = to.run(original.clone(), &[]).unwrap();
    let decoded = from.run(encoded, &[]).unwrap();
    assert_eq!(decoded, original);
}
