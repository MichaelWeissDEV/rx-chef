//! Graph commands bind local files to named byte inputs. Paths never cross the API.
use crate::{
    cli::{GraphCommand, GraphStoreCommand},
    error::CliError,
    output,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rxchef::graph::{Graph, GraphPolicy};
use rxchef_client::{
    graph_output_bytes, EngineClient, GraphRequest, LocalEngineClient, RemoteEngineClient,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, Write},
    path::Path,
    time::Duration,
};

fn read_bounded(path: &Path, limit: usize) -> Result<Vec<u8>, CliError> {
    let file = std::fs::File::open(path).map_err(|e| CliError::StoreIo(e.to_string()))?;
    let mut bytes = Vec::new();
    file.take(limit.saturating_add(1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| CliError::StoreIo(e.to_string()))?;
    if bytes.len() > limit {
        return Err(CliError::InvalidInput("file exceeds byte limit".into()));
    }
    Ok(bytes)
}
fn load(path: &Path) -> Result<Graph, CliError> {
    let policy = GraphPolicy::default();
    Graph::from_json(
        &read_bounded(path, policy.limits.max_definition_bytes)?,
        &policy,
    )
    .map_err(|e| CliError::InvalidInput(e.to_string()))
}
fn json(value: &impl serde::Serialize) -> Result<(), CliError> {
    serde_json::to_writer_pretty(std::io::stdout().lock(), value)
        .map_err(|e| CliError::StoreIo(e.to_string()))?;
    println!();
    Ok(())
}
fn destination_key(path: &Path) -> Result<std::path::PathBuf, CliError> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let name = path
        .file_name()
        .ok_or_else(|| CliError::InvalidInput("invalid output destination".into()))?;
    let absolute = parent
        .canonicalize()
        .map_err(|error| CliError::StoreIo(error.to_string()))?
        .join(name);
    #[cfg(any(target_os = "macos", windows))]
    {
        Ok(std::path::PathBuf::from(
            absolute.to_string_lossy().to_lowercase(),
        ))
    }
    #[cfg(not(any(target_os = "macos", windows)))]
    {
        Ok(absolute)
    }
}
pub(crate) fn cmd_graph(command: GraphCommand) -> Result<(), CliError> {
    match command {
        GraphCommand::Store { global, command } => {
            let scope = if global {
                rxchef_store::Scope::Global
            } else {
                rxchef_store::default_scope()
            };
            let repository = rxchef_store::GraphRepository::in_scope(scope)
                .map_err(|e| CliError::StoreIo(e.to_string()))?;
            let stored = |error: rxchef_store::StoreError| CliError::StoreIo(error.to_string());
            match command {
                GraphStoreCommand::List => json(&repository.list().map_err(stored)?),
                GraphStoreCommand::Save {
                    name,
                    graph,
                    replace,
                } => repository
                    .save(&name, &load(&graph)?, replace)
                    .map_err(stored),
                GraphStoreCommand::Load { name, destination } => {
                    let bytes = repository.export(&name).map_err(stored)?;
                    if let Some(path) = destination {
                        output::atomic_write(&path, &bytes).map_err(CliError::StoreIo)
                    } else {
                        std::io::stdout()
                            .lock()
                            .write_all(&bytes)
                            .map_err(|e| CliError::StoreIo(e.to_string()))?;
                        println!();
                        Ok(())
                    }
                }
                GraphStoreCommand::Rename { source, target } => {
                    repository.rename(&source, &target).map_err(stored)
                }
                GraphStoreCommand::Duplicate { source, target } => {
                    repository.duplicate(&source, &target).map_err(stored)
                }
                GraphStoreCommand::Delete { name } => repository.delete(&name).map_err(stored),
            }
        }
        GraphCommand::Validate { graph } => {
            let g = load(&graph)?;
            let plan = rxchef::graph::validate(&g, &GraphPolicy::default())
                .map_err(|e| CliError::InvalidInput(e.to_string()))?;
            json(
                &serde_json::json!({ "valid": true, "nodes": g.nodes.len(), "edges": g.edges.len(), "disconnected": plan.disconnected }),
            )
        }
        GraphCommand::Inspect { graph } => json(&load(&graph)?),
        GraphCommand::Run {
            graph,
            input,
            output: destinations,
            selected,
            json: as_json,
            history,
            server,
            token,
        } => {
            let g = load(&graph)?;
            let limit = GraphPolicy::default().limits.max_input_bytes;
            let mut inputs = BTreeMap::new();
            let mut total = 0usize;
            for binding in input {
                let (name, path) = binding
                    .split_once('=')
                    .ok_or_else(|| CliError::InvalidInput("input requires NAME=FILE".into()))?;
                if inputs.contains_key(name) {
                    return Err(CliError::InvalidInput("duplicate input name".into()));
                }
                let bytes = read_bounded(Path::new(path), limit.saturating_sub(total))?;
                total += bytes.len();
                inputs.insert(name.into(), STANDARD.encode(bytes));
            }
            let mut files = BTreeMap::new();
            let mut paths = BTreeSet::new();
            for binding in destinations {
                let (name, path) = binding
                    .split_once('=')
                    .ok_or_else(|| CliError::InvalidInput("output requires NAME=FILE".into()))?;
                if files.insert(name.to_owned(), path.to_owned()).is_some()
                    || !paths.insert(destination_key(Path::new(path))?)
                {
                    return Err(CliError::InvalidInput(
                        "duplicate output name or path".into(),
                    ));
                }
            }
            if as_json && !files.is_empty() {
                return Err(CliError::InvalidInput(
                    "--json cannot be combined with --output".into(),
                ));
            }
            let requested: Vec<String> = if let Some(name) = selected {
                vec![name]
            } else if !files.is_empty() {
                files.keys().cloned().collect()
            } else {
                vec![]
            };
            // Refuse ambiguous binary stdout before executing anything.
            let output_count = if requested.is_empty() {
                g.nodes
                    .iter()
                    .filter(|n| matches!(n.kind, rxchef::graph::NodeKind::Output { .. }))
                    .count()
            } else {
                requested.len()
            };
            if !as_json && files.is_empty() && output_count != 1 {
                return Err(CliError::InvalidInput(
                    "binary stdout requires exactly one output; use --select or --json".into(),
                ));
            }
            if !files.is_empty() && !requested.iter().all(|name| files.contains_key(name)) {
                return Err(CliError::InvalidInput(
                    "selected output has no destination".into(),
                ));
            }
            let token = token.or_else(|| std::env::var("RXCHEF_TOKEN").ok());
            let client: Box<dyn EngineClient> = match server {
                Some(url) => Box::new(
                    RemoteEngineClient::new(&url, token.as_deref(), Duration::from_secs(60))
                        .map_err(|e| CliError::Execution(e.to_string()))?,
                ),
                None => Box::new(LocalEngineClient::default()),
            };
            if !client
                .capabilities()
                .map_err(|e| CliError::Execution(e.to_string()))?
                .graphs
            {
                return Err(CliError::FeatureUnavailable(
                    "server does not support graphs".into(),
                ));
            }
            let graph_id = g.id.clone();
            let started = std::time::Instant::now();
            let started_at = chrono::Utc::now().to_rfc3339();
            let result = client
                .execute_graph(GraphRequest {
                    graph: g,
                    inputs_base64: inputs,
                    outputs: requested,
                })
                .map_err(|e| CliError::Execution(e.to_string()))?;
            let failed = result.outputs.iter().any(|o| o.error.is_some());
            if history {
                let id = rxchef_store::new_history_id();
                rxchef_store::workbench::WorkbenchRun {
                    id: id.clone(),
                    run_id: id,
                    graph_id,
                    started_at,
                    status: if failed {
                        rxchef_store::workbench::WorkbenchRunStatus::Failed
                    } else {
                        rxchef_store::workbench::WorkbenchRunStatus::Succeeded
                    },
                    elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
                    output_sizes: result
                        .outputs
                        .iter()
                        .map(|output| (output.name.clone(), output.output_bytes))
                        .collect(),
                    error_code: if failed {
                        Some(rxchef_store::workbench::WorkbenchErrorCode::Operation)
                    } else {
                        None
                    },
                }
                .append_history(total)
                .map_err(|e| CliError::StoreIo(e.to_string()))?;
            }
            if as_json {
                json(&result)?;
            } else if files.is_empty() {
                let value = result
                    .outputs
                    .first()
                    .ok_or_else(|| CliError::Execution("no graph output".into()))?;
                let bytes =
                    graph_output_bytes(value).map_err(|e| CliError::Execution(e.to_string()))?;
                std::io::stdout()
                    .lock()
                    .write_all(&bytes)
                    .map_err(|e| CliError::StoreIo(e.to_string()))?;
            } else {
                for value in &result.outputs {
                    if let Some(error) = &value.error {
                        eprintln!("{}: {error}", value.name);
                        continue;
                    }
                    let bytes = graph_output_bytes(value)
                        .map_err(|e| CliError::Execution(e.to_string()))?;
                    let path = files
                        .get(&value.name)
                        .ok_or_else(|| CliError::Execution("unexpected output name".into()))?;
                    output::atomic_write(Path::new(path), &bytes).map_err(CliError::StoreIo)?;
                }
            }
            if failed {
                return Err(CliError::Execution(
                    "one or more graph outputs failed".into(),
                ));
            }
            Ok(())
        }
        GraphCommand::List { directory } => {
            let mut paths = Vec::new();
            if directory.exists() {
                for entry in
                    std::fs::read_dir(directory).map_err(|e| CliError::StoreIo(e.to_string()))?
                {
                    let path = entry.map_err(|e| CliError::StoreIo(e.to_string()))?.path();
                    if path.is_file()
                        && path
                            .file_name()
                            .is_some_and(|n| n.to_string_lossy().ends_with(".rxgraph.json"))
                    {
                        paths.push(path);
                    }
                }
            }
            paths.sort();
            json(&paths)
        }
        GraphCommand::Import { graph, destination }
        | GraphCommand::Export { graph, destination } => {
            let g = load(&graph)?;
            // Explicit file operations preserve definitions only. Input payloads
            // and output artifacts are never included in the graph format.
            let bytes =
                serde_json::to_vec_pretty(&g).map_err(|e| CliError::StoreIo(e.to_string()))?;
            output::atomic_write(&destination, &bytes).map_err(CliError::StoreIo)
        }
    }
}
