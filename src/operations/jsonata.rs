/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: JSON Query mit jaq (Jsonata-Ersatz)
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Jsonata Query operation
pub struct Jsonata;

impl Operation for Jsonata {
    fn name(&self) -> &'static str {
        "Jsonata Query"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Query and transform JSON data using jaq. Jsonata is not natively available in Rust,
        so jaq is used as an alternative. Enable with: --features jsonata"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Query",
            description: "The jaq query to run",
            default_value: ".",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn is_broken(&self) -> bool {
        #[cfg(not(feature = "jsonata"))]
        return true;
        #[cfg(feature = "jsonata")]
        return false;
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        #[cfg(feature = "jsonata")]
        {
            self.run_jaq(input, args)
        }
        #[cfg(not(feature = "jsonata"))]
        {
            let _ = (input, args);
            Err(OperationError::ProcessingError(
                "Jsonata Query requires --features jsonata. Enable with: cargo build --features jsonata"
                    .to_string(),
            ))
        }
    }
}

#[cfg(feature = "jsonata")]
impl Jsonata {
    fn run_jaq(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        use jaq_core::Value as JaqValue;
        use serde_json::Value as JsonValue;

        let query = args.get(0).and_then(|a| a.as_str()).unwrap_or(".");

        let input_val: JsonValue = serde_json::from_slice(&input)
            .map_err(|e| OperationError::InvalidInput(e.to_string()))?;

        let input_jaq = json_to_jaq(&input_val);

        let filter = jaq_core::parse(query).map_err(|e| OperationError::InvalidArgument {
            name: "Query".into(),
            reason: e.to_string(),
        })?;

        let mut iter = std::iter::once(Ok(input_jaq.clone()));
        let mut results = Vec::new();

        while let Some(Ok(val)) = filter.run(
            &jaq_std::std::Ctx::new(jaq_std::std::Filter::default()),
            &mut iter,
        ) {
            results.push(jaq_to_json(val));
        }

        if results.is_empty() {
            Ok(serde_json::json!(null).to_string().into_bytes())
        } else if results.len() == 1 {
            serde_json::to_vec(&results[0])
                .map_err(|e| OperationError::ProcessingError(e.to_string()))
        } else {
            serde_json::to_vec(&results).map_err(|e| OperationError::ProcessingError(e.to_string()))
        }
    }
}

#[cfg(feature = "jsonata")]
fn json_to_jaq(value: &JsonValue) -> JaqValue {
    match value {
        JsonValue::Null => JaqValue::Null,
        JsonValue::Bool(b) => JaqValue::Bool(*b),
        JsonValue::Number(n) => {
            if let Some(n) = n.as_i64() {
                JaqValue::Int(n)
            } else if let Some(n) = n.as_f64() {
                JaqValue::Float(n)
            } else {
                JaqValue::Null
            }
        }
        JsonValue::String(s) => JaqValue::Str(s.clone()),
        JsonValue::Array(arr) => {
            let vec: Vec<_> = arr.iter().map(json_to_jaq).collect();
            JaqValue::Arr(vec.into_iter().map(Ok).collect::<Result<_, _>>().unwrap())
        }
        JsonValue::Object(obj) => {
            let mut map = jaq_core::Map::new();
            for (k, v) in obj {
                map.insert(k.clone(), json_to_jaq(v));
            }
            JaqValue::Obj(map)
        }
    }
}

#[cfg(feature = "jsonata")]
fn jaq_to_json(value: JaqValue) -> JsonValue {
    match value {
        JaqValue::Null => JsonValue::Null,
        JaqValue::Bool(b) => JsonValue::Bool(b),
        JaqValue::Int(n) => JsonValue::from(n),
        JaqValue::Float(f) => {
            if f.is_finite() {
                JsonValue::from(f)
            } else {
                JsonValue::Null
            }
        }
        JaqValue::Str(s) => JsonValue::String(s),
        JaqValue::Arr(arr) => {
            let vec: Vec<_> = arr.into_iter().map(jaq_to_json).collect();
            JsonValue::Array(vec)
        }
        JaqValue::Obj(obj) => {
            let mut map = serde_json::Map::new();
            for (k, v) in obj.into_iter() {
                map.insert(k, jaq_to_json(v));
            }
            JsonValue::Object(map)
        }
        JaqValue::Func(_) => JsonValue::Null,
    }
}
