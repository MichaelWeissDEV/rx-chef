//! Bounded job table over supervised worker processes.
//!
//! Every transformation (sync endpoints included) runs through this manager,
//! so queue limits, timeouts, cancellation and retention apply uniformly.
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, Instant},
};

use rxchef_client::{ClientError, EngineTask, ErrorKind, JobSnapshot, JobStatus, TaskResult};
use tokio::sync::{watch, Semaphore};

use crate::{
    config::ServerConfig,
    worker::{supervise, Invocation},
};

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
    config: Arc<ServerConfig>,
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
    pub fn new(config: Arc<ServerConfig>) -> Self {
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
        });
        let (cancel, cancel_rx) = watch::channel(false);
        {
            let mut table = lock(&self.table);
            if table.closed {
                return Err(ClientError::new(ErrorKind::Busy, "server is shutting down"));
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
        let bytes = result
            .as_ref()
            .map_or(0, |r| serde_json::to_vec(r).map_or(0, |v| v.len()));
        let mut table = lock(&self.table);
        if let Some(entry) = table.jobs.get_mut(id) {
            entry.snapshot.send_modify(|s| {
                s.status = status;
                s.result = result;
                s.error = error;
            });
            entry.finished = Some(Instant::now());
            entry.bytes = bytes;
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
            let total: usize = finished.iter().map(|f| f.2).sum();
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
