//! Bounded job table over supervised worker processes.
//!
//! Every transformation (sync endpoints included) runs through this manager,
//! so queue limits, timeouts, cancellation and retention apply uniformly.
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, Instant},
};

use crate::{ClientError, EngineTask, ErrorKind, JobSnapshot, JobStatus, TaskResult};
use tokio::sync::{watch, Semaphore};

use crate::worker::{supervise, Invocation};

/// Transport-independent configuration; no HTTP or GUI dependencies.
#[derive(Clone)]
pub struct WorkerConfig {
    pub worker_executable: std::path::PathBuf,
    pub policy: crate::ExecutionPolicy,
    pub workers: usize,
    pub max_pending: usize,
    pub timeout: Duration,
    pub memory_bytes: u64,
    pub retain: usize,
    pub retain_bytes: usize,
    pub retain_for: Duration,
}
impl WorkerConfig {
    pub fn local(executable: std::path::PathBuf) -> Self {
        Self {
            worker_executable: executable,
            policy: crate::ExecutionPolicy::default(),
            workers: 2,
            max_pending: 16,
            timeout: Duration::from_secs(60),
            memory_bytes: 512 << 20,
            retain: 16,
            retain_bytes: 32 << 20,
            retain_for: Duration::from_secs(300),
        }
    }
}

struct Entry {
    snapshot: watch::Sender<JobSnapshot>,
    cancel: watch::Sender<bool>,
    finished: Option<Instant>,
    bytes: usize,
}

#[derive(Default)]
struct Table {
    closed: bool,
    jobs: HashMap<String, Entry>,
    /// Queued plus running jobs.
    active: usize,
}

#[derive(Clone)]
pub struct JobManager {
    table: Arc<Mutex<Table>>,
    permits: Arc<Semaphore>,
    config: Arc<WorkerConfig>,
}

fn lock(table: &Mutex<Table>) -> MutexGuard<'_, Table> {
    table
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

async fn cancelled(mut rx: watch::Receiver<bool>) {
    loop {
        if *rx.borrow() {
            return;
        }
        if rx.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }
}

impl JobManager {
    pub fn new(config: Arc<WorkerConfig>) -> Self {
        Self {
            table: Arc::default(),
            permits: Arc::new(Semaphore::new(config.workers)),
            config,
        }
    }

    pub fn active(&self) -> usize {
        lock(&self.table).active
    }

    pub fn submit(
        &self,
        task: EngineTask,
    ) -> Result<(String, watch::Receiver<JobSnapshot>), ClientError> {
        let id = uuid::Uuid::new_v4().to_string();
        let (snapshot, snapshot_rx) = watch::channel(JobSnapshot {
            id: id.clone(),
            status: JobStatus::Queued,
            progress: None,
            result: None,
            error: None,
            graph_nodes: Vec::new(),
        });
        let (cancel, cancel_rx) = watch::channel(false);
        {
            let mut table = lock(&self.table);
            if table.closed {
                return Err(ClientError::new(
                    ErrorKind::Busy,
                    "worker manager is shutting down",
                ));
            }
            self.evict(&mut table);
            if table.active >= self.config.max_pending {
                return Err(ClientError::new(ErrorKind::Busy, "job queue is full"));
            }
            table.active += 1;
            table.jobs.insert(
                id.clone(),
                Entry {
                    snapshot,
                    cancel,
                    finished: None,
                    bytes: 0,
                },
            );
        }
        let manager = self.clone();
        let job = id.clone();
        tokio::spawn(async move { manager.run(job, task, cancel_rx).await });
        Ok((id, snapshot_rx))
    }

    async fn run(self, id: String, task: EngineTask, cancel: watch::Receiver<bool>) {
        let permit = tokio::select! {
            biased;
            _ = cancelled(cancel.clone()) => None,
            permit = self.permits.clone().acquire_owned() => permit.ok(),
        };
        let result = match permit {
            None => Err(ClientError::new(ErrorKind::Cancelled, "job cancelled")),
            Some(_permit) => {
                self.update(&id, |s| s.status = JobStatus::Running);
                supervise(
                    &self.config.worker_executable,
                    Invocation {
                        policy: self.config.policy.clone(),
                        task,
                    },
                    self.config.timeout,
                    self.config.memory_bytes,
                    cancel,
                    |event| self.graph_event(&id, event),
                )
                .await
            }
        };
        self.finish(&id, result);
    }

    fn update(&self, id: &str, change: impl FnOnce(&mut JobSnapshot)) {
        if let Some(entry) = lock(&self.table).jobs.get(id) {
            entry.snapshot.send_modify(change);
        }
    }
    fn graph_event(&self, id: &str, event: rxchef::graph::GraphEvent) {
        use crate::{GraphNodeProgress as Progress, GraphNodeState as State};
        use rxchef::graph::{GraphEvent, NodeStatus};
        let progress = match event {
            GraphEvent::Started { node_id } => Progress {
                node_id,
                state: State::Running,
                output_bytes: None,
                elapsed_ms: None,
                error: None,
            },
            GraphEvent::Finished { node } => Progress {
                node_id: node.node_id,
                state: match node.status {
                    NodeStatus::Succeeded => State::Succeeded,
                    NodeStatus::Failed => State::Failed,
                    NodeStatus::Blocked => State::Blocked,
                    NodeStatus::Skipped => State::Skipped,
                    NodeStatus::Disconnected => State::Disconnected,
                },
                output_bytes: Some(node.output_bytes),
                elapsed_ms: Some(node.elapsed_ms),
                error: node.error,
            },
        };
        self.update(id, |snapshot| {
            if snapshot.status != JobStatus::Running {
                return;
            }
            if let Some(existing) = snapshot
                .graph_nodes
                .iter_mut()
                .find(|node| node.node_id == progress.node_id)
            {
                *existing = progress;
            } else {
                snapshot.graph_nodes.push(progress);
            }
        });
    }

