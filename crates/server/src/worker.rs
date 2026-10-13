//! Length-framed, bounded IPC. A parent-owned stdin pipe doubles as a lifeline.
use std::{io::{Read, Write}, path::Path, process::Stdio, time::Duration};
use rxchef_client::{ClientError, EngineTask, ErrorKind, ExecutionPolicy, LocalEngineClient, TaskResult};
use serde::{Deserialize, Serialize};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, process::Command, sync::watch};

/// Sized for the default 8 MiB input policy after Base64 and JSON expansion.
pub const MAX_REQUEST: usize = 32 << 20;
pub const MAX_RESPONSE: usize = 48 << 20;

#[derive(Serialize, Deserialize)]
pub struct Invocation { pub policy: ExecutionPolicy, pub task: EngineTask }

/// Execute exactly one invocation, retaining a parent-death watchdog throughout.
/// This is an internal subprocess mode, never an HTTP-controlled command line.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin = std::io::stdin();
    let mut header = [0; 4];
    stdin.read_exact(&mut header)?;
    let len = u32::from_be_bytes(header) as usize;
    if len > MAX_REQUEST { return Err("worker request too large".into()); }
    let mut body = vec![0; len];
    stdin.read_exact(&mut body)?;
    // EOF means the supervisor went away. Do not leave work orphaned after a
    // crash or forcible parent termination, including on non-Unix platforms.
    std::thread::spawn(|| {
        let mut byte = [0];
        let _ = std::io::stdin().read(&mut byte);
        std::process::exit(75);
    });
    let invocation: Invocation = serde_json::from_slice(&body)?;
    let result = LocalEngineClient { policy: invocation.policy }.perform(invocation.task);
    let result = serde_json::to_vec(&result)?;
    let result = if result.len() > MAX_RESPONSE {
        serde_json::to_vec(&Err::<TaskResult, _>(ClientError::new(ErrorKind::LimitExceeded, "worker response limit exceeded")))?
    } else { result };
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&(result.len() as u32).to_be_bytes())?;
    stdout.write_all(&result)?;
    stdout.flush()?;
    Ok(())
}

pub async fn supervise(
    executable: &Path,
    invocation: Invocation,
    timeout: Duration,
    memory_bytes: u64,
    mut cancel: watch::Receiver<bool>,
) -> Result<TaskResult, ClientError> {
    let body = serde_json::to_vec(&invocation).map_err(|_| failure("cannot encode worker request"))?;
    if body.len() > MAX_REQUEST { return Err(ClientError::new(ErrorKind::LimitExceeded, "worker request limit exceeded")); }
    let mut command = Command::new(executable);
    command.arg("--internal-worker").env_clear()
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).kill_on_drop(true);
    // Enforced limits differ per platform; see docs/server/configuration.md.
    //  * Linux: RLIMIT_AS (address space) and RLIMIT_CPU.
    //  * Other Unix (macOS, BSD): RLIMIT_CPU only. RLIMIT_AS is not reliably
    //    enforced there, so memory is NOT limited beyond the input/output caps.
    //  * Windows: neither; only the wall-clock deadline (kill) and caps apply.
    #[cfg(unix)]
    {
        let cpu_seconds = timeout.as_secs().saturating_add(2);
        // Only async-signal-safe libc calls in the post-fork child.
        unsafe { command.pre_exec(move || {
            // Fixed-size array: no heap allocation after fork.
            #[cfg(target_os = "linux")]
            let limits = [(libc::RLIMIT_CPU, cpu_seconds), (libc::RLIMIT_AS, memory_bytes)];
            #[cfg(not(target_os = "linux"))]
            let limits = [(libc::RLIMIT_CPU, cpu_seconds)];
            for (resource, limit) in limits {
                let limit = libc::rlimit { rlim_cur: limit as _, rlim_max: limit as _ };
                if libc::setrlimit(resource, &limit) != 0 { return Err(std::io::Error::last_os_error()); }
            }
            Ok(())
        }); }
    }
    #[cfg(not(target_os = "linux"))]
    let _ = memory_bytes;
    let mut child = command.spawn().map_err(|_| failure("cannot start worker"))?;
    // Taking stdin is essential: Child::wait must not close the lifeline.
    let mut lifeline = child.stdin.take().ok_or_else(|| failure("missing worker stdin"))?;
    let mut output = child.stdout.take().ok_or_else(|| failure("missing worker stdout"))?;
    let result = {
        let work = async {
            lifeline.write_all(&(body.len() as u32).to_be_bytes()).await.map_err(|_| failure("worker input failed"))?;
            lifeline.write_all(&body).await.map_err(|_| failure("worker input failed"))?;
            lifeline.flush().await.map_err(|_| failure("worker input failed"))?;
            let size = output.read_u32().await.map_err(|_| failure("worker exited without a result"))? as usize;
            if size > MAX_RESPONSE { return Err(ClientError::new(ErrorKind::LimitExceeded, "worker response limit exceeded")); }
            let mut response = vec![0; size];
            output.read_exact(&mut response).await.map_err(|_| failure("incomplete worker result"))?;
            let status = child.wait().await.map_err(|_| failure("cannot reap worker"))?;
            if !status.success() { return Err(failure("worker failed (resource limit, panic, or process error)")); }
            serde_json::from_slice::<Result<TaskResult, ClientError>>(&response)
                .map_err(|_| failure("invalid worker result"))?
        };
        tokio::pin!(work);
        tokio::select! {
            biased;
            _ = async {
                // A dropped sender is not a cancellation request.
                loop {
                    if *cancel.borrow() { break; }
                    if cancel.changed().await.is_err() { std::future::pending::<()>().await; }
                }
            } => Err(ClientError::new(ErrorKind::Cancelled, "job cancelled")),
            _ = tokio::time::sleep(timeout) => Err(ClientError::new(ErrorKind::Timeout, "job deadline exceeded")),
            result = &mut work => result,
        }
    };
    drop(lifeline);
    // start_kill is harmless for an already exited child; wait always reaps it.
    let _ = child.start_kill();
    let _ = child.wait().await;
    result
}

fn failure(message: &str) -> ClientError { ClientError::new(ErrorKind::Operation, message) }
