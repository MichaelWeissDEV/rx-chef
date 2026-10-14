//! Server facade over the transport-independent supervised job manager.
use crate::config::ServerConfig;
pub use rxchef_client::jobs::WorkerConfig;
use std::sync::Arc;
#[derive(Clone)]
pub struct JobManager(rxchef_client::jobs::JobManager);
fn server_error(mut error: rxchef_client::ClientError) -> rxchef_client::ClientError {
    if error.message == "worker manager is shutting down" {
        error.message = "server is shutting down".into();
    }
    error
}
impl JobManager {
    pub fn submit(
        &self,
        task: rxchef_client::EngineTask,
    ) -> Result<
        (
            String,
            tokio::sync::watch::Receiver<rxchef_client::JobSnapshot>,
        ),
        rxchef_client::ClientError,
    > {
        self.0.submit(task).map_err(server_error)
    }
    pub async fn run_to_completion(
        &self,
        task: rxchef_client::EngineTask,
    ) -> Result<rxchef_client::TaskResult, rxchef_client::ClientError> {
        self.0.run_to_completion(task).await.map_err(server_error)
    }
    pub fn new(config: Arc<ServerConfig>) -> Self {
        Self(rxchef_client::jobs::JobManager::new(Arc::new(
            WorkerConfig {
                worker_executable: config.worker_executable.clone(),
                policy: config.policy.clone(),
                workers: config.workers,
                max_pending: config.max_pending,
                timeout: config.timeout,
                memory_bytes: config.memory_bytes,
                retain: config.retain,
                retain_bytes: config.retain_bytes,
                retain_for: config.retain_for,
            },
        )))
    }
}
impl std::ops::Deref for JobManager {
    type Target = rxchef_client::jobs::JobManager;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
