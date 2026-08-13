use std::io::Write;

use protobuf::{
    reflect::{
        MessageDescriptor, ReflectFieldRef, ReflectValueBox, ReflectValueRef, RuntimeFieldType,
        RuntimeType,
    },
    MessageDyn,
};
use serde_json::{Map, Number, Value};

pub fn descriptor(schema: &str) -> Result<MessageDescriptor, String> {
    let directory = tempfile::tempdir().map_err(|error| error.to_string())?;
    let path = directory.path().join("schema.proto");
    let mut file = std::fs::File::create(&path).map_err(|error| error.to_string())?;
    file.write_all(schema.as_bytes())
        .map_err(|error| error.to_string())?;
    let parsed = protobuf_parse::Parser::new()
        .pure()
        .include(directory.path())
        .input(&path)
        .parse_and_typecheck()
        .map_err(|error| format!("invalid Protobuf schema: {error:#}"))?;
    let descriptors =
        protobuf::reflect::FileDescriptor::new_dynamic_fds(parsed.file_descriptors, &[])
            .map_err(|error| format!("cannot construct Protobuf descriptor: {error}"))?;
    descriptors
        .iter()
        .find(|file| file.name().ends_with("schema.proto"))
        .and_then(|file| file.messages().next())
        .ok_or_else(|| "schema must declare at least one top-level message".to_string())
}

fn integer(value: &Value) -> Result<i64, String> {
    value
        .as_i64()
        .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
        .ok_or_else(|| format!("expected integer, got {value}"))
}

fn unsigned(value: &Value) -> Result<u64, String> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
        .ok_or_else(|| format!("expected unsigned integer, got {value}"))
}

fn value_box(value: &Value, runtime_type: &RuntimeType) -> Result<ReflectValueBox, String> {
    Ok(match runtime_type {
        RuntimeType::I32 => ReflectValueBox::I32(
            integer(value)?
                .try_into()
                .map_err(|_| "integer does not fit int32".to_string())?,
        ),
        RuntimeType::I64 => ReflectValueBox::I64(integer(value)?),
        RuntimeType::U32 => ReflectValueBox::U32(
            unsigned(value)?
                .try_into()
                .map_err(|_| "integer does not fit uint32".to_string())?,
        ),
        RuntimeType::U64 => ReflectValueBox::U64(unsigned(value)?),
        RuntimeType::F32 => ReflectValueBox::F32(
            value
                .as_f64()
                .ok_or_else(|| format!("expected number, got {value}"))? as f32,
        ),
        RuntimeType::F64 => ReflectValueBox::F64(
            value
                .as_f64()
                .ok_or_else(|| format!("expected number, got {value}"))?,
        ),
        RuntimeType::Bool => ReflectValueBox::Bool(
            value
                .as_bool()
                .ok_or_else(|| format!("expected boolean, got {value}"))?,
        ),
        RuntimeType::String => ReflectValueBox::String(
            value
                .as_str()
                .ok_or_else(|| format!("expected string, got {value}"))?
                .to_string(),
        ),
        RuntimeType::VecU8 => ReflectValueBox::Bytes(
            base64::Engine::decode(
                &base64::engine::general_purpose::STANDARD,
                value
                    .as_str()
                    .ok_or_else(|| "bytes fields require a Base64 string".to_string())?,
            )
            .map_err(|error| format!("invalid Base64 bytes field: {error}"))?,
        ),
        RuntimeType::Enum(descriptor) => {
            let number = if let Some(name) = value.as_str() {
                descriptor
                    .value_by_name(name)
                    .ok_or_else(|| format!("unknown enum value '{name}'"))?
                    .value()
            } else {
                integer(value)?
                    .try_into()
                    .map_err(|_| "enum number does not fit int32".to_string())?
            };
            ReflectValueBox::Enum(descriptor.clone(), number)
        }
        RuntimeType::Message(descriptor) => {
            ReflectValueBox::Message(json_to_message(value, descriptor.clone())?)
        }
    })
}

fn map_key(value: &str, runtime_type: &RuntimeType) -> Result<ReflectValueBox, String> {
    let json = match runtime_type {
        RuntimeType::String => Value::String(value.to_string()),
        RuntimeType::Bool => Value::Bool(
            value
                .parse()
                .map_err(|_| format!("invalid boolean map key '{value}'"))?,
        ),
        _ => Value::String(value.to_string()),
    };
    value_box(&json, runtime_type)
}

