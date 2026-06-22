/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PHP Serialize operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PHP Serialize operation
pub struct PHPSerialize;

impl Operation for PHPSerialize {
    fn name(&self) -> &'static str {
        "PHP Serialize"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Performs PHP serialization on JSON data.<br><br>This function does not support <code>object</code> tags.<br><br>Since PHP doesn't distinguish dicts and arrays, this operation is not always symmetric to <code>PHP Deserialize</code>.<br><br>Example:<br><code>[5,&quot;abc&quot;,true]</code><br>becomes<br><code>a:3:{i:0;i:5;i:1;s:3:&quot;abc&quot;;i:2;b:1;}<code>"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let json: Value = serde_json::from_slice(&input)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid JSON: {}", e)))?;

        let serialized = serialize(&json)?;
        Ok(serialized.into_bytes())
    }
}

fn serialize(value: &Value) -> Result<String, OperationError> {
    match value {
        Value::Null => Ok("N;".to_string()),
        Value::Bool(b) => Ok(format!("b:{};", if *b { 1 } else { 0 })),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(format!("i:{};", i))
            } else if let Some(f) = n.as_f64() {
                Ok(format!("d:{};", f))
            } else {
                Err(OperationError::ProcessingError(
                    "Invalid number".to_string(),
                ))
            }
        }
        Value::String(s) => Ok(format!("s:{}:\"{}\";", s.len(), s)),
        Value::Array(arr) => {
            let mut result = format!("a:{}:{{", arr.len());
            for (i, v) in arr.iter().enumerate() {
                result.push_str(&format!("i:{};", i));
                result.push_str(&serialize(v)?);
            }
            result.push('}');
            Ok(result)
        }
        Value::Object(obj) => {
            let mut result = format!("a:{}:{{", obj.len());
            for (k, v) in obj {
                result.push_str(&format!("s:{}:\"{}\";", k.len(), k));
                result.push_str(&serialize(v)?);
            }
            result.push('}');
            Ok(result)
        }
    }
}
