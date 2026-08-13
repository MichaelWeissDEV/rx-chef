//! Stable, machine-readable integration API for editors and other frontends.
//!
//! This module contains no terminal-specific code. It can be embedded directly
//! as a Rust library or exposed through the newline-delimited JSON protocol
//! implemented by [`serve_jsonl`].

use std::io::{BufRead, Write};

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub use crate::execution::RecipeStep;

use crate::{execution, runtime};

/// Current version of the JSONL integration protocol.
pub const PROTOCOL_VERSION: u32 = 1;
/// Default maximum size of one JSONL request, excluding the newline.
pub const DEFAULT_MAX_REQUEST_BYTES: usize = 1024 * 1024;

/// One operation argument in a machine-readable descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgumentDescriptor {
    pub name: String,
    pub description: String,
    pub default: String,
    pub kind: crate::operation::ArgKind,
    pub required: bool,
    pub choices: Vec<String>,
    pub minimum: Option<String>,
    pub maximum: Option<String>,
    pub sensitive: bool,
}

/// Serializable operation metadata used by CLI and editor integrations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationDescriptor {
    pub name: String,
    pub id: String,
    pub aliases: Vec<String>,
    pub module: String,
    pub description: String,
    pub input_type: String,
    pub output_type: String,
    pub broken: bool,
    pub input_requirement: crate::operation::InputRequirement,
    pub status: crate::operation::OperationStatus,
    pub available: bool,
    pub feature_requirements: Vec<String>,
    pub platform_requirements: Vec<String>,
    pub side_effects: Vec<crate::operation::SideEffect>,
    pub deterministic: bool,
    pub parity: crate::operation::ParityStatus,
    pub known_limitations: Vec<String>,
    pub documentation_slug: String,
    pub args: Vec<ArgumentDescriptor>,
}

/// Binary-safe result envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// UTF-8-lossy convenience representation for editor UIs.
    pub output: String,
    /// Exact output bytes encoded as standard padded Base64.
    pub output_base64: String,
    pub output_len: usize,
    /// Whether `output` is an exact UTF-8 representation (`false` means it is
    /// only a lossy UI convenience and `output_base64` is authoritative).
    pub output_is_utf8: bool,
}

impl ExecutionResult {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let output_is_utf8 = std::str::from_utf8(&bytes).is_ok();
        Self {
            output: String::from_utf8_lossy(&bytes).into_owned(),
            output_base64: general_purpose::STANDARD.encode(&bytes),
            output_len: bytes.len(),
            output_is_utf8,
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
        id: info.id.clone(),
        aliases: Vec::new(),
        module: info.module.to_string(),
        description: info.description.to_string(),
        input_type: runtime::data_type_name(info.input_type).to_string(),
        output_type: runtime::data_type_name(info.output_type).to_string(),
        broken: info.is_broken,
        input_requirement: info.input_requirement,
        status: info.status,
        available: !info.is_broken,
        feature_requirements: info
            .feature_requirements
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        platform_requirements: Vec::new(),
        side_effects: info.side_effects.to_vec(),
        deterministic: info.deterministic,
        parity: info.parity,
        known_limitations: info
            .known_limitations
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        documentation_slug: info.id.replace('_', "-"),
        args: info
            .args
            .iter()
            .map(|arg| ArgumentDescriptor {
                name: arg.name.to_string(),
                description: arg.description.to_string(),
                default: arg.default_value.to_string(),
                kind: runtime::inferred_arg_kind(arg),
                required: false,
                choices: Vec::new(),
                minimum: None,
                maximum: None,
                sensitive: runtime::is_sensitive_arg(arg),
            })
            .collect(),
    })
}

/// Execute a single operation through the shared execution engine.
pub fn run(operation: &str, input: Vec<u8>, args: &[String]) -> Result<ExecutionResult, String> {
    execution::run(operation, input, args.to_vec())
        .map(|outcome| ExecutionResult::from_bytes(outcome.output))
        .map_err(|error| error.to_string())
}

/// Execute an arbitrary recipe through the shared execution engine.
pub fn bake(input: Vec<u8>, recipe: &[RecipeStep]) -> Result<ExecutionResult, String> {
    execution::execute(execution::ExecutionRequest {
        input,
        recipe: recipe.to_vec().into(),
        variables: execution::VariableContext::default(),
        options: execution::ExecutionOptions::default(),
    })
    .map(|outcome| ExecutionResult::from_bytes(outcome.output))
    .map_err(|error| error.to_string())
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
    serve_jsonl_with_limit(reader, &mut writer, DEFAULT_MAX_REQUEST_BYTES)
}

