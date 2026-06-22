// Tests for the csv_to_json operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations csv_to_json::

use rxchef::operation::ArgValue;
use rxchef::operations::csv_to_json::CsvToJson;
use rxchef::Operation;

#[test]
fn test_csv_to_json_array_of_dicts() {
    let op = CsvToJson;
    let input = b"name,age\nAlice,30\nBob,25".to_vec();
    let args = [
        ArgValue::Str(",".to_string()),
        ArgValue::Str("\n".to_string()),
        ArgValue::Str("Array of dictionaries".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    let json: serde_json::Value = serde_json::from_str(&s).unwrap();
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["name"], "Alice");
    assert_eq!(arr[1]["age"], "25");
}
#[test]
fn test_csv_to_json_array_of_arrays() {
    let op = CsvToJson;
    let input = b"a,b\n1,2".to_vec();
    let args = [
        ArgValue::Str(",".to_string()),
        ArgValue::Str("\n".to_string()),
        ArgValue::Str("Array of arrays".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    let json: serde_json::Value = serde_json::from_str(&s).unwrap();
    let arr = json.as_array().unwrap();
    assert_eq!(arr[0][0], "a");
    assert_eq!(arr[1][1], "2");
}
#[test]
fn test_csv_quoted_fields() {
    let op = CsvToJson;
    let input = b"name,value\n\"hello,world\",42".to_vec();
    let args = [
        ArgValue::Str(",".to_string()),
        ArgValue::Str("\n".to_string()),
        ArgValue::Str("Array of dictionaries".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let s = String::from_utf8(result).unwrap();
    let json: serde_json::Value = serde_json::from_str(&s).unwrap();
    assert_eq!(json[0]["name"], "hello,world");
}
#[test]
fn test_csv_empty_input() {
    let op = CsvToJson;
    let result = op.run(vec![], &[]).unwrap();
    let s = String::from_utf8(result).unwrap();
    assert_eq!(s.trim(), "[]");
}
