use crate::{execution::RecipeStep, ArgKind};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GRAPH_VERSION: u32 = 1;

/// Definitions contain no payloads or local paths. Inputs are bound explicitly
/// at execution time. UI positions do not affect execution identities or order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Graph {
    pub version: u32,
    pub id: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    #[serde(default)]
    pub presentation: BTreeMap<String, Position>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Node {
    pub id: String,
    /// Optional display label; execution identity remains the stable ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(flatten)]
    pub kind: NodeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum NodeKind {
    Input {
        name: String,
    },
    Operation {
        operation: String,
        #[serde(default)]
        args: BTreeMap<String, String>,
        #[serde(default = "enabled")]
        enabled: bool,
    },
    Combine {
        combine: Combine,
    },
    /// Explicit strict UTF-8 interpretation as a registry argument kind.
    Argument {
        kind: ArgKind,
    },
    /// Retains Recipe Engine control-flow semantics. No graph references means
    /// recursion is unrepresentable; the first version supports one recipe level.
    Recipe {
        steps: Vec<RecipeStep>,
    },
    Output {
        name: String,
    },
}
fn enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Combine {
    /// Ports in order: input:0 .. input:(inputs-1).
    Concatenate { inputs: usize },
    /// Ports left/right; output is one byte: 1 for equal, 0 for unequal.
    Compare,
    /// Ports left/right. Unequal lengths fail, never truncate or pad.
    Xor,
    /// Strict UTF-8, ports input:0 .. input:(inputs-1).
    JoinText { inputs: usize, separator: String },
    /// All inputs are dependencies; selected is a zero-based port index.
    Select { inputs: usize, selected: usize },
    /// Alternate left/right bytes, then append remaining bytes in order.
    Interleave,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Edge {
    pub id: String,
    pub source_node: String,
    pub source_port: String,
    pub target_node: String,
    pub target_port: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "kind", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PortType {
    Bytes,
    Argument(ArgKind),
}

#[derive(Debug, Clone)]
pub struct GraphLimits {
    pub max_nodes: usize,
    pub max_edges: usize,
    pub max_depth: usize,
    pub max_input_bytes: usize,
    pub max_node_bytes: usize,
    pub max_output_bytes: usize,
    pub max_live_bytes: usize,
    pub max_definition_bytes: usize,
    pub max_recipe_steps: usize,
}
impl Default for GraphLimits {
    fn default() -> Self {
        Self {
            max_nodes: 1024,
            max_edges: 4096,
            max_depth: 256,
            max_input_bytes: 8 << 20,
            max_node_bytes: 8 << 20,
            max_output_bytes: 8 << 20,
            max_live_bytes: 32 << 20,
            max_definition_bytes: 4 << 20,
            max_recipe_steps: 100_000,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphPolicy {
    pub limits: GraphLimits,
    pub allow_host_effects: bool,
    pub allowed_operations: Option<BTreeSet<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GraphErrorKind {
    Validation,
    Version,
    Cycle,
    InvalidPort,
    InvalidArgument,
    UnknownOperation,
    Unavailable,
    Input,
    Operation,
    Dependency,
    Limit,
    OutputLimit,
    PolicyDenied,
    Cancelled,
    Timeout,
}
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
#[error("{kind:?} at {node:?}: {message}")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GraphError {
    pub kind: GraphErrorKind,
    pub node: Option<String>,
    pub message: String,
}
impl GraphError {
    pub(crate) fn new(
        kind: GraphErrorKind,
        node: Option<&str>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            node: node.map(str::to_owned),
            message: message.into(),
        }
    }
}

impl Graph {
    /// Parse a bounded definition. Unknown versions are rejected, never guessed.
    pub fn from_json(bytes: &[u8], policy: &GraphPolicy) -> Result<Self, GraphError> {
        if bytes.len() > policy.limits.max_definition_bytes {
            return Err(GraphError::new(
                GraphErrorKind::Limit,
                None,
                "definition byte limit",
            ));
        }
        let graph: Self = serde_json::from_slice(bytes)
            .map_err(|_| GraphError::new(GraphErrorKind::Validation, None, "invalid graph JSON"))?;
        super::validate(&graph, policy)?;
        Ok(graph)
    }

    /// Use one embedded recipe so control-flow semantics remain authoritative.
    pub fn from_recipe(id: String, steps: Vec<RecipeStep>) -> Self {
        Self {
            version: GRAPH_VERSION,
            id,
            nodes: vec![
                Node {
                    label: None,
                    id: "input".into(),
                    kind: NodeKind::Input {
                        name: "input".into(),
                    },
                },
                Node {
                    label: None,
                    id: "recipe".into(),
                    kind: NodeKind::Recipe { steps },
                },
                Node {
                    label: None,
                    id: "output".into(),
                    kind: NodeKind::Output {
                        name: "output".into(),
                    },
                },
            ],
            edges: vec![
                Edge {
                    id: "input-recipe".into(),
                    source_node: "input".into(),
                    source_port: "data".into(),
                    target_node: "recipe".into(),
                    target_port: "data".into(),
                },
                Edge {
                    id: "recipe-output".into(),
                    source_node: "recipe".into(),
                    source_port: "data".into(),
                    target_node: "output".into(),
                    target_port: "data".into(),
                },
            ],
            presentation: BTreeMap::new(),
        }
    }
}

impl Graph {
    /// Convert only a complete single-input/single-output chain. Detached nodes,
    /// additional ports, combine nodes and branches are never discarded.
    pub fn to_recipe(&self, policy: &GraphPolicy) -> Result<Vec<RecipeStep>, GraphError> {
        super::validate(self, policy)?;
        let fail = || {
            GraphError::new(
                GraphErrorKind::Validation,
                None,
                "graph is not one linear recipe",
            )
        };
        let inputs: Vec<_> = self
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, NodeKind::Input { .. }))
            .collect();
        let outputs = self
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, NodeKind::Output { .. }))
            .count();
        if inputs.len() != 1 || outputs != 1 {
            return Err(fail());
        }
        let mut current = inputs[0];
        let mut visited = BTreeSet::new();
        let mut steps = Vec::new();
        loop {
            if !visited.insert(current.id.as_str()) {
                return Err(fail());
            }
            match &current.kind {
                NodeKind::Input { .. } => {}
                NodeKind::Recipe { steps: inner } => steps.extend(inner.clone()),
                NodeKind::Operation {
                    operation,
                    args,
                    enabled,
                } => {
                    if *enabled {
                        let named = args
                            .iter()
                            .map(|(k, v)| format!("{k}={v}"))
                            .collect::<Vec<_>>();
                        let args = crate::runtime::bind_arguments(operation, &named, &[])
                            .map_err(|_| fail())?;
                        steps.push(RecipeStep {
                            op: operation.clone(),
                            args,
                        });
                    }
                }
                NodeKind::Output { .. } => break,
                _ => return Err(fail()),
            }
            let edges = self
                .edges
                .iter()
                .filter(|e| e.source_node == current.id)
                .collect::<Vec<_>>();
            if edges.len() != 1 || edges[0].source_port != "data" || edges[0].target_port != "data"
            {
                return Err(fail());
            }
            current = self
                .nodes
                .iter()
                .find(|n| n.id == edges[0].target_node)
                .ok_or_else(fail)?;
        }
        if visited.len() != self.nodes.len()
            || self.edges.len() != self.nodes.len().saturating_sub(1)
        {
            return Err(fail());
        }
        Ok(steps)
    }
}
