/*!
rxchef — CyberChef operations in the terminal

# Quick reference

```
rxchef run <op> [input]                  # single operation
rxchef pipe "to_hex" "sha2,256" -- Hello # pipeline inline
echo Hello | rxchef pipe "to_base64"     # from stdin
rxchef recipe recipe.json --input Hello  # JSON/YAML recipe file

rxchef pipeline list                     # list saved pipelines
rxchef pipeline new my-pipe             # create pipeline
rxchef pipeline add my-pipe to_hex None 0
rxchef pipeline add my-pipe sha2 256
rxchef pipeline run my-pipe --input Hello --trace
rxchef pipeline set my-pipe 1 1 Colon   # step 1, arg 1 = "Colon"
rxchef pipeline show my-pipe
rxchef pipeline export my-pipe --yaml

rxchef var set KEY secret123            # store variable
rxchef var list                          # show all variables
rxchef pipe "aes_encrypt,$KEY,$IV,CBC" --input Hello

rxchef history list                      # show run history
rxchef history show <id>
rxchef magic --input "SGVsbG8="         # auto-suggest operations
```
*/

use std::{
    collections::HashMap,
    fs,
    io::{self, IsTerminal, Read, Write},
    path::PathBuf,
};

use clap::{Args, Parser, Subcommand};
use rxchef::runtime;
use rxchef_store::{self as store, Scope};

// ─── CLI structure ────────────────────────────────────────────────────────────

#[derive(Debug, Parser)]
#[command(
    name = "rxchef",
    version,
    about = "CyberChef operations in the terminal",
    arg_required_else_help = true,
    after_help = "Use 'rxchef <command> --help' for details on each command."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List available operations.
    List(ListArgs),
    /// Show operation metadata and argument schema.
    Info(InfoArgs),
    /// Run a single operation.
    Run(RunArgs),
    /// Run a pipeline of operations inline.
    ///
    /// Each STEP is "OpName" or "OpName,arg1,arg2".
    /// Underscores, dashes, and camelCase are normalized: to_hex = ToHex = "To Hex".
    Pipe(PipeArgs),
    /// Run a JSON or YAML recipe file (or inline JSON string).
    Recipe(RecipeArgs),
    /// Manage saved pipelines (list, new, add, remove, set, run, export, import, delete).
    Pipeline(PipelineArgs),
    /// Manage stored variables used in pipeline args ($VAR expansion).
    Var(VarArgs),
    /// Browse and replay run history.
    History(HistoryArgs),
    /// Analyze input and suggest operations.
    Magic(MagicArgs),
}

// ─── List ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct ListArgs {
    /// Filter by name (case-insensitive).
    search: Option<String>,
    /// Show module/category.
    #[arg(short, long)]
    modules: bool,
    /// Output as JSON.
    #[arg(short, long)]
    json: bool,
}

// ─── Info ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct InfoArgs {
    /// Operation name (normalized: to_hex = ToHex = "To Hex").
    operation: String,
    /// Output as JSON.
    #[arg(short, long)]
    json: bool,
}

// ─── Run ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct RunArgs {
    /// Operation name.
    operation: String,
    /// Literal input text.
    #[arg(short, long, conflicts_with_all = ["input_file"])]
    input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with_all = ["input"])]
    input_file: Option<PathBuf>,
    /// Output raw bytes as hex.
    #[arg(long)]
    hex: bool,
    /// Output as JSON envelope.
    #[arg(short, long)]
    json: bool,
    /// Named argument: --arg Key=hex:00... (can repeat).
    #[arg(long = "arg", value_name = "NAME=VALUE")]
    named_args: Vec<String>,
    /// Override variables: KEY=value.
    #[arg(long = "set", value_name = "KEY=VALUE")]
    set_vars: Vec<String>,
    /// Positional operation arguments.
    #[arg(
        value_name = "ARG",
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    args: Vec<String>,
}

// ─── Pipe ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct PipeArgs {
    /// Steps: "OpName" or "OpName,arg1,arg2". Supports to_hex / ToHex / "To Hex".
    #[arg(value_name = "STEP")]
    steps: Vec<String>,
    #[arg(short, long, conflicts_with_all = ["input_file"])]
    input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with_all = ["input"])]
    input_file: Option<PathBuf>,
    /// Show output after each step.
    #[arg(short, long)]
    trace: bool,
    /// Output as hex.
    #[arg(long)]
    hex: bool,
    /// Output as JSON envelope.
    #[arg(short, long)]
    json: bool,
    /// Save run to history.
    #[arg(long)]
    save: bool,
    /// Override variables: KEY=value.
    #[arg(long = "set", value_name = "KEY=VALUE")]
    set_vars: Vec<String>,
}

