// Tests for the scatter_chart operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations scatter_chart::

use rxchef::operation::ArgValue;
use rxchef::operations::scatter_chart::ScatterChart;
use rxchef::Operation;

#[test]
fn test_scatter_chart_basic() {
    let op = ScatterChart;
    let input = b"1,2\n3,4\n5,6".to_vec();
    let args = [
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Str("Comma".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("X".to_string()),
        ArgValue::Str("Y".to_string()),
        ArgValue::Str("blue".to_string()),
        ArgValue::Num(5.0),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let svg = String::from_utf8(result).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("<circle"));
    assert!(svg.contains("blue"));
}
#[test]
fn test_scatter_chart_with_headers() {
    let op = ScatterChart;
    let input = b"Time,Value\n1,10\n2,20".to_vec();
    let args = [
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Str("Comma".to_string()),
        ArgValue::Bool(true),
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("red".to_string()),
        ArgValue::Num(5.0),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let svg = String::from_utf8(result).unwrap();
    assert!(svg.contains("Time"));
    assert!(svg.contains("Value"));
    assert!(svg.contains("red"));
}
#[test]
fn test_scatter_chart_with_colours() {
    let op = ScatterChart;
    let input = b"1,10,red\n2,20,green\n3,30,blue".to_vec();
    let args = [
        ArgValue::Str("Line feed".to_string()),
        ArgValue::Str("Comma".to_string()),
        ArgValue::Bool(false),
        ArgValue::Str("X".to_string()),
        ArgValue::Str("Y".to_string()),
        ArgValue::Str("black".to_string()),
        ArgValue::Num(5.0),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    let svg = String::from_utf8(result).unwrap();
    assert!(svg.contains("fill=\"red\""));
    assert!(svg.contains("fill=\"green\""));
    assert!(svg.contains("fill=\"blue\""));
}
