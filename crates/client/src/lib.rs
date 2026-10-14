//! UI-independent client contracts. Exact payloads always use Base64.
//!
//! The local implementation calls the core directly. The optional remote
//! implementation speaks the same versioned contract without duplicating recipes.
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rxchef::{execution, integration, runtime};
use serde::{Deserialize, Serialize};

#[cfg(feature = "remote")]
mod remote;
#[cfg(feature = "remote")]
pub use remote::RemoteEngineClient;

mod graph;
pub use graph::{graph_output_bytes, GraphRequest};
pub use rxchef::graph::GraphResult;
pub use rxchef_store::workbench::WorkbenchProject;

mod analysis;
pub use analysis::{MagicCandidate, MagicRequest, ScanRequest};

pub const PROTOCOL_VERSION: u32 = integration::PROTOCOL_VERSION;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ErrorKind {
    InvalidInput,
    InvalidRecipe,
    UnknownOperation,
    Unavailable,
    InvalidArgument,
    Operation,
    LimitExceeded,
    PolicyDenied,
    Transport,
    Protocol,
    Busy,
    NotFound,
    Cancelled,
    Timeout,
    Unauthorized,
}

/// Stable classification independent of human-readable messages.
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
#[error("{message}")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClientError {
    pub kind: ErrorKind,
    pub message: String,
    pub step: Option<usize>,
}

impl ClientError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            step: None,
        }
    }
}

