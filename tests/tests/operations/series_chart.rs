// Tests for the series_chart operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations series_chart::

use rxchef::operation::ArgValue;
use rxchef::operations::series_chart::SeriesChart;
use rxchef::Operation;

struct Series {
    name: String,
    data: Vec<Option<f64>>,
}

fn parse_series(input: &str, record_sep: &str, field_sep: &str) -> (Vec<String>, Vec<Series>) {
    let lines: Vec<&str> = input
        .split(record_sep)
        .filter(|s| !s.trim().is_empty())
        .collect();
    if lines.is_empty() {
        return (vec![], vec![]);
    }

    let header: Vec<String> = lines[0]
        .split(field_sep)
        .map(|s| s.trim().to_string())
        .collect();
    if header.len() < 2 {
        return (vec![], vec![]);
    }

    let mut x_values = Vec::new();
    let mut series_data: Vec<Vec<Option<f64>>> = vec![vec![]; header.len() - 1];

    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(field_sep).map(|s| s.trim()).collect();
        if fields.is_empty() {
            continue;
        }

        x_values.push(fields[0].to_string());
        for i in 1..header.len() {
            let val = fields.get(i).and_then(|&s| s.parse::<f64>().ok());
            series_data[i - 1].push(val);
        }
    }

    let series = header[1..]
        .iter()
        .enumerate()
        .map(|(i, name)| Series {
            name: name.clone(),
            data: series_data[i].clone(),
        })
        .collect();

    (x_values, series)
}

#[test]
fn test_series_chart_basic() {
    let op = SeriesChart;
    let input = b"X,Series1,Series2\n1,10,20\n2,15,25\n3,7,18".to_vec();
    let args = [
        ArgValue::Str("\\n".to_string()),
        ArgValue::Str(",".to_string()),
        ArgValue::Str("Test Chart".to_string()),
        ArgValue::Num(2.0),
        ArgValue::Str("red,blue".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    let svg = String::from_utf8(result).unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Test Chart"));
    assert!(svg.contains("Series1"));
    assert!(svg.contains("Series2"));
    assert!(svg.contains("red"));
    assert!(svg.contains("blue"));
}
#[test]
fn test_series_chart_empty_input() {
    let op = SeriesChart;
    let input = b"".to_vec();
    let args = [
        ArgValue::Str("\\n".to_string()),
        ArgValue::Str(",".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Num(1.0),
        ArgValue::Str("".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, b"<div>No data to display</div>");
}
#[test]
fn test_parse_series() {
    let input = "X,A,B\n1,10,20\n2,30,40";
    let (x, s) = parse_series(input, "\n", ",");
    assert_eq!(x, vec!["1", "2"]);
    assert_eq!(s.len(), 2);
    assert_eq!(s[0].name, "A");
    assert_eq!(s[0].data, vec![Some(10.0), Some(30.0)]);
}
