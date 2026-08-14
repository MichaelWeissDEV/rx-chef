/*!
rxchef TUI — interactive pipeline builder

Layout:
┌───── rxchef pipeline builder ──────────────────────────────────────────┐
│ PIPELINE  (↑/↓ navigate, a add, d del, e edit, r run)                 │
│ ┌────────────────────────────────┐  ┌─────────────────────────────┐   │
│ │▶ 1. To Hex  [Space, 0]        │  │ OUTPUT  (step 1/3)          │   │
│ │  2. SHA2    [256]             │  │ 48 65 6c 6c 6f              │   │
│ │  3. To Base64  [A-Za-z0-9+/=]│  │                             │   │
│ │  ── press (a) to add a step ──│  │                             │   │
│ └────────────────────────────────┘  └─────────────────────────────┘   │
│ INPUT                                STATUS                            │
│ ┌────────────────────────────────┐  ┌─────────────────────────────┐   │
│ │ Hello, World!                  │  │ Pipeline ran OK (3 steps)   │   │
│ └────────────────────────────────┘  └─────────────────────────────┘   │
│ [a]dd [d]el [e]dit [r]un [i]nput [s]ave [l]oad [?]help [q]uit         │
└────────────────────────────────────────────────────────────────────────┘

Keys (Normal mode):
  j/↓   next step          k/↑   previous step
  a     add step           d     delete selected step
  e     edit args          r     run pipeline
  i     edit input         s     save recipe
  l     load recipe        ?     toggle help
  q/Esc quit

Keys (Add mode):  type to search, ↑↓ navigate results, Enter select, Esc cancel
Keys (Edit mode): Tab/↑↓ next/prev arg, Enter confirm, Esc cancel
Keys (Input mode): edit freely, Enter/Esc confirm
*/

use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use rxchef::{execution, runtime};
use rxchef_store as store;

// ─── Data model ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct PipelineStep {
    op_name: String,
    args: Vec<String>,
}

impl PipelineStep {
    fn display_label(&self) -> String {
        if self.args.is_empty() {
            self.op_name.clone()
        } else {
            format!("{} [{}]", self.op_name, self.args.join(", "))
        }
    }
}

#[derive(Debug)]
struct StepResult {
    output: Vec<u8>,
    error: Option<String>,
}

impl StepResult {
    fn display(&self) -> String {
        if let Some(ref e) = self.error {
            return format!("ERROR: {}", e);
        }
        match std::str::from_utf8(&self.output) {
            Ok(s) => {
                if s.chars().count() > 4096 {
                    let preview: String = s.chars().take(4096).collect();
                    format!("{}… ({} bytes)", preview, self.output.len())
                } else {
                    s.to_string()
                }
            }
            Err(_) => {
                let hex: Vec<String> = self.output.iter().map(|b| format!("{:02x}", b)).collect();
                let preview = hex
                    .chunks(16)
                    .map(|c| c.join(" "))
                    .collect::<Vec<_>>()
                    .join("\n");
                if self.output.len() > 256 {
                    format!("{}\n… ({} bytes total)", preview, self.output.len())
                } else {
                    preview
                }
            }
        }
    }
}

// ─── App state ───────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum Mode {
    Normal,
    AddStep,
    EditArgs,
    EditInput,
    Help,
    SavePrompt,
    LoadPrompt,
    LoadPicker,
    TraceView,
    CheckpointsList,
    CheckpointSavePrompt,
}

struct App {
    mode: Mode,
    pipeline: Vec<PipelineStep>,
    selected: usize,
    input_text: String,
    checkpoints: Vec<TuiCheckpoint>,
    results: Vec<StepResult>,
    status: String,

    // Add-step search
    search_query: String,
    search_results: Vec<String>,
    search_list_state: ListState,

    // Arg editor
    editing_args: Vec<String>,
    editing_arg_names: Vec<(&'static str, &'static str)>,
    editing_arg_idx: usize,

    // File prompt
    file_prompt: String,

    // Scroll in output panel
    output_scroll: u16,
    output_step_view: usize,

    // Recipe picker
    recipe_list: Vec<String>,
    recipe_list_state: ListState,

    checkpoint_list_state: ListState,

    // Trace view scroll
    trace_scroll: u16,
}

