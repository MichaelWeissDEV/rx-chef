/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Jq operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Jq operation
pub struct Jq;

impl Operation for Jq {
    fn name(&self) -> &'static str {
        "Jq"
    }

    fn module(&self) -> &'static str {
        "Jq"
    }

    fn description(&self) -> &'static str {
        "jq is a lightweight and flexible command-line JSON processor."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Query",
                description: "The jq query to run",
                default_value: ".",
            },
            ArgSchema {
                name: "Raw",
                description: "If true, the output will be raw strings instead of JSON",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let query = args.first().and_then(|v| v.as_str()).unwrap_or(".");
        let raw = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

        let input_json: Value = serde_json::from_slice(&input)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid input JSON: {}", e)))?;

        // Lightweight fallback implementation: support simple `.` and
        // dotted object access like `.a.b[0]` (indices supported as numbers).
        // For complex jq expressions use a proper jaq-core integration later.
        if query == "." {
            if raw {
                if let Some(s) = input_json.as_str() {
                    return Ok(s.to_string().into_bytes());
                }
                if input_json.is_string() {
                    return Ok(input_json.as_str().unwrap_or("").to_string().into_bytes());
                }
                return Ok(serde_json::to_string(&input_json).unwrap().into_bytes());
            } else {
                return Ok(serde_json::to_string(&input_json).unwrap().into_bytes());
            }
        }

        if !query.starts_with('.') {
            return Err(OperationError::InvalidInput(
                "Only simple dot-queries supported".to_string(),
            ));
        }

        let mut cur = &input_json;
        for seg in query.split('.') {
            if seg.is_empty() {
                continue;
            }
            // array index?
            if let Ok(idx) = seg.parse::<usize>() {
                if let Some(arr) = cur.as_array() {
                    cur = arr.get(idx).ok_or_else(|| {
                        OperationError::InvalidInput(format!("Path segment {} not found", seg))
                    })?;
                    continue;
                } else {
                    return Err(OperationError::InvalidInput(format!(
                        "Path segment {} not found",
                        seg
                    )));
                }
            }

            if let Some(map) = cur.as_object() {
                cur = map.get(seg).ok_or_else(|| {
                    OperationError::InvalidInput(format!("Path segment {} not found", seg))
                })?;
            } else {
                return Err(OperationError::InvalidInput(format!(
                    "Path segment {} not found",
                    seg
                )));
            }
        }

        if raw {
            if let Some(s) = cur.as_str() {
                return Ok(s.to_string().into_bytes());
            }
            // For numbers/booleans return their JSON string form without quotes
            if cur.is_number() || cur.is_boolean() {
                return Ok(serde_json::to_string(cur).unwrap().into_bytes());
            }
            return Ok(serde_json::to_string(cur).unwrap().into_bytes());
        }

        Ok(serde_json::to_string(cur).unwrap().into_bytes())
    }
}