    fn finish(&self, id: &str, result: Result<TaskResult, ClientError>) {
        let (status, result, error) = match result {
            Ok(result) => (JobStatus::Succeeded, Some(result), None),
            Err(error) => {
                let status = match error.kind {
                    ErrorKind::Cancelled => JobStatus::Cancelled,
                    ErrorKind::Timeout => JobStatus::TimedOut,
                    _ => JobStatus::Failed,
                };
                (status, None, Some(error))
            }
        };
        let mut table = lock(&self.table);
        if let Some(entry) = table.jobs.get_mut(id) {
            entry.snapshot.send_modify(|s| {
                s.status = status;
                s.result = result;
                s.error = error;
                for node in &mut s.graph_nodes {
                    if node.state == crate::GraphNodeState::Running {
                        node.state = match status {
                            JobStatus::Cancelled => crate::GraphNodeState::Cancelled,
                            JobStatus::TimedOut => crate::GraphNodeState::TimedOut,
                            _ => crate::GraphNodeState::Failed,
                        };
                    }
                }
            });
            entry.finished = Some(Instant::now());
            entry.bytes = serde_json::to_vec(&*entry.snapshot.borrow())
                .map_or(usize::MAX, |bytes| bytes.len());
            table.active = table.active.saturating_sub(1);
        }
        self.evict(&mut table);
    }

    /// Drop expired results, then enforce the count and memory budgets
    /// (oldest finished job first). Running jobs are never evicted.
    fn evict(&self, table: &mut Table) {
        let now = Instant::now();
        let ttl = self.config.retain_for;
        table
            .jobs
            .retain(|_, e| e.finished.is_none_or(|t| now.duration_since(t) < ttl));
        loop {
            let finished: Vec<_> = table
                .jobs
                .iter()
                .filter_map(|(id, e)| e.finished.map(|t| (t, id.clone(), e.bytes)))
                .collect();
            let total: usize = finished
                .iter()
                .fold(0usize, |total, f| total.saturating_add(f.2));
            if finished.len() <= self.config.retain && total <= self.config.retain_bytes {
                return;
            }
            match finished.into_iter().min() {
                Some((_, id, _)) => {
                    table.jobs.remove(&id);
                }
                None => return,
            }
        }
    }

    pub fn get(&self, id: &str) -> Result<JobSnapshot, ClientError> {
        lock(&self.table)
            .jobs
            .get(id)
            .map(|e| e.snapshot.borrow().clone())
            .ok_or_else(|| ClientError::new(ErrorKind::NotFound, "unknown or expired job"))
    }

    pub fn subscribe(&self, id: &str) -> Result<watch::Receiver<JobSnapshot>, ClientError> {
        lock(&self.table)
            .jobs
            .get(id)
            .map(|e| e.snapshot.subscribe())
            .ok_or_else(|| ClientError::new(ErrorKind::NotFound, "unknown or expired job"))
    }

    fn signal_cancel(&self, id: &str) -> bool {
        lock(&self.table)
            .jobs
            .get(id)
            .map(|e| e.cancel.send_replace(true))
            .is_some()
    }

    /// Request cancellation and wait briefly for the terminal state, so the
    /// caller sees a deterministic result for jobs that die promptly.
    pub async fn cancel(&self, id: &str) -> Result<JobSnapshot, ClientError> {
        let mut rx = self.subscribe(id)?;
        self.signal_cancel(id);
        let _ = tokio::time::timeout(
            Duration::from_secs(5),
            rx.wait_for(|s| s.status.is_terminal()),
        )
        .await;
        let snapshot = rx.borrow().clone();
        Ok(snapshot)
    }

    /// Atomically stop admission before cancelling queued and running jobs.
    pub fn shutdown(&self) {
        let mut table = lock(&self.table);
        table.closed = true;
        for entry in table.jobs.values() {
            entry.cancel.send_replace(true);
        }
    }

    pub async fn drain(&self) {
        while self.active() > 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    pub fn cancel_all(&self) {
        for entry in lock(&self.table).jobs.values() {
            entry.cancel.send_replace(true);
        }
    }

    /// Run one task to completion. Dropping the future (client disconnect)
    /// cancels the worker; the finished entry is never retained.
    pub async fn run_to_completion(&self, task: EngineTask) -> Result<TaskResult, ClientError> {
        let (id, mut rx) = self.submit(task)?;
        let _guard = Guard {
            manager: self.clone(),
            id,
        };
        let snapshot = rx
            .wait_for(|s| s.status.is_terminal())
            .await
            .map_err(|_| ClientError::new(ErrorKind::Operation, "job disappeared"))?
            .clone();
        match (snapshot.result, snapshot.error) {
            (Some(result), _) => Ok(result),
            (_, Some(error)) => Err(error),
            _ => Err(ClientError::new(
                ErrorKind::Operation,
                "job produced no result",
            )),
        }
    }
}

struct Guard {
    manager: JobManager,
    id: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.manager.signal_cancel(&self.id);
        let mut table = lock(&self.manager.table);
        if table
            .jobs
            .get(&self.id)
            .is_some_and(|e| e.finished.is_some())
        {
            table.jobs.remove(&self.id);
        }
    }
}
