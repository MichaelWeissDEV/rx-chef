//! Test server connectivity and print capabilities: `rxchef connect`
use rxchef_client::{EngineClient, RemoteEngineClient};
use std::time::Duration;

pub(crate) fn cmd_connect(url: String, token: Option<String>) -> Result<(), String> {
    println!("Connecting to rxchef server at {}...", url);
    let client = RemoteEngineClient::new(&url, token.as_deref(), Duration::from_secs(10))
        .map_err(|e| format!("Client initialization error: {e}"))?;

    let caps = client
        .capabilities()
        .map_err(|e| format!("Failed to fetch server capabilities: {e}"))?;

    println!("✓ Successfully connected to rxchef-server!");
    println!("  Server Version:   {}", caps.version);
    println!("  Protocol Version: {}", caps.protocol_version);
    println!("  Jobs Supported:   {}", caps.jobs);
    println!("  Max Input Bytes:  {}", caps.max_input_bytes);
    println!("  Max Output Bytes: {}", caps.max_output_bytes);
    println!("  Max Steps:        {}", caps.max_steps);
    Ok(())
}
