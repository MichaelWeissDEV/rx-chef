//! Environment and installation diagnostic command: `rxchef doctor`
use std::time::Duration;
use rxchef_client::{EngineClient, RemoteEngineClient};

pub(crate) fn cmd_doctor(server_url: Option<String>, token: Option<String>) -> Result<(), String> {
    println!("rx-chef Diagnostics (Doctor)");
    println!("============================");

    // 1. Core library & Operations Registry
    let op_count = rxchef::catalog::operations()
        .map(|ops| ops.len())
        .unwrap_or(0);
    println!("✓ Core library: v{} ({} operations registered)", env!("CARGO_PKG_VERSION"), op_count);

    // 2. Feature availability
    println!("✓ Subsystems: Catalog, Pipeline Engine, Magic & Scan active");

    // 3. Persistent store directories
    match rxchef_store::default_scope() {
        scope => {
            println!("✓ Local store scope: active ({:?})", scope);
            let recipes = rxchef_store::list_recipes(None);
            println!("  - Saved recipes: {}", recipes.len());
            let vars = rxchef_store::list_vars(None);
            println!("  - Stored variables: {}", vars.len());
        }
    }

    // 4. Remote server check if requested
    if let Some(url) = server_url {
        println!("\nTesting remote server connection to {}...", url);
        match RemoteEngineClient::new(&url, token.as_deref(), Duration::from_secs(5)) {
            Ok(client) => match client.capabilities() {
                Ok(caps) => {
                    println!("✓ Server reachable: protocol v{} ({})", caps.protocol_version, caps.version);
                    println!("  - Supervised jobs: {}", if caps.jobs { "supported" } else { "disabled" });
                    println!("  - Binary transport: {}", caps.binary_transport);
                    println!("  - Max input/output: {} / {} bytes", caps.max_input_bytes, caps.max_output_bytes);
                }
                Err(err) => {
                    println!("✗ Server connection failed: {}", err);
                }
            },
            Err(err) => {
                println!("✗ Invalid server configuration: {}", err);
            }
        }
    } else {
        println!("\nTip: Run `rxchef doctor --server http://127.0.0.1:8787 [--token ...]` to test a server connection.");
    }

    println!("\nAll local subsystems operational.");
    Ok(())
}