// ─── Recipe ───────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct RecipeArgs {
    /// JSON/YAML recipe file path OR inline JSON string.
    recipe: String,
    #[arg(short, long, conflicts_with_all = ["input_file"])]
    input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with_all = ["input"])]
    input_file: Option<PathBuf>,
    #[arg(short, long)]
    trace: bool,
    #[arg(long)]
    hex: bool,
    #[arg(long)]
    save: bool,
    /// Override variables: KEY=value.
    #[arg(long = "set", value_name = "KEY=VALUE")]
    set_vars: Vec<String>,
}

// ─── Pipeline ─────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct PipelineArgs {
    #[command(subcommand)]
    action: PipelineAction,
}

#[derive(Debug, Subcommand)]
enum PipelineAction {
    /// List saved pipelines.
    List {
        /// Only global store.
        #[arg(long, conflicts_with = "project")]
        global: bool,
        /// Only project store.
        #[arg(long, conflicts_with = "global")]
        project: bool,
        #[arg(short, long)]
        json: bool,
    },
    /// Show full pipeline definition.
    Show {
        name: String,
        #[arg(short, long)]
        json: bool,
        #[arg(long, default_value = "yaml", value_parser = ["json","yaml"])]
        format: String,
    },
    /// Create a new empty named pipeline.
    New {
        name: String,
        #[arg(short, long)]
        description: Option<String>,
        /// Save to global store instead of project.
        #[arg(short, long)]
        global: bool,
    },
    /// Add a step to a saved pipeline. Step format: "OpName" or "OpName,arg1,arg2".
    Add {
        pipeline: String,
        /// Step: "OpName" or "OpName,arg1,arg2". Remaining positional args are step args.
        step: String,
        /// Additional step args.
        #[arg(value_name = "ARG")]
        args: Vec<String>,
        /// Save to global store.
        #[arg(short, long)]
        global: bool,
    },
    /// Remove a step from a pipeline by 1-based index.
    Remove {
        pipeline: String,
        /// Step index (1-based).
        index: usize,
        #[arg(short, long)]
        global: bool,
    },
    /// Set an argument on a pipeline step.
    ///
    /// rxchef pipeline set <pipeline> <step-index> <arg-name-or-index> <value>
    Set {
        pipeline: String,
        /// Step index (1-based).
        step: usize,
        /// Argument name or 1-based index.
        arg: String,
        value: String,
        #[arg(short, long)]
        global: bool,
    },
    /// Run a saved pipeline.
    Run {
        name: String,
        #[arg(short, long, conflicts_with_all = ["input_file"])]
        input: Option<String>,
        #[arg(short = 'f', long, value_name = "PATH", conflicts_with_all = ["input"])]
        input_file: Option<PathBuf>,
        #[arg(short, long)]
        trace: bool,
        #[arg(long)]
        hex: bool,
        /// Save result to history.
        #[arg(long)]
        save: bool,
        /// Override variables: KEY=value.
        #[arg(long = "set", value_name = "KEY=VALUE")]
        set_vars: Vec<String>,
    },
    /// Delete a saved pipeline.
    Delete {
        name: String,
        #[arg(short, long)]
        global: bool,
        /// Skip confirmation prompt.
        #[arg(short, long)]
        yes: bool,
    },
    /// Export pipeline to stdout as JSON or YAML.
    Export {
        name: String,
        #[arg(long, default_value = "yaml", value_parser = ["json","yaml"])]
        format: String,
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Import a pipeline from a JSON/YAML file.
    Import {
        /// File path (JSON or YAML).
        file: PathBuf,
        /// Override the recipe name.
        #[arg(long)]
        name: Option<String>,
        #[arg(short, long)]
        global: bool,
    },
    /// Rename a pipeline.
    Rename {
        old_name: String,
        new_name: String,
        #[arg(short, long)]
        global: bool,
    },
}

// ─── Var ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct VarArgs {
    #[command(subcommand)]
    action: VarAction,
}

#[derive(Debug, Subcommand)]
enum VarAction {
    /// Set a variable (project scope by default).
    Set {
        name: String,
        value: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        global: bool,
    },
    /// Get a variable value.
    Get { name: String },
    /// List all variables.
    List {
        #[arg(long, conflicts_with = "project")]
        global: bool,
        #[arg(long, conflicts_with = "global")]
        project: bool,
        #[arg(short, long)]
        json: bool,
    },
    /// Remove a variable.
    Unset {
        name: String,
        #[arg(short, long)]
        global: bool,
    },
}

// ─── History ──────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct HistoryArgs {
    #[command(subcommand)]
    action: HistoryAction,
}

