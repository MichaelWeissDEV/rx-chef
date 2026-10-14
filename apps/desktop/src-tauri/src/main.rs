#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rxchef_client::{ClientError, EngineClient, ErrorKind, LocalEngineClient, RemoteEngineClient};
use serde_json::Value;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::State;
use tauri_plugin_dialog::DialogExt;

type Client = Arc<dyn EngineClient + Send + Sync>;
struct Backend {
    client: Mutex<Client>,
    slots: Arc<tokio::sync::Semaphore>,
}
fn error(message: &str) -> ClientError {
    ClientError::new(ErrorKind::InvalidInput, message)
}
fn encode(value: impl serde::Serialize) -> Result<Value, ClientError> {
    serde_json::to_value(value).map_err(|_| error("Cannot encode response"))
}
fn dispatch(client: &dyn EngineClient, method: &str, params: Value) -> Result<Value, ClientError> {
    macro_rules! params {
        () => {
            serde_json::from_value(params).map_err(|_| error("Invalid request parameters"))?
        };
    }
    match method {
        "capabilities" => encode(client.capabilities()?),
        "operations" => encode(client.operations()?),
        "describe" => encode(
            client.describe(
                params["id"]
                    .as_str()
                    .ok_or_else(|| error("Missing operation"))?,
            )?,
        ),
        "validate" => {
            client.validate(&params!())?;
            Ok(Value::Null)
        }
        "graph_validate" => {
            client.validate_graph(&params!())?;
            Ok(Value::Null)
        }
        "graph_execute" => encode(client.execute_graph(params!())?),
        "execute" => encode(client.execute(params!())?),
        "magic" => encode(client.magic(params!())?),
        "scan" => encode(client.scan(params!())?),
        "submit_job" => encode(client.submit_job(params!())?),
        "get_job" => {
            encode(client.job(params["id"].as_str().ok_or_else(|| error("Missing job"))?)?)
        }
        "cancel_job" => {
            encode(client.cancel_job(params["id"].as_str().ok_or_else(|| error("Missing job"))?)?)
        }
        _ => Err(error("Unknown method")),
    }
}
#[tauri::command]
async fn engine_call(
    state: State<'_, Backend>,
    method: String,
    params: Value,
) -> Result<Value, ClientError> {
    let permit =
        state.slots.clone().try_acquire_owned().map_err(|_| {
            ClientError::new(ErrorKind::Busy, "Desktop execution capacity exhausted")
        })?;
    let client = state
        .client
        .lock()
        .map_err(|_| error("Backend unavailable"))?
        .clone();
    tauri::async_runtime::spawn_blocking(move || {
        let _permit = permit;
        dispatch(client.as_ref(), &method, params)
    })
    .await
    .map_err(|_| error("Engine task failed"))?
}
#[tauri::command]
async fn configure_connection(
    state: State<'_, Backend>,
    url: Option<String>,
    token: Option<String>,
) -> Result<(), ClientError> {
    let client: Client =
        tauri::async_runtime::spawn_blocking(move || -> Result<Client, ClientError> {
            match url {
                Some(url) if !url.trim().is_empty() => {
                    let remote =
                        RemoteEngineClient::new(&url, token.as_deref(), Duration::from_secs(60))?;
                    remote.capabilities()?;
                    Ok(Arc::new(remote))
                }
                _ => Ok(Arc::new(LocalEngineClient::default())),
            }
        })
        .await
        .map_err(|_| error("Connection failed"))??;
    *state
        .client
        .lock()
        .map_err(|_| error("Backend unavailable"))? = client;
    Ok(())
}
#[tauri::command]
async fn import_file(app: tauri::AppHandle) -> Result<Option<String>, ClientError> {
    tauri::async_runtime::spawn_blocking(move || {
        let Some(path) = app.dialog().file().blocking_pick_file() else {
            return Ok(None);
        };
        let path = path
            .into_path()
            .map_err(|_| error("Unsupported file location"))?;
        use std::io::Read;
        let file = std::fs::File::open(path).map_err(|_| error("Cannot open selected file"))?;
        let mut bytes = Vec::new();
        file.take((8 << 20) + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| error("Cannot read selected file"))?;
        if bytes.len() > 8 << 20 {
            return Err(error("File exceeds 8 MiB input limit"));
        }
        Ok(Some(STANDARD.encode(bytes)))
    })
    .await
    .map_err(|_| error("File dialog failed"))?
}
#[tauri::command]
async fn export_file(app: tauri::AppHandle, data: String) -> Result<(), ClientError> {
    if data.len() > 12 << 20 {
        return Err(error("Output exceeds export limit"));
    }
    let bytes = STANDARD
        .decode(data)
        .map_err(|_| error("Invalid output bytes"))?;
    tauri::async_runtime::spawn_blocking(move || {
        let Some(path) = app
            .dialog()
            .file()
            .set_file_name("rxchef_output.bin")
            .blocking_save_file()
        else {
            return Ok(());
        };
        let path = path
            .into_path()
            .map_err(|_| error("Unsupported file location"))?;
        std::fs::write(path, bytes).map_err(|_| error("Cannot write selected file"))
    })
    .await
    .map_err(|_| error("File dialog failed"))?
}
#[cfg(feature = "ui-test")]
#[tauri::command]
fn ui_test_result(app: tauri::AppHandle, result: Value) {
    if let Some(path) =
        std::env::args().find_map(|arg| arg.strip_prefix("--ui-test-output=").map(str::to_owned))
    {
        let passed = result["passed"] == true;
        let _ = std::fs::write(path, serde_json::to_vec_pretty(&result).unwrap());
        app.exit(if passed { 0 } else { 1 });
    }
}
fn main() {
    let builder = tauri::Builder::default();
    #[cfg(feature = "ui-test")]
    let builder = builder.on_page_load(|webview, payload| {
        if std::env::args().any(|arg| arg.starts_with("--ui-test-output="))
            && matches!(payload.event(), tauri::webview::PageLoadEvent::Finished)
        {
            let _ = webview.eval(include_str!("../../tests/webview-smoke.js"));
        }
    });
    #[cfg(feature = "ui-test")]
    let builder = builder.invoke_handler(tauri::generate_handler![
        engine_call,
        configure_connection,
        import_file,
        export_file,
        ui_test_result
    ]);
    #[cfg(not(feature = "ui-test"))]
    let builder = builder.invoke_handler(tauri::generate_handler![
        engine_call,
        configure_connection,
        import_file,
        export_file
    ]);
    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(Backend {
            client: Mutex::new(Arc::new(LocalEngineClient::default())),
            slots: Arc::new(tokio::sync::Semaphore::new(4)),
        })
        .run(tauri::generate_context!())
        .expect("Cannot start rx-chef desktop");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn local_ipc_roundtrip_needs_no_server() {
        let bytes: Vec<u8> = (0..=255).collect();
        let response = dispatch(
            &LocalEngineClient::default(),
            "execute",
            serde_json::json!({ "input_base64": STANDARD.encode(&bytes), "recipe": [] }),
        )
        .unwrap();
        assert_eq!(
            STANDARD
                .decode(response["result"]["output_base64"].as_str().unwrap())
                .unwrap(),
            bytes
        );
        assert_eq!(
            dispatch(&LocalEngineClient::default(), "operations", Value::Null)
                .unwrap()
                .as_array()
                .unwrap()
                .len(),
            478
        );
    }
}

#[cfg(test)]
mod graph_tests {
    use super::*;
    #[test]
    fn native_graph_dispatch_runs_without_a_server_and_preserves_binary() {
        let request = rxchef_client::GraphRequest {
            graph: serde_json::from_value(serde_json::json!({
                "version": 1, "id": "native",
                "nodes": [{"id":"input","type":"input","name":"input"}, {"id":"output","type":"output","name":"output"}],
                "edges": [{"id":"edge","source_node":"input","source_port":"data","target_node":"output","target_port":"data"}]
            })).unwrap(),
            inputs_base64: std::collections::BTreeMap::from([(
                "input".into(),
                STANDARD.encode([0, 255, 192]),
            )]),
            outputs: vec![],
        };
        let value = dispatch(
            &LocalEngineClient::default(),
            "graph_execute",
            serde_json::to_value(request).unwrap(),
        )
        .unwrap();
        let result: rxchef_client::GraphResult = serde_json::from_value(value).unwrap();
        assert_eq!(
            rxchef_client::graph_output_bytes(&result.outputs[0]).unwrap(),
            [0, 255, 192]
        );
    }
}