impl App {
    fn new() -> Self {
        let all_ops = runtime::operation_names(None);
        let mut state = ListState::default();
        state.select(Some(0));
        App {
            mode: Mode::Normal,
            pipeline: Vec::new(),
            selected: 0,
            input_text: String::new(),
            results: Vec::new(),
            status: "Welcome! Press (a) to add a step, (?) for help.".into(),
            search_query: String::new(),
            search_results: all_ops,
            search_list_state: state,
            editing_args: Vec::new(),
            editing_arg_names: Vec::new(),
            editing_arg_idx: 0,
            file_prompt: String::new(),
            output_scroll: 0,
            output_step_view: 0,
            recipe_list: Vec::new(),
            recipe_list_state: ListState::default(),
            checkpoints: Vec::new(),
            checkpoint_list_state: ListState::default(),
            trace_scroll: 0,
        }
    }

    fn refresh_search(&mut self) {
        let q = self.search_query.to_lowercase();
        self.search_results = runtime::operation_names(if q.is_empty() { None } else { Some(&q) });
        let idx = self
            .search_list_state
            .selected()
            .unwrap_or(0)
            .min(self.search_results.len().saturating_sub(1));
        self.search_list_state.select(Some(idx));
    }

    fn selected_search_result(&self) -> Option<&str> {
        let idx = self.search_list_state.selected()?;
        self.search_results.get(idx).map(|s| s.as_str())
    }

    fn run_pipeline(&mut self) {
        self.results.clear();
        if self.pipeline.is_empty() {
            self.status = "Pipeline is empty. Press (a) to add a step.".into();
            return;
        }

        let recipe = self
            .pipeline
            .iter()
            .map(|step| execution::RecipeStep {
                op: step.op_name.clone(),
                args: step.args.clone(),
            })
            .collect::<Vec<_>>();
        let result = execution::execute(execution::ExecutionRequest {
            input: self.input_text.as_bytes().to_vec(),
            input_supplied: true,
            recipe: recipe.into(),
            variables: execution::VariableContext::default(),
            options: execution::ExecutionOptions {
                trace: true,
                ..execution::ExecutionOptions::default()
            },
        });
        match result {
            Ok(outcome) => {
                self.results = outcome
                    .trace
                    .iter()
                    .map(|_| StepResult {
                        output: Vec::new(),
                        error: None,
                    })
                    .collect();
                if let Some(last) = self.results.last_mut() {
                    last.output = outcome.output.clone();
                }
                self.status = format!(
                    "Pipeline ran OK — {} execution(s), output {} bytes.",
                    outcome.trace.len(),
                    outcome.output.len()
                );
            }
            Err(error) => {
                self.results.push(StepResult {
                    output: Vec::new(),
                    error: Some(error.to_string()),
                });
                self.status = format!("Pipeline error: {error}");
            }
        }
        self.output_step_view = self.results.len().saturating_sub(1);
        self.output_scroll = 0;
    }

    fn start_add(&mut self) {
        self.mode = Mode::AddStep;
        self.search_query.clear();
        self.refresh_search();
    }

    fn confirm_add(&mut self) {
        if let Some(op_name) = self.selected_search_result().map(|s| s.to_string()) {
            // Build default args from the operation's schema
            let default_args: Vec<String> = runtime::operation_info(&op_name)
                .ok()
                .map(|info| {
                    info.args
                        .iter()
                        .map(|a| a.default_value.to_string())
                        .collect()
                })
                .unwrap_or_default();

            let insert_idx = if self.pipeline.is_empty() {
                0
            } else {
                self.selected + 1
            };
            self.pipeline.insert(
                insert_idx,
                PipelineStep {
                    op_name: op_name.clone(),
                    args: default_args,
                },
            );
            self.selected = insert_idx;
            self.mode = Mode::Normal;
            self.status = format!("Added '{}'.", op_name);
            self.run_pipeline();
        }
    }

    fn start_edit_args(&mut self) {
        if self.pipeline.is_empty() {
            self.status = "No step selected.".into();
            return;
        }
        let step = &self.pipeline[self.selected];
        self.editing_args = step.args.clone();
        self.editing_arg_names = runtime::operation_info(&step.op_name)
            .ok()
            .map(|info| info.args.iter().map(|a| (a.name, a.description)).collect())
            .unwrap_or_default();
        while self.editing_args.len() < self.editing_arg_names.len() {
            self.editing_args.push(String::new());
        }
        self.editing_arg_idx = 0;
        self.mode = Mode::EditArgs;
    }

