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
        let _input_str = String::from_utf8_lossy(&input);

        // Simplified implementation: producing a placeholder SVG
        let x_label = args.get(5).and_then(|a| a.as_str()).unwrap_or("X");
        let y_label = args.get(6).and_then(|a| a.as_str()).unwrap_or("Y");

        let svg = format!(
            r##"<div style="width: 100%; height: 100%;">
                <svg viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg">
                    <rect width="100%" height="100%" fill="#f0f0f0" />
                    <text x="250" y="480" text-anchor="middle">{}</text>
                    <text x="20" y="250" text-anchor="middle" transform="rotate(-90 20,250)">{}</text>
                    <text x="250" y="250" text-anchor="middle" font-size="20">[Hex Density Chart Placeholder]</text>
                    <text x="250" y="280" text-anchor="middle">Input length: {} bytes</text>
                </svg>
            </div>"##,
            x_label,
            y_label,
            input.len()
        );

        Ok(svg.into_bytes())
    }
}
