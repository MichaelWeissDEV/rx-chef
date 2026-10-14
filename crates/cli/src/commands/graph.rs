//! Graph commands bind local files to named byte inputs. Paths never cross the API.
use crate::{cli::GraphCommand, error::CliError, output};
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
pub(crate) fn cmd_graph(command: GraphCommand) -> Result<(), CliError> {
    match command {
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
                    || !paths.insert(path.to_owned())
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
            let result = client
                .execute_graph(GraphRequest {
                    graph: g,
                    inputs_base64: inputs,
                    outputs: requested,
                })
                .map_err(|e| CliError::Execution(e.to_string()))?;
            let failed = result.outputs.iter().any(|o| o.error.is_some());
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
