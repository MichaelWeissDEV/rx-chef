// Tests for the set_difference operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations set_difference::

use rxchef::operation::ArgValue;
use rxchef::operations::set_difference::SetDifference;
use rxchef::Operation;

#[test]
fn test_set_difference_basic() {
    let op = SetDifference;
    let input = b"a,b,c\n\nd,b,e".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "a,c");
}
#[test]
fn test_set_difference_no_overlap() {
    let op = SetDifference;
    let input = b"x,y,z\n\na,b,c".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "x,y,z");
}
#[test]
fn test_set_difference_wrong_number_of_sets() {
    let op = SetDifference;
    let input = b"a,b,c".to_vec();
    let args = vec![
        ArgValue::Str("\\n\\n".to_string()),
        ArgValue::Str(",".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
