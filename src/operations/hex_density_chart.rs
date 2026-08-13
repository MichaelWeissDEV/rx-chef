/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Hex Density chart operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};
use std::collections::BTreeMap;

/// Hex Density Chart operation
pub struct HexDensityChartOp;

impl Operation for HexDensityChartOp {
    fn name(&self) -> &'static str {
        "Hex Density chart"
    }

    fn module(&self) -> &'static str {
        "Charts"
    }

    fn description(&self) -> &'static str {
        "Hex density charts are used in a similar way to scatter charts, however rather than rendering tens of thousands of points, it groups the points into a few hundred hexagons to show the distribution."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Record delimiter",
                description: "Delimiter between records",
                default_value: "\\n",
            },
            ArgSchema {
                name: "Field delimiter",
                description: "Delimiter between fields",
                default_value: ",",
            },
            ArgSchema {
                name: "Pack radius",
                description: "Radius of the hexagons",
                default_value: "25",
            },
            ArgSchema {
                name: "Draw radius",
                description: "Radius of the hexagons to draw",
                default_value: "15",
            },
            ArgSchema {
                name: "Use column headers as labels",
                description: "Whether to use the first row as headers",
                default_value: "true",
            },
            ArgSchema {
                name: "X label",
                description: "Label for the X axis",
                default_value: "",
            },
            ArgSchema {
                name: "Y label",
                description: "Label for the Y axis",
                default_value: "",
            },
            ArgSchema {
                name: "Draw hexagon edges",
                description: "Whether to draw edges around hexagons",
                default_value: "false",
            },
            ArgSchema {
                name: "Min colour value",
                description: "Colour for low density",
                default_value: "#ffffff",
            },
            ArgSchema {
                name: "Max colour value",
                description: "Colour for high density",
                default_value: "#000000",
            },
            ArgSchema {
                name: "Draw empty hexagons within data boundaries",
                description: "Whether to draw empty hexagons",
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
        let record_delimiter =
            decode_delimiter(args.first().and_then(ArgValue::as_str).unwrap_or("\\n"));
        let field_delimiter =
            decode_delimiter(args.get(1).and_then(ArgValue::as_str).unwrap_or(","));
        if record_delimiter.is_empty() || field_delimiter.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Delimiter".into(),
                reason: "Delimiters must not be empty".into(),
            });
        }
        let pack_radius = args.get(2).and_then(ArgValue::as_f64).unwrap_or(25.0);
        let draw_radius = args.get(3).and_then(ArgValue::as_f64).unwrap_or(15.0);
        if pack_radius <= 0.0 || draw_radius <= 0.0 {
            return Err(OperationError::InvalidArgument {
                name: "Radius".into(),
                reason: "Radii must be positive".into(),
            });
        }
        let has_headers = args.get(4).and_then(ArgValue::as_bool).unwrap_or(true);
        let text = String::from_utf8(input)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;
        let rows: Vec<Vec<&str>> = text
            .split(&record_delimiter)
            .filter(|row| !row.trim().is_empty())
            .map(|row| row.split(&field_delimiter).map(str::trim).collect())
            .collect();
        let first = rows
            .first()
            .filter(|columns| columns.len() >= 2)
            .ok_or_else(|| {
                OperationError::InvalidInput("At least two columns are required".into())
            })?;
        let x_label = label(args.get(5), if has_headers { first[0] } else { "X" });
        let y_label = label(args.get(6), if has_headers { first[1] } else { "Y" });
        let draw_edges = args.get(7).and_then(ArgValue::as_bool).unwrap_or(false);
        let min_colour = parse_colour(args.get(8), "#ffffff", "Min colour value")?;
        let max_colour = parse_colour(args.get(9), "#000000", "Max colour value")?;
        let points = rows
            .iter()
            .skip(usize::from(has_headers))
            .map(|columns| parse_point(columns))
            .collect::<Result<Vec<_>, _>>()?;
        if points.is_empty() {
            return Err(OperationError::InvalidInput(
                "No numeric data rows were found".into(),
            ));
        }
        Ok(render_svg(
            &points,
            pack_radius,
            draw_radius,
            draw_edges,
            min_colour,
            max_colour,
            x_label,
            y_label,
        )
        .into_bytes())
    }
}

fn decode_delimiter(value: &str) -> String {
    value
        .replace("\\r", "\r")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
}

fn label<'a>(arg: Option<&'a ArgValue>, fallback: &'a str) -> &'a str {
    arg.and_then(ArgValue::as_str)
        .filter(|value| !value.is_empty())
        .unwrap_or(fallback)
}