    fn confirm_edit_args(&mut self) {
        if !self.pipeline.is_empty() {
            self.pipeline[self.selected].args = self.editing_args.clone();
            self.run_pipeline();
            self.status = format!(
                "Updated args for '{}'.",
                self.pipeline[self.selected].op_name
            );
        }
        self.mode = Mode::Normal;
    }

    fn delete_selected(&mut self) {
        if self.pipeline.is_empty() {
            return;
        }
        let name = self.pipeline.remove(self.selected).op_name;
        if self.selected >= self.pipeline.len() && self.selected > 0 {
            self.selected -= 1;
        }
        self.status = format!("Deleted '{}'.", name);
        self.run_pipeline();
    }

    fn save_recipe(&mut self, name: &str) {
        let recipe = store::Recipe {
            version: store::RECIPE_VERSION,
            name: name.to_string(),
            description: String::new(),
            steps: self
                .pipeline
                .iter()
                .map(|s| store::RecipeStep {
                    op: s.op_name.clone(),
                    args: s.args.clone(),
                })
                .collect(),
            tags: vec![],
        };
        match store::save_recipe(&recipe, store::default_scope()) {
            Ok(path) => self.status = format!("Saved '{}' to {}.", name, path.display()),
            Err(e) => self.status = format!("Save failed: {}", e),
        }
        // Also save to history if pipeline ran
        if !self.results.is_empty() {
            let steps: Vec<store::HistoryStep> = self
                .pipeline
                .iter()
                .zip(self.results.iter())
                .map(|(s, r)| store::HistoryStep {
                    op: s.op_name.clone(),
                    args: runtime::redact_sensitive_args(&s.op_name, &s.args),
                    output_preview: store::bytes_preview(&r.output, 200),
                    output_bytes: r.output.len(),
                    duration_ms: 0.0,
                    error: r.error.clone(),
                })
                .collect();
            let entry = store::HistoryEntry {
                id: store::new_history_id(),
                timestamp: chrono_now_tui(),
                pipeline_name: Some(name.to_string()),
                input_preview: store::bytes_preview(self.input_text.as_bytes(), 200),
                input_bytes: self.input_text.len(),
                steps,
                output_preview: self
                    .results
                    .last()
                    .map(|r| store::bytes_preview(&r.output, 200))
                    .unwrap_or_default(),
                output_bytes: self.results.last().map(|r| r.output.len()).unwrap_or(0),
                success: self
                    .results
                    .last()
                    .map(|r| r.error.is_none())
                    .unwrap_or(false),
            };
            let _ = store::append_history(&entry);
        }
    }

    fn load_recipe(&mut self, name: &str) {
        // First try store by name, then fall back to file path
        let recipe = if let Ok(r) = store::load_recipe(name) {
            r
        } else if let Ok(json) = std::fs::read_to_string(name) {
            match serde_json::from_str::<store::Recipe>(&json).or_else(|_| {
                serde_json::from_str::<Vec<store::RecipeStep>>(&json).map(|steps| store::Recipe {
                    version: store::RECIPE_VERSION,
                    name: name.to_string(),
                    description: String::new(),
                    steps,
                    tags: vec![],
                })
            }) {
                Ok(r) => r,
                Err(e) => {
                    self.status = format!("Load failed: {}", e);
                    return;
                }
            }
        } else {
            self.status = format!("Pipeline '{}' not found in store or as file.", name);
            return;
        };

        self.pipeline.clear();
        for step in &recipe.steps {
            self.pipeline.push(PipelineStep {
                op_name: step.op.clone(),
                args: step.args.clone(),
            });
        }
        self.selected = 0;
        self.status = format!("Loaded '{}' ({} steps).", recipe.name, recipe.steps.len());
        self.run_pipeline();
    }

    fn current_output(&self) -> String {
        if self.results.is_empty() {
            return String::new();
        }
        let idx = self.output_step_view.min(self.results.len() - 1);
        self.results[idx].display()
    }

    fn output_step_label(&self) -> String {
        if self.pipeline.is_empty() {
            return String::new();
        }
        if self.results.is_empty() {
            return "not yet run — press r".into();
        }
        let idx = self.output_step_view.min(self.results.len() - 1);
        let step_name = self
            .pipeline
            .get(idx)
            .map(|s| s.op_name.as_str())
            .unwrap_or("?");
        format!("step {}/{}: {}", idx + 1, self.pipeline.len(), step_name)
    }