/// Serve JSONL with an explicit maximum request-line size.
pub fn serve_jsonl_with_limit<R: BufRead, W: Write>(
    mut reader: R,
    mut writer: W,
    max_request_bytes: usize,
) -> Result<(), String> {
    if max_request_bytes == 0 {
        return Err("max_request_bytes must be greater than zero".into());
    }
    loop {
        let Some(line) = read_bounded_line(&mut reader, max_request_bytes)
            .map_err(|error| format!("cannot read request: {error}"))?
        else {
            break;
        };
        let line = match line {
            Ok(line) => line,
            Err(()) => {
                let response = error_response(
                    Value::Null,
                    -32004,
                    format!("request exceeds {max_request_bytes} byte limit"),
                );
                serde_json::to_writer(&mut writer, &response)
                    .map_err(|error| format!("cannot encode response: {error}"))?;
                writer.write_all(b"\n").map_err(|e| e.to_string())?;
                writer.flush().map_err(|e| e.to_string())?;
                continue;
            }
        };
        let line = match String::from_utf8(line) {
            Ok(line) => line,
            Err(error) => {
                let response = error_response(Value::Null, -32700, error.to_string());
                serde_json::to_writer(&mut writer, &response)
                    .map_err(|error| format!("cannot encode response: {error}"))?;
                writer.write_all(b"\n").map_err(|e| e.to_string())?;
                writer.flush().map_err(|e| e.to_string())?;
                continue;
            }
        };
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

fn read_bounded_line<R: BufRead>(
    reader: &mut R,
    limit: usize,
) -> std::io::Result<Option<Result<Vec<u8>, ()>>> {
    let mut line = Vec::new();
    let mut exceeded = false;
    let mut read_any = false;
    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            return if read_any {
                Ok(Some(if exceeded { Err(()) } else { Ok(line) }))
            } else {
                Ok(None)
            };
        }
        read_any = true;
        let consumed = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |index| index + 1);
        let chunk = &available[..consumed];
        let without_newline = chunk.strip_suffix(b"\n").unwrap_or(chunk);
        if !exceeded {
            if line.len().saturating_add(without_newline.len()) > limit {
                exceeded = true;
                line.clear();
            } else {
                line.extend_from_slice(without_newline);
            }
        }
        let finished = chunk.ends_with(b"\n");
        reader.consume(consumed);
        if finished {
            return Ok(Some(if exceeded { Err(()) } else { Ok(line) }));
        }
    }
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
    fn bake_executes_fork_merge_and_nested_steps() {
        let result = bake(
            b"one\ntwo\nthree".to_vec(),
            &[
                RecipeStep {
                    op: "Fork".into(),
                    args: vec!["\\n".into(), " | ".into(), "false".into()],
                },
                RecipeStep {
                    op: "To Upper case".into(),
                    args: vec![],
                },
                RecipeStep {
                    op: "Merge".into(),
                    args: vec![],
                },
            ],
        )
        .unwrap();
        assert_eq!(result.into_bytes().unwrap(), b"ONE | TWO | THREE");
    }

    #[test]
    fn bake_executes_subsections_without_touching_surrounding_bytes() {
        let result = bake(
            b"a12b345c".to_vec(),
            &[
                RecipeStep {
                    op: "Subsection".into(),
                    args: vec!["[0-9]+".into(), "true".into(), "true".into()],
                },
                RecipeStep {
                    op: "To Base64".into(),
                    args: vec![],
                },
                RecipeStep {
                    op: "Merge".into(),
                    args: vec![],
                },
            ],
        )
        .unwrap();
        assert_eq!(result.into_bytes().unwrap(), b"aMTI=bMzQ1c");
    }

    #[test]
    fn bake_expands_registers_in_later_arguments() {
        let result = bake(
            b"hello XYZ".to_vec(),
            &[
                RecipeStep {
                    op: "Register".into(),
                    args: vec!["([A-Z]+)".into(), "false".into()],
                },
                RecipeStep {
                    op: "Find / Replace".into(),
                    args: vec!["hello".into(), "Simple string".into(), "$R0".into()],
                },
            ],
        )
        .unwrap();
        assert_eq!(result.into_bytes().unwrap(), b"XYZ XYZ");
    }

    #[test]
    fn bake_honours_forward_and_bounded_backward_jumps() {
        let result = bake(
            b"abc".to_vec(),
            &[
                RecipeStep {
                    op: "Jump".into(),
                    args: vec!["finish".into()],
                },
                RecipeStep {
                    op: "Reverse".into(),
                    args: vec![],
                },
                RecipeStep {
                    op: "Label".into(),
                    args: vec!["finish".into()],
                },
                RecipeStep {
                    op: "To Upper case".into(),
                    args: vec![],
                },
                RecipeStep {
                    op: "Label".into(),
                    args: vec!["again".into()],
                },
                RecipeStep {
                    op: "Jump".into(),
                    args: vec!["again".into(), "2".into()],
                },
            ],
        )
        .unwrap();
        assert_eq!(result.into_bytes().unwrap(), b"ABC");
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

    #[test]
    fn request_limit_accepts_boundary_and_recovers_after_oversize() {
        let ping = r#"{"id":1,"method":"ping"}"#;
        let input = format!("{ping}\n{}\n{ping}\n", "x".repeat(ping.len() + 1));
        let mut output = Vec::new();
        serve_jsonl_with_limit(Cursor::new(input), &mut output, ping.len()).unwrap();
        let responses = String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(responses.len(), 3);
        assert_eq!(responses[0]["result"]["protocol_version"], 1);
        assert_eq!(responses[1]["error"]["code"], -32004);
        assert_eq!(responses[2]["result"]["protocol_version"], 1);
    }

    #[cfg(all(feature = "pgp", feature = "jsonata", feature = "tesseract"))]
    #[test]
    fn every_operation_is_available_with_all_features() {
        let broken = operations()
            .unwrap()
            .into_iter()
            .filter(|operation| operation.broken)
            .map(|operation| operation.name)
            .collect::<Vec<_>>();
        assert!(broken.is_empty(), "broken operations: {broken:?}");
    }
}
