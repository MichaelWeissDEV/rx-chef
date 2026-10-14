//! Exact named graph bindings shared by local, REST and supervised job clients.
use crate::{ClientError, ErrorKind, LocalEngineClient};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rxchef::graph::{
    self, Graph, GraphError, GraphErrorKind, GraphLimits, GraphPolicy, GraphResult,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GraphRequest {
    pub graph: Graph,
    #[serde(default)]
    pub inputs_base64: BTreeMap<String, String>,
    /// Empty means all named outputs, otherwise exact output names.
    #[serde(default)]
    pub outputs: Vec<String>,
}
impl From<GraphError> for ClientError {
    fn from(error: GraphError) -> Self {
        let kind = match error.kind {
            GraphErrorKind::UnknownOperation => ErrorKind::UnknownOperation,
            GraphErrorKind::Unavailable => ErrorKind::Unavailable,
            GraphErrorKind::PolicyDenied => ErrorKind::PolicyDenied,
            GraphErrorKind::Limit | GraphErrorKind::OutputLimit => ErrorKind::LimitExceeded,
            GraphErrorKind::Cancelled => ErrorKind::Cancelled,
            GraphErrorKind::Timeout => ErrorKind::Timeout,
            GraphErrorKind::Input => ErrorKind::InvalidInput,
            GraphErrorKind::InvalidArgument => ErrorKind::InvalidArgument,
            GraphErrorKind::Operation | GraphErrorKind::Dependency => ErrorKind::Operation,
            _ => ErrorKind::InvalidRecipe,
        };
        Self::new(kind, error.to_string())
    }
}
impl LocalEngineClient {
    fn graph_policy(&self) -> GraphPolicy {
        let defaults = GraphLimits::default();
        GraphPolicy {
            limits: GraphLimits {
                max_nodes: defaults.max_nodes.min(self.policy.max_steps),
                max_input_bytes: self.policy.max_input_bytes,
                max_node_bytes: self.policy.max_output_bytes,
                max_output_bytes: self.policy.max_output_bytes,
                max_recipe_steps: self.policy.max_steps,
                ..defaults
            },
            allow_host_effects: self.policy.allow_host_effects,
            allowed_operations: self.policy.allowed_operations.clone(),
        }
    }
    fn graph_inputs(
        &self,
        request: &GraphRequest,
    ) -> Result<BTreeMap<String, Vec<u8>>, ClientError> {
        let input_names: std::collections::BTreeSet<&str> = request
            .graph
            .nodes
            .iter()
            .filter_map(|node| {
                if let graph::NodeKind::Input { name } = &node.kind {
                    Some(name.as_str())
                } else {
                    None
                }
            })
            .collect();
        let output_names: std::collections::BTreeSet<&str> = request
            .graph
            .nodes
            .iter()
            .filter_map(|node| {
                if let graph::NodeKind::Output { name } = &node.kind {
                    Some(name.as_str())
                } else {
                    None
                }
            })
            .collect();
        if request
            .inputs_base64
            .keys()
            .any(|name| !input_names.contains(name.as_str()))
        {
            return Err(ClientError::new(
                ErrorKind::InvalidInput,
                "unknown graph input binding",
            ));
        }
        let selected: std::collections::BTreeSet<&str> =
            request.outputs.iter().map(String::as_str).collect();
        if selected.len() != request.outputs.len()
            || selected.iter().any(|name| !output_names.contains(name))
        {
            return Err(ClientError::new(
                ErrorKind::InvalidInput,
                "invalid graph output selection",
            ));
        }
        let max_encoded = self
            .policy
            .max_input_bytes
            .saturating_add(2)
            .saturating_div(3)
            .saturating_mul(4);
        let mut encoded_total = 0usize;
        let mut byte_total = 0usize;
        let mut inputs = BTreeMap::new();
        for (name, raw) in &request.inputs_base64 {
            // Allow each field's Base64 padding while still bounding total work.
            encoded_total = encoded_total.saturating_add(raw.len());
            if encoded_total
                > max_encoded.saturating_add(request.inputs_base64.len().saturating_mul(4))
            {
                return Err(ClientError::new(
                    ErrorKind::LimitExceeded,
                    "total graph input byte limit",
                ));
            }
            let bytes = self.decode_input(raw)?;
            byte_total = byte_total.saturating_add(bytes.len());
            if byte_total > self.policy.max_input_bytes {
                return Err(ClientError::new(
                    ErrorKind::LimitExceeded,
                    "total graph input byte limit",
                ));
            }
            inputs.insert(name.clone(), bytes);
        }
        Ok(inputs)
    }
    pub(crate) fn validate_graph_request(&self, request: &GraphRequest) -> Result<(), ClientError> {
        graph::validate(&request.graph, &self.graph_policy())?;
        self.graph_inputs(request)?;
        Ok(())
    }
    pub(crate) fn execute_graph_request(
        &self,
        request: GraphRequest,
    ) -> Result<GraphResult, ClientError> {
        self.execute_graph_observed(request, None)
    }
    pub fn execute_graph_observed(
        &self,
        request: GraphRequest,
        observer: Option<std::sync::Arc<dyn Fn(graph::GraphEvent) + Send + Sync>>,
    ) -> Result<GraphResult, ClientError> {
        graph::validate(&request.graph, &self.graph_policy())?;
        Ok(graph::execute(
            &request.graph,
            self.graph_inputs(&request)?,
            &request.outputs,
            &self.graph_policy(),
            &graph::ExecutionControl {
                observer,
                ..Default::default()
            },
        )?)
    }
}
/// Validate complete graph output payloads at a transport boundary.
pub fn graph_output_bytes(output: &graph::OutputResult) -> Result<Vec<u8>, ClientError> {
    if let Some(error) = &output.error {
        return Err(error.clone().into());
    }
    let raw = output
        .output_base64
        .as_ref()
        .ok_or_else(|| ClientError::new(ErrorKind::Protocol, "missing graph output payload"))?;
    let bytes = STANDARD
        .decode(raw)
        .map_err(|_| ClientError::new(ErrorKind::Protocol, "invalid graph output Base64"))?;
    if bytes.len() != output.output_bytes {
        return Err(ClientError::new(
            ErrorKind::Protocol,
            "graph output byte count mismatch",
        ));
    }
    Ok(bytes)
}