    fn save_checkpoint(&mut self, name: &str) {
        self.checkpoints.push(TuiCheckpoint {
            name: name.to_string(),
            pipeline: self.pipeline.clone(),
            input_text: self.input_text.clone(),
        });
        self.status = format!("Checkpoint '{}' saved.", name);
    }

    fn restore_checkpoint(&mut self, idx: usize) {
        if let Some(cp) = self.checkpoints.get(idx).cloned() {
            self.pipeline = cp.pipeline;
            self.input_text = cp.input_text;
            self.selected = 0;
            self.status = format!("Restored checkpoint '{}'.", cp.name);
            self.run_pipeline();
        }
    }
}

#[derive(Debug, Clone)]
struct TuiCheckpoint {
    name: String,
    pipeline: Vec<PipelineStep>,
    input_text: String,
}

// ─── UI rendering ─────────────────────────────────────────────────────────────

fn ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Outer layout: main area + status bar
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(size);

    let main_area = outer[0];
    let status_area = outer[1];

    // Main: left column (pipeline + input) | right column (output)
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(main_area);

    // Left: pipeline (top) + input (bottom)
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(4), Constraint::Length(5)])
        .split(columns[0]);

    // Right: output (top) + step selector (bottom)
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(4), Constraint::Length(3)])
        .split(columns[1]);

    render_pipeline(f, app, left[0]);
    render_input(f, app, left[1]);
    render_output(f, app, right[0]);
    render_step_selector(f, app, right[1]);
    render_status(f, app, status_area);

    // Overlay panels
    match app.mode {
        Mode::AddStep => render_add_overlay(f, app, size),
        Mode::EditArgs => render_edit_args_overlay(f, app, size),
        Mode::Help => render_help_overlay(f, size),
        Mode::SavePrompt | Mode::LoadPrompt | Mode::CheckpointSavePrompt => {
            render_file_prompt(f, app, size)
        }
        Mode::LoadPicker => render_load_picker(f, app, size),
        Mode::TraceView => render_trace_view(f, app, size),
        Mode::CheckpointsList => render_checkpoints_list(f, app, size),
        _ => {}
    }
}

fn render_pipeline(f: &mut Frame, app: &mut App, area: Rect) {
    let title = format!(
        " Pipeline ({} step{}) ",
        app.pipeline.len(),
        if app.pipeline.len() == 1 { "" } else { "s" }
    );
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(if app.mode == Mode::Normal {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        });

    let items: Vec<ListItem> = app
        .pipeline
        .iter()
        .enumerate()
        .map(|(i, step)| {
            let marker = if i == app.selected && app.mode == Mode::Normal {
                "▶"
            } else {
                " "
            };
            let style = if i == app.selected && app.mode == Mode::Normal {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} {:>2}. ", marker, i + 1), style),
                Span::styled(step.display_label(), style),
            ]))
        })
        .collect();

    let hint_item = ListItem::new(Line::from(Span::styled(
        " ── press (a) to add a step ──",
        Style::default().fg(Color::DarkGray),
    )));
    let mut all_items = items;
    all_items.push(hint_item);

    let mut state = ListState::default();
    if app.mode == Mode::Normal && !app.pipeline.is_empty() {
        state.select(Some(app.selected));
    }

    let list = List::new(all_items).block(block).highlight_symbol("");
    f.render_stateful_widget(list, area, &mut state);
}

fn render_input(f: &mut Frame, app: &App, area: Rect) {
    let style = if app.mode == Mode::EditInput {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(" Input (i to edit) ")
        .borders(Borders::ALL)
        .border_style(style);

    let display = if app.input_text.is_empty() {
        Span::styled(
            "(empty — press i to enter text)",
            Style::default().fg(Color::DarkGray),
        )
    } else {
        Span::raw(app.input_text.as_str())
    };

    let para = Paragraph::new(Line::from(display))
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

fn render_output(f: &mut Frame, app: &App, area: Rect) {
    let step_label = app.output_step_label();
    let title = if step_label.is_empty() {
        " Output ".to_string()
    } else {
        format!(" Output: {} ", step_label)
    };

    let has_error = app
        .results
        .get(
            app.output_step_view
                .min(app.results.len().saturating_sub(1)),
        )
        .and_then(|r| r.error.as_ref())
        .is_some();

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(if has_error {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        });

    let text = app.current_output();
    let para = Paragraph::new(text.as_str())
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.output_scroll, 0));
    f.render_widget(para, area);
}

