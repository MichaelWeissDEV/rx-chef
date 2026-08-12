//! Stable, machine-readable integration API for editors and other frontends.
//!
//! This module contains no terminal-specific code. It can be embedded directly
//! as a Rust library or exposed through the newline-delimited JSON protocol
//! implemented by [`serve_jsonl`].

use std::io::{BufRead, Write};

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::runtime;

/// Current version of the JSONL integration protocol.
pub const PROTOCOL_VERSION: u32 = 1;

/// One operation argument in a machine-readable descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgumentDescriptor {
    pub name: String,
    pub description: String,
    pub default: String,
}

/// Serializable operation metadata used by CLI and editor integrations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationDescriptor {
    pub name: String,
    pub module: String,
    pub description: String,
    pub input_type: String,
    pub output_type: String,
    pub broken: bool,
    pub args: Vec<ArgumentDescriptor>,
}

/// One operation and its ordered argument values in a recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeStep {
    #[serde(alias = "operation")]
    pub op: String,
    #[serde(default)]
    pub args: Vec<String>,
}

/// Binary-safe result envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// UTF-8-lossy convenience representation for editor UIs.
    pub output: String,
    /// Exact output bytes encoded as standard padded Base64.
    pub output_base64: String,
    pub output_len: usize,
}

impl ExecutionResult {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            output: String::from_utf8_lossy(&bytes).into_owned(),
            output_base64: general_purpose::STANDARD.encode(&bytes),
            output_len: bytes.len(),
        }
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, String> {
        general_purpose::STANDARD
            .decode(self.output_base64)
            .map_err(|error| format!("invalid result Base64: {error}"))
    }
}

/// Return every registered operation with its complete argument schema.
pub fn operations() -> Result<Vec<OperationDescriptor>, String> {
    runtime::operation_names(None)
        .into_iter()
        .map(|name| describe(&name))
        .collect()
}

/// Resolve an operation alias and return its complete descriptor.
pub fn describe(operation: &str) -> Result<OperationDescriptor, String> {
    let info = runtime::operation_info(operation)?;
    Ok(OperationDescriptor {
        name: info.name.to_string(),
        module: info.module.to_string(),
        description: info.description.to_string(),
        input_type: runtime::data_type_name(info.input_type).to_string(),
        output_type: runtime::data_type_name(info.output_type).to_string(),
        broken: info.is_broken,
        args: info
            .args
            .iter()
            .map(|arg| ArgumentDescriptor {
                name: arg.name.to_string(),
                description: arg.description.to_string(),
                default: arg.default_value.to_string(),
            })
            .collect(),
    })
}

/// Execute a single operation through the shared runtime.
pub fn run(operation: &str, input: Vec<u8>, args: &[String]) -> Result<ExecutionResult, String> {
    runtime::run_operation(operation, input, args).map(ExecutionResult::from_bytes)
}

/// Execute an arbitrary recipe from left to right.
pub fn bake(input: Vec<u8>, recipe: &[RecipeStep]) -> Result<ExecutionResult, String> {
    let mut current = input;
    for (index, step) in recipe.iter().enumerate() {
        current = runtime::run_operation(&step.op, current, &step.args)
            .map_err(|error| format!("step {} ({}): {error}", index + 1, step.op))?;
    }
    Ok(ExecutionResult::from_bytes(current))
}

#[derive(Debug, Deserialize)]
struct Request {
    #[serde(default)]
    jsonrpc: Option<String>,
    #[serde(default)]
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Deserialize)]
struct DescribeParams {
    operation: String,
}

#[derive(Debug, Deserialize)]
struct RunParams {
    operation: String,
    #[serde(default)]
    input: Option<String>,
    #[serde(default)]
    input_base64: Option<String>,
    #[serde(default)]
    args: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct BakeParams {
    #[serde(default)]
    input: Option<String>,
    #[serde(default)]
    input_base64: Option<String>,
    recipe: Vec<RecipeStep>,
}

fn decode_input(input: Option<String>, input_base64: Option<String>) -> Result<Vec<u8>, String> {
    match (input, input_base64) {
        (Some(_), Some(_)) => Err("provide only one of input or input_base64".to_string()),
        (Some(text), None) => Ok(text.into_bytes()),
        (None, Some(encoded)) => general_purpose::STANDARD
            .decode(encoded)
            .map_err(|error| format!("invalid input_base64: {error}")),
        (None, None) => Ok(Vec::new()),
    }
}

fn error_response(id: Value, code: i64, message: impl Into<String>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {"code": code, "message": message.into()}
    })
}

