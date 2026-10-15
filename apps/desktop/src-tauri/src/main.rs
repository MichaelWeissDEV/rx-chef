#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rxchef_client::{
    ClientError, EngineClient, ErrorKind, RemoteEngineClient, SupervisedEngineClient,
};
use serde_json::Value;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::State;
use tauri_plugin_dialog::DialogExt;

#[cfg(test)]
use rxchef_client::LocalEngineClient;

type Client = Arc<dyn EngineClient + Send + Sync>;
struct JobRoute {
    client: Client,
    _permit: tokio::sync::OwnedSemaphorePermit,
}
struct Backend {
    client: Mutex<Client>,
    slots: Arc<tokio::sync::Semaphore>,
    local: Arc<SupervisedEngineClient>,
    jobs: Mutex<std::collections::BTreeMap<String, JobRoute>>,
    job_slots: Arc<tokio::sync::Semaphore>,
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
        "project_validate" => encode(client.validate_project(params!())?),
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
    let job_id = params.get("id").and_then(Value::as_str).map(str::to_owned);
    let job_client = job_id.as_ref().and_then(|id| {
        state
            .jobs
            .lock()
            .ok()?
            .get(id)
            .map(|route| route.client.clone())
    });
    let job_permit = if method == "submit_job" {
        Some(state.job_slots.clone().try_acquire_owned().map_err(|_| {
            ClientError::new(
                ErrorKind::Busy,
                "Desktop job registry capacity exhausted; collect or cancel previous jobs",
            )
        })?)
    } else {
        None
    };
    let client = state
        .client
        .lock()
        .map_err(|_| error("Backend unavailable"))?
        .clone();
    let client = job_client.unwrap_or(client);
    let remembered = client.clone();
    let job_method = method.clone();
    let response = tauri::async_runtime::spawn_blocking(move || {
        let _permit = permit;
        dispatch(client.as_ref(), &method, params)
    })
    .await
    .map_err(|_| error("Engine task failed"))??;
    if job_method == "submit_job" {
        if let Some(id) = response.get("id").and_then(Value::as_str) {
            state
                .jobs
                .lock()
                .map_err(|_| error("Job registry unavailable"))?
                .insert(
                    id.to_owned(),
                    JobRoute {
                        client: remembered,
                        _permit: job_permit.expect("submit lease"),
                    },
                );
        }
    }
    if matches!(job_method.as_str(), "get_job" | "cancel_job")
        && !matches!(
            response.get("status").and_then(Value::as_str),
            Some("queued" | "running")
        )
    {
        if let Some(id) = job_id {
            state
                .jobs
                .lock()
                .map_err(|_| error("Job registry unavailable"))?
                .remove(&id);
        }
    }
    Ok(response)
}
#[tauri::command]
async fn configure_connection(
    state: State<'_, Backend>,
    url: Option<String>,
    token: Option<String>,
) -> Result<(), ClientError> {
    let local = state.local.clone();
    let client: Client =
        tauri::async_runtime::spawn_blocking(move || -> Result<Client, ClientError> {
            match url {
                Some(url) if !url.trim().is_empty() => {
                    let remote =
                        RemoteEngineClient::new(&url, token.as_deref(), Duration::from_secs(60))?;
                    remote.capabilities()?;
                    Ok(Arc::new(remote))
                }
                _ => Ok(local),
            }
        })
        .await
        .map_err(|_| error("Connection failed"))??;
    let jobs = std::mem::take(
        &mut *state
            .jobs
            .lock()
            .map_err(|_| error("Job registry unavailable"))?,
    );
    tauri::async_runtime::spawn_blocking(move || {
        for (id, route) in jobs {
            let _ = route.client.cancel_job(&id);
        }
    })
    .await
    .map_err(|_| error("Session cancellation failed"))?;
    *state
        .client
        .lock()
        .map_err(|_| error("Backend unavailable"))? = client;
    Ok(())
}
#[tauri::command]
async fn import_file(
    app: tauri::AppHandle,
    limit: Option<usize>,
) -> Result<Option<String>, ClientError> {
    let limit = limit.unwrap_or(8 << 20);
    if limit == 0 || limit > 16 << 20 {
        return Err(error("Invalid file import limit"));
    }
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
        file.take(limit as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| error("Cannot read selected file"))?;
        if bytes.len() > limit {
            return Err(error("File exceeds selected import limit"));
        }
        Ok(Some(STANDARD.encode(bytes)))
    })
    .await
    .map_err(|_| error("File dialog failed"))?
}
fn checked_export_filename(filename: Option<String>) -> Result<String, ClientError> {
    let filename = filename.unwrap_or_else(|| "rxchef_output.bin".into());
    if filename.is_empty()
        || filename.len() > 512
        || filename == "."
        || filename == ".."
        || filename
            .chars()
            .any(|c| c.is_control() || c == '/' || c == '\\')
    {
        return Err(error("Invalid suggested export filename"));
    }
    Ok(filename)
}