#[derive(Debug, Subcommand)]
enum HistoryAction {
    /// List recent runs.
    List {
        #[arg(short, long, default_value = "20")]
        limit: usize,
        #[arg(short, long)]
        json: bool,
    },
    /// Show detailed output of one run.
    Show {
        /// Run ID (from `rxchef history list`).
        id: String,
    },
    /// Re-run the steps from a history entry with new or original input.
    Run {
        id: String,
        #[arg(short, long)]
        input: Option<String>,
        #[arg(short, long)]
        trace: bool,
    },
    /// Clear all history.
    Clear {
        #[arg(short, long)]
        yes: bool,
    },
}

// ─── Magic ────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct MagicArgs {
    #[arg(short, long, conflicts_with = "input_file")]
    input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with = "input")]
    input_file: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

// ─── Entry point ─────────────────────────────────────────────────────────────

fn main() {
    if let Err(e) = run() {
        eprintln!("rxchef: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    match Cli::parse().command {
        Command::List(a) => cmd_list(a),
        Command::Info(a) => cmd_info(a),
        Command::Run(a) => cmd_run(a),
        Command::Pipe(a) => cmd_pipe(a),
        Command::Recipe(a) => cmd_recipe(a),
        Command::Pipeline(a) => cmd_pipeline(a),
        Command::Var(a) => cmd_var(a),
        Command::History(a) => cmd_history(a),
        Command::Magic(a) => cmd_magic(a),
    }
}

// ─── List ─────────────────────────────────────────────────────────────────────

fn cmd_list(a: ListArgs) -> Result<(), String> {
    let names = runtime::operation_names(a.search.as_deref());
    if a.json {
        let v: Vec<_> = names.iter().map(|n| serde_json::json!(n)).collect();
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
        return Ok(());
    }
    if a.modules {
        for (module, name) in runtime::operation_names_with_modules(a.search.as_deref())? {
            println!("{:<32} {}", module, name);
        }
    } else {
        for n in &names {
            println!("{}", n);
        }
    }
    eprintln!("\n{} operation(s)", names.len());
    Ok(())
}

// ─── Info ─────────────────────────────────────────────────────────────────────

fn cmd_info(a: InfoArgs) -> Result<(), String> {
    let op = runtime::operation_info(&a.operation)?;
    if a.json {
        let args: Vec<_> = op
            .args
            .iter()
            .map(|x| {
                serde_json::json!({
                    "name": x.name, "description": x.description, "default": x.default_value
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "name": op.name, "module": op.module, "description": op.description,
                "input_type": runtime::data_type_name(op.input_type),
                "output_type": runtime::data_type_name(op.output_type),
                "broken": op.is_broken, "args": args
            }))
            .unwrap()
        );
        return Ok(());
    }
    println!("Name:        {}", op.name);
    println!("Module:      {}", op.module);
    println!("Description: {}", op.description);
    println!(
        "I/O:         {} → {}",
        runtime::data_type_name(op.input_type),
        runtime::data_type_name(op.output_type)
    );
    if op.is_broken {
        println!("Broken:      yes");
    }
    if op.args.is_empty() {
        println!("Args:        none");
    } else {
        println!("Args:");
        for (i, a) in op.args.iter().enumerate() {
            println!(
                "  {}. {} [{}]  {}",
                i + 1,
                a.name,
                runtime::display_default(a.default_value),
                a.description
            );
        }
    }
    Ok(())
}

// ─── Run ──────────────────────────────────────────────────────────────────────

fn cmd_run(a: RunArgs) -> Result<(), String> {
    let input = load_input_from(a.input, a.input_file, &[])?;
    let var_overrides = parse_set_vars(&a.set_vars)?;
    let resolved = runtime::resolve_named_args(&a.operation, &a.named_args, &a.args)?;
    let expanded: Vec<String> = resolved
        .iter()
        .map(|arg| store::expand_vars(arg, &var_overrides))
        .collect();
    let output = runtime::run_operation(&a.operation, input.bytes, &expanded)?;
    if a.json {
        write_json_output(&output)
    } else {
        write_output(&output, a.hex)
    }
}

// ─── Pipe ─────────────────────────────────────────────────────────────────────

fn cmd_pipe(a: PipeArgs) -> Result<(), String> {
    if a.steps.is_empty() {
        return Err("no steps — usage: rxchef pipe \"to_hex,Space\" \"sha2,256\" -- Hello".into());
    }
    let var_overrides = parse_set_vars(&a.set_vars)?;
    let input = load_input_from(a.input, a.input_file, &[])?;
    let steps: Vec<_> = a.steps.iter().map(|s| parse_step_str(s)).collect();
    let input_bytes = input.bytes.clone();
    let result = run_steps(
        &steps,
        input.bytes,
        &var_overrides,
        a.trace && !a.json,
        a.hex,
    )?;
    if a.save {
        save_to_history(&steps, None, &input_bytes, &result)?;
    }
    if a.json {
        write_json_pipe_output(&result, if a.trace { Some(&steps) } else { None })
    } else {
        write_output(&result.final_output, a.hex)
    }
}

// ─── Recipe ───────────────────────────────────────────────────────────────────

fn cmd_recipe(a: RecipeArgs) -> Result<(), String> {
    let var_overrides = parse_set_vars(&a.set_vars)?;
    let recipe = load_recipe_arg(&a.recipe)?;
    let input = load_input_from(a.input, a.input_file, &[])?;
    let steps: Vec<_> = recipe
        .steps
        .iter()
        .map(|s| Step {
            op: s.op.clone(),
            args: s.args.clone(),
        })
        .collect();
    let input_bytes = input.bytes.clone();
    let result = run_steps(&steps, input.bytes, &var_overrides, a.trace, a.hex)?;
    if a.save {
        save_to_history(&steps, Some(&recipe.name), &input_bytes, &result)?;
    }
    write_output(&result.final_output, a.hex)
}

fn load_recipe_arg(arg: &str) -> Result<store::Recipe, String> {
    // Check if it's a file path
    let as_path = std::path::Path::new(arg);
    if as_path.exists() {
        return store::import_recipe(as_path, None, Scope::Project)
            .map_err(|e| e.to_string())
            .or_else(|_| {
                // If save fails (e.g. no project dir), just load it
                load_recipe_from_file(as_path)
            });
    }
    // Try named recipe from store
    if !arg.trim_start().starts_with('[') && !arg.trim_start().starts_with('{') {
        if let Ok(r) = store::load_recipe(arg) {
            return Ok(r);
        }
    }
    // Try inline JSON
    if arg.trim_start().starts_with('[') {
        let steps: Vec<store::RecipeStep> =
            serde_json::from_str(arg).map_err(|e| format!("invalid recipe JSON: {e}"))?;
        return Ok(store::Recipe {
            name: "inline".into(),
            description: String::new(),
            steps,
            tags: vec![],
        });
    }
    Err(format!("recipe not found: '{}'", arg))
}

fn load_recipe_from_file(path: &std::path::Path) -> Result<store::Recipe, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("json");
    if ext == "yaml" || ext == "yml" {
        if content.trim_start().starts_with('-') {
            let steps: Vec<store::RecipeStep> =
                serde_yaml::from_str(&content).map_err(|e| format!("YAML error: {e}"))?;
            return Ok(store::Recipe {
                name: path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .into(),
                description: String::new(),
                steps,
                tags: vec![],
            });
        }
        serde_yaml::from_str(&content).map_err(|e| format!("YAML error: {e}"))
    } else {
        if content.trim_start().starts_with('[') {
            let steps: Vec<store::RecipeStep> =
                serde_json::from_str(&content).map_err(|e| format!("JSON error: {e}"))?;
            return Ok(store::Recipe {
                name: path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .into(),
                description: String::new(),
                steps,
                tags: vec![],
            });
        }
        serde_json::from_str(&content).map_err(|e| format!("JSON error: {e}"))
    }
}