fn render_step_selector(f: &mut Frame, app: &App, area: Rect) {
    let n = app.results.len();
    let hint = if n == 0 {
        "press r to run".to_string()
    } else {
        format!(
            "← → change view   showing {}/{}",
            (app.output_step_view + 1).min(n),
            n
        )
    };
    let block = Block::default()
        .title(" Step view ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let para = Paragraph::new(hint).block(block);
    f.render_widget(para, area);
}

fn render_status(f: &mut Frame, app: &App, area: Rect) {
    let keys = match app.mode {
        Mode::Normal => {
            " [a]dd [d]el [e]dit [r]un [i]nput [t]race [←→]view [s]ave [l]oad [c]hkpts [C]save chkpt [?]help [q]uit"
        }
        Mode::AddStep => " type to search  [↑↓] select  [Enter] add  [Esc] cancel",
        Mode::EditArgs => " [Tab/↑↓] next arg  type to edit  [Enter] confirm  [Esc] cancel",
        Mode::EditInput => " type your input  [Enter] confirm  [Esc] cancel",
        Mode::Help => " [Esc/?/q] close help",
        Mode::SavePrompt => " enter name  [Enter] save  [Esc] cancel",
        Mode::LoadPrompt => " enter name  [Enter] load  [Esc] cancel",
        Mode::LoadPicker | Mode::CheckpointsList => " [↑↓] select  [Enter] load/restore  [Esc] cancel",
        Mode::TraceView => " [↑↓/PgUp/PgDn] scroll  [Esc/t/q] close",
        Mode::CheckpointSavePrompt => " enter checkpoint name  [Enter] save  [Esc] cancel",
    };

    let status = format!("{}  │  {}", app.status, keys);
    let para = Paragraph::new(status).style(Style::default().bg(Color::DarkGray).fg(Color::White));
    f.render_widget(para, area);
}

fn render_add_overlay(f: &mut Frame, app: &mut App, area: Rect) {
    let overlay = centered_rect(70, 80, area);

    // Split: search bar + list
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(overlay);

    // Search bar
    let search_block = Block::default()
        .title(" Add step — search operations ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let search_text = Paragraph::new(app.search_query.as_str()).block(search_block);
    f.render_widget(search_text, layout[0]);

    // Results list
    let results_block = Block::default()
        .title(format!(" {} result(s) ", app.search_results.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let items: Vec<ListItem> = app
        .search_results
        .iter()
        .map(|name| ListItem::new(name.as_str()))
        .collect();

    let list = List::new(items)
        .block(results_block)
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, layout[1], &mut app.search_list_state);
}

fn render_help_overlay(f: &mut Frame, area: Rect) {
    let overlay = centered_rect(60, 75, area);
    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let text = vec![
        "rxchef pipeline builder",
        "",
        "NORMAL MODE",
        "  j / ↓       next step",
        "  k / ↑       previous step",
        "  a           add step (opens search)",
        "  d           delete selected step",
        "  e           edit args for selected step",
        "  r           run pipeline",
        "  i           edit input text",
        "  ← / →       change output step view",
        "  s           save pipeline as JSON",
        "  l           load pipeline from JSON",
        "  c           view/restore checkpoints",
        "  C           save new checkpoint",
        "  ? / q       quit / close help",
        "",
        "ADD MODE",
        "  type        filter operations",
        "  ↑/↓         navigate results",
        "  Enter       add selected operation",
        "  Esc         cancel",
        "",
        "EDIT ARGS MODE",
        "  Tab / ↑↓    next/prev argument",
        "  Backspace   delete last char",
        "  Enter       confirm changes",
        "  Esc         cancel",
        "",
        "PIPELINE FORMAT",
        "  Saved as CyberChef-compatible JSON.",
        "  Compatible with rxchef recipe files.",
    ];

    let para = Paragraph::new(text.join("\n"))
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, overlay);
}

fn render_file_prompt(f: &mut Frame, app: &App, area: Rect) {
    let overlay = centered_rect(50, 12, area);
    let title = if app.mode == Mode::SavePrompt {
        " Save recipe to file "
    } else if app.mode == Mode::CheckpointSavePrompt {
        " Save checkpoint "
    } else {
        " Load recipe from file "
    };
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let hint = if app.mode == Mode::SavePrompt {
        "Enter filename (e.g. recipe.json):"
    } else if app.mode == Mode::CheckpointSavePrompt {
        "Enter checkpoint name:"
    } else {
        "Enter filename to load:"
    };

    let content = format!("{}\n> {}", hint, app.file_prompt);
    let para = Paragraph::new(content).block(block);
    f.render_widget(para, overlay);
}

fn render_edit_args_overlay(f: &mut Frame, app: &App, area: Rect) {
    let n_args = app.editing_args.len().max(1);
    let height = (n_args * 2 + 4).min(area.height as usize - 4) as u16;
    let overlay = centered_rect(65, (height * 100 / area.height).max(30), area);
    let step_name = app
        .pipeline
        .get(app.selected)
        .map(|s| s.op_name.as_str())
        .unwrap_or("?");
    let block = Block::default()
        .title(format!(" Edit args: {} ", step_name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let mut lines: Vec<Line> = Vec::new();
    for (i, val) in app.editing_args.iter().enumerate() {
        let (name, desc) = app.editing_arg_names.get(i).copied().unwrap_or(("?", ""));
        let is_current = i == app.editing_arg_idx;
        let marker = if is_current { "▶ " } else { "  " };
        let label_style = if is_current {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let val_style = if is_current {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        // Highlight $VAR patterns
        let display_val = if val.contains('$') {
            val.to_string()
        } else {
            val.to_string()
        };

        lines.push(Line::from(vec![
            Span::styled(marker, label_style),
            Span::styled(format!("{}: ", name), label_style),
            Span::styled(
                if display_val.is_empty() {
                    "(empty)".to_string()
                } else {
                    display_val
                },
                val_style,
            ),
        ]));
        if !desc.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("    {}", desc),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, overlay);
}

fn render_load_picker(f: &mut Frame, app: &mut App, area: Rect) {
    let overlay = centered_rect(50, 60, area);
    let block = Block::default()
        .title(format!(" Load recipe ({}) ", app.recipe_list.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let items: Vec<ListItem> = app
        .recipe_list
        .iter()
        .map(|name| ListItem::new(name.as_str()))
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, overlay, &mut app.recipe_list_state);
}

fn render_trace_view(f: &mut Frame, app: &App, area: Rect) {
    let overlay = centered_rect(85, 85, area);
    let block = Block::default()
        .title(" Trace — all step outputs ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let mut lines: Vec<Line> = Vec::new();
    for (i, result) in app.results.iter().enumerate() {
        let step_name = app
            .pipeline
            .get(i)
            .map(|s| s.op_name.as_str())
            .unwrap_or("?");
        let header_style = if result.error.is_some() {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        };
        lines.push(Line::from(Span::styled(
            format!("── Step {}: {} ──", i + 1, step_name),
            header_style,
        )));
        let preview = result.display();
        for line in preview.lines().take(6) {
            lines.push(Line::from(Span::raw(format!("  {}", line))));
        }
        let total_lines = preview.lines().count();
        if total_lines > 6 {
            lines.push(Line::from(Span::styled(
                format!(
                    "  … ({} more lines, {} bytes)",
                    total_lines - 6,
                    result.output.len()
                ),
                Style::default().fg(Color::DarkGray),
            )));
        }
        lines.push(Line::from(""));
    }

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.trace_scroll, 0));
    f.render_widget(para, overlay);
}

// ─── Event handling ───────────────────────────────────────────────────────────

fn handle_key(app: &mut App, key: KeyEvent) {
    match app.mode {
        Mode::Normal => handle_normal(app, key),
        Mode::AddStep => handle_add(app, key),
        Mode::EditArgs => handle_edit_args(app, key),
        Mode::EditInput => handle_edit_input(app, key),
        Mode::Help => handle_help(app, key),
        Mode::SavePrompt | Mode::LoadPrompt | Mode::CheckpointSavePrompt => {
            handle_file_prompt(app, key)
        }
        Mode::LoadPicker => handle_load_picker(app, key),
        Mode::TraceView => handle_trace_view(app, key),
        Mode::CheckpointsList => handle_checkpoints_list(app, key),
    }
}

fn handle_normal(app: &mut App, key: KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            app.mode = Mode::Help; // triggers quit in main loop
                                   // Actually we signal quit differently — see main loop
        }
        (KeyCode::Char('a'), _) => app.start_add(),
        (KeyCode::Char('d'), _) => app.delete_selected(),
        (KeyCode::Char('e'), _) => app.start_edit_args(),
        (KeyCode::Char('r'), _) => {
            app.run_pipeline();
        }
        (KeyCode::Char('i'), _) => app.mode = Mode::EditInput,
        (KeyCode::Char('?'), _) => app.mode = Mode::Help,
        (KeyCode::Char('s'), _) => {
            app.file_prompt.clear();
            app.mode = Mode::SavePrompt;
        }
        (KeyCode::Char('l'), _) => {
            let names: Vec<String> = store::list_recipes(None)
                .into_iter()
                .map(|r| r.name)
                .collect();
            if names.is_empty() {
                app.file_prompt.clear();
                app.mode = Mode::LoadPrompt;
            } else {
                app.recipe_list = names;
                app.recipe_list_state.select(Some(0));
                app.mode = Mode::LoadPicker;
            }
        }
        (KeyCode::Char('C'), KeyModifiers::SHIFT) => {
            app.file_prompt.clear();
            app.mode = Mode::CheckpointSavePrompt;
        }
        (KeyCode::Char('c'), KeyModifiers::NONE) => {
            if !app.checkpoints.is_empty() {
                app.checkpoint_list_state.select(Some(0));
                app.mode = Mode::CheckpointsList;
            } else {
                app.status = "No checkpoints saved yet. Press (C) to save one.".into();
            }
        }
        (KeyCode::Char('t'), _) => {
            if !app.results.is_empty() {
                app.trace_scroll = 0;
                app.mode = Mode::TraceView;
            }
        }
        (KeyCode::Down | KeyCode::Char('j'), _) => {
            if !app.pipeline.is_empty() {
                app.selected = (app.selected + 1).min(app.pipeline.len() - 1);
            }
        }
        (KeyCode::Up | KeyCode::Char('k'), _) => {
            if app.selected > 0 {
                app.selected -= 1;
            }
        }
        (KeyCode::Left, _) => {
            if app.output_step_view > 0 {
                app.output_step_view -= 1;
                app.output_scroll = 0;
            }
        }
        (KeyCode::Right, _) => {
            if !app.results.is_empty() {
                app.output_step_view = (app.output_step_view + 1).min(app.results.len() - 1);
                app.output_scroll = 0;
            }
        }
        (KeyCode::PageDown, _) => app.output_scroll = app.output_scroll.saturating_add(5),
        (KeyCode::PageUp, _) => app.output_scroll = app.output_scroll.saturating_sub(5),
        _ => {}
    }
}

fn handle_add(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.search_query.clear();
        }
        KeyCode::Enter => app.confirm_add(),
        KeyCode::Backspace => {
            app.search_query.pop();
            app.refresh_search();
        }
        KeyCode::Down => {
            let max = app.search_results.len().saturating_sub(1);
            let next = app
                .search_list_state
                .selected()
                .unwrap_or(0)
                .saturating_add(1)
                .min(max);
            app.search_list_state.select(Some(next));
        }
        KeyCode::Up => {
            let prev = app
                .search_list_state
                .selected()
                .unwrap_or(0)
                .saturating_sub(1);
            app.search_list_state.select(Some(prev));
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.refresh_search();
        }
        _ => {}
    }
}

fn handle_edit_args(app: &mut App, key: KeyEvent) {
    let n = app.editing_args.len();
    match key.code {
        KeyCode::Esc => app.mode = Mode::Normal,
        KeyCode::Enter => app.confirm_edit_args(),
        KeyCode::Tab | KeyCode::Down => {
            if n > 0 {
                app.editing_arg_idx = (app.editing_arg_idx + 1) % n;
            }
        }
        KeyCode::Up => {
            if n > 0 {
                app.editing_arg_idx = app.editing_arg_idx.checked_sub(1).unwrap_or(n - 1);
            }
        }
        KeyCode::Backspace => {
            if app.editing_arg_idx < n {
                app.editing_args[app.editing_arg_idx].pop();
            }
        }
        KeyCode::Char(c) => {
            if app.editing_arg_idx < n {
                app.editing_args[app.editing_arg_idx].push(c);
            }
        }
        _ => {}
    }
}

fn handle_edit_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => {
            app.run_pipeline();
            app.mode = Mode::Normal;
        }
        KeyCode::Backspace => {
            app.input_text.pop();
        }
        KeyCode::Char(c) => app.input_text.push(c),
        _ => {}
    }
}

