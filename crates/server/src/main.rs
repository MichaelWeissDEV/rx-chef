use std::process::ExitCode;

use clap::Parser;
use rxchef_server::{worker, Cli, ServerConfig};

fn main() -> ExitCode {
    // Internal supervised-worker mode: only ever spawned by the server itself,
    // before any async runtime exists. Never reachable from a request.
    if std::env::args().nth(1).as_deref() == Some("--internal-worker") {
        return match worker::run() {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::from(70),
        };
    }
    let cli = Cli::parse();
    let listen = cli.listen;
    let executable = match std::env::current_exe() {
        Ok(path) => path,
        Err(error) => return fail(format!("cannot locate own executable: {error}")),
    };
    let config = match ServerConfig::from_cli(cli, executable) {
        Ok(config) => config,
        Err(error) => return fail(error),
    };
    let runtime = match tokio::runtime::Builder::new_multi_thread().enable_all().build() {
        Ok(runtime) => runtime,
        Err(error) => return fail(format!("cannot start runtime: {error}")),
    };
    runtime.block_on(async move {
        let listener = match tokio::net::TcpListener::bind(listen).await {
            Ok(listener) => listener,
            Err(error) => return fail(format!("cannot bind {listen}: {error}")),
        };
        eprintln!("rxchef-server {} listening on http://{}", env!("CARGO_PKG_VERSION"), listener.local_addr().map_or(listen, |a| a));
        if config.generated_token {
            // Shown exactly once, on stderr only; never written to disk or logs.
            eprintln!("generated access token (not stored): {}", config.full_token);
        }
        match rxchef_server::serve(listener, config, shutdown_signal()).await {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => fail(format!("server error: {error}")),
        }
    })
}

fn fail(message: String) -> ExitCode {
    eprintln!("rxchef-server: {message}");
    ExitCode::from(2)
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        if let Ok(mut term) = signal(SignalKind::terminate()) {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {}
                _ = term.recv() => {}
            }
            return;
        }
    }
    let _ = tokio::signal::ctrl_c().await;
}
