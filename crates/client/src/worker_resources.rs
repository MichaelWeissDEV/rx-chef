//! Platform constraints are installed before any invocation reaches the child.
use crate::ClientError;
#[cfg(any(target_os = "macos", windows))]
use crate::ErrorKind;
pub(crate) struct Guard {
    #[cfg(target_os = "macos")]
    pid: u32,
    #[cfg(target_os = "macos")]
    memory_bytes: u64,
    #[cfg(windows)]
    _job: std::os::windows::io::OwnedHandle,
}
#[cfg(any(target_os = "macos", windows))]
fn failed() -> ClientError {
    ClientError::new(
        ErrorKind::Unavailable,
        "cannot install worker resource constraints",
    )
}
impl Guard {
    pub fn prepare(child: &tokio::process::Child, memory_bytes: u64) -> Result<Self, ClientError> {
        #[cfg(target_os = "macos")]
        {
            let pid = child.id().ok_or_else(failed)?;
            footprint(pid).map_err(|_| failed())?;
            return Ok(Self { pid, memory_bytes });
        }
        #[cfg(windows)]
        {
            use std::os::windows::io::{AsRawHandle, FromRawHandle};
            use windows_sys::Win32::System::JobObjects::*;
            let budget = usize::try_from(memory_bytes).map_err(|_| failed())?;
            let process = child.raw_handle().ok_or_else(failed)?;
            // SAFETY: unnamed private job, valid owned process handle, initialized
            // Win32 structure with its exact size. OwnedHandle closes on all paths.
            unsafe {
                let handle = CreateJobObjectW(std::ptr::null(), std::ptr::null());
                if handle.is_null() {
                    return Err(failed());
                }
                let job = std::os::windows::io::OwnedHandle::from_raw_handle(handle);
                let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
                info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
                    | JOB_OBJECT_LIMIT_PROCESS_MEMORY
                    | JOB_OBJECT_LIMIT_JOB_MEMORY;
                info.ProcessMemoryLimit = budget;
                info.JobMemoryLimit = budget;
                if SetInformationJobObject(
                    job.as_raw_handle(),
                    JobObjectExtendedLimitInformation,
                    (&info as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
                    std::mem::size_of_val(&info) as u32,
                ) == 0
                    || AssignProcessToJobObject(job.as_raw_handle(), process) == 0
                {
                    return Err(failed());
                }
                return Ok(Self { _job: job });
            }
        }
        #[cfg(not(any(target_os = "macos", windows)))]
        {
            let _ = (child, memory_bytes);
            Ok(Self {})
        }
    }
    pub async fn exceeded(&self) -> ClientError {
        #[cfg(target_os = "macos")]
        {
            let mut ticker = tokio::time::interval(std::time::Duration::from_millis(5));
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                ticker.tick().await;
                match footprint(self.pid) {
                    Ok(bytes) if bytes > self.memory_bytes => {
                        return ClientError::new(
                            ErrorKind::LimitExceeded,
                            "worker physical memory limit exceeded",
                        )
                    }
                    Ok(_) => {}
                    // The IPC/wait future observes completion and exit status.
                    Err(error) if error.raw_os_error() == Some(libc::ESRCH) => {
                        return std::future::pending::<ClientError>().await
                    }
                    Err(_) => return failed(),
                }
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            std::future::pending::<ClientError>().await
        }
    }
}
#[cfg(target_os = "macos")]
fn footprint(pid: u32) -> std::io::Result<u64> {
    // SAFETY: rusage_info_v2 consists solely of integers and a byte array; the
    // Darwin API fills exactly that selected version. No task-for-pid entitlement.
    let mut info: libc::rusage_info_v2 = unsafe { std::mem::zeroed() };
    let result = unsafe {
        libc::proc_pid_rusage(
            pid as libc::c_int,
            libc::RUSAGE_INFO_V2,
            (&mut info as *mut libc::rusage_info_v2).cast(),
        )
    };
    if result != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(info.ri_phys_footprint)
}
#[cfg(all(test, target_os = "macos"))]
mod tests {
    #[test]
    fn darwin_reports_real_process_memory() {
        assert!(super::footprint(std::process::id()).unwrap() > 0);
    }
}
