/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Convert data units operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ConvertDataUnits;

const UNITS: &[(&str, f64)] = &[
    ("Bits (b)", 1.0),
    ("Bytes (B)", 8.0),
    ("Kilobits (kb)", 1e3),
    ("Kilobytes (kB)", 8e3),
    ("Kibibits (Kib)", 1024.0),
    ("Kibibytes (KiB)", 8.0 * 1024.0),
    ("Megabits (Mb)", 1e6),
    ("Megabytes (MB)", 8e6),
    ("Mebibits (Mib)", 1024.0 * 1024.0),
    ("Mebibytes (MiB)", 8.0 * 1024.0 * 1024.0),
    ("Gigabits (Gb)", 1e9),
    ("Gigabytes (GB)", 8e9),
    ("Gibibits (Gib)", 1024.0 * 1024.0 * 1024.0),
    ("Gibibytes (GiB)", 8.0 * 1024.0 * 1024.0 * 1024.0),
    ("Terabits (Tb)", 1e12),
    ("Terabytes (TB)", 8e12),
    ("Tebibits (Tib)", 1024.0 * 1024.0 * 1024.0 * 1024.0),
    ("Tebibytes (TiB)", 8.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0),
    ("Petabits (Pb)", 1e15),
    ("Petabytes (PB)", 8e15),
    ("Exabits (Eb)", 1e18),
    ("Exabytes (EB)", 8e18),
];

fn factor(name: &str) -> Option<f64> {
    UNITS.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

impl Operation for ConvertDataUnits {
    fn name(&self) -> &'static str {
        "Convert data units"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Converts a unit of data to another format (bits, bytes, kilobytes, etc.)."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static S: &[ArgSchema] = &[
            ArgSchema {
                name: "Input units",
                description: "Input data unit",
                default_value: "Bytes (B)",
            },
            ArgSchema {
                name: "Output units",
                description: "Output data unit",
                default_value: "Kilobytes (kB)",
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
        let from = args.first().and_then(|a| a.as_str()).unwrap_or("Bytes (B)");
        let to = args
            .get(1)
            .and_then(|a| a.as_str())
            .unwrap_or("Kilobytes (kB)");
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
