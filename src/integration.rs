//! Stable, machine-readable integration API for editors and other frontends.
//!
//! This module contains no terminal-specific code. It can be embedded directly
//! as a Rust library or exposed through the newline-delimited JSON protocol
//! implemented by [`serve_jsonl`].

use std::{
    collections::HashMap,
    io::{BufRead, Write},
};

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

const MAX_RECIPE_EXECUTIONS: usize = 1_000_000;

fn flow_name(name: &str) -> String {
    name.chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn argument<'a>(step: &'a RecipeStep, index: usize, default: &'a str) -> &'a str {
    step.args.get(index).map(String::as_str).unwrap_or(default)
}

fn bool_argument(step: &RecipeStep, index: usize, default: bool) -> Result<bool, String> {
    let raw = step
        .args
        .get(index)
        .map(String::as_str)
        .unwrap_or(if default { "true" } else { "false" });
    let raw = raw.strip_prefix("bool:").unwrap_or(raw);
    match raw.to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => Err(format!("invalid boolean argument '{raw}'")),
    }
}

fn usize_argument(step: &RecipeStep, index: usize, default: usize) -> Result<usize, String> {
    let raw = step.args.get(index).map(String::as_str).unwrap_or("");
    if raw.is_empty() {
        return Ok(default);
    }
    raw.strip_prefix("num:")
        .unwrap_or(raw)
        .parse::<usize>()
        .map_err(|error| format!("invalid integer argument '{raw}': {error}"))
}

fn unescape_delimiter(value: &str) -> Vec<u8> {
    let mut output = Vec::with_capacity(value.len());
    let mut characters = value.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            let mut encoded = [0; 4];
            output.extend_from_slice(character.encode_utf8(&mut encoded).as_bytes());
            continue;
        }
        match characters.next() {
            Some('n') => output.push(b'\n'),
            Some('r') => output.push(b'\r'),
            Some('t') => output.push(b'\t'),
            Some('0') => output.push(0),
            Some('\\') => output.push(b'\\'),
            Some(other) => {
                output.push(b'\\');
                let mut encoded = [0; 4];
                output.extend_from_slice(other.encode_utf8(&mut encoded).as_bytes());
            }
            None => output.push(b'\\'),
        }
    }
    output
}

fn split_bytes<'a>(input: &'a [u8], delimiter: &[u8]) -> Result<Vec<&'a [u8]>, String> {
    if delimiter.is_empty() {
        return Err("Fork split delimiter must not be empty".to_string());
    }
    let mut parts = Vec::new();
    let mut start = 0;
    while let Some(offset) = input[start..]
        .windows(delimiter.len())
        .position(|window| window == delimiter)
    {
        let end = start + offset;
        parts.push(&input[start..end]);
        start = end + delimiter.len();
    }
    parts.push(&input[start..]);
    Ok(parts)
}

fn expand_registers(value: &str, registers: &[String]) -> String {
    let matcher = regex::Regex::new(r"\$R(\d+)").expect("static register regex");
    matcher
        .replace_all(value, |captures: &regex::Captures<'_>| {
            captures[1]
                .parse::<usize>()
                .ok()
                .and_then(|index| registers.get(index))
                .cloned()
                .unwrap_or_else(|| captures[0].to_string())
        })
        .into_owned()
}

struct RecipeEngine<'a> {
    recipe: &'a [RecipeStep],
    labels: HashMap<String, usize>,
    executions: usize,
}

impl<'a> RecipeEngine<'a> {
    fn new(recipe: &'a [RecipeStep]) -> Result<Self, String> {
        let mut labels = HashMap::new();
        for (index, step) in recipe.iter().enumerate() {
            if flow_name(&step.op) == "label" {
                let name = argument(step, 0, "").to_string();
                if name.is_empty() {
                    return Err(format!("step {} (Label): label name is empty", index + 1));
                }
                if labels.insert(name.clone(), index).is_some() {
                    return Err(format!(
                        "step {} (Label): duplicate label '{name}'",
                        index + 1
                    ));
                }
            }
        }
        Ok(Self {
            recipe,
            labels,
            executions: 0,
        })
    }

