/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Convert area operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ConvertArea;

const UNITS: &[(&str, f64)] = &[
    ("Square metre (sq m)", 1.0),
    ("Square kilometre (sq km)", 1e6),
    ("Centiare (ca)", 1.0),
    ("Deciare (da)", 10.0),
    ("Are (a)", 100.0),
    ("Decare (daa)", 1000.0),
    ("Hectare (ha)", 10000.0),
    ("Square inch (sq in)", 0.00064516),
    ("Square foot (sq ft)", 0.09290304),
    ("Square yard (sq yd)", 0.83612736),
    ("Square mile (sq mi)", 2_589_988.110336),
    ("Perch (sq per)", 25.29285264),
    ("Rood (ro)", 1011.7141056),
    ("International acre (ac)", 4046.8564224),
    ("US survey acre (ac)", 4046.8726099),
    ("US survey square mile (sq mi)", 2_589_998.470319),
    ("US survey township", 93_239_944.930884),
];

fn factor(name: &str) -> Option<f64> {
    UNITS.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

impl Operation for ConvertArea {
    fn name(&self) -> &'static str {
        "Convert area"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Converts a unit of area to another format."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static S: &[ArgSchema] = &[
            ArgSchema {
                name: "Input units",
                description: "Input area unit",
                default_value: "Square metre (sq m)",
            },
            ArgSchema {
                name: "Output units",
                description: "Output area unit",
                default_value: "Square kilometre (sq km)",
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
            .unwrap_or("Square metre (sq m)");
        let to = args
            .get(1)
            .and_then(|a| a.as_str())
            .unwrap_or("Hectare (ha)");
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
