//! Native process-backed engine: shares the server's worker and job table.
use crate::jobs::{JobManager, WorkerConfig};
use crate::*;
use std::sync::Arc;

pub struct SupervisedEngineClient {
    manager: JobManager,
    runtime: tokio::runtime::Handle,
    local: LocalEngineClient,
}
impl SupervisedEngineClient {
    /// The host owns a runtime that remains alive until jobs have been drained.
    pub fn new(config: WorkerConfig, runtime: tokio::runtime::Handle) -> Result<Self, ClientError> {
        if config.workers == 0
            || config.max_pending == 0
            || config.timeout.is_zero()
            || config.retain == 0
            || config.retain_bytes == 0
            || config.memory_bytes == 0
        {
            return Err(ClientError::new(
                ErrorKind::InvalidInput,
                "invalid worker configuration",
            ));
        }
        Ok(Self {
            local: LocalEngineClient {
                policy: config.policy.clone(),
            },
            manager: JobManager::new(Arc::new(config)),
            runtime,
        })
    }
    /// Must be called from a worker thread, never from a Tokio render/event loop.
    fn perform(&self, task: EngineTask) -> Result<TaskResult, ClientError> {
        self.runtime.block_on(self.manager.run_to_completion(task))
    }
    pub async fn shutdown(&self) {
        self.manager.shutdown();
        self.manager.drain().await;
    }
}
impl Drop for SupervisedEngineClient {
    fn drop(&mut self) {
        self.manager.shutdown();
    }
}
impl EngineClient for SupervisedEngineClient {
    fn validate_project(&self, project: WorkbenchProject) -> Result<WorkbenchProject, ClientError> {
        match self.perform(EngineTask::ProjectValidate(project))? {
            TaskResult::Project(project) => Ok(project),
            _ => Err(ClientError::new(
                ErrorKind::Protocol,
                "wrong project result",
            )),
        }
    }
    fn capabilities(&self) -> Result<Capabilities, ClientError> {
        let mut caps = self.local.capabilities()?;
        caps.jobs = true;
        caps.graph_events = true;
        Ok(caps)
    }
    fn operations(&self) -> Result<Vec<integration::OperationDescriptor>, ClientError> {
        self.local.operations()
    }
    fn describe(&self, id: &str) -> Result<integration::OperationDescriptor, ClientError> {
        self.local.describe(id)
    }
    fn validate(&self, request: &ExecuteRequest) -> Result<(), ClientError> {
        self.local.validate(request)
    }
    fn validate_graph(&self, request: &GraphRequest) -> Result<(), ClientError> {
        self.local.validate_graph(request)
    }
    fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse, ClientError> {
        match self.perform(EngineTask::Execute(request))? {
            TaskResult::Execution(value) => Ok(value),
            _ => Err(ClientError::new(ErrorKind::Protocol, "wrong worker result")),
        }
    }
    fn execute_graph(&self, request: GraphRequest) -> Result<GraphResult, ClientError> {
        match self.perform(EngineTask::GraphExecute(request))? {
            TaskResult::Graph(value) => Ok(value),
            _ => Err(ClientError::new(ErrorKind::Protocol, "wrong worker result")),
        }
    }
    fn magic(&self, request: MagicRequest) -> Result<Vec<MagicCandidate>, ClientError> {
        match self.perform(EngineTask::Magic(request))? {
            TaskResult::Magic(value) => Ok(value),
            _ => Err(ClientError::new(ErrorKind::Protocol, "wrong worker result")),
        }
    }
    fn scan(&self, request: ScanRequest) -> Result<Vec<rxchef::scan::Finding>, ClientError> {
        match self.perform(EngineTask::Scan(request))? {
            TaskResult::Scan(value) => Ok(value),
            _ => Err(ClientError::new(ErrorKind::Protocol, "wrong worker result")),
        }
    }
    fn submit_job(&self, task: EngineTask) -> Result<JobSnapshot, ClientError> {
        let _context = self.runtime.enter();
        let (id, _) = self.manager.submit(task)?;
        self.manager.get(&id)
    }
    fn job(&self, id: &str) -> Result<JobSnapshot, ClientError> {
        self.manager.get(id)
    }
    fn cancel_job(&self, id: &str) -> Result<JobSnapshot, ClientError> {
        self.runtime.block_on(self.manager.cancel(id))
    }
}
