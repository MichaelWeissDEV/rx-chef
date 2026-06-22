/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Series chart operation.
 * -----------------------------------------------------------------------------
 */

use html_escape::encode_safe;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Series chart operation
pub struct SeriesChart;

impl Operation for SeriesChart {
    fn name(&self) -> &'static str {
        "Series chart"
    }

    fn module(&self) -> &'static str {
        "Charts"
    }

    fn description(&self) -> &'static str {
        "A time series graph is a line graph of repeated measurements taken over regular time intervals."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Record delimiter",
                description: "Character(s) that separate records",
                default_value: "\\n",
            },
            ArgSchema {
                name: "Field delimiter",
                description: "Character(s) that separate fields",
                default_value: ",",
            },
            ArgSchema {
                name: "X label",
                description: "Label for the X axis",
                default_value: "",
            },
            ArgSchema {
                name: "Point radius",
                description: "Radius of points in the graph",
                default_value: "1",
            },
            ArgSchema {
                name: "Series colours",
                description: "Comma-separated list of colours for each series",
                default_value: "mediumseagreen, dodgerblue, tomato",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Html
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let record_delimiter =
            unescape_char(args.first().and_then(|a| a.as_str()).unwrap_or("\\n"));
        let field_delimiter = unescape_char(args.get(1).and_then(|a| a.as_str()).unwrap_or(","));
        let x_label = args.get(2).and_then(|a| a.as_str()).unwrap_or("");
        let pip_radius = args.get(3).and_then(|a| a.as_f64()).unwrap_or(1.0);
        let series_colours_str = args
            .get(4)
            .and_then(|a| a.as_str())
            .unwrap_or("mediumseagreen, dodgerblue, tomato");
        let series_colours: Vec<&str> = series_colours_str.split(',').map(|s| s.trim()).collect();

        let (x_values, series) = parse_series(&input_str, &record_delimiter, &field_delimiter);

        if x_values.is_empty() || series.is_empty() {
            return Ok(b"<div>No data to display</div>".to_vec());
        }

        let svg_width = 500.0;
        let inter_series_padding = 20.0;
        let x_axis_height = 50.0;
        let series_label_width = 50.0;
        let series_height = 100.0;
        let series_width = svg_width - series_label_width - inter_series_padding;

        let all_series_height = series.len() as f64 * (inter_series_padding + series_height);
        let svg_height = all_series_height + x_axis_height + inter_series_padding;

        let mut svg = format!(
            r#"<svg width="100%" height="100%" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            svg_width, svg_height
        );

        // X Axis label
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" style="text-anchor: middle; font-family: sans-serif; font-size: 12px;">{}</text>"#,
            svg_width / 2.0,
            x_axis_height / 2.0,
            encode_safe(x_label)
        ));

        // Draw each series
        for (i, serie) in series.iter().enumerate() {
            let color = series_colours[i % series_colours.len()];
            let y_offset = x_axis_height
                + (i as f64 * (series_height + inter_series_padding))
                + inter_series_padding;

            // Series Label
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" style="text-anchor: middle; font-family: sans-serif; font-size: 10px;" transform="rotate(-90, {}, {})">{}</text>"#,
                10.0,
                y_offset + series_height / 2.0,
                10.0,
                y_offset + series_height / 2.0,
                encode_safe(&serie.name)
            ));

            // Calculate scales
            let min_y = serie
                .data
                .iter()
                .filter_map(|&y| y)
                .fold(f64::INFINITY, f64::min);
            let max_y = serie
                .data
                .iter()
                .filter_map(|&y| y)
                .fold(f64::NEG_INFINITY, f64::max);
            let y_range = if max_y == min_y { 1.0 } else { max_y - min_y };

            let mut path = String::new();
            for (j, &val) in serie.data.iter().enumerate() {
                if let Some(y_val) = val {
                    let x = series_label_width
                        + (j as f64 * (series_width / (x_values.len() - 1) as f64));
                    let y = y_offset + series_height - ((y_val - min_y) / y_range * series_height);

                    if path.is_empty() {
                        path.push_str(&format!("M {} {}", x, y));
                    } else {
                        path.push_str(&format!(" L {} {}", x, y));
                    }

                    // Draw point
                    svg.push_str(&format!(
                        r#"<circle cx="{}" cy="{}" r="{}" fill="{}" />"#,
                        x, y, pip_radius, color
                    ));
                }
            }

            svg.push_str(&format!(
                r#"<path d="{}" fill="none" stroke="{}" stroke-width="1" />"#,
                path, color
            ));
        }

        svg.push_str("</svg>");
        Ok(svg.into_bytes())
    }
}

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

fn unescape_char(s: &str) -> String {
    s.replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
}
