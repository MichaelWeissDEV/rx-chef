/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Convert speed operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ConvertSpeed;

const UNITS: &[(&str, f64)] = &[
    ("Metres per second (m/s)", 1.0),
    ("Kilometres per hour (km/h)", 1.0 / 3.6),
    ("Miles per hour (mph)", 0.44704),
    ("Knots (kn)", 0.514444),
    ("Feet per second (ft/s)", 0.3048),
    ("Mach (at standard atmosphere)", 340.3),
    ("Speed of light (c)", 299_792_458.0),
];

fn factor(name: &str) -> Option<f64> {
    UNITS.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

impl Operation for ConvertSpeed {
    fn name(&self) -> &'static str {
        "Convert speed"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Converts a unit of speed to another format."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static S: &[ArgSchema] = &[
            ArgSchema {
                name: "Input units",
                description: "Input speed unit",
                default_value: "Metres per second (m/s)",
            },
            ArgSchema {
                name: "Output units",
                description: "Output speed unit",
                default_value: "Kilometres per hour (km/h)",
            },
        ];
        S
    }
    fn input_type(&self) -> DataType {
        DataType::String
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let s = String::from_utf8_lossy(&input);
        let val: f64 = s.trim().parse().map_err(|_| {
            OperationError::InvalidInput(format!("Cannot parse number: {}", s.trim()))
        })?;
        let from = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("Metres per second (m/s)");
        let to = args
            .get(1)
            .and_then(|a| a.as_str())
            .unwrap_or("Kilometres per hour (km/h)");
        let f_from = factor(from).ok_or_else(|| OperationError::InvalidArgument {
            name: "Input units".into(),
            reason: format!("Unknown unit: {}", from),
        })?;
        let f_to = factor(to).ok_or_else(|| OperationError::InvalidArgument {
            name: "Output units".into(),
            reason: format!("Unknown unit: {}", to),
        })?;
        let result = val * f_from / f_to;
        Ok(format!("{}", result).into_bytes())
    }
}