#[tauri::command]
async fn export_file(
    app: tauri::AppHandle,
    data: String,
    filename: Option<String>,
) -> Result<bool, ClientError> {
    let filename = checked_export_filename(filename)?;
    if data.len() > 24 << 20 {
        return Err(error("Output exceeds export limit"));
    }
    let bytes = STANDARD
        .decode(data)
        .map_err(|_| error("Invalid output bytes"))?;
    if bytes.len() > 16 << 20 {
        return Err(error("Output exceeds 16 MiB export limit"));
    }
    tauri::async_runtime::spawn_blocking(move || {
        let Some(path) = app
            .dialog()
            .file()
            .set_file_name(&filename)
            .blocking_save_file()
        else {
            return Ok(false);
        };
        let path = path
            .into_path()
            .map_err(|_| error("Unsupported file location"))?;
        write_export(&path, &bytes)?;
        Ok(true)
    })
    .await
    .map_err(|_| error("File dialog failed"))?
}
fn write_export(path: &std::path::Path, bytes: &[u8]) -> Result<(), ClientError> {
    use std::io::Write;
    let parent = path
        .parent()
        .ok_or_else(|| error("Invalid export location"))?;
    let mut temporary =
        tempfile::NamedTempFile::new_in(parent).map_err(|_| error("Cannot create export file"))?;
    temporary
        .write_all(bytes)
        .and_then(|_| temporary.as_file().sync_all())
        .map_err(|_| error("Cannot write selected file"))?;
    temporary
        .persist(path)
        .map_err(|_| error("Cannot replace selected file"))?;
    Ok(())
}
#[cfg(feature = "ui-test")]
#[tauri::command]
fn ui_test_progress(stage: String) {
    if let Some(path) =
        std::env::args().find_map(|arg| arg.strip_prefix("--ui-test-output=").map(str::to_owned))
    {
        let _ = std::fs::write(format!("{path}.progress"), stage);
    }
}
#[cfg(feature = "ui-test")]
#[tauri::command]
fn ui_test_result(app: tauri::AppHandle, mut result: Value) {
    if let Some(path) =
        std::env::args().find_map(|arg| arg.strip_prefix("--ui-test-output=").map(str::to_owned))
    {
        result["executable"] = std::env::current_exe()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_default()
            .into();
        let passed = result["passed"] == true;
        let _ = std::fs::write(path, serde_json::to_vec_pretty(&result).unwrap());
        app.exit(if passed { 0 } else { 1 });
    }
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("--internal-worker") {
        std::process::exit(if rxchef_client::worker::run().is_ok() {
            0
        } else {
            70
        });
    }
    let worker_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Cannot start native worker runtime");
    let local = Arc::new(
        SupervisedEngineClient::new(
            rxchef_client::jobs::WorkerConfig::local(
                std::env::current_exe().expect("Cannot locate desktop executable"),
            ),
            worker_runtime.handle().clone(),
        )
        .expect("Cannot initialize local engine"),
    );
    let shutdown_local = local.clone();
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
        ui_test_result,
        ui_test_progress
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
            client: Mutex::new(local.clone()),
            local,
            jobs: Mutex::default(),
            job_slots: Arc::new(tokio::sync::Semaphore::new(64)),
            slots: Arc::new(tokio::sync::Semaphore::new(4)),
        })
        .run(tauri::generate_context!())
        .expect("Cannot start rx-chef desktop");
    worker_runtime.block_on(shutdown_local.shutdown());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn suggested_export_names_cannot_be_paths_or_controls() {
        assert_eq!(checked_export_filename(None).unwrap(), "rxchef_output.bin");
        assert_eq!(
            checked_export_filename(Some("Team.rxproject.json".into())).unwrap(),
            "Team.rxproject.json"
        );
        for name in [
            "",
            ".",
            "..",
            "../outside.json",
            "folder/file.json",
            "folder\\file.json",
            "name\n.json",
        ] {
            assert!(
                checked_export_filename(Some(name.into())).is_err(),
                "{name:?}"
            );
        }
    }
    #[test]
    fn exports_replace_existing_files_with_exact_bytes() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("output.bin");
        std::fs::write(&path, b"old data").unwrap();
        let bytes: Vec<u8> = (0..=255).collect();
        write_export(&path, &bytes).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 1);
        assert!(write_export(&directory.path().join("missing/output.bin"), b"data").is_err());
    }
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
