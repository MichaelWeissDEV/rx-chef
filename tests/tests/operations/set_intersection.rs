// Tests for the set_intersection operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations set_intersection::

use rxchef::operation::ArgValue;
use rxchef::operations::set_intersection::SetIntersection;
use rxchef::Operation;

#[test]
fn test_set_intersection_basic() {
    let op = SetIntersection;
    let input = b"a,b,c\n\nb,c,d".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "b,c");
}
#[test]
fn test_set_intersection_no_overlap() {
    let op = SetIntersection;
    let input = b"x,y\n\na,b".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "");
}
#[test]
fn test_set_intersection_error_on_single_set() {
    let op = SetIntersection;
    let input = b"a,b,c".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    assert!(op.run(input, &args).is_err());
}