fn parse_point(columns: &[&str]) -> Result<(f64, f64), OperationError> {
    if columns.len() < 2 {
        return Err(OperationError::InvalidInput(
            "A data row has fewer than two columns".into(),
        ));
    }
    let parse = |value: &str, axis: &str| {
        value.parse::<f64>().map_err(|_| {
            OperationError::InvalidInput(format!("Invalid {axis} coordinate: {value}"))
        })
    };
    let point = (parse(columns[0], "X")?, parse(columns[1], "Y")?);
    if point.0.is_finite() && point.1.is_finite() {
        Ok(point)
    } else {
        Err(OperationError::InvalidInput(
            "Coordinates must be finite".into(),
        ))
    }
}

fn parse_colour(
    arg: Option<&ArgValue>,
    fallback: &str,
    name: &str,
) -> Result<(u8, u8, u8), OperationError> {
    let value = arg.and_then(ArgValue::as_str).unwrap_or(fallback);
    let hex = value.strip_prefix('#').unwrap_or(value);
    if hex.len() != 6 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(OperationError::InvalidArgument {
            name: name.into(),
            reason: "Expected a six-digit hexadecimal colour".into(),
        });
    }
    Ok((
        u8::from_str_radix(&hex[0..2], 16).unwrap(),
        u8::from_str_radix(&hex[2..4], 16).unwrap(),
        u8::from_str_radix(&hex[4..6], 16).unwrap(),
    ))
}

#[allow(clippy::too_many_arguments)]
fn render_svg(
    points: &[(f64, f64)],
    pack_radius: f64,
    draw_radius: f64,
    draw_edges: bool,
    min_colour: (u8, u8, u8),
    max_colour: (u8, u8, u8),
    x_label: &str,
    y_label: &str,
) -> String {
    let (min_x, max_x) = extent(points.iter().map(|point| point.0));
    let (min_y, max_y) = extent(points.iter().map(|point| point.1));
    let scale = |value: f64, min: f64, max: f64, length: f64| {
        if (max - min).abs() < f64::EPSILON {
            length / 2.0
        } else {
            (value - min) / (max - min) * length
        }
    };
    let x_step = pack_radius * 1.5;
    let y_step = pack_radius * 3.0_f64.sqrt();
    let mut bins = BTreeMap::<(i32, i32), usize>::new();
    for &(x, y) in points {
        let row = (scale(y, min_y, max_y, 400.0) / y_step).round() as i32;
        let offset = f64::from(row.rem_euclid(2)) * x_step / 2.0;
        let column = ((scale(x, min_x, max_x, 680.0) - offset) / x_step).round() as i32;
        *bins.entry((column, row)).or_default() += 1;
    }
    let peak = bins.values().copied().max().unwrap_or(1) as f64;
    let stroke = if draw_edges { "#555" } else { "none" };
    let mut svg = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 800 500\" role=\"img\"><rect width=\"800\" height=\"500\" fill=\"white\"/><path d=\"M70 25V435H770\" fill=\"none\" stroke=\"#333\"/>".to_string();
    for ((column, row), count) in bins {
        let cx = 70.0 + column as f64 * x_step + f64::from(row.rem_euclid(2)) * x_step / 2.0;
        let cy = 425.0 - row as f64 * y_step;
        let colour = interpolate(min_colour, max_colour, count as f64 / peak);
        let vertices = (0..6)
            .map(|index| {
                let angle = std::f64::consts::PI / 3.0 * index as f64;
                format!(
                    "{:.2},{:.2}",
                    cx + draw_radius * angle.cos(),
                    cy + draw_radius * angle.sin()
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        svg.push_str(&format!("<polygon points=\"{vertices}\" fill=\"{colour}\" stroke=\"{stroke}\"><title>{count} point(s)</title></polygon>"));
    }
    svg.push_str(&format!(
        "<text x=\"420\" y=\"485\" text-anchor=\"middle\">{}</text><text x=\"18\" y=\"230\" text-anchor=\"middle\" transform=\"rotate(-90 18 230)\">{}</text></svg>",
        escape_xml(x_label), escape_xml(y_label)
    ));
    svg
}

fn extent(values: impl Iterator<Item = f64>) -> (f64, f64) {
    values.fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), value| {
        (min.min(value), max.max(value))
    })
}

fn interpolate(start: (u8, u8, u8), end: (u8, u8, u8), ratio: f64) -> String {
    let channel = |a: u8, b: u8| (a as f64 + (b as f64 - a as f64) * ratio).round() as u8;
    format!(
        "#{:02x}{:02x}{:02x}",
        channel(start.0, end.0),
        channel(start.1, end.1),
        channel(start.2, end.2)
    )
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