/// Handle one decoded JSON-RPC/JSONL request.
///
/// Requests may omit `jsonrpc` for lightweight JSONL clients. Requests without
/// an `id` are notifications and produce no response. Supported methods are
/// `ping`, `operations`, `describe`, `run`, `bake`, and `shutdown`.
pub fn handle_request(value: Value) -> Option<Value> {
    let request: Request = match serde_json::from_value(value) {
        Ok(request) => request,
        Err(error) => return Some(error_response(Value::Null, -32600, error.to_string())),
    };
    let id = request.id.clone();
    let response_id = id.clone().unwrap_or(Value::Null);

    if let Some(version) = request.jsonrpc.as_deref() {
        if version != "2.0" {
            return id.map(|_| error_response(response_id, -32600, "jsonrpc must be '2.0'"));
        }
    }

    let result: Result<Value, (i64, String)> = match request.method.as_str() {
        "ping" => Ok(json!({
            "name": "rxchef",
            "version": env!("CARGO_PKG_VERSION"),
            "protocol_version": PROTOCOL_VERSION
        })),
        "operations" => operations()
            .and_then(|value| serde_json::to_value(value).map_err(|error| error.to_string()))
            .map_err(|message| (-32603, message)),
        "describe" => serde_json::from_value::<DescribeParams>(request.params)
            .map_err(|error| (-32602, error.to_string()))
            .and_then(|params| describe(&params.operation).map_err(|error| (-32001, error)))
            .and_then(|value| {
                serde_json::to_value(value).map_err(|error| (-32603, error.to_string()))
            }),
        "run" => serde_json::from_value::<RunParams>(request.params)
            .map_err(|error| (-32602, error.to_string()))
            .and_then(|params| {
                let input = decode_input(params.input, params.input_base64)
                    .map_err(|error| (-32602, error))?;
                run(&params.operation, input, &params.args).map_err(|error| (-32002, error))
            })
            .and_then(|value| {
                serde_json::to_value(value).map_err(|error| (-32603, error.to_string()))
            }),
        "bake" => serde_json::from_value::<BakeParams>(request.params)
            .map_err(|error| (-32602, error.to_string()))
            .and_then(|params| {
                let input = decode_input(params.input, params.input_base64)
                    .map_err(|error| (-32602, error))?;
                bake(input, &params.recipe).map_err(|error| (-32002, error))
            })
            .and_then(|value| {
                serde_json::to_value(value).map_err(|error| (-32603, error.to_string()))
            }),
        "shutdown" => Ok(json!({"shutdown": true})),
        _ => Err((-32601, format!("method '{}' was not found", request.method))),
    };

    id.map(|_| match result {
        Ok(result) => json!({"jsonrpc": "2.0", "id": response_id, "result": result}),
        Err((code, message)) => error_response(response_id, code, message),
    })
}

/// Serve newline-delimited JSON requests until EOF or a `shutdown` request.
///
/// Exactly one compact JSON response is flushed per request carrying an `id`.
/// Malformed JSON produces a JSON-RPC parse error and does not stop the server.
pub fn serve_jsonl<R: BufRead, W: Write>(reader: R, mut writer: W) -> Result<(), String> {
    for line in reader.lines() {
        let line = line.map_err(|error| format!("cannot read request: {error}"))?;
        if line.trim().is_empty() {
            continue;
        }
        let parsed = serde_json::from_str::<Value>(&line);
        let shutdown = parsed
            .as_ref()
            .ok()
            .and_then(|value| value.get("method"))
            .and_then(Value::as_str)
            == Some("shutdown");
        let response = match parsed {
            Ok(value) => handle_request(value),
            Err(error) => Some(error_response(Value::Null, -32700, error.to_string())),
        };
        if let Some(response) = response {
            serde_json::to_writer(&mut writer, &response)
                .map_err(|error| format!("cannot encode response: {error}"))?;
            writer
                .write_all(b"\n")
                .map_err(|error| format!("cannot write response: {error}"))?;
            writer
                .flush()
                .map_err(|error| format!("cannot flush response: {error}"))?;
        }
        if shutdown {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn bake_chains_arbitrary_operations() {
        let result = bake(
            b"Hello".to_vec(),
            &[
                RecipeStep {
                    op: "to_upper_case".into(),
                    args: vec![],
                },
                RecipeStep {
                    op: "to_base64".into(),
                    args: vec![],
                },
                RecipeStep {
                    op: "from_base64".into(),
                    args: vec![],
                },
            ],
        )
        .unwrap();
        assert_eq!(result.into_bytes().unwrap(), b"HELLO");
    }

    #[test]
    fn jsonl_server_is_persistent_binary_safe_and_recovers_from_bad_json() {
        let requests = concat!(
            "{bad json}\n",
            "{\"id\":1,\"method\":\"ping\"}\n",
            "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"bake\",\"params\":{\"input\":\"Hello\",\"recipe\":[{\"op\":\"to_upper_case\"},{\"op\":\"to_base64\"}]}}\n",
            "{\"id\":3,\"method\":\"shutdown\"}\n",
            "{\"id\":4,\"method\":\"ping\"}\n"
        );
        let mut output = Vec::new();
        serve_jsonl(Cursor::new(requests), &mut output).unwrap();
        let responses: Vec<Value> = String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(responses.len(), 4);
        assert_eq!(responses[0]["error"]["code"], -32700);
        assert_eq!(responses[1]["result"]["protocol_version"], 1);
        assert_eq!(responses[2]["result"]["output"], "SEVMTE8=");
        assert_eq!(responses[3]["result"]["shutdown"], true);
    }

    #[test]
    fn notifications_do_not_emit_responses() {
        let mut output = Vec::new();
        serve_jsonl(Cursor::new("{\"method\":\"ping\"}\n"), &mut output).unwrap();
        assert!(output.is_empty());
    }
}
