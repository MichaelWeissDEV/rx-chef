/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Avro to JSON operation.
 * -----------------------------------------------------------------------------
 */

use apache_avro::Reader;
use serde_json::{json, Value};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Avro to JSON operation
pub struct AvroToJSON;

impl Operation for AvroToJSON {
    fn name(&self) -> &'static str {
        "Avro to JSON"
    }

    fn module(&self) -> &'static str {
        "Serialise"
    }

    fn description(&self) -> &'static str {
        "Converts Avro encoded data into JSON."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Force Valid JSON",
            description: "Wrap multiple records in an array to ensure valid JSON output",
            default_value: "true",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Err(OperationError::InvalidInput(
                "Please provide an input.".to_string(),
            ));
        }

        let force_json = if let Some(arg) = args.first() {
            arg.as_bool().unwrap_or(true)
        } else {
            true
        };

        let reader = Reader::new(&input[..])
            .map_err(|e| OperationError::InvalidInput(format!("Error parsing Avro file: {}", e)))?;

        let mut records = Vec::new();
        for value in reader {
            let avro_value = value.map_err(|e| {
                OperationError::InvalidInput(format!("Error reading Avro record: {}", e))
            })?;

            let json_value = avro_to_json(avro_value)?;
            records.push(json_value);
        }

        let output = if force_json {
            if records.len() == 1 {
                serde_json::to_string_pretty(&records[0]).map_err(|e| {
                    OperationError::InvalidInput(format!("Error serializing JSON: {}", e))
                })?
            } else {
                serde_json::to_string_pretty(&records).map_err(|e| {
                    OperationError::InvalidInput(format!("Error serializing JSON: {}", e))
                })?
            }
        } else {
            let mut s = String::new();
            for record in records {
                s.push_str(&serde_json::to_string(&record).map_err(|e| {
                    OperationError::InvalidInput(format!("Error serializing JSON: {}", e))
                })?);
                s.push_str("\\n");
            }
            s
        };

        Ok(output.into_bytes())
    }
}

fn avro_to_json(value: apache_avro::types::Value) -> Result<Value, OperationError> {
    match value {
        apache_avro::types::Value::Null => Ok(Value::Null),
        apache_avro::types::Value::Boolean(b) => Ok(json!(b)),
        apache_avro::types::Value::Int(i) => Ok(json!(i)),
        apache_avro::types::Value::Long(l) => Ok(json!(l)),
        apache_avro::types::Value::Float(f) => Ok(json!(f)),
        apache_avro::types::Value::Double(d) => Ok(json!(d)),
        apache_avro::types::Value::String(s) => Ok(json!(s)),
        apache_avro::types::Value::Bytes(b) => Ok(json!(b)),
        apache_avro::types::Value::Array(a) => {
            let mut arr = Vec::new();
            for item in a {
                arr.push(avro_to_json(item)?);
            }
            Ok(json!(arr))
        }
        apache_avro::types::Value::Record(mut fields) => {
            let mut map = serde_json::Map::new();
            for (key, value) in fields.drain(..) {
                map.insert(key, avro_to_json(value)?);
            }
            Ok(json!(map))
        }
        apache_avro::types::Value::Map(map) => {
            let mut json_map = serde_json::Map::new();
            for (key, value) in map {
                json_map.insert(key, avro_to_json(value)?);
            }
            Ok(json!(json_map))
        }
        apache_avro::types::Value::Union(_, value) => avro_to_json(*value),
        _ => Err(OperationError::InvalidInput(
            "Unsupported Avro type".to_string(),
        )),
    }
}
