//! Evidence-based environment checks: requested remote failures are command failures.
use rxchef_client::{EngineClient, LocalEngineClient, RemoteEngineClient, PROTOCOL_VERSION};
use std::time::Duration;

pub(crate) fn cmd_doctor(server_url: Option<String>, token: Option<String>) -> Result<(), String> {
    let local = LocalEngineClient::default();
    let operations = local.operations().map_err(|e| e.to_string())?;
    if operations.is_empty() {
        println!("FAIL Core: empty operations registry");
        return Err("Core registry check failed".into());
    }
    let capabilities = local.capabilities().map_err(|e| e.to_string())?;
    println!(
        "PASS Core/client v{}: {} operations, protocol {}",
        capabilities.version,
        operations.len(),
        capabilities.protocol_version
    );
    println!("PASS Graph capability: {}", capabilities.graphs);
    let available = operations
        .iter()
        .filter(|op| matches!(op.availability, rxchef::operation::Availability::Available))
        .count();
    println!(
        "PASS Feature matrix: {available}/{} operations available in this build",
        operations.len()
    );
    let scope = rxchef_store::default_scope();
    let directory = match scope {
        rxchef_store::Scope::Global => rxchef_store::global_dir(),
        rxchef_store::Scope::Project => rxchef_store::project_dir(),
    };
    if directory.exists() {
        if let Err(error) = std::fs::read_dir(&directory) {
            println!("FAIL Store: cannot read {}", directory.display());
            return Err(format!("Store access check failed: {error}"));
        }
        println!(
            "PASS Store read access: {} ({scope:?})",
            directory.display()
        );
        println!("NOT TESTED Store write access (diagnostics do not modify stored data)");
    } else {
        println!(
            "WARN Store directory not initialized: {}",
            directory.display()
        );
    }
    if let Some(url) = server_url {
        let result = RemoteEngineClient::new(&url, token.as_deref(), Duration::from_secs(5))
            .and_then(|client| client.capabilities());
        match result {
            Ok(caps) => {
                if caps.protocol_version != PROTOCOL_VERSION
                    || !caps.graphs
                    || caps.binary_transport != "base64"
                {
                    println!(
                        "FAIL Server compatibility: protocol {}, graphs {}, binary {}",
                        caps.protocol_version, caps.graphs, caps.binary_transport
                    );
                    return Err("Requested server is incompatible with graph workflows".into());
                }
                println!(
                    "PASS Server v{}: protocol {}, graphs {}, supervised jobs {}",
                    caps.version, caps.protocol_version, caps.graphs, caps.jobs
                );
                println!(
                    "PASS Server input/output limits: {} / {} bytes",
                    caps.max_input_bytes, caps.max_output_bytes
                );
            }
            Err(error) => {
                println!("FAIL Server check: {error}");
                return Err("Requested remote diagnostic failed".into());
            }
        }
    } else {
        println!("NOT TESTED Remote server (supply --server to check)");
    }
    println!("NOT TESTED Desktop installation and native launch");
    Ok(())
}
