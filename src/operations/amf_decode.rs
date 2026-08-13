/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the AMF Decode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// AMF Decode operation
///
/// Action Message Format (AMF) is a binary format used to serialize object
/// graphs such as ActionScript objects and XML, or send messages between an
/// Adobe Flash client and a remote service.
pub struct AmfDecode;

impl Operation for AmfDecode {
    fn name(&self) -> &'static str {
        "AMF Decode"
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
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let format = args.first().and_then(|a| a.as_str()).unwrap_or("AMF3");

        let version = match format {
            "AMF0" => amf::Version::Amf0,
            "AMF3" => amf::Version::Amf3,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Format".into(),
                    reason: "Expected AMF0 or AMF3".into(),
                })
            }
        };
        let decoded = amf::Value::read_from(input.as_slice(), version)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;
        let json = match decoded {
            amf::Value::Amf0(value) => from_amf0(value),
            amf::Value::Amf3(value) => from_amf3(value),
        };
        serde_json::to_vec_pretty(&json)
            .map_err(|error| OperationError::ProcessingError(error.to_string()))
    }
}

fn object(
    entries: impl IntoIterator<Item = amf::Pair<String, serde_json::Value>>,
) -> serde_json::Value {
    serde_json::Value::Object(
        entries
            .into_iter()
            .map(|pair| (pair.key, pair.value))
            .collect(),
    )
}

fn from_amf0(value: amf::Amf0Value) -> serde_json::Value {
    use amf::Amf0Value as A;
    match value {
        A::Number(value) => serde_json::json!(value),
        A::Boolean(value) => serde_json::json!(value),
        A::String(value) | A::XmlDocument(value) => serde_json::json!(value),
        A::Null | A::Undefined => serde_json::Value::Null,
        A::Array { entries } => {
            serde_json::Value::Array(entries.into_iter().map(from_amf0).collect())
        }
        A::Object { entries, .. } | A::EcmaArray { entries } => {
            object(entries.into_iter().map(|pair| amf::Pair {
                key: pair.key,
                value: from_amf0(pair.value),
            }))
        }
        A::Date {
            unix_time,
            time_zone,
        } => serde_json::json!({ "$dateMillis": unix_time.as_millis(), "$timeZone": time_zone }),
        A::AvmPlus(value) => from_amf3(value),
    }
}

fn from_amf3(value: amf::Amf3Value) -> serde_json::Value {
    use amf::Amf3Value as A;
    match value {
        A::Undefined | A::Null => serde_json::Value::Null,
        A::Boolean(value) => serde_json::json!(value),
        A::Integer(value) => serde_json::json!(value),
        A::Double(value) => serde_json::json!(value),
        A::String(value) | A::XmlDocument(value) | A::Xml(value) => serde_json::json!(value),
        A::Date { unix_time } => serde_json::json!({ "$dateMillis": unix_time.as_millis() }),
        A::Array {
            assoc_entries,
            dense_entries,
        } => {
            if assoc_entries.is_empty() {
                serde_json::Value::Array(dense_entries.into_iter().map(from_amf3).collect())
            } else {
                serde_json::json!({
                    "$associative": object(assoc_entries.into_iter().map(|pair| amf::Pair { key: pair.key, value: from_amf3(pair.value) })),
                    "$dense": dense_entries.into_iter().map(from_amf3).collect::<Vec<_>>()
                })
            }
        }
        A::Object {
            class_name,
            entries,
            ..
        } => {
            let mut value = object(entries.into_iter().map(|pair| amf::Pair {
                key: pair.key,
                value: from_amf3(pair.value),
            }));
            if let (Some(class), Some(map)) = (class_name, value.as_object_mut()) {
                map.insert("$className".into(), serde_json::json!(class));
            }
            value
        }
        A::ByteArray(value) => serde_json::json!({ "$bytes": hex::encode(value) }),
        A::IntVector { is_fixed, entries } => {
            serde_json::json!({ "$fixed": is_fixed, "$intVector": entries })
        }
        A::UintVector { is_fixed, entries } => {
            serde_json::json!({ "$fixed": is_fixed, "$uintVector": entries })
        }
        A::DoubleVector { is_fixed, entries } => {
            serde_json::json!({ "$fixed": is_fixed, "$doubleVector": entries })
        }
        A::ObjectVector {
            class_name,
            is_fixed,
            entries,
        } => {
            serde_json::json!({ "$className": class_name, "$fixed": is_fixed, "$objectVector": entries.into_iter().map(from_amf3).collect::<Vec<_>>() })
        }
        A::Dictionary { is_weak, entries } => {
            serde_json::json!({ "$weak": is_weak, "$dictionary": entries.into_iter().map(|pair| [from_amf3(pair.key), from_amf3(pair.value)]).collect::<Vec<_>>() })
        }
    }
}