    fn matching_merge(&self, start: usize, end: usize) -> Result<usize, String> {
        let mut depth = 0usize;
        for index in start + 1..end {
            match flow_name(&self.recipe[index].op).as_str() {
                "fork" | "subsection" => depth += 1,
                "merge" if depth == 0 => return Ok(index),
                "merge" => depth -= 1,
                _ => {}
            }
        }
        Err(format!(
            "step {} ({}): missing matching Merge",
            start + 1,
            self.recipe[start].op
        ))
    }

    fn regex(
        &self,
        pattern: &str,
        case_insensitive: bool,
        multiline: bool,
        dot_matches_new_line: bool,
    ) -> Result<regex::bytes::Regex, String> {
        regex::bytes::RegexBuilder::new(pattern)
            .case_insensitive(case_insensitive)
            .multi_line(multiline)
            .dot_matches_new_line(dot_matches_new_line)
            .unicode(false)
            .build()
            .map_err(|error| format!("invalid regular expression: {error}"))
    }

    fn run_range(
        &mut self,
        start: usize,
        end: usize,
        mut current: Vec<u8>,
        registers: &mut Vec<String>,
    ) -> Result<Vec<u8>, String> {
        let mut pc = start;
        let mut jump_counts: HashMap<usize, usize> = HashMap::new();
        while pc < end {
            self.executions += 1;
            if self.executions > MAX_RECIPE_EXECUTIONS {
                return Err(format!(
                    "recipe exceeded the safety limit of {MAX_RECIPE_EXECUTIONS} step executions"
                ));
            }

            let step = &self.recipe[pc];
            let kind = flow_name(&step.op);
            let result = match kind.as_str() {
                "fork" => {
                    let merge = self.matching_merge(pc, end)?;
                    let split = unescape_delimiter(argument(step, 0, "\\n"));
                    let join = unescape_delimiter(argument(step, 1, "\\n"));
                    let ignore_errors = bool_argument(step, 2, false)
                        .map_err(|error| format!("step {} (Fork): {error}", pc + 1))?;
                    let branches = split_bytes(&current, &split)
                        .map_err(|error| format!("step {} (Fork): {error}", pc + 1))?;
                    let mut joined = Vec::new();
                    for (branch_index, branch) in branches.iter().enumerate() {
                        if branch_index > 0 {
                            joined.extend_from_slice(&join);
                        }
                        let mut branch_registers = registers.clone();
                        match self.run_range(pc + 1, merge, branch.to_vec(), &mut branch_registers)
                        {
                            Ok(output) => joined.extend_from_slice(&output),
                            Err(_) if ignore_errors => joined.extend_from_slice(branch),
                            Err(error) => {
                                return Err(format!(
                                    "step {} (Fork), branch {}: {error}",
                                    pc + 1,
                                    branch_index + 1
                                ));
                            }
                        }
                    }
                    pc = merge + 1;
                    Some(joined)
                }
                "subsection" => {
                    let merge = self.matching_merge(pc, end)?;
                    let pattern = expand_registers(argument(step, 0, ""), registers);
                    let case_sensitive = bool_argument(step, 1, true)
                        .map_err(|error| format!("step {} (Subsection): {error}", pc + 1))?;
                    let global = bool_argument(step, 2, true)
                        .map_err(|error| format!("step {} (Subsection): {error}", pc + 1))?;
                    let ignore_errors = bool_argument(step, 3, false)
                        .map_err(|error| format!("step {} (Subsection): {error}", pc + 1))?;
                    let matcher = self
                        .regex(&pattern, !case_sensitive, true, true)
                        .map_err(|error| format!("step {} (Subsection): {error}", pc + 1))?;
                    let matches: Vec<_> = matcher.find_iter(&current).collect();
                    let mut output = Vec::with_capacity(current.len());
                    let mut offset = 0;
                    for (match_index, found) in matches.iter().enumerate() {
                        if !global && match_index > 0 {
                            break;
                        }
                        output.extend_from_slice(&current[offset..found.start()]);
                        let original = &current[found.start()..found.end()];
                        let mut section_registers = registers.clone();
                        match self.run_range(
                            pc + 1,
                            merge,
                            original.to_vec(),
                            &mut section_registers,
                        ) {
                            Ok(section) => output.extend_from_slice(&section),
                            Err(_) if ignore_errors => output.extend_from_slice(original),
                            Err(error) => {
                                return Err(format!(
                                    "step {} (Subsection), match {}: {error}",
                                    pc + 1,
                                    match_index + 1
                                ));
                            }
                        }
                        offset = found.end();
                    }
                    output.extend_from_slice(&current[offset..]);
                    pc = merge + 1;
                    Some(output)
                }
                "register" => {
                    let pattern = expand_registers(argument(step, 0, "([\\s\\S]*)"), registers);
                    let case_insensitive = bool_argument(step, 1, true)
                        .map_err(|error| format!("step {} (Register): {error}", pc + 1))?;
                    let multiline = bool_argument(step, 2, false)
                        .map_err(|error| format!("step {} (Register): {error}", pc + 1))?;
                    let dot_all = bool_argument(step, 3, false)
                        .map_err(|error| format!("step {} (Register): {error}", pc + 1))?;
                    let matcher = self
                        .regex(&pattern, case_insensitive, multiline, dot_all)
                        .map_err(|error| format!("step {} (Register): {error}", pc + 1))?;
                    registers.clear();
                    if let Some(captures) = matcher.captures(&current) {
                        let first = if captures.len() > 1 { 1 } else { 0 };
                        registers.extend((first..captures.len()).map(|index| {
                            captures
                                .get(index)
                                .map(|value| String::from_utf8_lossy(value.as_bytes()).into_owned())
                                .unwrap_or_default()
                        }));
                    }
                    pc += 1;
                    None
                }
                "jump" | "conditionaljump" => {
                    let (should_jump, label_index, maximum_index) = if kind == "jump" {
                        (true, 0, 1)
                    } else {
                        let pattern = expand_registers(argument(step, 0, ""), registers);
                        let invert = bool_argument(step, 1, false).map_err(|error| {
                            format!("step {} (Conditional Jump): {error}", pc + 1)
                        })?;
                        let matches = self
                            .regex(&pattern, false, false, false)
                            .map_err(|error| {
                                format!("step {} (Conditional Jump): {error}", pc + 1)
                            })?
                            .is_match(&current);
                        (matches != invert, 2, 3)
                    };
                    if !should_jump {
                        pc += 1;
                        None
                    } else {
                        let label = expand_registers(argument(step, label_index, ""), registers);
                        let target = *self.labels.get(&label).ok_or_else(|| {
                            format!("step {} ({}): unknown label '{label}'", pc + 1, step.op)
                        })?;
                        if target < start || target >= end {
                            return Err(format!(
                                "step {} ({}): label '{label}' is outside the current branch",
                                pc + 1,
                                step.op
                            ));
                        }
                        if target <= pc {
                            let maximum =
                                usize_argument(step, maximum_index, 10).map_err(|error| {
                                    format!("step {} ({}): {error}", pc + 1, step.op)
                                })?;
                            let count = jump_counts.entry(pc).or_default();
                            if *count >= maximum {
                                pc += 1;
                                None
                            } else {
                                *count += 1;
                                pc = target;
                                None
                            }
                        } else {
                            pc = target;
                            None
                        }
                    }
                }
                "label" | "merge" => {
                    pc += 1;
                    None
                }
                _ => {
                    let args = step
                        .args
                        .iter()
                        .map(|value| expand_registers(value, registers))
                        .collect::<Vec<_>>();
                    let output = runtime::run_operation(&step.op, current.clone(), &args)
                        .map_err(|error| format!("step {} ({}): {error}", pc + 1, step.op))?;
                    pc += 1;
                    Some(output)
                }
            };
            if let Some(output) = result {
                current = output;
            }
        }
        Ok(current)
    }
}

/// Execute an arbitrary recipe with flow control and register expansion.
///
/// Ordinary operations run from left to right. `Fork`/`Merge`, `Subsection`,
/// `Register`, `Label`, `Jump`, and `Conditional Jump` are interpreted by this
/// shared engine so library, CLI, and JSONL clients have identical semantics.
pub fn bake(input: Vec<u8>, recipe: &[RecipeStep]) -> Result<ExecutionResult, String> {
    let mut engine = RecipeEngine::new(recipe)?;
    let output = engine.run_range(0, recipe.len(), input, &mut Vec::new())?;
    Ok(ExecutionResult::from_bytes(output))
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
