use super::*;
use crate::{execution, runtime, ArgKind};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

#[derive(Clone, Default)]
pub struct ExecutionControl {
    pub cancelled: Option<Arc<AtomicBool>>,
    pub timeout: Option<Duration>,
    pub observer: Option<Arc<dyn Fn(GraphEvent) + Send + Sync>>,
}
impl std::fmt::Debug for ExecutionControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExecutionControl")
            .field("cancelled", &self.cancelled)
            .field("timeout", &self.timeout)
            .field("observer", &self.observer.is_some())
            .finish()
    }
}
impl ExecutionControl {
    fn emit(&self, event: GraphEvent) {
        if let Some(observer) = &self.observer {
            observer(event);
        }
    }
    fn check(&self, started: Instant) -> Result<(), GraphError> {
        if self
            .cancelled
            .as_ref()
            .is_some_and(|flag| flag.load(Ordering::Acquire))
        {
            return Err(GraphError::new(
                GraphErrorKind::Cancelled,
                None,
                "graph cancelled",
            ));
        }
        if self.timeout.is_some_and(|limit| started.elapsed() >= limit) {
            return Err(GraphError::new(
                GraphErrorKind::Timeout,
                None,
                "graph deadline exceeded",
            ));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum NodeStatus {
    Succeeded,
    Failed,
    Blocked,
    Disconnected,
    Skipped,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NodeResult {
    pub node_id: String,
    pub status: NodeStatus,
    pub output_bytes: usize,
    pub elapsed_ms: f64,
    pub error: Option<GraphError>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GraphEvent {
    Started { node_id: String },
    Finished { node: NodeResult },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OutputResult {
    pub node_id: String,
    pub name: String,
    /// Present only for complete successful output; empty output is Some("").
    pub output_base64: Option<String>,
    pub output_bytes: usize,
    pub error: Option<GraphError>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GraphResult {
    pub outputs: Vec<OutputResult>,
    /// Metadata only. No intermediate payloads or argument values retained.
    pub nodes: Vec<NodeResult>,
}
#[derive(Clone)]
enum Value {
    Bytes(Arc<[u8]>),
    Argument { kind: ArgKind, raw: Arc<str> },
}
impl Value {
    fn len(&self) -> usize {
        match self {
            Self::Bytes(v) => v.len(),
            Self::Argument { raw, .. } => raw.len(),
        }
    }
    fn bytes(&self) -> Result<&[u8], GraphError> {
        match self {
            Self::Bytes(v) => Ok(v),
            _ => Err(GraphError::new(
                GraphErrorKind::InvalidPort,
                None,
                "expected bytes",
            )),
        }
    }
}

/// Execute each required node once, with immutable fan-out and stable ordering.
/// Memory accounting conservatively charges each live alias. Node limits are
/// checked after registered operations return; they are not an OS memory limit.
pub fn execute(
    graph: &Graph,
    inputs: BTreeMap<String, Vec<u8>>,
    selected_outputs: &[String],
    policy: &GraphPolicy,
    control: &ExecutionControl,
) -> Result<GraphResult, GraphError> {
    let started = Instant::now();
    control.check(started)?;
    let plan = validate(graph, policy)?;
    let mut active = BTreeSet::new();
    let mut stack = Vec::new();
    let names: BTreeSet<&str> = selected_outputs.iter().map(String::as_str).collect();
    if names.len() != selected_outputs.len() {
        return Err(GraphError::new(
            GraphErrorKind::Validation,
            None,
            "duplicate selected output",
        ));
    }
    for (i, node) in graph.nodes.iter().enumerate() {
        if let NodeKind::Output { name } = &node.kind {
            if selected_outputs.is_empty() || names.contains(name.as_str()) {
                stack.push(i);
            }
        }
    }
    if stack.is_empty() {
        return Err(GraphError::new(
            GraphErrorKind::Validation,
            None,
            "execution requires at least one output",
        ));
    }
    if stack.len() < names.len() {
        return Err(GraphError::new(
            GraphErrorKind::Validation,
            None,
            "unknown selected output",
        ));
    }
    while let Some(i) = stack.pop() {
        if active.insert(i) {
            stack.extend(
                plan.inbound
                    .iter()
                    .filter(|((target, _), _)| *target == i)
                    .map(|(_, source)| *source),
            );
        }
    }
    let valid_inputs: BTreeSet<&str> = graph
        .nodes
        .iter()
        .filter_map(|node| match &node.kind {
            NodeKind::Input { name } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    let input_bytes = inputs
        .values()
        .try_fold(0usize, |n, v| n.checked_add(v.len()));
    if input_bytes.is_none_or(|n| n > policy.limits.max_input_bytes) {
        return Err(GraphError::new(
            GraphErrorKind::Limit,
            None,
            "total input byte limit",
        ));
    }
    if inputs
        .keys()
        .any(|name| !valid_inputs.contains(name.as_str()))
    {
        return Err(GraphError::new(
            GraphErrorKind::Input,
            None,
            "unknown input binding",
        ));
    }
    let mut inputs: BTreeMap<String, Arc<[u8]>> =
        inputs.into_iter().map(|(k, v)| (k, Arc::from(v))).collect();
    let mut remaining = vec![0usize; graph.nodes.len()];
    for ((target, _), source) in &plan.inbound {
        if active.contains(target) {
            remaining[*source] += 1;
        }
    }
    let mut values: BTreeMap<usize, Result<Value, GraphError>> = BTreeMap::new();
    let mut result = GraphResult {
        nodes: Vec::new(),
        outputs: Vec::new(),
    };
    let mut live = inputs.values().map(|v| v.len()).sum::<usize>();
    if live > policy.limits.max_live_bytes {
        return Err(GraphError::new(
            GraphErrorKind::Limit,
            None,
            "live byte limit",
        ));
    }
    let mut output_total = 0usize;
    for &index in &plan.order {
        control.check(started)?;
        let node = &graph.nodes[index];
        if !active.contains(&index) {
            let finished = NodeResult {
                node_id: node.id.clone(),
                status: if plan.disconnected.contains(&node.id) {
                    NodeStatus::Disconnected
                } else {
                    NodeStatus::Skipped
                },
                output_bytes: 0,
                elapsed_ms: 0.0,
                error: None,
            };
            control.emit(GraphEvent::Finished {
                node: finished.clone(),
            });
            result.nodes.push(finished);
            continue;
        }
        let node_started = Instant::now();
        let incoming: BTreeMap<&str, &Value> = plan
            .inbound
            .iter()
            .filter(|((i, _), _)| *i == index)
            .filter_map(|((_, port), source)| {
                values
                    .get(source)
                    .and_then(|v| v.as_ref().ok())
                    .map(|v| (port.as_str(), v))
            })
            .collect();
        let failed_dependency = plan
            .inbound
            .iter()
            .any(|((i, _), source)| *i == index && values.get(source).is_some_and(Result::is_err));
        let bypassed = matches!(&node.kind, NodeKind::Operation { enabled: false, .. });
        let mut value = if failed_dependency {
            Err(GraphError::new(
                GraphErrorKind::Dependency,
                Some(&node.id),
                "upstream dependency failed",
            ))
        } else {
            if !bypassed {
                control.emit(GraphEvent::Started {
                    node_id: node.id.clone(),
                });
            }
            run_node(node, &incoming, &mut inputs, policy)
        };
        control.check(started)?;
        // Released input ownership was replaced by the node's Arc.
        if matches!(node.kind, NodeKind::Input { .. }) {
            if let Ok(v) = &value {
                live = live.saturating_sub(v.len());
            }
        }
        if value
            .as_ref()
            .is_ok_and(|v| v.len() > policy.limits.max_node_bytes)
        {
            value = Err(GraphError::new(
                GraphErrorKind::Limit,
                Some(&node.id),
                "node byte limit",
            ));
        }
        if value.as_ref().is_ok_and(|v| {
            live.checked_add(v.len())
                .is_none_or(|n| n > policy.limits.max_live_bytes)
        }) {
            value = Err(GraphError::new(
                GraphErrorKind::Limit,
                Some(&node.id),
                "live byte limit",
            ));
        }
        if let NodeKind::Output { name } = &node.kind {
            if value.as_ref().is_ok_and(|v| {
                output_total
                    .checked_add(v.len())
                    .is_none_or(|n| n > policy.limits.max_output_bytes)
            }) {
                value = Err(GraphError::new(
                    GraphErrorKind::OutputLimit,
                    Some(&node.id),
                    "total output byte limit",
                ));
            }
            let (encoded, len, error) = match &value {
                Ok(v) => {
                    output_total += v.len();
                    (Some(STANDARD.encode(v.bytes()?)), v.len(), None)
                }
                Err(e) => (None, 0, Some(e.clone())),
            };
            result.outputs.push(OutputResult {
                node_id: node.id.clone(),
                name: name.clone(),
                output_base64: encoded,
                output_bytes: len,
                error,
            });
        }
        let (status, len, error) = match &value {
            Ok(v) => (
                if bypassed {
                    NodeStatus::Skipped
                } else {
                    NodeStatus::Succeeded
                },
                v.len(),
                None,
            ),
            Err(e) => (
                if e.kind == GraphErrorKind::Dependency {
                    NodeStatus::Blocked
                } else {
                    NodeStatus::Failed
                },
                0,
                Some(e.clone()),
            ),
        };
        let finished = NodeResult {
            node_id: node.id.clone(),
            status,
            output_bytes: len,
            elapsed_ms: node_started.elapsed().as_secs_f64() * 1000.0,
            error,
        };
        control.emit(GraphEvent::Finished {
            node: finished.clone(),
        });
        result.nodes.push(finished);
        live += len;
        values.insert(index, value);
        // Each target port counts as one reference, including multiple ports
        // consuming the same upstream value.
        for ((target, _), source) in &plan.inbound {
            if *target == index {
                remaining[*source] -= 1;
                if remaining[*source] == 0 {
                    if let Some(Ok(v)) = values.remove(source) {
                        live = live.saturating_sub(v.len());
                    }
                }
            }
        }
        if remaining[index] == 0 {
            if let Some(Ok(v)) = values.remove(&index) {
                live = live.saturating_sub(v.len());
            }
        }
    }
    Ok(result)
}

fn run_node(
    node: &Node,
    incoming: &BTreeMap<&str, &Value>,
    inputs: &mut BTreeMap<String, Arc<[u8]>>,
    policy: &GraphPolicy,
) -> Result<Value, GraphError> {
    let fail = |kind, message| GraphError::new(kind, Some(&node.id), message);
    let data = |port: &str| -> Result<&[u8], GraphError> {
        incoming
            .get(port)
            .ok_or_else(|| fail(GraphErrorKind::Input, "missing input"))?
            .bytes()
    };
    match &node.kind {
        NodeKind::Input { name } => inputs
            .remove(name)
            .map(Value::Bytes)
            .ok_or_else(|| fail(GraphErrorKind::Input, "named input not supplied")),
        NodeKind::Output { .. } => incoming
            .get("data")
            .map(|v| (*v).clone())
            .ok_or_else(|| fail(GraphErrorKind::Input, "missing output data")),
        NodeKind::Argument { kind } => {
            if matches!(
                kind,
                ArgKind::Bytes | ArgKind::HexBytes | ArgKind::Base64Bytes
            ) {
                return incoming
                    .get("data")
                    .map(|value| (*value).clone())
                    .ok_or_else(|| fail(GraphErrorKind::Input, "missing adapter data"));
            }
            let text = std::str::from_utf8(data("data")?).map_err(|_| {
                fail(
                    GraphErrorKind::InvalidArgument,
                    "argument requires valid UTF-8",
                )
            })?;
            crate::runtime::validate_argument_kind(text, *kind).map_err(|_| {
                fail(
                    GraphErrorKind::InvalidArgument,
                    "argument adapter value has invalid syntax",
                )
            })?;
            Ok(Value::Argument {
                kind: *kind,
                raw: Arc::from(text),
            })
        }
        NodeKind::Operation {
            operation,
            args,
            enabled,
        } => {
            if !enabled {
                return incoming.get("data").map(|v| (*v).clone()).ok_or_else(|| {
                    fail(GraphErrorKind::Input, "disabled operation requires data")
                });
            }
            let info = super::validation::operation_info(operation, &node.id, policy)?;
            let mut raw = Vec::new();
            for schema in info.args {
                let port = format!("arg:{}", runtime::canonical_identifier(schema.name));
                let value = match incoming.get(port.as_str()) {
                    Some(Value::Bytes(bytes)) => match schema.kind {
                        ArgKind::Bytes | ArgKind::HexBytes => format!("hex:{}", hex::encode(bytes)),
                        ArgKind::Base64Bytes => format!("base64:{}", STANDARD.encode(bytes)),
                        _ => {
                            return Err(fail(
                                GraphErrorKind::InvalidArgument,
                                "incompatible argument binding",
                            ))
                        }
                    },
                    Some(Value::Argument { kind, raw }) if *kind == schema.kind => raw.to_string(),
                    Some(_) => {
                        return Err(fail(
                            GraphErrorKind::InvalidArgument,
                            "incompatible argument kind",
                        ))
                    }
                    None => args
                        .get(schema.name)
                        .cloned()
                        .unwrap_or_else(|| schema.default_value.into()),
                };
                runtime::validate_argument(&value, schema).map_err(|_| {
                    fail(
                        GraphErrorKind::InvalidArgument,
                        "argument failed schema validation",
                    )
                })?;
                raw.push(value);
            }
            let bytes = incoming
                .get("data")
                .map(|v| v.bytes())
                .transpose()?
                .unwrap_or(&[]);
            runtime::run_operation(operation, bytes.to_vec(), &raw)
                .map(|v| Value::Bytes(v.into()))
                .map_err(|_| fail(GraphErrorKind::Operation, "registered operation failed"))
        }
        NodeKind::Recipe { steps } => execution::execute(execution::ExecutionRequest {
            input: data("data")?.to_vec(),
            input_supplied: true,
            recipe: steps.clone().into(),
            variables: execution::VariableContext::default(),
            options: execution::ExecutionOptions {
                trace: false,
                max_steps: policy.limits.max_recipe_steps,
                max_output_bytes: Some(policy.limits.max_node_bytes),
            },
        })
        .map(|out| Value::Bytes(out.output.into()))
        .map_err(|_| fail(GraphErrorKind::Operation, "embedded recipe failed")),
        NodeKind::Combine { combine } => {
            let limit = policy.limits.max_node_bytes;
            let mut output = Vec::new();
            let mut append = |bytes: &[u8]| -> Result<(), GraphError> {
                if output.len().saturating_add(bytes.len()) > limit {
                    return Err(fail(GraphErrorKind::Limit, "combine byte limit"));
                }
                output.extend_from_slice(bytes);
                Ok(())
            };
            match combine {
                Combine::Concatenate { inputs } => {
                    for i in 0..*inputs {
                        append(data(&format!("input:{i}"))?)?;
                    }
                }
                Combine::JoinText { inputs, separator } => {
                    for i in 0..*inputs {
                        let bytes = data(&format!("input:{i}"))?;
                        std::str::from_utf8(bytes).map_err(|_| {
                            fail(GraphErrorKind::Input, "join requires valid UTF-8")
                        })?;
                        if i > 0 {
                            append(separator.as_bytes())?;
                        }
                        append(bytes)?;
                    }
                }
                Combine::Select { selected, .. } => {
                    return incoming
                        .get(format!("input:{selected}").as_str())
                        .map(|v| (*v).clone())
                        .ok_or_else(|| fail(GraphErrorKind::Input, "selected input absent"))
                }
                Combine::Compare => append(&[u8::from(data("left")? == data("right")?)])?,
                Combine::Xor => {
                    let left = data("left")?;
                    let right = data("right")?;
                    if left.len() != right.len() {
                        return Err(fail(GraphErrorKind::Input, "XOR requires equal lengths"));
                    }
                    if left.len() > limit {
                        return Err(fail(GraphErrorKind::Limit, "XOR byte limit"));
                    }
                    output = left.iter().zip(right).map(|(a, b)| a ^ b).collect();
                }
                Combine::Interleave => {
                    let left = data("left")?;
                    let right = data("right")?;
                    for i in 0..left.len().max(right.len()) {
                        if let Some(byte) = left.get(i) {
                            append(&[*byte])?;
                        }
                        if let Some(byte) = right.get(i) {
                            append(&[*byte])?;
                        }
                    }
                }
            }
            Ok(Value::Bytes(output.into()))
        }
    }
}
