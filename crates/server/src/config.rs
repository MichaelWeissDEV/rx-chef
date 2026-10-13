//! Validated server configuration. Defaults are loopback-only and authenticated.
use std::{net::SocketAddr, path::PathBuf, time::Duration};

use clap::Parser;
use rxchef_client::ExecutionPolicy;

/// Command-line / environment configuration of `rxchef-server`.
#[derive(Debug, Parser)]
#[command(name = "rxchef-server", version, about = "Headless rxchef HTTP / JSON-RPC / WebSocket server")]
pub struct Cli {
    /// Address to listen on. Non-loopback addresses require --allow-remote.
    #[arg(long, default_value = "127.0.0.1:8787")]
    pub listen: SocketAddr,
    /// Explicitly permit binding a non-loopback address.
    #[arg(long)]
    pub allow_remote: bool,
    /// Acknowledge that TLS is terminated in front of this server (the server
    /// itself speaks plain HTTP). Required together with --allow-remote.
    #[arg(long)]
    pub trust_tls_terminator: bool,
    /// Full-access bearer token (min. 16 chars). Prefer --token-file or the
    /// environment; command lines are visible to other local users.
    #[arg(long, env = "RXCHEF_TOKEN", hide_env_values = true)]
    pub token: Option<String>,
    /// Read the full-access token from a file (first line).
    #[arg(long)]
    pub token_file: Option<PathBuf>,
    /// Optional token that may only call discovery (GET) endpoints.
    #[arg(long, env = "RXCHEF_READ_TOKEN", hide_env_values = true)]
    pub read_token: Option<String>,
    /// Additional accepted Host header value (host:port). Required for remote binds.
    #[arg(long = "allowed-host")]
    pub allowed_hosts: Vec<String>,
    /// Additional accepted CORS/WebSocket Origin (scheme://host[:port]).
    #[arg(long = "allowed-origin")]
    pub allowed_origins: Vec<String>,
    /// Maximum HTTP request body in MiB.
    #[arg(long, default_value_t = 16)]
    pub max_body_mib: usize,
    /// Per-job wall-clock limit in seconds; the worker process is killed.
    #[arg(long, default_value_t = 30)]
    pub timeout_secs: u64,
    /// Worker address-space limit in MiB (enforced on Linux only).
    #[arg(long, default_value_t = 1024)]
    pub memory_mib: u64,
    /// Maximum number of concurrently running worker processes.
    #[arg(long, default_value_t = 4)]
    pub workers: usize,
    /// Maximum queued plus running jobs.
    #[arg(long, default_value_t = 64)]
    pub max_pending: usize,
    /// Number of finished jobs retained for result retrieval.
    #[arg(long, default_value_t = 32)]
    pub retain: usize,
    /// Memory budget for retained job results in MiB.
    #[arg(long, default_value_t = 256)]
    pub retain_mib: usize,
    /// Seconds a finished job stays retrievable.
    #[arg(long, default_value_t = 900)]
    pub retain_secs: u64,
    /// Maximum simultaneously served HTTP requests and WebSockets.
    #[arg(long, default_value_t = 256)]
    pub max_connections: usize,
    /// Maximum simultaneous WebSocket connections.
    #[arg(long, default_value_t = 64)]
    pub max_ws_connections: usize,
    /// Maximum concurrent in-flight RPC requests per WebSocket connection.
    #[arg(long, default_value_t = 8)]
    pub max_ws_inflight_per_conn: usize,
    /// Maximum active job subscriptions per WebSocket connection.
    #[arg(long, default_value_t = 16)]
    pub max_ws_subscriptions_per_conn: usize,
    /// WebSocket idle timeout in seconds (close if no ping/pong/message).
    #[arg(long, default_value_t = 120)]
    pub ws_idle_timeout_secs: u64,
    /// Maximum input size in MiB.
    #[arg(long, default_value_t = 8)]
    pub max_input_mib: usize,
    /// Maximum output size in MiB.
    #[arg(long, default_value_t = 8)]
    pub max_output_mib: usize,
    /// Permit operations with network/filesystem/process side effects.
    #[arg(long)]
    pub allow_host_effects: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// Discovery only.
    Read,
    /// Execution and job management.
    Full,
}

