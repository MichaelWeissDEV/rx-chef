/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Scatter chart operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Scatter chart operation
pub struct ScatterChart;

impl Operation for ScatterChart {
    fn name(&self) -> &'static str {
        "Scatter chart"
    }

    fn module(&self) -> &'static str {
        "Charts"
    }

    fn description(&self) -> &'static str {
        "Plots two-variable data as single points on a graph."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Record delimiter",
                description: "The record delimiter",
                default_value: "Line feed",
            },
            ArgSchema {
                name: "Field delimiter",
                description: "The field delimiter",
                default_value: "Comma",
            },
            ArgSchema {
                name: "Use column headers as labels",
                description: "Use column headers as labels",
                default_value: "true",
            },
            ArgSchema {
                name: "X label",
                description: "X label",
                default_value: "",
            },
            ArgSchema {
                name: "Y label",
                description: "Y label",
                default_value: "",
            },
            ArgSchema {
                name: "Colour",
                description: "The colour of the points",
                default_value: "blue",
            },
            ArgSchema {
                name: "Point radius",
                description: "The radius of the points",
                default_value: "5",
            },
            ArgSchema {
                name: "Use colour from third column",
                description: "Use colour from third column",
                default_value: "false",
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
        let record_delim =
            get_char_rep(args.first().and_then(|v| v.as_str()).unwrap_or("Line feed"));
        let field_delim = get_char_rep(args.get(1).and_then(|v| v.as_str()).unwrap_or("Comma"));
        let use_headers = args.get(2).and_then(|v| v.as_bool()).unwrap_or(true);
        let mut x_label = args
            .get(3)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let mut y_label = args
            .get(4)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let fill_colour = args.get(5).and_then(|v| v.as_str()).unwrap_or("blue");
        let radius = args.get(6).and_then(|v| v.as_f64()).unwrap_or(5.0);
        let colour_in_input = args.get(7).and_then(|v| v.as_bool()).unwrap_or(false);

        let (headings, values) = get_scatter_values(
            &input_str,
            record_delim,
            field_delim,
            use_headers,
            colour_in_input,
        )?;

        if let Some(h) = headings {
            if x_label.is_empty() {
                x_label = h.0;
            }
            if y_label.is_empty() {
                y_label = h.1;
            }
        }

        if values.is_empty() {
            return Ok(Vec::new());
        }

        let mut x_min = values[0].0;
        let mut x_max = values[0].0;
        let mut y_min = values[0].1;
        let mut y_max = values[0].1;

        for &(x, y, _) in &values {
            if x < x_min {
                x_min = x;
            }
            if x > x_max {
                x_max = x;
            }
            if y < y_min {
                y_min = y;
            }
            if y > y_max {
                y_max = y;
            }
        }

        // Add 10% margin to extent
        let x_delta = if x_max == x_min { 1.0 } else { x_max - x_min };
        let y_delta = if y_max == y_min { 1.0 } else { y_max - y_min };

        let x_range_min = x_min - 0.1 * x_delta;
        let x_range_max = x_max + 0.1 * x_delta;
        let y_range_min = y_min - 0.1 * y_delta;
        let y_range_max = y_max + 0.1 * y_delta;

        let dimension = 500.0;
        let margin_left = 50.0;
        let margin_bottom = 50.0;
        let margin_top = 20.0;
        let margin_right = 20.0;
        let width = dimension - margin_left - margin_right;
        let height = dimension - margin_top - margin_bottom;

        let scale_x =
            |x: f64| margin_left + (x - x_range_min) / (x_range_max - x_range_min) * width;
        let scale_y =
            |y: f64| margin_top + height - (y - y_range_min) / (y_range_max - y_range_min) * height;

        let mut svg = format!(
            r#"<svg width="100%" height="100%" viewBox="0 0 {0} {0}" xmlns="http://www.w3.org/2000/svg">"#,
            dimension
        );
        svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);

        // Clip path for points
        svg.push_str(&format!(
            r#"<clipPath id="clip"><rect x="{}" y="{}" width="{}" height="{}"/></clipPath>"#,
            margin_left, margin_top, width, height
        ));

        // Draw points
        svg.push_str(r#"<g clip-path="url(#clip)">"#);
        for &(x, y, ref color) in &values {
            let cx = scale_x(x);
            let cy = scale_y(y);
            let c = if colour_in_input {
                color.as_deref().unwrap_or(fill_colour)
            } else {
                fill_colour
            };
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="rgba(0,0,0,0.5)" stroke-width="0.5"><title>X: {}&#10;Y: {}</title></circle>"#,
                cx, cy, radius, c, x, y
            ));
        }
        svg.push_str("</g>");

        // Axis lines
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="1"/>"#,
            margin_left,
            margin_top + height,
            margin_left + width,
            margin_top + height
        ));
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="1"/>"#,
            margin_left,
            margin_top,
            margin_left,
            margin_top + height
        ));

        // Labels
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" text-anchor="middle" font-family="sans-serif" font-size="14">{}</text>"#,
            margin_left + width / 2.0, dimension - 15.0, x_label
        ));
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" transform="rotate(-90, {}, {})" text-anchor="middle" font-family="sans-serif" font-size="14">{}</text>"#,
            15.0, margin_top + height / 2.0, 15.0, margin_top + height / 2.0, y_label
        ));

        svg.push_str("</svg>");
        Ok(svg.into_bytes())
    }
}

fn get_char_rep(token: &str) -> &str {
    match token {
        "Space" => " ",
        "Comma" => ",",
        "Semi-colon" => ";",
        "Colon" => ":",
        "Tab" => "\t",
        "Line feed" => "\n",
        "CRLF" => "\r\n",
        _ => token,
    }
}

fn get_scatter_values(
    input: &str,
    record_delim: &str,
    field_delim: &str,
    use_headers: bool,
    use_colour: bool,
) -> Result<(Option<(String, String)>, Vec<(f64, f64, Option<String>)>), OperationError> {
    let mut values = Vec::new();
    let mut headings = None;

    for (i, row) in input.split(record_delim).enumerate() {
        if row.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = row.split(field_delim).collect();
        if parts.len() < 2 {
            continue;
        }

        if i == 0 && use_headers {
            headings = Some((parts[0].to_string(), parts[1].to_string()));
        } else {
            let x = parts[0].trim().parse::<f64>().map_err(|_| {
                OperationError::InvalidInput(format!("X value '{}' must be a number", parts[0]))
            })?;
            let y = parts[1].trim().parse::<f64>().map_err(|_| {
                OperationError::InvalidInput(format!("Y value '{}' must be a number", parts[1]))
            })?;
            let color = if use_colour && parts.len() >= 3 {
                Some(parts[2].trim().to_string())
            } else {
                None
            };
            values.push((x, y, color));
        }
    }
    Ok((headings, values))
}