// ─── Pipeline management ─────────────────────────────────────────────────────

fn cmd_pipeline(a: PipelineArgs) -> Result<(), String> {
    match a.action {
        PipelineAction::List {
            global,
            project,
            json,
        } => {
            let scope = if global {
                Some(Scope::Global)
            } else if project {
                Some(Scope::Project)
            } else {
                None
            };
            let recipes = store::list_recipes(scope);
            if json {
                let v: Vec<_> = recipes
                    .iter()
                    .map(|r| {
                        serde_json::json!({
                            "name": r.name, "description": r.description,
                            "steps": r.step_count,
                            "scope": if r.scope == Scope::Global { "global" } else { "project" },
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
                return Ok(());
            }
            if recipes.is_empty() {
                println!("No saved pipelines. Create one with: rxchef pipeline new <name>");
                return Ok(());
            }
            println!(
                "{:<24} {:>5}  {:<8}  {}",
                "NAME", "STEPS", "SCOPE", "DESCRIPTION"
            );
            println!("{}", "-".repeat(70));
            for r in &recipes {
                let scope_tag = if r.scope == Scope::Global {
                    "global"
                } else {
                    "project"
                };
                println!(
                    "{:<24} {:>5}  {:<8}  {}",
                    r.name, r.step_count, scope_tag, r.description
                );
            }
            eprintln!("\n{} pipeline(s)", recipes.len());
        }

        PipelineAction::Show { name, json, format } => {
            let recipe = store::load_recipe(&name).map_err(|e| e.to_string())?;
            if json {
                println!("{}", serde_json::to_string_pretty(&recipe).unwrap());
            } else {
                let out = store::export_recipe(&recipe, &format).map_err(|e| e.to_string())?;
                println!("{}", out);
            }
        }

        PipelineAction::New {
            name,
            description,
            global,
        } => {
            let scope = if global {
                Scope::Global
            } else {
                Scope::Project
            };
            let mut recipe = store::Recipe::new(&name);
            if let Some(d) = description {
                recipe.description = d;
            }
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            println!("Created pipeline '{}' ({:?} scope).", name, scope);
        }

        PipelineAction::Add {
            pipeline,
            step,
            args,
            global,
        } => {
            let scope = if global {
                Scope::Global
            } else {
                Scope::Project
            };
            let mut recipe = store::load_recipe(&pipeline).map_err(|e| e.to_string())?;
            let parsed = parse_step_str(&step);
            let mut all_args = parsed.args;
            all_args.extend(args);
            recipe.steps.push(store::RecipeStep {
                op: parsed.op.clone(),
                args: all_args,
            });
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            println!(
                "Added '{}' as step {} to '{}'.",
                parsed.op,
                recipe.steps.len(),
                pipeline
            );
        }

        PipelineAction::Remove {
            pipeline,
            index,
            global,
        } => {
            let scope = if global {
                Scope::Global
            } else {
                Scope::Project
            };
            let mut recipe = store::load_recipe(&pipeline).map_err(|e| e.to_string())?;
            if index == 0 || index > recipe.steps.len() {
                return Err(format!(
                    "step index {} out of range (1–{})",
                    index,
                    recipe.steps.len()
                ));
            }
            let removed = recipe.steps.remove(index - 1);
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            println!("Removed step {}: '{}'.", index, removed.op);
        }

        PipelineAction::Set {
            pipeline,
            step,
            arg,
            value,
            global,
        } => {
            let scope = if global {
                Scope::Global
            } else {
                Scope::Project
            };
            let mut recipe = store::load_recipe(&pipeline).map_err(|e| e.to_string())?;
            if step == 0 || step > recipe.steps.len() {
                return Err(format!(
                    "step {} out of range (1–{})",
                    step,
                    recipe.steps.len()
                ));
            }
            let s = &mut recipe.steps[step - 1];

            // Resolve arg position: numeric index or arg name
            let arg_idx = if let Ok(n) = arg.parse::<usize>() {
                if n == 0 || n > s.args.len() {
                    // Extend args if needed
                    while s.args.len() < n {
                        s.args.push(String::new());
                    }
                    n - 1
                } else {
                    n - 1
                }
            } else {
                // Look up arg name from schema
                let op_info = runtime::operation_info(&s.op).map_err(|e| e.to_string())?;
                let arg_lower = arg.to_lowercase();
                let idx = op_info
                    .args
                    .iter()
                    .position(|a| a.name.to_lowercase() == arg_lower)
                    .ok_or_else(|| format!("argument '{}' not found in '{}'", arg, s.op))?;
                while s.args.len() <= idx {
                    s.args.push(String::new());
                }
                idx
            };

            let old = s.args.get(arg_idx).cloned().unwrap_or_default();
            s.args[arg_idx] = value.clone();
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            println!(
                "Step {}, arg {}: '{}' → '{}'.",
                step,
                arg_idx + 1,
                old,
                value
            );
        }

        PipelineAction::Run {
            name,
            input,
            input_file,
            trace,
            hex,
            save,
            set_vars,
        } => {
            let recipe = store::load_recipe(&name).map_err(|e| e.to_string())?;
            let var_overrides = parse_set_vars(&set_vars)?;
            let loaded_input = load_input_from(input, input_file, &[])?;
            let steps: Vec<_> = recipe
                .steps
                .iter()
                .map(|s| Step {
                    op: s.op.clone(),
                    args: s.args.clone(),
                })
                .collect();
            let result = run_steps(
                &steps,
                loaded_input.bytes.clone(),
                &var_overrides,
                trace,
                hex,
            )?;
            if save {
                save_to_history(&steps, Some(&name), &loaded_input.bytes, &result)?;
            }
            write_output(&result.final_output, hex)?;
        }

        PipelineAction::Delete { name, global, yes } => {
            let scope = if global {
                Scope::Global
            } else {
                Scope::Project
            };
            if !yes {
                eprint!("Delete pipeline '{}'? [y/N] ", name);
                io::stderr().flush().ok();
                let mut ans = String::new();
                io::stdin().read_line(&mut ans).ok();
                if !ans.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled.");
                    return Ok(());
                }
            }
            store::delete_recipe(&name, scope).map_err(|e| e.to_string())?;
            println!("Deleted '{}'.", name);
        }

        PipelineAction::Export {
            name,
            format,
            output,
        } => {
            let recipe = store::load_recipe(&name).map_err(|e| e.to_string())?;
            let out = store::export_recipe(&recipe, &format).map_err(|e| e.to_string())?;
            if let Some(path) = output {
                fs::write(&path, &out).map_err(|e| format!("write error: {e}"))?;
                println!("Exported to '{}'.", path.display());
            } else {
                println!("{}", out);
            }
        }

        PipelineAction::Import { file, name, global } => {
            let scope = if global {
                Scope::Global
            } else {
                Scope::Project
            };
            let recipe =
                store::import_recipe(&file, name.as_deref(), scope).map_err(|e| e.to_string())?;
            println!(
                "Imported pipeline '{}' ({} step(s)).",
                recipe.name,
                recipe.steps.len()
            );
        }

        PipelineAction::Rename {
            old_name,
            new_name,
            global,
        } => {
            let scope = if global {
                Scope::Global
            } else {
                Scope::Project
            };
            let mut recipe = store::load_recipe(&old_name).map_err(|e| e.to_string())?;
            recipe.name = new_name.clone();
            store::delete_recipe(&old_name, scope).map_err(|e| e.to_string())?;
            store::save_recipe(&recipe, scope).map_err(|e| e.to_string())?;
            println!("Renamed '{}' → '{}'.", old_name, new_name);
        }
    }
    Ok(())
}

// ─── Var ──────────────────────────────────────────────────────────────────────

fn cmd_var(a: VarArgs) -> Result<(), String> {
    match a.action {
        VarAction::Set {
            name,
            value,
            description,
            global,
        } => {
            let scope = if global {
                Scope::Global
            } else {
                Scope::Project
            };
            store::set_var(&name, &value, description.as_deref().unwrap_or(""), scope)
                .map_err(|e| e.to_string())?;
            println!("Set ${} = '{}'  ({:?})", name.to_uppercase(), value, scope);
        }
        VarAction::Get { name } => match store::get_var(&name) {
            Some(v) => println!("{}", v),
            None => return Err(format!("variable '{}' not found", name)),
        },
        VarAction::List {
            global,
            project,
            json,
        } => {
            let scope = if global {
                Some(Scope::Global)
            } else if project {
                Some(Scope::Project)
            } else {
                None
            };
            let vars = store::list_vars(scope);
            if json {
                let v: Vec<_> = vars
                    .iter()
                    .map(|v| {
                        serde_json::json!({
                            "name": v.name, "value": v.value, "description": v.description
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
                return Ok(());
            }
            if vars.is_empty() {
                println!("No variables set. Use: rxchef var set <name> <value>");
                return Ok(());
            }
            println!("{:<20} {:<40}  {}", "NAME", "VALUE", "DESCRIPTION");
            println!("{}", "-".repeat(70));
            for v in &vars {
                println!("{:<20} {:<40}  {}", v.name, v.value, v.description);
            }
        }
        VarAction::Unset { name, global } => {
            let scope = if global {
                Scope::Global
            } else {
                Scope::Project
            };
            store::unset_var(&name, scope).map_err(|e| e.to_string())?;
            println!("Removed ${}.", name.to_uppercase());
        }
    }
    Ok(())
}

// ─── History ──────────────────────────────────────────────────────────────────

fn cmd_history(a: HistoryArgs) -> Result<(), String> {
    match a.action {
        HistoryAction::List { limit, json } => {
            let entries = store::list_history(Some(limit));
            if json {
                let v: Vec<_> = entries
                    .iter()
                    .map(|e| {
                        serde_json::json!({
                            "id": e.id, "timestamp": e.timestamp,
                            "pipeline": e.pipeline_name,
                            "steps": e.steps.len(),
                            "input_preview": e.input_preview,
                            "output_preview": e.output_preview,
                            "success": e.success,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
                return Ok(());
            }
            if entries.is_empty() {
                println!("No history. Use --save with pipe/recipe/pipeline run to record runs.");
                return Ok(());
            }
            println!(
                "{:<20} {:<22} {:>5}  {}",
                "ID", "TIMESTAMP", "STEPS", "INPUT PREVIEW"
            );
            println!("{}", "-".repeat(75));
            for e in &entries {
                println!(
                    "{:<20} {:<22} {:>5}  {}",
                    e.id,
                    e.timestamp,
                    e.steps.len(),
                    e.input_preview
                );
            }
        }

        HistoryAction::Show { id } => {
            let entry = store::get_history(&id)
                .ok_or_else(|| format!("history entry '{}' not found", id))?;
            println!("ID:        {}", entry.id);
            println!("Timestamp: {}", entry.timestamp);
            if let Some(n) = &entry.pipeline_name {
                println!("Pipeline:  {}", n);
            }
            println!(
                "Input:     {} ({} bytes)",
                entry.input_preview, entry.input_bytes
            );
            println!("Success:   {}", entry.success);
            println!();
            for (i, step) in entry.steps.iter().enumerate() {
                println!("Step {}  {} [{}]", i + 1, step.op, step.args.join(", "));
                if let Some(e) = &step.error {
                    println!("  ERROR: {}", e);
                } else {
                    println!(
                        "  output ({} bytes): {}",
                        step.output_bytes, step.output_preview
                    );
                }
            }
            println!(
                "\nFinal output ({} bytes):\n{}",
                entry.output_bytes, entry.output_preview
            );
        }

        HistoryAction::Run { id, input, trace } => {
            let entry = store::get_history(&id)
                .ok_or_else(|| format!("history entry '{}' not found", id))?;
            let input_bytes = if let Some(t) = input {
                t.into_bytes()
            } else {
                if entry.input_bytes > entry.input_preview.len() {
                    eprintln!("warning: original input was {} bytes but only {}-char preview is available — output may differ",
                        entry.input_bytes, entry.input_preview.len());
                }
                entry.input_preview.as_bytes().to_vec()
            };
            let steps: Vec<_> = entry
                .steps
                .iter()
                .map(|s| Step {
                    op: s.op.clone(),
                    args: s.args.clone(),
                })
                .collect();
            let result = run_steps(&steps, input_bytes, &HashMap::new(), trace, false)?;
            write_output(&result.final_output, false)?;
        }

        HistoryAction::Clear { yes } => {
            if !yes {
                eprint!("Clear all run history? [y/N] ");
                io::stderr().flush().ok();
                let mut ans = String::new();
                io::stdin().read_line(&mut ans).ok();
                if !ans.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled.");
                    return Ok(());
                }
            }
            store::clear_history().map_err(|e| e.to_string())?;
            println!("History cleared.");
        }
    }
    Ok(())
}

// ─── Magic ────────────────────────────────────────────────────────────────────

fn cmd_magic(a: MagicArgs) -> Result<(), String> {
    let input = load_input_from(a.input, a.input_file, &[])?.bytes;
    let results = rxchef::magic::analyze_input(&input);
    if results.is_empty() {
        println!("No operation suggestions.");
        return Ok(());
    }
    if a.json {
        let v: Vec<_> = results
            .iter()
            .map(|r| {
                serde_json::json!({
                    "op": r.op_name, "confidence": r.confidence, "description": r.description
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
        return Ok(());
    }
    println!("Suggested operations:\n");
    for r in &results {
        println!(
            "  {:<28}  {:>3.0}%  {}",
            r.op_name,
            r.confidence * 100.0,
            r.description
        );
    }
    Ok(())
}

// ─── Pipeline execution helpers ───────────────────────────────────────────────

struct Step {
    op: String,
    args: Vec<String>,
}

struct RunResult {
    final_output: Vec<u8>,
    steps: Vec<store::HistoryStep>,
}

fn run_steps(
    steps: &[Step],
    input: Vec<u8>,
    var_overrides: &HashMap<String, String>,
    trace: bool,
    hex: bool,
) -> Result<RunResult, String> {
    let mut current = input.clone();
    let mut all_bytes = vec![input];
    let mut history_steps = Vec::new();

    for (i, step) in steps.iter().enumerate() {
        let expanded_args: Vec<String> = step
            .args
            .iter()
            .map(|a| store::expand_vars(a, var_overrides))
            .collect();

        match runtime::run_operation(&step.op, current.clone(), &expanded_args) {
            Ok(output) => {
                if trace {
                    let label = format!("── step {}/{}: {} ──", i + 1, steps.len(), step.op);
                    eprintln!("{}", label);
                    let _ = write_output_raw(&output, hex, &mut io::stderr().lock());
                    eprintln!();
                }
                history_steps.push(store::HistoryStep {
                    op: step.op.clone(),
                    args: expanded_args,
                    output_preview: store::bytes_preview(&output, 300),
                    output_bytes: output.len(),
                    error: None,
                });
                all_bytes.push(output.clone());
                current = output;
            }
            Err(e) => {
                history_steps.push(store::HistoryStep {
                    op: step.op.clone(),
                    args: expanded_args,
                    output_preview: String::new(),
                    output_bytes: 0,
                    error: Some(e.clone()),
                });
                return Err(format!("step {} ({}): {}", i + 1, step.op, e));
            }
        }
    }

    drop(all_bytes);
    Ok(RunResult {
        final_output: current,
        steps: history_steps,
    })
}

fn save_to_history(
    _steps: &[Step],
    pipeline_name: Option<&str>,
    input: &[u8],
    result: &RunResult,
) -> Result<(), String> {
    let entry = store::HistoryEntry {
        id: store::new_history_id(),
        timestamp: chrono_now(),
        pipeline_name: pipeline_name.map(|s| s.to_string()),
        input_preview: store::bytes_preview(input, 300),
        input_bytes: input.len(),
        steps: result.steps.clone(),
        output_preview: store::bytes_preview(&result.final_output, 300),
        output_bytes: result.final_output.len(),
        success: true,
    };
    store::append_history(&entry).map_err(|e| e.to_string())
}

fn chrono_now() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

// ─── Step parsing ─────────────────────────────────────────────────────────────

fn parse_step_str(s: &str) -> Step {
    let mut parts = s.splitn(2, ',');
    let op = parts.next().unwrap_or("").trim().to_string();
    let args: Vec<String> = parts
        .next()
        .map(|rest| rest.split(',').map(|a| a.trim().to_string()).collect())
        .unwrap_or_default();
    Step { op, args }
}

fn parse_set_vars(raw: &[String]) -> Result<HashMap<String, String>, String> {
    raw.iter()
        .map(|kv| {
            let mut split = kv.splitn(2, '=');
            let k = split.next().unwrap_or("").to_uppercase();
            let v = split.next().unwrap_or("").to_string();
            if k.is_empty() {
                Err(format!("invalid --set value '{}': expected KEY=value", kv))
            } else {
                Ok((k, v))
            }
        })
        .collect()
}

// ─── Input loading ────────────────────────────────────────────────────────────

struct LoadedInput {
    bytes: Vec<u8>,
}

fn load_input_from(
    text: Option<String>,
    file: Option<PathBuf>,
    trailing_args: &[String],
) -> Result<LoadedInput, String> {
    if let Some(t) = text {
        return Ok(LoadedInput {
            bytes: t.into_bytes(),
        });
    }
    if let Some(p) = file {
        let b = fs::read(&p).map_err(|e| format!("cannot read '{}': {}", p.display(), e))?;
        return Ok(LoadedInput { bytes: b });
    }
    if !trailing_args.is_empty() {
        return Ok(LoadedInput {
            bytes: trailing_args[0].as_bytes().to_vec(),
        });
    }
    if !io::stdin().is_terminal() {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .map_err(|e| format!("stdin read error: {e}"))?;
        return Ok(LoadedInput { bytes: buf });
    }
    Ok(LoadedInput { bytes: Vec::new() })
}

// ─── Output ───────────────────────────────────────────────────────────────────

fn write_output(output: &[u8], hex: bool) -> Result<(), String> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    write_output_raw(output, hex, &mut out)?;
    if !hex && io::stdout().is_terminal() && !output.ends_with(b"\n") {
        out.write_all(b"\n").map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn write_json_output(output: &[u8]) -> Result<(), String> {
    use base64::{engine::general_purpose, Engine as _};
    let json = serde_json::json!({
        "output": String::from_utf8_lossy(output),
        "output_base64": general_purpose::STANDARD.encode(output),
        "output_len": output.len(),
    });
    println!("{}", serde_json::to_string(&json).unwrap());
    Ok(())
}

fn write_json_pipe_output(result: &RunResult, trace_steps: Option<&[Step]>) -> Result<(), String> {
    use base64::{engine::general_purpose, Engine as _};
    let mut json = serde_json::json!({
        "output": String::from_utf8_lossy(&result.final_output),
        "output_base64": general_purpose::STANDARD.encode(&result.final_output),
        "output_len": result.final_output.len(),
    });
    if let Some(steps) = trace_steps {
        let step_arr: Vec<_> = result
            .steps
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let op_name = steps.get(i).map(|st| st.op.as_str()).unwrap_or("?");
                serde_json::json!({
                    "step": i + 1,
                    "op": op_name,
                    "output": &s.output_preview,
                    "output_bytes": s.output_bytes,
                    "error": s.error,
                })
            })
            .collect();
        json["steps"] = serde_json::json!(step_arr);
    }
    println!("{}", serde_json::to_string(&json).unwrap());
    Ok(())
}

fn write_output_raw<W: Write>(output: &[u8], hex: bool, w: &mut W) -> Result<(), String> {
    if hex {
        for (i, b) in output.iter().enumerate() {
            if i > 0 && i % 16 == 0 {
                w.write_all(b"\n").map_err(|e| e.to_string())?;
            } else if i > 0 {
                w.write_all(b" ").map_err(|e| e.to_string())?;
            }
            write!(w, "{:02x}", b).map_err(|e| e.to_string())?;
        }
        w.write_all(b"\n").map_err(|e| e.to_string())
    } else {
        match std::str::from_utf8(output) {
            Ok(s) => w.write_all(s.as_bytes()).map_err(|e| e.to_string()),
            Err(_) => {
                // Binary: hex dump
                for (i, b) in output.iter().enumerate() {
                    if i > 0 && i % 16 == 0 {
                        w.write_all(b"\n").map_err(|e| e.to_string())?;
                    } else if i > 0 {
                        w.write_all(b" ").map_err(|e| e.to_string())?;
                    }
                    write!(w, "{:02x}", b).map_err(|e| e.to_string())?;
                }
                w.write_all(b"\n").map_err(|e| e.to_string())
            }
        }
    }
}
