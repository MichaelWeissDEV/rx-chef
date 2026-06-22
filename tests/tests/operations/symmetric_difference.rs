// Tests for the symmetric_difference operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations symmetric_difference::

use rxchef::operation::ArgValue;
use rxchef::operations::symmetric_difference::SymmetricDifference;
use rxchef::Operation;

#[test]
fn test_symmetric_difference_basic() {
    let op = SymmetricDifference;
    let input = b"a,b,c\n\nb,c,d".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "a,d");
}
#[test]
fn test_symmetric_difference_no_overlap() {
    let op = SymmetricDifference;
    let input = b"x,y\n\na,b".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "x,y,a,b");
}
#[test]
fn test_symmetric_difference_identical() {
    let op = SymmetricDifference;
    let input = b"a,b\n\na,b".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "");
}