fn handle_help(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => app.mode = Mode::Normal,
        _ => {}
    }
}

fn handle_load_picker(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = Mode::Normal,
        KeyCode::Enter => {
            if let Some(idx) = app.recipe_list_state.selected() {
                if let Some(name) = app.recipe_list.get(idx).cloned() {
                    app.load_recipe(&name);
                }
            }
            app.mode = Mode::Normal;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let max = app.recipe_list.len().saturating_sub(1);
            let next = app
                .recipe_list_state
                .selected()
                .unwrap_or(0)
                .saturating_add(1)
                .min(max);
            app.recipe_list_state.select(Some(next));
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let prev = app
                .recipe_list_state
                .selected()
                .unwrap_or(0)
                .saturating_sub(1);
            app.recipe_list_state.select(Some(prev));
        }
        _ => {}
    }
}

fn handle_trace_view(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('t') | KeyCode::Char('q') => app.mode = Mode::Normal,
        KeyCode::Down | KeyCode::Char('j') => app.trace_scroll = app.trace_scroll.saturating_add(1),
        KeyCode::Up | KeyCode::Char('k') => app.trace_scroll = app.trace_scroll.saturating_sub(1),
        KeyCode::PageDown => app.trace_scroll = app.trace_scroll.saturating_add(10),
        KeyCode::PageUp => app.trace_scroll = app.trace_scroll.saturating_sub(10),
        _ => {}
    }
}

