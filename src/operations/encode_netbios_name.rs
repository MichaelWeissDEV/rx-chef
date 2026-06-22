/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Encode NetBIOS Name operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Encode NetBIOS Name operation
pub struct EncodeNetBIOSName;

impl Operation for EncodeNetBIOSName {
    fn name(&self) -> &'static str {
        "Encode NetBIOS Name"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "NetBIOS names as seen across the client interface to NetBIOS are exactly 16 bytes long. Within the NetBIOS-over-TCP protocols, a longer representation is used.<br><br>There are two levels of encoding. The first level maps a NetBIOS name into a domain system name.  The second level maps the domain system name into the 'compressed' representation required for interaction with the domain name system.<br><br>This operation carries out the first level of encoding. See RFC 1001 for full details."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Offset",
            description: "The offset value used for encoding",
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
        let offset = args.first().and_then(|v| v.as_i64()).unwrap_or(65) as u8;

        if input.len() > 16 {
            return Ok(vec![]);
        }

        let mut padded_input = input.clone();
        padded_input.resize(16, 32);

        let mut output = Vec::with_capacity(32);
        for &byte in &padded_input {
            output.push((byte >> 4) + offset);
            output.push((byte & 0x0f) + offset);
        }

        Ok(output)
    }
}
