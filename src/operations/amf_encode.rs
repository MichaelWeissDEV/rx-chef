/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the AMF Encode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// AMF Encode operation
///
/// Action Message Format (AMF) is a binary format used to serialize object
/// graphs such as ActionScript objects and XML, or send messages between an
/// Adobe Flash client and a remote service.
pub struct AmfEncode;

impl Operation for AmfEncode {
    fn name(&self) -> &'static str {
        "AMF Encode"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Action Message Format (AMF) is a binary format used to serialize object graphs such as ActionScript objects and XML, or send messages between an Adobe Flash client and a remote service, usually a Flash Media Server or third party alternatives."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Format",
            description: "AMF format (AMF0 or AMF3)",
            default_value: "AMF3",
            kind: crate::operation::ArgKind::String,
            required: false,
            choices: &[],
            minimum: None,
            maximum: None,
            sensitive: false,
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let format = args.first().and_then(ArgValue::as_str).unwrap_or("AMF3");
        let json: serde_json::Value = serde_json::from_slice(&input)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;
        let value = match format {
            "AMF0" => amf::Value::Amf0(to_amf0(&json)?),
            "AMF3" => amf::Value::Amf3(to_amf3(&json)?),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Format".into(),
                    reason: "Expected AMF0 or AMF3".into(),
                })
            }
        };
        let mut output = Vec::new();
        value
            .write_to(&mut output)
            .map_err(|error| OperationError::ProcessingError(error.to_string()))?;
        Ok(output)
    }
}

fn to_amf0(value: &serde_json::Value) -> Result<amf::Amf0Value, OperationError> {
    use amf::Amf0Value as A;
    Ok(match value {
        serde_json::Value::Null => A::Null,
        serde_json::Value::Bool(value) => A::Boolean(*value),
        serde_json::Value::Number(value) => A::Number(value.as_f64().ok_or_else(|| {
            OperationError::InvalidInput("AMF0 number is outside the f64 range".into())
        })?),
        serde_json::Value::String(value) => A::String(value.clone()),
        serde_json::Value::Array(values) => A::Array {
            entries: values.iter().map(to_amf0).collect::<Result<_, _>>()?,
        },
        serde_json::Value::Object(values) => A::Object {
            class_name: None,
            entries: values
                .iter()
                .map(|(key, value)| {
                    Ok(amf::Pair {
                        key: key.clone(),
                        value: to_amf0(value)?,
                    })
                })
                .collect::<Result<_, OperationError>>()?,
        },
    })
}

fn to_amf3(value: &serde_json::Value) -> Result<amf::Amf3Value, OperationError> {
    use amf::Amf3Value as A;
    Ok(match value {
        serde_json::Value::Null => A::Null,
        serde_json::Value::Bool(value) => A::Boolean(*value),
        serde_json::Value::Number(value) => {
            if let Some(integer) = value
                .as_i64()
                .filter(|value| (-268_435_456..=268_435_455).contains(value))
            {
                A::Integer(integer as i32)
            } else {
                A::Double(value.as_f64().ok_or_else(|| {
                    OperationError::InvalidInput("AMF3 number is outside the f64 range".into())
                })?)
            }
        }
        serde_json::Value::String(value) => A::String(value.clone()),
        serde_json::Value::Array(values) => A::Array {
            assoc_entries: Vec::new(),
            dense_entries: values.iter().map(to_amf3).collect::<Result<_, _>>()?,
        },
        serde_json::Value::Object(values) => A::Object {
            class_name: None,
            sealed_count: 0,
            entries: values
                .iter()
                .map(|(key, value)| {
                    Ok(amf::Pair {
                        key: key.clone(),
                        value: to_amf3(value)?,
                    })
                })
                .collect::<Result<_, OperationError>>()?,
        },
    })
}
