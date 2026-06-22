/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Rison Encode operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Rison Encode operation
pub struct RisonEncode;

impl Operation for RisonEncode {
    fn name(&self) -> &'static str {
        "Rison Encode"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Rison, a data serialization format optimized for compactness in URIs. Rison is a slight variation of JSON that looks vastly superior after URI encoding. Rison still expresses exactly the same set of data structures as JSON, so data can be translated back and forth without loss or guesswork."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Encode Option",
            description: "Encode, Encode Object, Encode Array, or Encode URI",
            default_value: "Encode",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let value: Value = serde_json::from_slice(&input)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid JSON input: {}", e)))?;

        let encode_option = args.first().and_then(|a| a.as_str()).unwrap_or("Encode");

        let mut result = String::new();
        match encode_option {
            "Encode" => encode_value(&value, &mut result),
            "Encode Object" => {
                if let Value::Object(_) = value {
                    encode_value(&value, &mut result);
                } else {
                    return Err(OperationError::InvalidInput(
                        "Input must be a JSON object for 'Encode Object'".to_string(),
                    ));
                }
            }
            "Encode Array" => {
                if let Value::Array(_) = value {
                    encode_value(&value, &mut result);
                } else {
                    return Err(OperationError::InvalidInput(
                        "Input must be a JSON array for 'Encode Array'".to_string(),
                    ));
                }
            }
            "Encode URI" => {
                encode_value(&value, &mut result);
                // Simple URI encoding of characters that are not URI-safe but used by Rison if needed.
                // Actually Rison is designed to be URI-safe.
                // CyberChef's Rison.encode_uri does more aggressive encoding.
                result = result
                    .replace(' ', "%20")
                    .replace('#', "%23")
                    .replace('&', "%26")
                    .replace('+', "%2B")
                    .replace('=', "%3D");
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Encode Option".to_string(),
                    reason: format!("Invalid encode option: {}", encode_option),
                })
            }
        }

        Ok(result.into_bytes())
    }
}

fn encode_value(value: &Value, s: &mut String) {
    match value {
        Value::Null => s.push_str("!n"),
        Value::Bool(b) => s.push_str(if *b { "!t" } else { "!f" }),
        Value::Number(n) => s.push_str(&n.to_string()),
        Value::String(val) => encode_string(val, s),
        Value::Array(arr) => {
            s.push_str("!(");
            for (i, v) in arr.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                encode_value(v, s);
            }
            s.push(')');
        }
        Value::Object(map) => {
            s.push('(');
            for (i, (k, v)) in map.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                encode_string(k, s);
                s.push(':');
                encode_value(v, s);
            }
            s.push(')');
        }
    }
}

fn encode_string(val: &str, s: &mut String) {
    if val.is_empty() {
        s.push_str("''");
        return;
    }

    // Check if it's a "simple" string
    // Rison simple string: first char is alpha or _ or /, rest are alphanumeric or _ or / or - or .
    // Wait, some definitions are stricter. Let's use a safe subset.
    let first = val.chars().next().unwrap();
    let is_simple = (first.is_ascii_alphabetic() || first == '_' || first == '/')
        && val
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '/' || c == '-' || c == '.');

    if is_simple {
        s.push_str(val);
    } else {
        s.push('\'');
        for c in val.chars() {
            if c == '\'' || c == '!' {
                s.push('!');
            }
            s.push(c);
        }
        s.push('\'');
    }
}