fn handle_file_prompt(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = Mode::Normal,
        KeyCode::Enter => {
            let path = app.file_prompt.trim().to_string();
            if path.is_empty() {
                app.status = "No filename entered.".into();
                app.mode = Mode::Normal;
                return;
            }
            if app.mode == Mode::SavePrompt {
                app.save_recipe(&path);
            } else if app.mode == Mode::CheckpointSavePrompt {
                app.save_checkpoint(&path);
            } else {
                app.load_recipe(&path);
            }
            app.mode = Mode::Normal;
        }
        KeyCode::Backspace => {
            app.file_prompt.pop();
        }
        KeyCode::Char(c) => app.file_prompt.push(c),
        _ => {}
    }
}

fn handle_checkpoints_list(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.mode = Mode::Normal,
        KeyCode::Enter => {
            if let Some(idx) = app.checkpoint_list_state.selected() {
                app.restore_checkpoint(idx);
            }
            app.mode = Mode::Normal;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let max = app.checkpoints.len().saturating_sub(1);
            let next = app
                .checkpoint_list_state
                .selected()
                .unwrap_or(0)
                .saturating_add(1)
                .min(max);
            app.checkpoint_list_state.select(Some(next));
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let prev = app
                .checkpoint_list_state
                .selected()
                .unwrap_or(0)
                .saturating_sub(1);
            app.checkpoint_list_state.select(Some(prev));
        }
        _ => {}
    }
}

