/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Decode NetBIOS Name operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Decode NetBIOS Name operation
pub struct DecodeNetBIOSName;

impl Operation for DecodeNetBIOSName {
    fn name(&self) -> &'static str {
        "Decode NetBIOS Name"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "NetBIOS names as seen across the client interface to NetBIOS are exactly 16 bytes long. Within the NetBIOS-over-TCP protocols, a longer representation is used.<br><br>There are two levels of encoding. The first level maps a NetBIOS name into a domain system name.  The second level maps the domain system name into the 'compressed' representation required for interaction with the domain name system.<br><br>This operation decodes the first level of encoding. See RFC 1001 for full details."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Offset",
            description: "The offset to use for decoding",
            default_value: "65",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let offset = args.first().and_then(|v| v.as_i64()).ok_or_else(|| {
            OperationError::InvalidArgument {
                name: "Offset".to_string(),
                reason: "Offset must be a number".to_string(),
            }
        })? as u8;

        let mut output = Vec::new();

        if input.len() <= 32 && (input.len() % 2) == 0 {
            for i in (0..input.len()).step_by(2) {
                let high = input[i].wrapping_sub(offset);
                let low = input[i + 1].wrapping_sub(offset);
                output.push((high << 4) | (low & 0xf));
            }

            // Remove trailing spaces
            while let Some(&last) = output.last() {
                if last == 32 {
                    output.pop();
                } else {
                    break;
                }
            }
        }

        Ok(output)
    }
}
