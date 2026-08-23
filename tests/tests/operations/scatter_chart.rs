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

#[test]
fn test_scatter_chart_single_point_boundary_has_finite_geometry() {
    let result = ScatterChart
        .run(
            b"1,2".to_vec(),
            &[
                ArgValue::Str("Line feed".into()),
                ArgValue::Str("Comma".into()),
                ArgValue::Bool(false),
                ArgValue::Str("X".into()),
                ArgValue::Str("Y".into()),
                ArgValue::Str("blue".into()),
                ArgValue::Num(5.0),
                ArgValue::Bool(false),
            ],
        )
        .unwrap();
    let svg = String::from_utf8(result).unwrap();
    assert!(
        svg.contains(r#"<circle cx="265" cy="235" r="5" fill="blue""#),
        "{svg}"
    );
    assert!(!svg.contains("NaN"));
}

#[test]
fn test_scatter_chart_rejects_non_numeric_point() {
    let result = ScatterChart.run(
        b"nope,2".to_vec(),
        &[
            ArgValue::Str("Line feed".into()),
            ArgValue::Str("Comma".into()),
            ArgValue::Bool(false),
        ],
    );
    assert!(result.is_err());
}

#[test]
fn test_scatter_chart_empty_input_boundary() {
    assert_eq!(ScatterChart.run(Vec::new(), &[]).unwrap(), Vec::<u8>::new());
}
