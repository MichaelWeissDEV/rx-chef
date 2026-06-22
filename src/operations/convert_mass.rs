/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Convert mass operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ConvertMass;

const UNITS: &[(&str, f64)] = &[
    ("Nanograms (ng)", 1e-12),
    ("Micrograms (ug)", 1e-9),
    ("Milligrams (mg)", 1e-6),
    ("Grams (g)", 1e-3),
    ("Kilograms (kg)", 1.0),
    ("Tonnes (t)", 1e3),
    ("Ounces (oz)", 0.028349523125),
    ("Pounds (lb)", 0.45359237),
    ("Stone (st)", 6.35029318),
    ("US ton (ton)", 907.18474),
    ("Imperial ton (ton)", 1016.0469088),
];

fn factor(name: &str) -> Option<f64> {
    UNITS.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

impl Operation for ConvertMass {
    fn name(&self) -> &'static str {
        "Convert mass"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Converts a unit of mass to another format."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static S: &[ArgSchema] = &[
            ArgSchema {
                name: "Input units",
                description: "Input mass unit",
                default_value: "Kilograms (kg)",
            },
            ArgSchema {
                name: "Output units",
                description: "Output mass unit",
                default_value: "Grams (g)",
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
            .unwrap_or("Kilograms (kg)");
        let to = args.get(1).and_then(|a| a.as_str()).unwrap_or("Grams (g)");
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
