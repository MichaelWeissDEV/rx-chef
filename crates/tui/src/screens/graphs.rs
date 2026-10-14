//! File-based graph workbench. Domain validation and execution remain in Rust Core.
use crate::events::{execute_task, Client};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use rxchef::graph::{Graph, NodeKind};
use rxchef_client::{graph_output_bytes, EngineTask, GraphRequest, GraphResult, TaskResult};
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
};

enum Event {
    Loaded(Result<Graph, String>),
    Listed(Result<Vec<PathBuf>, String>),
    Bound(u64, String, Result<Vec<u8>, String>),
    Status(u64, String),
    Progress(u64, u64, Vec<rxchef_client::GraphNodeProgress>),
    Done(u64, u64, Result<GraphResult, String>),
}
#[derive(Clone)]
enum Prompt {
    Load,
    Bind(String, &'static str),
    Export,
    Save,
}
struct Active {
    id: u64,
    cancel: Arc<AtomicBool>,
    thread: std::thread::JoinHandle<()>,
}
pub struct GraphScreen {
    client: Client,
    graph: Option<Graph>,
    loading: bool,
    inputs: BTreeMap<String, Vec<u8>>,
    pub status: String,
    result: Option<GraphResult>,
    progress: Vec<rxchef_client::GraphNodeProgress>,
    selected: usize,
    output: usize,
    revision: u64,
    next_run: u64,
    active: Option<Active>,
    prompt: Option<Prompt>,
    text: String,
    listing: Vec<PathBuf>,
    list_index: usize,
    tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,
}
fn read_bounded(path: &Path, maximum: usize) -> Result<Vec<u8>, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("Select a regular file".into());
    }
    let mut bytes = Vec::new();
    file.take(maximum as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > maximum {
        return Err(format!("File exceeds {maximum} bytes"));
    }
    Ok(bytes)
}
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut file = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    file.write_all(bytes)
        .and_then(|_| file.as_file().sync_all())
        .map_err(|e| e.to_string())?;
    file.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}