fn render_checkpoints_list(f: &mut Frame, app: &mut App, area: Rect) {
    let overlay = centered_rect(50, 60, area);
    let block = Block::default()
        .title(format!(" Checkpoints ({}) ", app.checkpoints.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let items: Vec<ListItem> = app
        .checkpoints
        .iter()
        .map(|cp| ListItem::new(cp.name.as_str()))
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, overlay, &mut app.checkpoint_list_state);
}

// ─── Layout helpers ───────────────────────────────────────────────────────────

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn chrono_now_tui() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if matches!(args[1].as_str(), "--help" | "-h") {
            println!("rxchef_tui — interactive pipeline builder");
            println!("Run without arguments. For headless recipes use `rxchef bake`.");
            return Ok(());
        }
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "rxchef_tui is interactive; use `rxchef bake --recipe FILE` for headless execution",
        ));
    }

    // Interactive TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut quit = false;

    while !quit {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                // Global quit
                if (key.code == KeyCode::Char('q') && app.mode == Mode::Normal)
                    || (key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL))
                {
                    quit = true;
                } else {
                    handle_key(&mut app, key);
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::StepResult;

    #[test]
    fn preview_is_unicode_safe_past_the_character_limit() {
        for value in ["ä", "€", "日本語", "😀"] {
            let text = value.repeat(5000);
            let result = StepResult {
                output: text.into_bytes(),
                error: None,
            };
            assert!(result.display().contains('…'));
        }
    }
}