impl From<execution::ExecutionError> for ClientError {
    fn from(error: execution::ExecutionError) -> Self {
        use execution::ExecutionError as E;
        let (kind, step) = match &error {
            E::InvalidRecipe(_) => (ErrorKind::InvalidRecipe, None),
            E::StepLimitExceeded { .. } => (ErrorKind::LimitExceeded, None),
            E::OutputLimitExceeded { step_index, .. } => {
                (ErrorKind::LimitExceeded, Some(*step_index))
            }
            E::Step { step_index, .. } => (ErrorKind::Operation, Some(*step_index)),
            E::RuntimeStep {
                step_index, source, ..
            } => {
                let kind = match source {
                    runtime::RuntimeError::UnknownOperation(_) => ErrorKind::UnknownOperation,
                    runtime::RuntimeError::Unavailable { .. } => ErrorKind::Unavailable,
                    runtime::RuntimeError::MissingArgument { .. }
                    | runtime::RuntimeError::InvalidArgument { .. } => ErrorKind::InvalidArgument,
                    _ => ErrorKind::Operation,
                };
                (kind, Some(*step_index))
            }
        };
        Self {
            kind,
            step,
            message: error.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Capabilities {
    pub version: String,
    pub protocol_version: u32,
    pub binary_transport: String,
    pub trace: bool,
    /// True when the endpoint offers process-supervised, cancellable jobs.
    pub jobs: bool,
    /// Versioned multi-input graph execution is available.
    #[serde(default)]
    pub graphs: bool,
    #[serde(default)]
    pub graph_events: bool,
    pub max_input_bytes: usize,
    pub max_output_bytes: usize,
    pub max_steps: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TraceLimits {
    pub max_entries: usize,
    pub max_bytes_per_entry: usize,
    pub max_total_bytes: usize,
}

impl Default for TraceLimits {
    fn default() -> Self {
        Self {
            max_entries: 4096,
            max_bytes_per_entry: 4096,
            max_total_bytes: 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ExecuteRequest {
    /// None means no input source; Some("") means explicitly empty input.
    pub input_base64: Option<String>,
    pub recipe: Vec<execution::RecipeStep>,
    #[serde(default)]
    pub variables: std::collections::BTreeMap<String, String>,
    /// Opt-in: trace payloads can contain sensitive data.
    #[serde(default)]
    pub trace: Option<TraceLimits>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TraceArtifact {
    /// Zero-based index into the original recipe, including repeated steps.
    pub step_index: usize,
    pub operation: String,
    pub input_bytes: usize,
    pub output_bytes: usize,
    pub elapsed_ms: f64,
    pub control_flow: bool,
    /// Exact prefix, never a reconstruction from a lossy text preview.
    pub output_base64: String,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ExecuteResponse {
    pub result: integration::ExecutionResult,
    pub trace: Vec<TraceArtifact>,
    pub trace_truncated: bool,
}

/// A synchronous interface intended for a frontend's worker thread.
/// UI event loops must not call long-running transformations inline.
pub trait EngineClient {
    fn validate_project(
        &self,
        _project: WorkbenchProject,
    ) -> Result<WorkbenchProject, ClientError> {
        Err(ClientError::new(
            ErrorKind::Unavailable,
            "project API unavailable",
        ))
    }
    fn capabilities(&self) -> Result<Capabilities, ClientError>;
    fn operations(&self) -> Result<Vec<integration::OperationDescriptor>, ClientError>;
    fn describe(&self, operation: &str) -> Result<integration::OperationDescriptor, ClientError>;
    fn validate(&self, request: &ExecuteRequest) -> Result<(), ClientError>;
    fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse, ClientError>;

    fn validate_graph(&self, _request: &GraphRequest) -> Result<(), ClientError> {
        Err(ClientError::new(
            ErrorKind::Unavailable,
            "graph API unavailable",
        ))
    }
    fn execute_graph(&self, _request: GraphRequest) -> Result<GraphResult, ClientError> {
        Err(ClientError::new(
            ErrorKind::Unavailable,
            "graph API unavailable",
        ))
    }

    fn submit_job(&self, _task: EngineTask) -> Result<JobSnapshot, ClientError> {
        Err(ClientError::new(
            ErrorKind::Unavailable,
            "this client has no job manager",
        ))
    }
    fn job(&self, _id: &str) -> Result<JobSnapshot, ClientError> {
        Err(ClientError::new(
            ErrorKind::Unavailable,
            "this client has no job manager",
        ))
    }
    fn cancel_job(&self, _id: &str) -> Result<JobSnapshot, ClientError> {
        Err(ClientError::new(
            ErrorKind::Unavailable,
            "this client has no job manager",
        ))
    }

    fn magic(&self, request: MagicRequest) -> Result<Vec<MagicCandidate>, ClientError>;
    fn scan(&self, request: ScanRequest) -> Result<Vec<rxchef::scan::Finding>, ClientError>;

    fn search(&self, query: &str) -> Result<Vec<integration::OperationDescriptor>, ClientError> {
        let query = query.to_lowercase();
        Ok(self
            .operations()?
            .into_iter()
            .filter(|op| {
                op.name.to_lowercase().contains(&query)
                    || op.description.to_lowercase().contains(&query)
            })
            .collect())
    }

    fn run(
        &self,
        operation: &str,
        input: &[u8],
        args: Vec<String>,
    ) -> Result<ExecuteResponse, ClientError> {
        self.execute(ExecuteRequest {
            input_base64: Some(STANDARD.encode(input)),
            recipe: vec![execution::RecipeStep {
                op: operation.into(),
                args,
            }],
            variables: Default::default(),
            trace: None,
        })
    }
}

/// Execution policy for a client boundary; not an OS sandbox.
/// Output limits are checked after an operation returns. Hard memory and time
/// isolation require a supervised worker process around this client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    pub max_input_bytes: usize,
    pub max_output_bytes: usize,
    pub max_steps: usize,
    pub allow_host_effects: bool,
    /// Canonical registry IDs; None allows all operations permitted by effects.
    pub allowed_operations: Option<std::collections::BTreeSet<String>>,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            max_input_bytes: 8 << 20,
            max_output_bytes: 8 << 20,
            max_steps: 100_000,
            allow_host_effects: false,
            allowed_operations: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LocalEngineClient {
    pub policy: ExecutionPolicy,
}

impl LocalEngineClient {
    fn decode_input(&self, encoded: &str) -> Result<Vec<u8>, ClientError> {
        let max_encoded = (self.policy.max_input_bytes.saturating_add(2) / 3).saturating_mul(4);
        if encoded.len() > max_encoded {
            return Err(ClientError::new(
                ErrorKind::LimitExceeded,
                "input exceeds byte limit",
            ));
        }
        let input = STANDARD
            .decode(encoded)
            .map_err(|_| ClientError::new(ErrorKind::InvalidInput, "invalid input_base64"))?;
        if input.len() > self.policy.max_input_bytes {
            return Err(ClientError::new(
                ErrorKind::LimitExceeded,
                "input exceeds byte limit",
            ));
        }
        Ok(input)
    }

    pub fn check_operation(&self, operation: &str) -> Result<(), ClientError> {
        let descriptor = self.describe(operation)?;
        use rxchef::SideEffect as S;
        let host_effect = descriptor.side_effects.iter().any(|effect| {
            matches!(
                effect,
                S::Network | S::FilesystemRead | S::FilesystemWrite | S::ExternalProcess
            )
        });
        if (!self.policy.allow_host_effects && host_effect)
            || self
                .policy
                .allowed_operations
                .as_ref()
                .is_some_and(|ids| !ids.contains(&descriptor.id))
        {
            return Err(ClientError::new(
                ErrorKind::PolicyDenied,
                format!(
                    "operation '{}' is denied by execution policy",
                    descriptor.name
                ),
            ));
        }
        Ok(())
    }

    fn check(&self, request: &ExecuteRequest) -> Result<Vec<u8>, ClientError> {
        let input = self.decode_input(request.input_base64.as_deref().unwrap_or(""))?;
        if request.recipe.len() > self.policy.max_steps {
            return Err(ClientError::new(
                ErrorKind::LimitExceeded,
                "recipe exceeds step limit",
            ));
        }
        for (index, step) in request.recipe.iter().enumerate() {
            self.check_operation(&step.op).map_err(|mut error| {
                error.step = Some(index + 1);
                error
            })?;
        }
        execution::validate_recipe(
            &request.recipe,
            &execution::VariableContext::new(request.variables.clone()),
        )?;
        Ok(input)
    }
}

impl EngineClient for LocalEngineClient {
    fn validate_project(&self, project: WorkbenchProject) -> Result<WorkbenchProject, ClientError> {
        project
            .validate()
            .map_err(|error| ClientError::new(ErrorKind::InvalidInput, error.to_string()))?;
        for graph in &project.graphs {
            self.validate_graph(&GraphRequest {
                graph: graph.clone(),
                inputs_base64: Default::default(),
                outputs: Vec::new(),
            })?;
        }
        Ok(project)
    }
    fn validate_graph(&self, request: &GraphRequest) -> Result<(), ClientError> {
        self.validate_graph_request(request)
    }
    fn execute_graph(&self, request: GraphRequest) -> Result<GraphResult, ClientError> {
        self.execute_graph_request(request)
    }

    fn magic(&self, request: MagicRequest) -> Result<Vec<MagicCandidate>, ClientError> {
        self.magic_request(request)
    }
    fn scan(&self, request: ScanRequest) -> Result<Vec<rxchef::scan::Finding>, ClientError> {
        self.scan_request(request)
    }

    fn capabilities(&self) -> Result<Capabilities, ClientError> {
        Ok(Capabilities {
            version: env!("CARGO_PKG_VERSION").into(),
            protocol_version: PROTOCOL_VERSION,
            binary_transport: "base64".into(),
            trace: true,
            jobs: false,
            graphs: true,
            graph_events: false,
            max_input_bytes: self.policy.max_input_bytes,
            max_output_bytes: self.policy.max_output_bytes,
            max_steps: self.policy.max_steps,
        })
    }

    fn operations(&self) -> Result<Vec<integration::OperationDescriptor>, ClientError> {
        integration::operations().map_err(|error| ClientError::new(ErrorKind::Protocol, error))
    }

    fn describe(&self, operation: &str) -> Result<integration::OperationDescriptor, ClientError> {
        integration::describe(operation)
            .map_err(|error| ClientError::new(ErrorKind::UnknownOperation, error))
    }

    fn validate(&self, request: &ExecuteRequest) -> Result<(), ClientError> {
        self.check(request).map(|_| ())
    }

    fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse, ClientError> {
        let input = self.check(&request)?;
        let mut trace = Vec::new();
        let mut remaining = request
            .trace
            .as_ref()
            .map_or(0, |limits| limits.max_total_bytes.min(1024 * 1024));
        let mut trace_truncated = false;
        let outcome = execution::execute_observed(
            execution::ExecutionRequest {
                input,
                input_supplied: request.input_base64.is_some(),
                recipe: request.recipe.into(),
                variables: execution::VariableContext::new(request.variables),
                options: execution::ExecutionOptions {
                    trace: false,
                    max_steps: self.policy.max_steps,
                    max_output_bytes: Some(self.policy.max_output_bytes),
                },
            },
            &mut |entry, output| {
                let Some(limits) = &request.trace else {
                    return;
                };
                if trace.len() >= limits.max_entries.min(4096) {
                    trace_truncated = true;
                    return;
                }
                let len = output
                    .len()
                    .min(limits.max_bytes_per_entry.min(4096))
                    .min(remaining);
                remaining -= len;
                trace_truncated |= len < output.len();
                trace.push(TraceArtifact {
                    step_index: entry.step_index,
                    operation: entry.operation.clone(),
                    input_bytes: entry.input_bytes,
                    output_bytes: entry.output_bytes,
                    elapsed_ms: entry.elapsed.as_secs_f64() * 1000.0,
                    control_flow: entry.status == execution::TraceStatus::ControlFlow,
                    output_base64: STANDARD.encode(&output[..len]),
                    truncated: len < output.len(),
                });
            },
        )?;
        Ok(ExecuteResponse {
            result: integration::ExecutionResult::from_bytes(outcome.output),
            trace,
            trace_truncated,
        })
    }
}

/// Decode an exported result using its authoritative payload.
pub fn export_bytes(result: &integration::ExecutionResult) -> Result<Vec<u8>, ClientError> {
    let bytes = STANDARD
        .decode(&result.output_base64)
        .map_err(|_| ClientError::new(ErrorKind::Protocol, "invalid result Base64"))?;
    if bytes.len() != result.output_len {
        return Err(ClientError::new(
            ErrorKind::Protocol,
            "result byte count does not match payload",
        ));
    }
    Ok(bytes)
}

/// Shared job payload, also used for supervised local worker execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum EngineTask {
    ProjectValidate(WorkbenchProject),
    Execute(ExecuteRequest),
    GraphExecute(GraphRequest),
    GraphValidate(GraphRequest),
    Validate(ExecuteRequest),
    Magic(MagicRequest),
    Scan(ScanRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TaskResult {
    Project(WorkbenchProject),
    Execution(ExecuteResponse),
    Graph(GraphResult),
    Validated,
    Magic(Vec<MagicCandidate>),
    Scan(Vec<rxchef::scan::Finding>),
}

impl LocalEngineClient {
    pub fn perform(&self, task: EngineTask) -> Result<TaskResult, ClientError> {
        match task {
            EngineTask::ProjectValidate(project) => {
                self.validate_project(project).map(TaskResult::Project)
            }
            EngineTask::GraphExecute(request) => self.execute_graph(request).map(TaskResult::Graph),
            EngineTask::GraphValidate(request) => {
                self.validate_graph(&request).map(|_| TaskResult::Validated)
            }
            EngineTask::Execute(request) => self.execute(request).map(TaskResult::Execution),
            EngineTask::Validate(request) => self.validate(&request).map(|_| TaskResult::Validated),
            EngineTask::Magic(request) => self.magic(request).map(TaskResult::Magic),
            EngineTask::Scan(request) => self.scan(request).map(TaskResult::Scan),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum JobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
}

impl JobStatus {
    /// True once a job can no longer change state.
    pub fn is_terminal(self) -> bool {
        !matches!(self, JobStatus::Queued | JobStatus::Running)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct JobSnapshot {
    pub id: String,
    pub status: JobStatus,
    /// No fabricated percentage: None when operation progress is unknown.
    pub progress: Option<f64>,
    pub result: Option<TaskResult>,
    pub error: Option<ClientError>,
    #[serde(default)]
    pub graph_nodes: Vec<GraphNodeProgress>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GraphNodeState {
    Running,
    Succeeded,
    Failed,
    Blocked,
    Skipped,
    Disconnected,
    Cancelled,
    TimedOut,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GraphNodeProgress {
    pub node_id: String,
    pub state: GraphNodeState,
    pub output_bytes: Option<usize>,
    pub elapsed_ms: Option<f64>,
    pub error: Option<rxchef::graph::GraphError>,
}

#[cfg(feature = "worker")]
pub mod jobs;
#[cfg(feature = "worker")]
mod supervised;
#[cfg(feature = "worker")]
pub mod worker;
#[cfg(feature = "worker")]
mod worker_resources;
#[cfg(feature = "worker")]
pub use supervised::SupervisedEngineClient;