#[derive(Clone)]
pub struct ServerConfig {
    pub policy: ExecutionPolicy,
    pub full_token: String,
    pub read_token: Option<String>,
    pub allowed_hosts: Vec<String>,
    pub allowed_origins: Vec<String>,
    pub max_body: usize,
    pub timeout: Duration,
    pub memory_bytes: u64,
    pub workers: usize,
    pub max_pending: usize,
    pub retain: usize,
    pub retain_bytes: usize,
    pub retain_for: Duration,
    pub max_connections: usize,
    pub max_ws_connections: usize,
    pub max_ws_inflight_per_conn: usize,
    pub max_ws_subscriptions_per_conn: usize,
    pub ws_idle_timeout: Duration,
    pub worker_executable: PathBuf,
    /// Set when the token was generated at start-up and must be shown once.
    pub generated_token: bool,
}

// Secrets must never reach logs through `{:?}`.
impl std::fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerConfig")
            .field("full_token", &"<redacted>")
            .field("read_token", &self.read_token.as_ref().map(|_| "<redacted>"))
            .field("allowed_hosts", &self.allowed_hosts)
            .field("allowed_origins", &self.allowed_origins)
            .finish_non_exhaustive()
    }
}

pub fn random_token() -> Result<String, String> {
    let mut bytes = [0u8; 32];
    getrandom::getrandom(&mut bytes).map_err(|e| format!("no system randomness: {e}"))?;
    Ok(hex::encode(bytes))
}

impl ServerConfig {
    /// Test/default constructor for a loopback server on `addr`.
    pub fn loopback(addr: SocketAddr, token: &str, worker_executable: PathBuf) -> Self {
        let mut cfg = Self::from_cli(Cli::parse_from(["rxchef-server"]), worker_executable).expect("defaults");
        cfg.full_token = token.into();
        cfg.generated_token = false;
        cfg.set_hosts(addr, false, &[], &[]);
        cfg
    }

    fn set_hosts(&mut self, addr: SocketAddr, tls: bool, hosts: &[String], origins: &[String]) {
        let port = addr.port();
        let mut allowed: Vec<String> = hosts.iter().map(|h| h.to_lowercase()).collect();
        if addr.ip().is_loopback() {
            allowed.extend([format!("localhost:{port}"), format!("127.0.0.1:{port}"), format!("[::1]:{port}")]);
        }
        if port != 0 {
            allowed.push(addr.to_string().to_lowercase());
        }
        allowed.sort();
        allowed.dedup();
        let mut allowed_origins: Vec<String> = origins.iter().map(|o| o.trim_end_matches('/').to_lowercase()).collect();
        for host in &allowed {
            allowed_origins.push(format!("http://{host}"));
            if tls {
                allowed_origins.push(format!("https://{host}"));
            }
        }
        allowed_origins.sort();
        allowed_origins.dedup();
        self.allowed_hosts = allowed;
        self.allowed_origins = allowed_origins;
    }

