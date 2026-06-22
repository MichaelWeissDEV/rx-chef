/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Protobuf Encode operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Protobuf Encode operation
pub struct ProtobufEncode;

impl Operation for ProtobufEncode {
    fn name(&self) -> &'static str {
        "Protobuf Encode"
    }

    fn module(&self) -> &'static str {
        "Protobuf"
    }

    fn description(&self) -> &'static str {
        "Encodes a valid JSON object into a protobuf byte array. Note: This implementation currently only supports encoding based on numeric keys in the JSON input (field numbers) as runtime schema compilation is not supported."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Schema (.proto text)",
            description: "Optional schema (not implemented in this version)",
            default_value: "",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let json_val: Value = serde_json::from_slice(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Failed to parse input as JSON: {}", e))
        })?;

        let mut output = Vec::new();
        encode_protobuf_val(&json_val, &mut output).map_err(OperationError::ProcessingError)?;

        Ok(output)
    }
}

fn encode_protobuf_val(value: &Value, output: &mut Vec<u8>) -> Result<(), String> {
    match value {
        Value::Object(map) => {
            let mut sorted_keys: Vec<_> = map.keys().collect();
            sorted_keys.sort_by_key(|k| k.parse::<u64>().unwrap_or(0));

            for k in sorted_keys {
                let v = &map[k];
                let field_number: u64 = k
                    .parse()
                    .map_err(|_| format!("Keys must be numeric field numbers, got: {}", k))?;
                encode_field(field_number, v, output)?;
            }
        }
        _ => return Err("Input must be a JSON object".to_string()),
    }
    Ok(())
}

fn encode_field(field_number: u64, value: &Value, output: &mut Vec<u8>) -> Result<(), String> {
    match value {
        Value::Null => {}
        Value::Bool(b) => {
            write_tag(field_number, 0, output);
            write_varint(if *b { 1 } else { 0 }, output);
        }
        Value::Number(n) => {
            write_tag(field_number, 0, output);
            if let Some(u) = n.as_u64() {
                write_varint(u, output);
            } else if let Some(i) = n.as_i64() {
                write_varint(i as u64, output);
            } else {
                return Err("Float encoding not supported without schema".to_string());
            }
        }
        Value::String(s) => {
            write_tag(field_number, 2, output);
            let bytes = s.as_bytes();
            write_varint(bytes.len() as u64, output);
            output.extend_from_slice(bytes);
        }
        Value::Array(arr) => {
            for item in arr {
                encode_field(field_number, item, output)?;
            }
        }
        Value::Object(_) => {
            write_tag(field_number, 2, output);
            let mut sub_output = Vec::new();
            encode_protobuf_val(value, &mut sub_output)?;
            write_varint(sub_output.len() as u64, output);
            output.extend_from_slice(&sub_output);
        }
    }
    Ok(())
}

fn write_tag(field_number: u64, wire_type: u8, output: &mut Vec<u8>) {
    let tag = (field_number << 3) | (wire_type as u64);
    write_varint(tag, output);
}

fn write_varint(mut value: u64, output: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output.push(byte);
        if value == 0 {
            break;
        }
    }
}
