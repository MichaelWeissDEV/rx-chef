/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Heatmap chart operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Heatmap chart operation
pub struct HeatmapChart;

impl Operation for HeatmapChart {
    fn name(&self) -> &'static str {
        "Heatmap chart"
    }

    fn module(&self) -> &'static str {
        "Charts"
    }

    fn description(&self) -> &'static str {
        "A heatmap is a graphical representation of data where the individual values contained in a matrix are represented as colors."
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
                name: "Number of vertical bins",
                description: "Number of vertical bins",
                default_value: "25",
            },
            ArgSchema {
                name: "Number of horizontal bins",
                description: "Number of horizontal bins",
                default_value: "25",
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
                name: "Draw bin edges",
                description: "Draw bin edges",
                default_value: "false",
            },
            ArgSchema {
                name: "Min colour value",
                description: "Min colour value",
                default_value: "white",
            },
            ArgSchema {
                name: "Max colour value",
                description: "Max colour value",
                default_value: "black",
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
        let v_bins = args.get(2).and_then(|v| v.as_usize()).unwrap_or(25);
        let h_bins = args.get(3).and_then(|v| v.as_usize()).unwrap_or(25);
        let use_headers = args.get(4).and_then(|v| v.as_bool()).unwrap_or(true);
        let mut x_label = args
            .get(5)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let mut y_label = args
            .get(6)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let draw_edges = args.get(7).and_then(|v| v.as_bool()).unwrap_or(false);
        let min_colour = args.get(8).and_then(|v| v.as_str()).unwrap_or("white");
        let max_colour = args.get(9).and_then(|v| v.as_str()).unwrap_or("black");

        if v_bins == 0 || h_bins == 0 {
            return Err(OperationError::InvalidArgument {
                name: "Bins".to_string(),
                reason: "Number of bins must be greater than 0".to_string(),
            });
        }

        let (headings, values) =
            get_scatter_values(&input_str, record_delim, field_delim, use_headers)?;

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

        for &(x, y) in &values {
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

        if x_min == x_max {
            x_max += 1.0;
        }
        if y_min == y_max {
            y_max += 1.0;
        }

        let mut bins = vec![vec![0; h_bins]; v_bins];
        let epsilon = 0.000000001;

        for &(x, y) in &values {
            let fx = (x - x_min) / ((x_max + epsilon) - x_min);
            let fy = (y - y_min) / ((y_max + epsilon) - y_min);
            let ix = (fx * h_bins as f64).floor() as usize;
            let iy = (fy * v_bins as f64).floor() as usize;
            if ix < h_bins && iy < v_bins {
                bins[iy][ix] += 1;
            }
        }

        let mut max_count = 0;
        for row in &bins {
            for &count in row {
                if count > max_count {
                    max_count = count;
                }
            }
        }

        let dimension = 500.0;
        let margin_left = 50.0;
        let margin_bottom = 50.0;
        let margin_top = 20.0;
        let margin_right = 20.0;
        let width = dimension - margin_left - margin_right;
        let height = dimension - margin_top - margin_bottom;
        let bin_width = width / h_bins as f64;
        let bin_height = height / v_bins as f64;

        let mut svg = format!(
            r#"<svg width="100%" height="100%" viewBox="0 0 {0} {0}" xmlns="http://www.w3.org/2000/svg">"#,
            dimension
        );
        svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);

        // Draw bins
        for (iy, row) in bins.iter().enumerate() {
            for (ix, &count) in row.iter().enumerate() {
                if count == 0 && !draw_edges {
                    continue;
                }
                let ratio = if max_count > 0 {
                    count as f64 / max_count as f64
                } else {
                    0.0
                };
                let color = interpolate_color(min_colour, max_colour, ratio);
                let x = margin_left + ix as f64 * bin_width;
                let y = margin_top + (v_bins - 1 - iy) as f64 * bin_height;
                let stroke = if draw_edges {
                    r#"stroke="rgba(0,0,0,0.5)" stroke-width="0.5""#
                } else {
                    r#"stroke="none""#
                };
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" {}><title>Count: {}</title></rect>"#,
                    x, y, bin_width, bin_height, color, stroke, count
                ));
            }
        }

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
) -> Result<(Option<(String, String)>, Vec<(f64, f64)>), OperationError> {
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
            values.push((x, y));
        }
    }
    Ok((headings, values))
}

fn interpolate_color(min: &str, max: &str, ratio: f64) -> String {
    let c1 = parse_color(min);
    let c2 = parse_color(max);
    let r = (c1.0 as f64 + (c2.0 as f64 - c1.0 as f64) * ratio) as u8;
    let g = (c1.1 as f64 + (c2.1 as f64 - c1.1 as f64) * ratio) as u8;
    let b = (c1.2 as f64 + (c2.2 as f64 - c1.2 as f64) * ratio) as u8;
    format!("rgb({}, {}, {})", r, g, b)
}

fn parse_color(c: &str) -> (u8, u8, u8) {
    match c.to_lowercase().as_str() {
        "white" => (255, 255, 255),
        "black" => (0, 0, 0),
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "blue" => (0, 0, 255),
        _ => {
            if c.starts_with('#') {
                if c.len() == 7 {
                    let r = u8::from_str_radix(&c[1..3], 16).unwrap_or(128);
                    let g = u8::from_str_radix(&c[3..5], 16).unwrap_or(128);
                    let b = u8::from_str_radix(&c[5..7], 16).unwrap_or(128);
                    return (r, g, b);
                }
            }
            (128, 128, 128)
        }
    }
}
