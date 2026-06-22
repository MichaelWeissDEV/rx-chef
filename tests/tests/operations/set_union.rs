// Tests for the set_union operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations set_union::

use rxchef::operation::ArgValue;
use rxchef::operations::set_union::SetUnion;
use rxchef::Operation;

#[test]
fn test_set_union_basic() {
    let op = SetUnion;
    let input = b"a,b,c\n\nb,c,d".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "a,b,c,d");
}
#[test]
fn test_set_union_no_overlap() {
    let op = SetUnion;
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
fn test_set_union_error_on_single_set() {
    let op = SetUnion;
    let input = b"a,b,c".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    assert!(op.run(input, &args).is_err());
}
