/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse UDP operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse UDP operation
pub struct ParseUDP;

impl Operation for ParseUDP {
    fn name(&self) -> &'static str {
        "Parse UDP"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Parses a UDP header and payload (if present)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Input format",
            description: "Format of the input data",
            default_value: "Hex",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let format = if args.len() > 0 {
            args[0].as_str().unwrap_or("Hex")
        } else {
            "Hex"
        };

        let data = if format == "Hex" {
            let input_str = String::from_utf8_lossy(&input).trim().to_string();
            hex::decode(input_str)
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?
        } else {
            input
        };

        if data.len() < 8 {
            return Err(OperationError::InvalidInput(
                "Need 8 bytes for a UDP Header".to_string(),
            ));
        }

        let mut rdr = Cursor::new(&data);
        let src_port = rdr
            .read_u16::<BigEndian>()
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        let dst_port = rdr
            .read_u16::<BigEndian>()
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        let length = rdr
            .read_u16::<BigEndian>()
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        let checksum_bytes = &data[6..8];
        let checksum = format!("0x{}", hex::encode(checksum_bytes));

        let mut packet = serde_json::json!({
            "Source port": src_port,
            "Destination port": dst_port,
            "Length": length,
            "Checksum": checksum,
        });

        if data.len() > 8 {
            let payload_len = (length as usize).saturating_sub(8);
            let actual_payload_len = std::cmp::min(payload_len, data.len() - 8);
            let payload = &data[8..8 + actual_payload_len];
            packet.as_object_mut().unwrap().insert(
                "Data".to_string(),
                serde_json::Value::String(format!("0x{}", hex::encode(payload))),
            );
        }

        let output = serde_json::to_string_pretty(&packet)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        Ok(output.into_bytes())
    }
}
