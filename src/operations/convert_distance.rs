/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Convert distance operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ConvertDistance;

const UNITS: &[(&str, f64)] = &[
    ("Nanometres (nm)", 1e-9),
    ("Micrometres (um)", 1e-6),
    ("Millimetres (mm)", 1e-3),
    ("Centimetres (cm)", 1e-2),
    ("Metres (m)", 1.0),
    ("Kilometers (km)", 1e3),
    ("Thou (th)", 0.0000254),
    ("Inches (in)", 0.0254),
    ("Feet (ft)", 0.3048),
    ("Yards (yd)", 0.9144),
    ("Chains (ch)", 20.1168),
    ("Furlongs (fur)", 201.168),
    ("Miles (mi)", 1609.344),
    ("Leagues (lea)", 4828.032),
    ("Fathoms (ftm)", 1.853184),
    ("Cables", 185.3184),
    ("Nautical miles", 1853.184),
    ("Cars (4m)", 4.0),
    ("Buses (8.4m)", 8.4),
    ("American football fields (91m)", 91.0),
    ("Football pitches (105m)", 105.0),
    ("Earth-to-Moons", 380_000_000.0),
    ("Astronomical units (au)", 149_597_870_700.0),
    ("Light-years (ly)", 9_460_730_472_580_800.0),
    ("Parsecs (pc)", 3.0856776e16),
];

fn factor(name: &str) -> Option<f64> {
    UNITS.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

impl Operation for ConvertDistance {
    fn name(&self) -> &'static str {
        "Convert distance"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Converts a unit of distance to another format."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static S: &[ArgSchema] = &[
            ArgSchema {
                name: "Input units",
                description: "Input distance unit",
                default_value: "Metres (m)",
            },
            ArgSchema {
                name: "Output units",
                description: "Output distance unit",
                default_value: "Kilometres (km)",
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
            .first()
            .and_then(|a| a.as_str())
            .unwrap_or("Metres (m)");
        let to = args
            .get(1)
            .and_then(|a| a.as_str())
            .unwrap_or("Kilometers (km)");
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
