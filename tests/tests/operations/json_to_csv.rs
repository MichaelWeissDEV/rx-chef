// Tests for the json_to_csv operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations json_to_csv::

use rxchef::operation::ArgValue;
use rxchef::operations::json_to_csv::JSONToCSV;
use rxchef::Operation;

#[test]
fn test_json_to_csv_array_of_arrays() {
    let op = JSONToCSV;
    let input = b"[[\"a\", \"b\"], [\"c\", \"d\"]]".to_vec();
    let args = [
        ArgValue::Str(",".to_string()),
        ArgValue::Str("\\r\\n".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "a,b\r\nc,d\r\n");
}
#[test]
fn test_json_to_csv_array_of_objects() {
    let op = JSONToCSV;
    let input = b"[{\"name\": \"John\", \"age\": 30}, {\"name\": \"Jane\", \"age\": 25}]".to_vec();
    let args = [
        ArgValue::Str(",".to_string()),
        ArgValue::Str("\\n".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    // keys might be in any order, but usually they are sorted in serde_json
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "name,age\nJohn,30\nJane,25\n"
    );
}
#[test]
fn test_json_to_csv_escaping() {
    let op = JSONToCSV;
    let input = b"[[\"a,b\", \"c\\\"d\"]]".to_vec();
    let args = [
        ArgValue::Str(",".to_string()),
        ArgValue::Str("\\n".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "\"a,b\",\"c\"\"d\"\n");
}
