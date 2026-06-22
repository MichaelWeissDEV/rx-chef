// Tests for the jsonata operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations jsonata::

use rxchef::operation::ArgValue;
use rxchef::operations::jsonata::Jsonata;
use rxchef::Operation;

#[test]
fn test_jsonata_is_broken() {
    let op = Jsonata;
    #[cfg(not(feature = "jsonata"))]
    assert!(op.is_broken());
    #[cfg(feature = "jsonata")]
    assert!(!op.is_broken());
}
#[test]
fn test_jsonata_without_feature() {
    let op = Jsonata;
    let input = b"{\"a\": 1}".to_vec();
    let args = [ArgValue::Str("a".to_string())];
    let result = op.run(input.clone(), &args);
    #[cfg(not(feature = "jsonata"))]
    assert!(result.is_err());
}