pub fn json_to_message(
    value: &Value,
    descriptor: MessageDescriptor,
) -> Result<Box<dyn MessageDyn>, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "schema-based Protobuf input must be a JSON object".to_string())?;
    let mut message = descriptor.new_instance();
    for (name, value) in object {
        let field = descriptor
            .field_by_name_or_json_name(name)
            .ok_or_else(|| format!("unknown field '{name}' in {}", descriptor.full_name()))?;
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(runtime_type) => {
                if !value.is_null() {
                    field.set_singular_field(&mut *message, value_box(value, &runtime_type)?);
                }
            }
            RuntimeFieldType::Repeated(runtime_type) => {
                let values = value
                    .as_array()
                    .ok_or_else(|| format!("field '{name}' must be an array"))?;
                let mut repeated = field.mut_repeated(&mut *message);
                for value in values {
                    repeated.push(value_box(value, &runtime_type)?);
                }
            }
            RuntimeFieldType::Map(key_type, value_type) => {
                let values = value
                    .as_object()
                    .ok_or_else(|| format!("map field '{name}' must be an object"))?;
                let mut map = field.mut_map(&mut *message);
                for (key, value) in values {
                    map.insert(map_key(key, &key_type)?, value_box(value, &value_type)?);
                }
            }
        }
    }
    Ok(message)
}

fn json_number(value: f64) -> Value {
    Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

fn value_json(value: ReflectValueRef<'_>, show_types: bool) -> Value {
    let (value, kind) = match value {
        ReflectValueRef::I32(value) => (Value::from(value), "int32"),
        ReflectValueRef::I64(value) => (Value::String(value.to_string()), "int64"),
        ReflectValueRef::U32(value) => (Value::from(value), "uint32"),
        ReflectValueRef::U64(value) => (Value::String(value.to_string()), "uint64"),
        ReflectValueRef::F32(value) => (json_number(value as f64), "float"),
        ReflectValueRef::F64(value) => (json_number(value), "double"),
        ReflectValueRef::Bool(value) => (Value::Bool(value), "bool"),
        ReflectValueRef::String(value) => (Value::String(value.to_string()), "string"),
        ReflectValueRef::Bytes(value) => (
            Value::String(base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                value,
            )),
            "bytes",
        ),
        ReflectValueRef::Enum(descriptor, number) => (
            descriptor
                .value_by_number(number)
                .map(|value| Value::String(value.name().to_string()))
                .unwrap_or_else(|| Value::from(number)),
            "enum",
        ),
        ReflectValueRef::Message(message) => (message_json(&*message, show_types), "message"),
    };
    if show_types {
        serde_json::json!({"type": kind, "value": value})
    } else {
        value
    }
}

pub fn message_json(message: &dyn MessageDyn, show_types: bool) -> Value {
    let descriptor = message.descriptor_dyn();
    let mut object = Map::new();
    for field in descriptor.fields() {
        let value = match field.get_reflect(message) {
            ReflectFieldRef::Optional(optional) => {
                optional.value().map(|value| value_json(value, show_types))
            }
            ReflectFieldRef::Repeated(repeated) if !repeated.is_empty() => Some(Value::Array(
                (0..repeated.len())
                    .map(|index| value_json(repeated.get(index), show_types))
                    .collect(),
            )),
            ReflectFieldRef::Map(map) if !map.is_empty() => {
                let mut values = Map::new();
                for (key, value) in &map {
                    let key = match key {
                        ReflectValueRef::String(value) => value.to_string(),
                        ReflectValueRef::Bool(value) => value.to_string(),
                        ReflectValueRef::I32(value) => value.to_string(),
                        ReflectValueRef::I64(value) => value.to_string(),
                        ReflectValueRef::U32(value) => value.to_string(),
                        ReflectValueRef::U64(value) => value.to_string(),
                        _ => continue,
                    };
                    values.insert(key, value_json(value, show_types));
                }
                Some(Value::Object(values))
            }
            _ => None,
        };
        if let Some(value) = value {
            object.insert(field.json_name().to_string(), value);
        }
    }
    Value::Object(object)
}