fn check_export(graph: &Graph) -> Result<(), String> {
    rxchef_store::check_graph_export(graph).map_err(|error| error.to_string())
}
impl GraphScreen {
    pub fn new(client: Client) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            client,
            graph: None,
            loading: false,
            inputs: BTreeMap::new(),
            status: "Load a graph file or directory with l. Tab returns to recipes.".into(),
            result: None,
            progress: Vec::new(),
            selected: 0,
            output: 0,
            revision: 0,
            next_run: 0,
            active: None,
            prompt: None,
            text: String::new(),
            listing: Vec::new(),
            list_index: 0,
            tx,
            rx,
        }
    }
    pub fn editing(&self) -> bool {
        self.prompt.is_some() || !self.listing.is_empty()
    }
    pub fn load(&mut self, path: PathBuf) {
        if self.loading {
            self.status = "A graph load is already pending".into();
            return;
        }
        if self.active.is_some() {
            self.status = "Cancel the current job before loading another graph".into();
            return;
        }
        self.status = "Loading and validating graph…".into();
        self.loading = true;
        let tx = self.tx.clone();
        let client = self.client.clone();
        std::thread::spawn(move || {
            if path.is_dir() {
                let result = (|| {
                    let mut files = Vec::new();
                    for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
                        let entry = entry.map_err(|e| e.to_string())?;
                        if entry.file_type().map_err(|e| e.to_string())?.is_file()
                            && entry.path().extension().is_some_and(|ext| ext == "json")
                        {
                            if files.len() == 256 {
                                return Err("Directory contains more than 256 JSON files; select a file directly".into());
                            }
                            files.push(entry.path());
                        }
                    }
                    files.sort();
                    Ok(files)
                })();
                let _ = tx.send(Event::Listed(result));
                return;
            }
            let result = (|| {
                let graph: Graph = serde_json::from_slice(&read_bounded(&path, 4 << 20)?)
                    .map_err(|e| e.to_string())?;
                client
                    .validate_graph(&GraphRequest {
                        graph: graph.clone(),
                        inputs_base64: BTreeMap::new(),
                        outputs: Vec::new(),
                    })
                    .map_err(|e| e.to_string())?;
                Ok(graph)
            })();
            let _ = tx.send(Event::Loaded(result));
        });
    }
    fn changed(&mut self) {
        self.revision += 1;
        self.result = None;
        self.progress.clear();
    }
    pub fn poll(&mut self) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Event::Loaded(Ok(graph)) => {
                    self.loading = false;
                    self.changed();
                    self.graph = Some(graph);
                    self.inputs.clear();
                    self.selected = 0;
                    self.output = 0;
                    self.status = format!(
                        "Graph valid · revision {}. Bind every named input before running.",
                        self.revision
                    );
                }
                Event::Loaded(Err(error)) | Event::Listed(Err(error)) => {
                    self.loading = false;
                    self.status = error;
                }
                Event::Listed(Ok(files)) => {
                    self.loading = false;
                    self.listing = files;
                    self.list_index = 0;
                    self.status = if self.listing.is_empty() {
                        "No JSON graphs in this directory".into()
                    } else {
                        "Select a graph with arrows and Enter; Esc closes list".into()
                    };
                }
                Event::Bound(revision, name, result) => {
                    if self.revision == revision {
                        match result {
                            Ok(bytes) => {
                                self.changed();
                                self.inputs.insert(name, bytes);
                                self.status = "Input bound; prior output invalidated".into();
                            }
                            Err(error) => self.status = error,
                        }
                    } else {
                        self.status =
                            "File import discarded: inputs or graph changed while reading".into();
                    }
                }
                Event::Status(id, status) => {
                    if self.active.as_ref().is_some_and(|a| a.id == id) {
                        self.status = status
                    }
                }
                Event::Progress(id, revision, nodes) => {
                    if self.active.as_ref().is_some_and(|active| active.id == id)
                        && self.revision == revision
                    {
                        self.progress = nodes;
                    }
                }
                Event::Done(id, revision, result) => {
                    if self.active.as_ref().is_some_and(|a| a.id == id) {
                        if self.revision != revision {
                            self.status =
                                "Result discarded: graph inputs changed during execution".into();
                        } else {
                            match result {
                                Ok(result) => {
                                    self.status = format!(
                                        "Completed run {id}, revision {revision}: {} output(s)",
                                        result.outputs.len()
                                    );
                                    self.result = Some(result)
                                }
                                Err(error) => self.status = error,
                            }
                        }
                    }
                }
            }
        }
        if self.active.as_ref().is_some_and(|a| a.thread.is_finished()) {
            let _ = self.active.take().unwrap().thread.join();
        }
    }
    fn run(&mut self) {
        if self.loading {
            self.status = "Wait for graph validation before running".into();
            return;
        }
        if self.active.is_some() {
            self.status = "Execution already active; c cancels".into();
            return;
        }
        let Some(graph) = self.graph.clone() else {
            self.status = "Load a graph first".into();
            return;
        };
        for node in &graph.nodes {
            if let NodeKind::Input { name } = &node.kind {
                if !self.inputs.contains_key(name) {
                    self.status = format!("Missing input: {name}");
                    return;
                }
            }
        }
        self.next_run += 1;
        let id = self.next_run;
        let revision = self.revision;
        let request = GraphRequest {
            graph,
            inputs_base64: self
                .inputs
                .iter()
                .map(|(name, bytes)| (name.clone(), STANDARD.encode(bytes)))
                .collect(),
            outputs: Vec::new(),
        };
        self.result = None;
        self.status = format!("Starting run {id}, revision {revision}…");
        let client = self.client.clone();
        let tx = self.tx.clone();
        let cancel = Arc::new(AtomicBool::new(false));
        let flag = cancel.clone();
        let thread = std::thread::spawn(move || {
            let result = (|| {
                client.validate_graph(&request).map_err(|e| e.to_string())?;
                match execute_task(
                    client.as_ref(),
                    EngineTask::GraphExecute(request),
                    &flag,
                    |snapshot| {
                        let _ =
                            tx.send(Event::Progress(id, revision, snapshot.graph_nodes.clone()));
                        let _ = tx.send(Event::Status(
                            id,
                            format!("Run {id}, revision {revision}: {:?}", snapshot.status),
                        ));
                    },
                )
                .map_err(|e| e.to_string())?
                {
                    TaskResult::Graph(result) => Ok(result),
                    _ => Err("Wrong graph job result".into()),
                }
            })();
            let _ = tx.send(Event::Done(id, revision, result));
        });
        self.active = Some(Active { id, cancel, thread });
    }
    pub fn cancel(&mut self) {
        if let Some(active) = &self.active {
            active.cancel.store(true, Ordering::Release);
            self.status = "Cancelling; waiting for worker termination…".into();
        }
    }
    fn begin(&mut self, prompt: Prompt) {
        self.prompt = Some(prompt);
        self.text.clear();
    }
    fn confirm(&mut self, prompt: Prompt) -> Result<(), String> {
        match prompt {
            Prompt::Load => self.load(PathBuf::from(&self.text)),
            Prompt::Bind(name, mode) => {
                if mode == "file" {
                    let path = PathBuf::from(&self.text);
                    let tx = self.tx.clone();
                    let client = self.client.clone();
                    let revision = self.revision;
                    std::thread::spawn(move || {
                        let result = client
                            .capabilities()
                            .map_err(|e| e.to_string())
                            .and_then(|caps| read_bounded(&path, caps.max_input_bytes));
                        let _ = tx.send(Event::Bound(revision, name, result));
                    });
                } else {
                    let bytes = match mode {
                        "hex" => {
                            if self.text.len() % 2 != 0
                                || !self.text.bytes().all(|b| b.is_ascii_hexdigit())
                            {
                                return Err("HEX requires complete byte pairs".into());
                            }
                            hex::decode(&self.text).map_err(|e| e.to_string())?
                        }
                        "base64" => STANDARD.decode(&self.text).map_err(|e| e.to_string())?,
                        _ => self.text.as_bytes().to_vec(),
                    };
                    if bytes.len() > 8 << 20 {
                        return Err("Input exceeds 8 MiB".into());
                    }
                    self.changed();
                    self.inputs.insert(name, bytes);
                    self.status = "Input bound".into();
                }
            }
            Prompt::Export => {
                let output = self
                    .result
                    .as_ref()
                    .and_then(|r| r.outputs.get(self.output))
                    .ok_or("No current output")?;
                write_atomic(
                    Path::new(&self.text),
                    &graph_output_bytes(output).map_err(|e| e.to_string())?,
                )?;
                self.status = "Output exported byte-for-byte".into();
            }
            Prompt::Save => {
                let graph = self.graph.as_ref().ok_or("No graph loaded")?;
                check_export(graph)?;
                write_atomic(
                    Path::new(&self.text),
                    &serde_json::to_vec_pretty(graph).map_err(|e| e.to_string())?,
                )?;
                self.status = "Graph saved without input payloads or results".into();
            }
        }
        Ok(())
    }
    pub fn key(&mut self, key: KeyEvent) {
        if let Some(prompt) = self.prompt.clone() {
            match key.code {
                KeyCode::Esc => self.prompt = None,
                KeyCode::Enter => {
                    self.prompt = None;
                    if let Err(error) = self.confirm(prompt) {
                        self.status = error
                    }
                }
                KeyCode::Backspace => {
                    self.text.pop();
                }
                KeyCode::Char(c) => {
                    if self.text.len() < 12 << 20 {
                        self.text.push(c)
                    }
                }
                _ => {}
            };
            return;
        }
        if !self.listing.is_empty() {
            match key.code {
                KeyCode::Esc => self.listing.clear(),
                KeyCode::Down => {
                    self.list_index = (self.list_index + 1).min(self.listing.len() - 1)
                }
                KeyCode::Up => self.list_index = self.list_index.saturating_sub(1),
                KeyCode::Enter => {
                    let path = self.listing[self.list_index].clone();
                    self.listing.clear();
                    self.load(path)
                }
                _ => {}
            };
            return;
        }
        match key.code {
            KeyCode::Char('l') => self.begin(Prompt::Load),
            KeyCode::Char('r') => self.run(),
            KeyCode::Char('c') => self.cancel(),
            KeyCode::Char('s') => self.begin(Prompt::Save),
            KeyCode::Char('e') => self.begin(Prompt::Export),
            KeyCode::Down => {
                self.selected = (self.selected + 1).min(
                    self.graph
                        .as_ref()
                        .map_or(0, |g| g.nodes.len().saturating_sub(1)),
                )
            }
            KeyCode::Up => self.selected = self.selected.saturating_sub(1),
            KeyCode::Char('o') => {
                let count = self.result.as_ref().map_or(0, |r| r.outputs.len());
                if count > 0 {
                    self.output = (self.output + 1) % count
                }
            }
            KeyCode::Char(c @ ('i' | 'h' | 'b' | 'f')) => {
                if let Some(NodeKind::Input { name }) = self
                    .graph
                    .as_ref()
                    .and_then(|g| g.nodes.get(self.selected))
                    .map(|n| &n.kind)
                {
                    self.begin(Prompt::Bind(
                        name.clone(),
                        match c {
                            'h' => "hex",
                            'b' => "base64",
                            'f' => "file",
                            _ => "text",
                        },
                    ));
                } else {
                    self.status = "Select an Input node first".into();
                }
            }
            _ => {}
        }
    }
    pub fn render(&self, frame: &mut Frame) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(8),
                Constraint::Length(3),
            ])
            .split(frame.area());
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
            .split(rows[0]);
        let nodes = self
            .graph
            .as_ref()
            .map(|g| {
                g.nodes
                    .iter()
                    .enumerate()
                    .map(|(i, n)| {
                        format!(
                            "{} {} · {}",
                            if i == self.selected { ">" } else { " " },
                            n.id,
                            match &n.kind {
                                NodeKind::Input { name } => format!(
                                    "Input {name}: {}",
                                    self.inputs
                                        .get(name)
                                        .map_or("unbound".into(), |b| format!("{} bytes", b.len()))
                                ),
                                NodeKind::Output { name } => format!("Output {name}"),
                                NodeKind::Operation { operation, .. } => operation.clone(),
                                NodeKind::Argument { kind } => format!("Argument {kind:?}"),
                                NodeKind::Combine { .. } => "Combine".into(),
                                NodeKind::Recipe { steps } =>
                                    format!("Recipe: {} steps", steps.len()),
                            }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap_or_else(|| "No graph loaded".into());
        let node_lines = if self.listing.is_empty() {
            nodes
        } else {
            self.listing
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    format!(
                        "{} {}",
                        if i == self.list_index { ">" } else { " " },
                        p.display()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        frame.render_widget(
            Paragraph::new(node_lines)
                .block(
                    Block::default()
                        .title("Graph nodes / files · ↑↓ select")
                        .borders(Borders::ALL),
                )
                .scroll((self.selected.saturating_sub(5) as u16, 0)),
            columns[0],
        );
        let detail =
            self.graph
                .as_ref()
                .and_then(|g| {
                    g.nodes.get(self.selected).map(|node| {
                        let status =
                            self.result
                                .as_ref()
                                .and_then(|r| r.nodes.iter().find(|n| n.node_id == node.id))
                                .map(|n| {
                                    format!(
                                        "{:?} · {} B · {:.2} ms\n{}",
                                        n.status,
                                        n.output_bytes,
                                        n.elapsed_ms,
                                        n.error
                                            .as_ref()
                                            .map_or(String::new(), |e| e.message.clone())
                                    )
                                })
                                .unwrap_or_else(|| {
                                    self.progress
                                        .iter()
                                        .find(|progress| progress.node_id == node.id)
                                        .map(|progress| {
                                            format!(
                                                "{:?} · {} · {}",
                                                progress.state,
                                                progress.output_bytes.map_or(
                                                    "byte count pending".into(),
                                                    |bytes| format!("{bytes} B")
                                                ),
                                                progress.elapsed_ms.map_or(
                                                    "timing pending".into(),
                                                    |time| format!("{time:.2} ms")
                                                )
                                            )
                                        })
                                        .unwrap_or_else(|| "Waiting for node events".into())
                                });
                        let edges = g
                            .edges
                            .iter()
                            .filter(|e| e.source_node == node.id || e.target_node == node.id)
                            .map(|e| {
                                format!(
                                    "{}:{} → {}:{}",
                                    e.source_node, e.source_port, e.target_node, e.target_port
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        format!(
                            "{}\n{status}\n\n{edges}",
                            node.label.as_ref().unwrap_or(&node.id)
                        )
                    })
                })
                .unwrap_or_default();
        frame.render_widget(
            Paragraph::new(detail).wrap(Wrap { trim: false }).block(
                Block::default()
                    .title("Inspector / connections")
                    .borders(Borders::ALL),
            ),
            columns[1],
        );
        let output = self
            .result
            .as_ref()
            .and_then(|r| r.outputs.get(self.output))
            .map(|o| {
                format!(
                    "{} · {} bytes\n{}",
                    o.name,
                    o.output_bytes,
                    graph_output_bytes(o)
                        .map(|b| hex::encode(&b[..b.len().min(1024)]))
                        .unwrap_or_else(|e| e.to_string())
                )
            })
            .unwrap_or_else(|| "No current result".into());
        frame.render_widget(
            Paragraph::new(output).wrap(Wrap { trim: false }).block(
                Block::default()
                    .title("Outputs · o next · e exact export · HEX preview ≤1024 bytes")
                    .borders(Borders::ALL),
            ),
            rows[1],
        );
        let status = if let Some(prompt) = &self.prompt {
            format!(
                "{}: {} · Enter confirm / Esc cancel",
                match prompt {
                    Prompt::Load => "Graph file or directory",
                    Prompt::Bind(_, mode) => mode,
                    Prompt::Export => "Export output path",
                    Prompt::Save => "Save graph path",
                },
                self.text
            )
        } else {
            format!("{}\nl load · i text · h HEX · b Base64 · f input file · r run · c cancel · s save · Tab recipes · q quit",self.status)
        };
        frame.render_widget(Paragraph::new(status).wrap(Wrap { trim: false }), rows[2]);
    }
}
impl Drop for GraphScreen {
    fn drop(&mut self) {
        if let Some(active) = self.active.take() {
            active.cancel.store(true, Ordering::Release);
            let _ = active.thread.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rxchef_client::LocalEngineClient;
    fn wait(screen: &mut GraphScreen, condition: impl Fn(&GraphScreen) -> bool) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        while !condition(screen) {
            screen.poll();
            assert!(std::time::Instant::now() < deadline, "{}", screen.status);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
    #[test]
    fn load_bind_run_export_and_revision_invalidation_preserve_every_byte() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("graph.json");
        let graph = Graph::from_recipe("test".into(), Vec::new());
        std::fs::write(&path, serde_json::to_vec(&graph).unwrap()).unwrap();
        let mut screen = GraphScreen::new(Arc::new(LocalEngineClient::default()));
        screen.load(path);
        wait(&mut screen, |s| s.graph.is_some());
        let name = match &screen.graph.as_ref().unwrap().nodes[0].kind {
            NodeKind::Input { name } => name.clone(),
            _ => panic!(),
        };
        screen.text = hex::encode((0..=255).collect::<Vec<u8>>());
        screen.confirm(Prompt::Bind(name.clone(), "hex")).unwrap();
        screen.run();
        wait(&mut screen, |s| s.result.is_some());
        let output = directory.path().join("output.bin");
        screen.text = output.display().to_string();
        screen.confirm(Prompt::Export).unwrap();
        assert_eq!(
            std::fs::read(output).unwrap(),
            (0..=255).collect::<Vec<u8>>()
        );
        screen.text = "00".into();
        screen.confirm(Prompt::Bind(name, "hex")).unwrap();
        assert!(screen.result.is_none());
        let saved = directory.path().join("saved.json");
        screen.text = saved.display().to_string();
        screen.confirm(Prompt::Save).unwrap();
        let saved: Graph = serde_json::from_slice(&std::fs::read(saved).unwrap()).unwrap();
        assert_eq!(
            serde_json::to_value(saved).unwrap(),
            serde_json::to_value(graph).unwrap()
        );
    }
    #[test]
    fn invalid_inputs_missing_bindings_and_small_render_are_safe() {
        let mut screen = GraphScreen::new(Arc::new(LocalEngineClient::default()));
        screen.graph = Some(Graph::from_recipe("test".into(), Vec::new()));
        screen.run();
        assert!(screen.status.starts_with("Missing input"));
        assert!(screen.active.is_none());
        for text in ["0", "zz", "00 11"] {
            screen.text = text.into();
            assert!(screen.confirm(Prompt::Bind("input".into(), "hex")).is_err());
        }
        screen.text = "not base64".into();
        assert!(screen
            .confirm(Prompt::Bind("input".into(), "base64"))
            .is_err());
        let mut terminal =
            ratatui::Terminal::new(ratatui::backend::TestBackend::new(20, 8)).unwrap();
        terminal.draw(|frame| screen.render(frame)).unwrap();
    }
    #[test]
    fn stale_runs_cannot_publish_and_sensitive_recipe_arguments_cannot_be_saved() {
        let mut screen = GraphScreen::new(Arc::new(LocalEngineClient::default()));
        let thread = std::thread::spawn(|| {});
        screen.active = Some(Active {
            id: 1,
            cancel: Arc::new(AtomicBool::new(false)),
            thread,
        });
        screen.revision = 2;
        screen
            .tx
            .send(Event::Done(
                1,
                1,
                Ok(GraphResult {
                    nodes: Vec::new(),
                    outputs: Vec::new(),
                }),
            ))
            .unwrap();
        screen.poll();
        assert!(screen.result.is_none());
        assert!(screen.status.contains("discarded"));
        let graph = Graph::from_recipe(
            "secret".into(),
            vec![rxchef::execution::RecipeStep {
                op: "AES Decrypt".into(),
                args: vec!["SECRET".into()],
            }],
        );
        assert!(check_export(&graph).is_err());
    }
}
