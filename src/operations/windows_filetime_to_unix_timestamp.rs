/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Windows Filetime to UNIX Timestamp operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::BigInt;
use num_traits::Num;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Windows Filetime to UNIX Timestamp operation
pub struct WindowsFiletimeToUnixTimestampOp;

impl Operation for WindowsFiletimeToUnixTimestampOp {
    fn name(&self) -> &'static str {
        "Windows Filetime to UNIX Timestamp"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a Windows Filetime value to a UNIX timestamp.<br><br>A Windows Filetime is a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 UTC.<br><br>A UNIX timestamp is a 32-bit value representing the number of seconds since January 1, 1970 UTC (the UNIX epoch).<br><br>This operation also supports UNIX timestamps in milliseconds, microseconds and nanoseconds."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Output units",
                description: "Units for the output timestamp",
                default_value: "Seconds (s)",
            },
            ArgSchema {
                name: "Input format",
                description: "Format of the input filetime",
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
        let mut input_str = String::from_utf8_lossy(&input).trim().to_string();
        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let units = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("Seconds (s)");
        let format = args.get(1).and_then(|a| a.as_str()).unwrap_or("Decimal");

        if format == "Hex (little endian)" {
            // Swap endianness of hex string
            let mut hex_bytes =
                hex::decode(&input_str).map_err(|e| OperationError::InvalidArgument {
                    name: "Input".to_string(),
                    reason: format!("Invalid hex: {}", e),
                })?;
            hex_bytes.reverse();
            input_str = hex::encode(hex_bytes);
        }

        let val = if format.starts_with("Hex") {
            BigInt::from_str_radix(&input_str, 16).map_err(|e| OperationError::InvalidArgument {
                name: "Input".to_string(),
                reason: format!("Invalid hex: {}", e),
            })?
        } else {
            BigInt::from_str_radix(&input_str, 10).map_err(|e| OperationError::InvalidArgument {
                name: "Input".to_string(),
                reason: format!("Invalid decimal: {}", e),
            })?
        };

        let epoch_offset = BigInt::from_str_radix("116444736000000000", 10).unwrap();
        let mut result = val - epoch_offset;

        match units {
            "Seconds (s)" => {
                result /= BigInt::from(10_000_000);
            }
            "Milliseconds (ms)" => {
                result /= BigInt::from(10_000);
            }
            "Microseconds (s)" => {
                result /= BigInt::from(10);
            }
            "Nanoseconds (ns)" => {
                result *= BigInt::from(100);
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Output units".to_string(),
                    reason: "Unrecognised unit".to_string(),
                });
            }
        }

        Ok(result.to_str_radix(10).into_bytes())
    }
}
