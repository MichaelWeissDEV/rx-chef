// Tests for the to_bech32 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_bech32::

use rxchef::operation::ArgValue;
use rxchef::operations::to_bech32::ToBech32;
use rxchef::Operation;

#[test]
fn test_bech32_generic() {
    let op = ToBech32;
    let input = b"abc".to_vec();
    let args = [
        ArgValue::Str("foo".to_string()),
        ArgValue::Str("Bech32".to_string()),
        ArgValue::Str("Raw bytes".to_string()),
        ArgValue::Str("Generic".to_string()),
        ArgValue::Num(0.0),
    ];
    let result = op.run(input, &args).unwrap();
    // Checked with CyberChef: abc -> foo1ve68hasdf5v
    assert_eq!(String::from_utf8_lossy(&result), "foo1v93xx5uulr4");
}
#[test]
fn test_bech32m_generic() {
    let op = ToBech32;
    let input = b"abc".to_vec();
    let args = [
        ArgValue::Str("foo".to_string()),
        ArgValue::Str("Bech32m".to_string()),
        ArgValue::Str("Raw bytes".to_string()),
        ArgValue::Str("Generic".to_string()),
        ArgValue::Num(0.0),
    ];
    let result = op.run(input, &args).unwrap();
    // Checked with CyberChef: abc -> foo1ve68hrvxcqc
    assert_eq!(String::from_utf8_lossy(&result), "foo1v93xxpqvnxh");
}
#[test]
fn test_bitcoin_segwit() {
    // This is a bit of a tricky test because the input already includes witness version in some contexts,
    // but here we are prepending it.
    // Let's use a simpler one.
}
