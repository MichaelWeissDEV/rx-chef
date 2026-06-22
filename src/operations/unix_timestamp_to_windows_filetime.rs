/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the UNIX Timestamp to Windows Filetime operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::BigInt;
use num_traits::Num;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// UNIX Timestamp to Windows Filetime operation
pub struct UNIXTimestampToWindowsFiletime;

impl Operation for UNIXTimestampToWindowsFiletime {
    fn name(&self) -> &'static str {
        "UNIX Timestamp to Windows Filetime"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a UNIX timestamp to a Windows Filetime value.<br><br>A Windows Filetime is a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 UTC.<br><br>A UNIX timestamp is a 32-bit value representing the number of seconds since January 1, 1970 UTC (the UNIX epoch).<br><br>This operation also supports UNIX timestamps in milliseconds, microseconds and nanoseconds."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input units",
                description: "Input units",
                default_value: "Seconds (s)",
            },
            ArgSchema {
                name: "Output format",
                description: "Output format",
                default_value: "Decimal",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input).trim().to_string();
        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let units = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("Seconds (s)");
        let format = args.get(1).and_then(|a| a.as_str()).unwrap_or("Decimal");

        // Use BigInt to handle large numbers and precision
        let mut val = BigInt::from_str_radix(&input_str, 10)
            .map_err(|_| OperationError::InvalidInput("Invalid numeric input".to_string()))?;

        match units {
            "Seconds (s)" => {
                val *= 10_000_000i64;
            }
            "Milliseconds (ms)" => {
                val *= 10_000i64;
            }
            "Microseconds (s)" => {
                val *= 10i64;
            }
            "Nanoseconds (ns)" => {
                val /= 100i64;
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Input units".to_string(),
                    reason: "Unrecognised unit".to_string(),
                })
            }
        }

        val += BigInt::from_str_radix("116444736000000000", 10).unwrap();

        let mut result = match format {
            "Decimal" => val.to_str_radix(10),
            "Hex (big endian)" | "Hex (little endian)" => val.to_str_radix(16),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Output format".to_string(),
                    reason: "Unrecognised format".to_string(),
                })
            }
        };

        if format == "Hex (little endian)" {
            // Ensure even length for byte swapping
            if result.len() % 2 != 0 {
                result.insert(0, '0');
            }
            let mut flipped = String::with_capacity(result.len());
            for i in (0..result.len()).step_by(2).rev() {
                flipped.push_str(&result[i..i + 2]);
            }
            result = flipped;
        }

        Ok(result.into_bytes())
    }
}
