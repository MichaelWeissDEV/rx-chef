use super::*;
use crate::{execution, runtime, ArgKind, Availability, SideEffect};
use std::collections::{BTreeMap, BTreeSet};
use unicode_normalization::UnicodeNormalization;

/// Stable lexical Kahn ordering. Indices refer to the original definition.
#[derive(Debug, Clone)]
pub struct GraphPlan {
    pub order: Vec<usize>,
    pub disconnected: Vec<String>,
    pub(crate) inbound: BTreeMap<(usize, String), usize>,
}

pub(crate) fn output_type(node: &Node) -> Option<PortType> {
    match node.kind {
        NodeKind::Output { .. } => None,
        NodeKind::Argument { kind } => Some(argument_type(kind)),
        _ => Some(PortType::Bytes),
    }
}
pub fn argument_type(kind: ArgKind) -> PortType {
    match kind {
        ArgKind::Bytes | ArgKind::HexBytes | ArgKind::Base64Bytes => PortType::Bytes,
        _ => PortType::Argument(kind),
    }
}

/// Registry IDs whose control semantics belong to the Recipe Engine.
pub fn operation_requires_recipe(id: &str) -> bool {
    matches!(
        id,
        "fork"
            | "merge"
            | "subsection"
            | "register"
            | "label"
            | "jump"
            | "conditional_jump"
            | "return"
    )
}

pub(crate) fn operation_info(
    operation: &str,
    node: &str,
    policy: &GraphPolicy,
) -> Result<runtime::OperationInfo, GraphError> {
    let info = runtime::operation_info(operation).map_err(|_| {
        GraphError::new(
            GraphErrorKind::UnknownOperation,
            Some(node),
            "unknown operation",
        )
    })?;
    if operation_requires_recipe(&info.id) {
        return Err(GraphError::new(
            GraphErrorKind::Validation,
            Some(node),
            "control flow requires a recipe node",
        ));
    }
    check_policy(&info, node, policy)?;
    Ok(info)
}
fn check_policy(
    info: &runtime::OperationInfo,
    node: &str,
    policy: &GraphPolicy,
) -> Result<(), GraphError> {
    if info.availability != Availability::Available {
        return Err(GraphError::new(
            GraphErrorKind::Unavailable,
            Some(node),
            "operation unavailable in this build",
        ));
    }
    if (!policy.allow_host_effects
        && info.side_effects.iter().any(|s| {
            matches!(
                s,
                SideEffect::Network
                    | SideEffect::FilesystemRead
                    | SideEffect::FilesystemWrite
                    | SideEffect::ExternalProcess
            )
        }))
        || policy
            .allowed_operations
            .as_ref()
            .is_some_and(|ids| !ids.contains(&info.id))
    {
        return Err(GraphError::new(
            GraphErrorKind::PolicyDenied,
            Some(node),
            "operation denied by policy",
        ));
    }
    Ok(())
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && !id.chars().any(char::is_control)
        && id.nfc().eq(id.chars())
}

