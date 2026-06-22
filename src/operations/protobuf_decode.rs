/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Protobuf Decode operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::{Map, Value};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Protobuf Decode operation
pub struct ProtobufDecode;

impl Operation for ProtobufDecode {
    fn name(&self) -> &'static str {
        "Protobuf Decode"
    }

    fn module(&self) -> &'static str {
        "Protobuf"
    }

    fn description(&self) -> &'static str {
        "Decodes any Protobuf encoded data to a JSON representation of the data using the field number as the field key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Schema (.proto text)",
                description: "Optional schema (not implemented in this version)",
                default_value: "",
            },
            ArgSchema {
                name: "Show Unknown Fields",
                description: "Show fields not in schema",
                default_value: "false",
            },
            ArgSchema {
                name: "Show Types",
                description: "Show type information",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(b"{}".to_vec());
        }

        let decoded = decode_protobuf(&input, 0).map_err(|e| OperationError::ProcessingError(e))?;
        serde_json::to_vec_pretty(&decoded)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))
    }
}

fn decode_protobuf(data: &[u8], mut offset: usize) -> Result<Value, String> {
    let mut map = Map::new();

    while offset < data.len() {
        if offset >= data.len() {
            break;
        }

        let tag = read_varint(data, &mut offset)?;
        let wire_type = tag & 0x07;
        let field_number = tag >> 3;

        let value = match wire_type {
            0 => {
                // Varint
                Value::from(read_varint(data, &mut offset)?)
            }
            1 => {
                // 64-bit
                if offset + 8 > data.len() {
                    return Err("Unexpected end of data (64-bit)".to_string());
                }
                let bytes = &data[offset..offset + 8];
                offset += 8;
                Value::from(u64::from_le_bytes(bytes.try_into().unwrap()))
            }
            2 => {
                // Length-delimited
                let len = read_varint(data, &mut offset)? as usize;
                if offset + len > data.len() {
                    return Err("Unexpected end of data (len-delimited)".to_string());
                }
                let sub_data = &data[offset..offset + len];
                offset += len;

                // Try to parse as sub-message
                match decode_protobuf(sub_data, 0) {
                    Ok(sub_val) => sub_val,
                    Err(_) => {
                        // If it fails, treat as string or hex
                        match String::from_utf8(sub_data.to_vec()) {
                            Ok(s) => Value::from(s),
                            Err(_) => Value::from(hex::encode(sub_data)),
                        }
                    }
                }
            }
            5 => {
                // 32-bit
                if offset + 4 > data.len() {
                    return Err("Unexpected end of data (32-bit)".to_string());
                }
                let bytes = &data[offset..offset + 4];
                offset += 4;
                Value::from(u32::from_le_bytes(bytes.try_into().unwrap()))
            }
            _ => return Err(format!("Unknown wire type: {}", wire_type)),
        };

        let key = field_number.to_string();
        if let Some(existing) = map.get_mut(&key) {
            if let Some(arr) = existing.as_array_mut() {
                arr.push(value);
            } else {
                let old = existing.take();
                *existing = Value::Array(vec![old, value]);
            }
        } else {
            map.insert(key, value);
        }
    }

    Ok(Value::Object(map))
}

fn read_varint(data: &[u8], offset: &mut usize) -> Result<u64, String> {
    let mut result = 0u64;
    let mut shift = 0;
    loop {
        if *offset >= data.len() {
            return Err("Unexpected end of data (varint)".to_string());
        }
        let byte = data[*offset];
        *offset += 1;
        result |= ((byte & 0x7F) as u64) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return Err("Varint too long".to_string());
        }
    }
    Ok(result)
}