    pub fn from_cli(cli: Cli, worker_executable: PathBuf) -> Result<Self, String> {
        if !cli.listen.ip().is_loopback() {
            if !cli.allow_remote {
                return Err("refusing to bind a non-loopback address without --allow-remote".into());
            }
            if !cli.trust_tls_terminator {
                return Err("remote access requires TLS in front of the server; pass --trust-tls-terminator once a TLS reverse proxy is in place".into());
            }
            if cli.allowed_hosts.is_empty() {
                return Err("remote binds require at least one --allowed-host (DNS-rebinding protection)".into());
            }
        }
        for (name, value) in [
            ("workers", cli.workers),
            ("max-pending", cli.max_pending),
            ("max-connections", cli.max_connections),
            ("max-ws-connections", cli.max_ws_connections),
            ("max-ws-inflight-per-conn", cli.max_ws_inflight_per_conn),
            ("max-ws-subscriptions-per-conn", cli.max_ws_subscriptions_per_conn),
            ("max-body-mib", cli.max_body_mib),
            ("max-input-mib", cli.max_input_mib),
            ("max-output-mib", cli.max_output_mib),
        ] {
            if value == 0 {
                return Err(format!("--{name} must be positive"));
            }
        }
        if cli.timeout_secs == 0 {
            return Err("--timeout-secs must be positive".into());
        }
        if cli.ws_idle_timeout_secs == 0 {
            return Err("--ws-idle-timeout-secs must be positive".into());
        }
        // Protect against integer overflow on MiB calculations and ensure bounds match IPC limits
        let mib = 1usize << 20;
        let max_body = cli.max_body_mib.checked_mul(mib).ok_or("max_body_mib overflow")?;
        let max_input_bytes = cli.max_input_mib.checked_mul(mib).ok_or("max_input_mib overflow")?;
        let max_output_bytes = cli.max_output_mib.checked_mul(mib).ok_or("max_output_mib overflow")?;
        let memory_bytes = (cli.memory_mib as u64).checked_mul(1 << 20).ok_or("memory_mib overflow")?;
        let retain_bytes = cli.retain_mib.checked_mul(mib).ok_or("retain_mib overflow")?;

        // Enforce IPC limits: input and output cannot exceed worker IPC envelope
        if max_input_bytes > crate::worker::MAX_REQUEST {
            return Err(format!("max-input-mib exceeds maximum worker IPC request capacity ({} MiB)", crate::worker::MAX_REQUEST / mib));
        }
        if max_output_bytes > crate::worker::MAX_RESPONSE {
            return Err(format!("max-output-mib exceeds maximum worker IPC response capacity ({} MiB)", crate::worker::MAX_RESPONSE / mib));
        }

        let mut generated = false;
        let token = match (cli.token.clone(), &cli.token_file) {
            (Some(_), Some(_)) => return Err("use either --token or --token-file".into()),
            (Some(token), None) => token,
            (None, Some(path)) => std::fs::read_to_string(path)
                .map_err(|_| "cannot read token file".to_string())?
                .lines().next().unwrap_or("").trim().to_string(),
            (None, None) => {
                generated = true;
                random_token()?
            }
        };
        for (label, secret) in [("token", Some(&token)), ("read token", cli.read_token.as_ref())] {
            if secret.is_some_and(|s| s.len() < 16 || !s.bytes().all(|b| b.is_ascii_graphic())) {
                return Err(format!("{label} must be at least 16 printable ASCII characters"));
            }
        }
        if cli.read_token.as_ref() == Some(&token) {
            return Err("read token must differ from the full-access token".into());
        }
        let mut cfg = Self {
            policy: ExecutionPolicy {
                max_input_bytes,
                max_output_bytes,
                allow_host_effects: cli.allow_host_effects,
                ..ExecutionPolicy::default()
            },
            full_token: token,
            read_token: cli.read_token.clone(),
            allowed_hosts: vec![],
            allowed_origins: vec![],
            max_body,
            timeout: Duration::from_secs(cli.timeout_secs),
            memory_bytes,
            workers: cli.workers,
            max_pending: cli.max_pending,
            retain: cli.retain,
            retain_bytes,
            retain_for: Duration::from_secs(cli.retain_secs),
            max_connections: cli.max_connections,
            max_ws_connections: cli.max_ws_connections,
            max_ws_inflight_per_conn: cli.max_ws_inflight_per_conn,
            max_ws_subscriptions_per_conn: cli.max_ws_subscriptions_per_conn,
            ws_idle_timeout: Duration::from_secs(cli.ws_idle_timeout_secs),
            worker_executable,
            generated_token: generated,
        };
        cfg.set_hosts(cli.listen, cli.trust_tls_terminator, &cli.allowed_hosts, &cli.allowed_origins);
        Ok(cfg)
    }

    /// Constant-time token comparison; `None` when the token is unknown.
    pub fn scope_of(&self, presented: &str) -> Option<Scope> {
        use subtle::ConstantTimeEq;
        let eq = |a: &str, b: &str| a.len() == b.len() && bool::from(a.as_bytes().ct_eq(b.as_bytes()));
        let full = eq(presented, &self.full_token);
        let read = self.read_token.as_deref().is_some_and(|t| eq(presented, t));
        if full {
            Some(Scope::Full)
        } else if read {
            Some(Scope::Read)
        } else {
            None
        }
    }
}
