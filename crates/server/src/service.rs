//! Transport-independent service: REST, JSON-RPC and WebSocket all call
//! [`AppState::call`], so there is exactly one implementation per method.
use std::sync::Arc;

use rxchef_client::{
    Capabilities, ClientError, EngineClient, EngineTask, ErrorKind, LocalEngineClient, TaskResult,
    PROTOCOL_VERSION,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use tokio::sync::Semaphore;

use crate::{
    config::{Scope, ServerConfig},
    jobs::JobManager,
};

pub struct AppState {
    pub config: Arc<ServerConfig>,
    pub jobs: JobManager,
    pub connections: Arc<Semaphore>,
    pub ws_connections: Arc<Semaphore>,
    local: LocalEngineClient,
}

/// Methods a read-only token may call. Everything else needs full scope.
const READ_METHODS: &[&str] = &["version", "capabilities", "operations", "describe", "search"];
/// Every method known to the service (used for JSON-RPC "method not found").
pub const METHODS: &[&str] = &[
    "version", "capabilities", "operations", "describe", "search", "validate", "execute", "magic",
    "scan", "job.submit", "job.get", "job.cancel",
];

fn params<T: DeserializeOwned>(value: Value) -> Result<T, ClientError> {
    // Never echo the offending payload: it may contain secrets.
    serde_json::from_value(value).map_err(|_| ClientError::new(ErrorKind::InvalidInput, "invalid parameters"))
}

fn text(value: &Value, key: &str) -> Result<String, ClientError> {
    match value.get(key).and_then(Value::as_str) {
        Some(s) if !s.is_empty() && s.len() <= 256 => Ok(s.to_owned()),
        _ => Err(ClientError::new(ErrorKind::InvalidInput, format!("missing or invalid '{key}'"))),
    }
}

fn encode<T: serde::Serialize>(value: T) -> Result<Value, ClientError> {
    serde_json::to_value(value).map_err(|_| ClientError::new(ErrorKind::Protocol, "cannot encode result"))
}

impl AppState {
    pub fn new(config: ServerConfig) -> Arc<Self> {
        let config = Arc::new(config);
        Arc::new(Self {
            jobs: JobManager::new(config.clone()),
            connections: Arc::new(Semaphore::new(config.max_connections)),
            ws_connections: Arc::new(Semaphore::new(config.max_ws_connections)),
            local: LocalEngineClient { policy: config.policy.clone() },
            config,
        })
    }

    pub fn capabilities(&self) -> Result<Capabilities, ClientError> {
        let mut capabilities = self.local.capabilities()?;
        capabilities.jobs = true;
        Ok(capabilities)
    }

    /// Authorize then perform one method. Discovery is answered from the
    /// pure registry; everything that runs data goes through a worker process.
    pub async fn call(&self, scope: Option<Scope>, method: &str, input: Value) -> Result<Value, ClientError> {
        let allowed = match scope {
            Some(Scope::Full) => true,
            Some(Scope::Read) => READ_METHODS.contains(&method),
            None => false,
        };
        if !allowed {
            let (kind, message) = if scope.is_some() {
                (ErrorKind::PolicyDenied, "token scope does not permit this method")
            } else {
                (ErrorKind::Unauthorized, "authentication required")
            };
            return Err(ClientError::new(kind, message));
        }
        match method {
            "version" => Ok(json!({
                "name": "rxchef-server",
                "version": env!("CARGO_PKG_VERSION"),
                "protocol_version": PROTOCOL_VERSION,
            })),
            "capabilities" => encode(self.capabilities()?),
            "operations" => encode(self.local.operations()?),
            "describe" => encode(self.local.describe(&text(&input, "operation")?)?),
            "search" => encode(self.local.search(&text(&input, "query")?)?),
            "validate" => self.run(EngineTask::Validate(params(input)?)).await,
            "execute" => self.run(EngineTask::Execute(params(input)?)).await,
            "magic" => self.run(EngineTask::Magic(params(input)?)).await,
            "scan" => self.run(EngineTask::Scan(params(input)?)).await,
            "job.submit" => {
                let (id, _) = self.jobs.submit(params(input)?)?;
                encode(self.jobs.get(&id)?)
            }
            "job.get" => encode(self.jobs.get(&text(&input, "id")?)?),
            "job.cancel" => encode(self.jobs.cancel(&text(&input, "id")?).await?),
            _ => Err(ClientError::new(ErrorKind::Unavailable, "unknown method")),
        }
    }

    async fn run(&self, task: EngineTask) -> Result<Value, ClientError> {
        match self.jobs.run_to_completion(task).await? {
            TaskResult::Execution(r) => encode(r),
            TaskResult::Validated => Ok(Value::Null),
            TaskResult::Magic(r) => encode(r),
            TaskResult::Scan(r) => encode(r),
        }
    }
}

/// HTTP status for each structured error kind.
pub fn status_of(kind: ErrorKind) -> u16 {
    use ErrorKind::*;
    match kind {
        InvalidInput | InvalidRecipe | InvalidArgument => 400,
        Unauthorized => 401,
        PolicyDenied => 403,
        UnknownOperation | NotFound => 404,
        Cancelled => 409,
        LimitExceeded => 413,
        Operation => 422,
        Busy | Unavailable => 503,
        Timeout => 504,
        Transport => 502,
        Protocol => 500,
    }
}
