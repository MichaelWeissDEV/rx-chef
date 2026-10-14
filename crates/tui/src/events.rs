//! Blocking transport work belongs on background threads, never in rendering.
use rxchef_client::{ClientError, EngineClient, EngineTask, ErrorKind, TaskResult};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
pub type Client = Arc<dyn EngineClient + Send + Sync>;
pub fn execute_task(
    client: &dyn EngineClient,
    task: EngineTask,
    cancel: &AtomicBool,
    mut status: impl FnMut(&rxchef_client::JobSnapshot),
) -> Result<TaskResult, ClientError> {
    let cancelled = || ClientError::new(ErrorKind::Cancelled, "Execution cancelled");
    if cancel.load(Ordering::Acquire) {
        return Err(cancelled());
    }
    if !client.capabilities()?.jobs {
        return match task {
            EngineTask::Execute(request) => client.execute(request).map(TaskResult::Execution),
            EngineTask::GraphExecute(request) => {
                client.execute_graph(request).map(TaskResult::Graph)
            }
            _ => Err(ClientError::new(
                ErrorKind::Unavailable,
                "Task requires supervised jobs",
            )),
        };
    }
    let job = client.submit_job(task)?;
    loop {
        if cancel.load(Ordering::Acquire) {
            // Returns only after the supervised worker has terminated.
            let ended = client.cancel_job(&job.id)?;
            status(&ended);
            if ended.status == rxchef_client::JobStatus::Succeeded {
                return Err(ClientError::new(
                    ErrorKind::Cancelled,
                    "Execution completed before cancellation; result discarded",
                ));
            }
            return Err(cancelled());
        }
        let snapshot = match client.job(&job.id) {
            Ok(snapshot) => snapshot,
            Err(error) => {
                let _ = client.cancel_job(&job.id);
                return Err(error);
            }
        };
        status(&snapshot);
        if snapshot.status.is_terminal() {
            return snapshot.result.ok_or_else(|| {
                snapshot.error.unwrap_or_else(|| {
                    ClientError::new(ErrorKind::Protocol, "Job ended without a result")
                })
            });
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