/// Validate all components, including cycles and invalid ports in disconnected
/// editor nodes. Missing connections are allowed only outside output closures.
pub fn validate(graph: &Graph, policy: &GraphPolicy) -> Result<GraphPlan, GraphError> {
    let fail = |kind, node: Option<&str>, message| GraphError::new(kind, node, message);
    let limits = &policy.limits;
    if graph.version != GRAPH_VERSION {
        return Err(fail(
            GraphErrorKind::Version,
            None,
            "unsupported graph version",
        ));
    }
    if !valid_id(&graph.id) {
        return Err(fail(GraphErrorKind::Validation, None, "invalid graph ID"));
    }
    if graph.nodes.len() > limits.max_nodes || graph.edges.len() > limits.max_edges {
        return Err(fail(GraphErrorKind::Limit, None, "node or edge limit"));
    }
    // Count serialized size without allocating another full definition.
    struct Counter {
        bytes: usize,
        limit: usize,
    }
    impl std::io::Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.bytes = self.bytes.saturating_add(bytes.len());
            if self.bytes > self.limit {
                return Err(std::io::Error::other("definition byte limit"));
            }
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    if serde_json::to_writer(
        Counter {
            bytes: 0,
            limit: limits.max_definition_bytes,
        },
        graph,
    )
    .is_err()
    {
        return Err(fail(GraphErrorKind::Limit, None, "definition byte limit"));
    }
    let mut nodes = BTreeMap::new();
    let mut ids = BTreeSet::new();
    let mut inputs = BTreeSet::new();
    let mut outputs = BTreeSet::new();
    let mut ports = Vec::new();
    let mut required_ports = Vec::new();
    for (index, node) in graph.nodes.iter().enumerate() {
        if node.label.as_ref().is_some_and(|label| !valid_id(label)) {
            return Err(fail(
                GraphErrorKind::Validation,
                Some(&node.id),
                "invalid node label",
            ));
        }
        if !valid_id(&node.id) || !ids.insert(node.id.as_str()) {
            return Err(fail(
                GraphErrorKind::Validation,
                Some(&node.id),
                "invalid or duplicate node ID",
            ));
        }
        nodes.insert(node.id.clone(), index);
        let mut node_ports = BTreeMap::new();
        let mut required = Vec::new();
        let mut add = |name: String, kind: PortType, needed: bool| {
            if needed {
                required.push(name.clone());
            }
            node_ports.insert(name, kind);
        };
        match &node.kind {
            NodeKind::Input { name } => {
                if !valid_id(name) || !inputs.insert(name) {
                    return Err(fail(
                        GraphErrorKind::Validation,
                        Some(&node.id),
                        "invalid or duplicate input name",
                    ));
                }
            }
            NodeKind::Output { name } => {
                if !valid_id(name) || !outputs.insert(name) {
                    return Err(fail(
                        GraphErrorKind::Validation,
                        Some(&node.id),
                        "invalid or duplicate output name",
                    ));
                }
                add("data".into(), PortType::Bytes, true);
            }
            NodeKind::Operation {
                operation,
                args,
                enabled,
            } => {
                let info = operation_info(operation, &node.id, policy)?;
                if info.id != *operation {
                    return Err(fail(
                        GraphErrorKind::Validation,
                        Some(&node.id),
                        "use canonical registry operation ID",
                    ));
                }
                add(
                    "data".into(),
                    PortType::Bytes,
                    !*enabled || info.input_requirement == crate::InputRequirement::Required,
                );
                for name in args.keys() {
                    if !info.args.iter().any(|a| a.name == name) {
                        return Err(fail(
                            GraphErrorKind::InvalidArgument,
                            Some(&node.id),
                            "unknown argument name",
                        ));
                    }
                }
                let mut argument_ports = BTreeSet::new();
                for schema in info.args {
                    if !argument_ports.insert(runtime::canonical_identifier(schema.name)) {
                        return Err(fail(
                            GraphErrorKind::InvalidPort,
                            Some(&node.id),
                            "duplicate canonical argument port ID",
                        ));
                    }
                    if let Some(raw) = args.get(schema.name) {
                        runtime::validate_argument(raw, schema).map_err(|_| {
                            fail(
                                GraphErrorKind::InvalidArgument,
                                Some(&node.id),
                                "invalid literal argument",
                            )
                        })?;
                    }
                    add(
                        format!("arg:{}", runtime::canonical_identifier(schema.name)),
                        argument_type(schema.kind),
                        *enabled && schema.required && !args.contains_key(schema.name),
                    );
                }
            }
            NodeKind::Combine { combine } => {
                let count = match combine {
                    Combine::Concatenate { inputs }
                    | Combine::JoinText { inputs, .. }
                    | Combine::Select { inputs, .. } => *inputs,
                    _ => 2,
                };
                if count == 0 || count > limits.max_edges {
                    return Err(fail(
                        GraphErrorKind::Limit,
                        Some(&node.id),
                        "invalid combine port count",
                    ));
                }
                if let Combine::Select { selected, .. } = combine {
                    if *selected >= count {
                        return Err(fail(
                            GraphErrorKind::InvalidPort,
                            Some(&node.id),
                            "selected port out of range",
                        ));
                    }
                }
                for i in 0..count {
                    let name = match combine {
                        Combine::Compare | Combine::Xor | Combine::Interleave => {
                            if i == 0 {
                                "left".into()
                            } else {
                                "right".into()
                            }
                        }
                        _ => format!("input:{i}"),
                    };
                    add(name, PortType::Bytes, true);
                }
            }
            NodeKind::Argument { .. } => add("data".into(), PortType::Bytes, true),
            NodeKind::Recipe { steps } => {
                if steps.len() > limits.max_recipe_steps {
                    return Err(fail(
                        GraphErrorKind::Limit,
                        Some(&node.id),
                        "recipe step limit",
                    ));
                }
                for step in steps {
                    let info = runtime::operation_info(&step.op).map_err(|_| {
                        fail(
                            GraphErrorKind::UnknownOperation,
                            Some(&node.id),
                            "unknown recipe operation",
                        )
                    })?;
                    check_policy(&info, &node.id, policy)?;
                }
                execution::validate_recipe(steps, &execution::VariableContext::default()).map_err(
                    |_| {
                        fail(
                            GraphErrorKind::InvalidArgument,
                            Some(&node.id),
                            "invalid embedded recipe",
                        )
                    },
                )?;
                add("data".into(), PortType::Bytes, true);
            }
        }
        ports.push(node_ports);
        required_ports.push(required);
    }
    for (id, position) in &graph.presentation {
        if !nodes.contains_key(id)
            || !position.x.is_finite()
            || !position.y.is_finite()
            || position.x.abs() > 1_000_000.0
            || position.y.abs() > 1_000_000.0
        {
            return Err(fail(
                GraphErrorKind::Validation,
                None,
                "invalid presentation position",
            ));
        }
    }
    let mut inbound = BTreeMap::new();
    let mut outgoing = vec![Vec::new(); graph.nodes.len()];
    let mut degree = vec![0usize; graph.nodes.len()];
    for edge in &graph.edges {
        if !valid_id(&edge.id) || !ids.insert(edge.id.as_str()) {
            return Err(fail(
                GraphErrorKind::Validation,
                None,
                "invalid or duplicate edge ID",
            ));
        }
        let source = *nodes
            .get(&edge.source_node)
            .ok_or_else(|| fail(GraphErrorKind::Validation, None, "unknown source node"))?;
        let target = *nodes
            .get(&edge.target_node)
            .ok_or_else(|| fail(GraphErrorKind::Validation, None, "unknown target node"))?;
        let source_kind = output_type(&graph.nodes[source]);
        if edge.source_port != "data"
            || source_kind.is_none()
            || ports[target].get(&edge.target_port).copied() != source_kind
        {
            return Err(fail(
                GraphErrorKind::InvalidPort,
                Some(&edge.target_node),
                "unknown or incompatible port",
            ));
        }
        if let NodeKind::Operation { args, .. } = &graph.nodes[target].kind {
            if edge.target_port.strip_prefix("arg:").is_some_and(|name| {
                args.keys()
                    .any(|key| runtime::canonical_identifier(key) == name)
            }) {
                return Err(fail(
                    GraphErrorKind::InvalidArgument,
                    Some(&edge.target_node),
                    "argument has both a literal and an edge",
                ));
            }
        }
        if inbound
            .insert((target, edge.target_port.clone()), source)
            .is_some()
        {
            return Err(fail(
                GraphErrorKind::InvalidPort,
                Some(&edge.target_node),
                "duplicate connection to port",
            ));
        }
        outgoing[source].push(target);
        degree[target] += 1;
    }
    let mut ready: BTreeSet<&str> = nodes
        .iter()
        .filter(|(_, i)| degree[**i] == 0)
        .map(|(id, _)| id.as_str())
        .collect();
    let mut order = Vec::new();
    let mut depth = vec![1usize; graph.nodes.len()];
    while let Some(id) = ready.pop_first() {
        let index = nodes[id];
        if depth[index] > limits.max_depth {
            return Err(fail(GraphErrorKind::Limit, Some(id), "graph depth limit"));
        }
        order.push(index);
        for &target in &outgoing[index] {
            depth[target] = depth[target].max(depth[index].saturating_add(1));
            degree[target] -= 1;
            if degree[target] == 0 {
                ready.insert(&graph.nodes[target].id);
            }
        }
    }
    if order.len() != nodes.len() {
        return Err(fail(GraphErrorKind::Cycle, None, "graph contains a cycle"));
    }
    let mut active = BTreeSet::new();
    let mut stack: Vec<usize> = graph
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| matches!(n.kind, NodeKind::Output { .. }))
        .map(|(i, _)| i)
        .collect();
    while let Some(index) = stack.pop() {
        if active.insert(index) {
            stack.extend(
                inbound
                    .iter()
                    .filter(|((i, _), _)| *i == index)
                    .map(|(_, source)| *source),
            );
        }
    }
    for &index in &active {
        for name in &required_ports[index] {
            if !inbound.contains_key(&(index, name.clone())) {
                return Err(fail(
                    GraphErrorKind::InvalidPort,
                    Some(&graph.nodes[index].id),
                    "missing required input port",
                ));
            }
        }
    }
    let disconnected = order
        .iter()
        .filter(|i| !active.contains(i))
        .map(|i| graph.nodes[*i].id.clone())
        .collect();
    Ok(GraphPlan {
        order,
        disconnected,
        inbound,
    })
}
