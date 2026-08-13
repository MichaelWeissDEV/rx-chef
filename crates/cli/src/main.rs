/*!
rxchef — CyberChef operations in the terminal

# Quick reference

```
rxchef run <op> --input <text>           # single operation
rxchef operations --json                 # complete machine-readable registry
rxchef operation describe to_base64 --json
rxchef bake --recipe recipe.json --input Hello
rxchef serve --stdio                     # persistent JSONL/JSON-RPC transport
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
rxchef pipeline export my-pipe --format yaml

rxchef var set KEY secret123            # store variable
rxchef var list                          # show all variables
rxchef pipe "aes_encrypt,$KEY,$IV,CBC" --input Hello

rxchef history list                      # show run history
rxchef history show <id>

rxchef magic --input "U0dWc2JHOD0="     # recursively detect + decode
rxchef magic --input "…" --decode        # print best plaintext only (pipe-friendly)
rxchef magic --input "…" --crib SECRET   # rank decodes matching a known string

rxchef scan dump.bin --decode            # find + decode encoded strings in a file
rxchef scan ./logs -r --decode --json    # recurse a dir, emit NDJSON for jq/grep
cat huge.pcap | rxchef scan --entropy 4.5 # stream stdin, flag high-entropy blobs
```
*/

use std::{
    collections::HashMap,
    fs,
    io::{self, BufReader, IsTerminal, Read, Write},
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
    /// List complete operation descriptors for machine integrations.
    Operations(OperationsArgs),
    /// Inspect an operation through the integration API.
    Operation(OperationArgs),
    /// List available operations.
    List(ListArgs),
    /// Show operation metadata and argument schema.
    Info(InfoArgs),
    /// Run a single operation (input: --input, --input-file, or stdin).
    Run(RunArgs),
    /// Run a pipeline of operations inline.
    ///
    /// Each STEP is "OpName" or "OpName,arg1,arg2".
    /// Underscores, dashes, and camelCase are normalized: to_hex = ToHex = "To Hex".
    Pipe(PipeArgs),
    /// Run a JSON or YAML recipe file (or inline JSON string).
    Recipe(RecipeArgs),
    /// Execute a JSON/YAML recipe without persistent store behavior.
    Bake(BakeArgs),
    /// Manage saved pipelines (list, new, add, remove, set, run, export, import, delete).
    Pipeline(PipelineArgs),
    /// Manage stored variables used in pipeline args ($VAR expansion).
    Var(VarArgs),
    /// Browse and replay run history.
    History(HistoryArgs),
    /// Analyze input: recursively detect and decode encoded/encrypted data.
    Magic(MagicArgs),
    /// Scan files or streams for encoded/high-entropy strings and auto-decode.
    Scan(ScanArgs),
    /// Load and run full CTF projects (YAML/JSON) with data, vars, and pipelines.
    Project(ProjectArgs),
    /// Run a persistent machine-readable transport.
    Serve(ServeArgs),
}

// ─── Machine integration ─────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct OperationsArgs {
    /// Emit complete descriptors as JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct OperationArgs {
    #[command(subcommand)]
    action: OperationAction,
}

#[derive(Debug, Subcommand)]
enum OperationAction {
    /// Describe one operation and its complete argument schema.
    Describe {
        operation: String,
        /// Emit a machine-readable descriptor.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Args)]
struct BakeArgs {
    /// JSON/YAML recipe file (step array or object containing `steps`).
    #[arg(
        long,
        value_name = "PATH",
        conflicts_with = "recipe_json",
        required_unless_present = "recipe_json"
    )]
    recipe: Option<PathBuf>,
    /// Inline JSON recipe (step array or object containing `steps`).
    #[arg(
        long,
        value_name = "JSON",
        conflicts_with = "recipe",
        required_unless_present = "recipe"
    )]
    recipe_json: Option<String>,
    #[arg(short, long, conflicts_with = "input_file")]
    input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with = "input")]
    input_file: Option<PathBuf>,
    /// Render output as a hex dump.
    #[arg(long)]
    hex: bool,
    /// Emit the binary-safe result envelope as JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct ServeArgs {
    /// Read one JSON request per stdin line and write one response per stdout line.
    #[arg(long, required = true)]
    stdio: bool,
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
#[command(
    after_long_help = "INPUT\n  Use --input for literal UTF-8 text, --input-file for exact file bytes, or omit\n  both to read stdin. Operation arguments support num:, bool:, hex:, and bytes:\n  type prefixes. Use --arg NAME=VALUE to address an argument by schema name.\n\nEXAMPLES\n  printf 'hello' | rxchef run to_base64\n  rxchef run sha2 --input hello --arg Size=256\n  rxchef run xor --input-file data.bin hex:deadbeef Standard false"
)]
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
#[command(
    after_long_help = "STEP SYNTAX\n  Each STEP is a comma-separated operation and its arguments. Quote or escape a\n  comma inside an argument: 'find_replace,\"a,b\",Simple string,x'.\n  Steps execute left-to-right; stdout contains only the final bytes and --trace\n  writes intermediate results to stderr. Input comes from --input, --input-file,\n  or stdin.\n\nEXAMPLES\n  printf 'hello' | rxchef pipe to_upper_case to_base64\n  rxchef pipe 'find_replace,\"a,b\",Simple string,x' to_base64 --input 'a,b'"
)]
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
    /// Maximum recursion depth (chained decodes).
    #[arg(short, long, default_value = "3")]
    depth: usize,
    /// Known-plaintext filter (substring/regex); matching candidates rank first.
    #[arg(long, value_name = "REGEX")]
    crib: Option<String>,
    /// Try aggressive decoders too (ROT13, Base58/85).
    #[arg(long)]
    intensive: bool,
    /// Print only the best decoded output (raw, pipe-friendly).
    #[arg(long)]
    decode: bool,
    /// Output as hex when using --decode.
    #[arg(long)]
    hex: bool,
    /// Output as JSON.
    #[arg(short, long)]
    json: bool,
}

// ─── Scan ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct ScanArgs {
    /// Files or directories to scan. Omit to read from stdin.
    #[arg(value_name = "PATH")]
    paths: Vec<PathBuf>,
    /// Recurse into directories.
    #[arg(short, long)]
    recursive: bool,
    /// Minimum token length to consider.
    #[arg(long, default_value = "16")]
    min_len: usize,
    /// Attempt to decode each finding via the magic engine.
    #[arg(short, long)]
    decode: bool,
    /// Recursion depth when decoding.
    #[arg(long, default_value = "3")]
    depth: usize,
    /// Only report findings whose decode matches this crib (regex; implies --decode).
    #[arg(long, value_name = "REGEX")]
    crib: Option<String>,
    /// Also report tokens with entropy >= this (bits/byte), even if no decoder fired.
    #[arg(long, value_name = "BITS")]
    entropy: Option<f64>,
    /// Restrict to these encodings, comma-separated (e.g. base64,hex).
    #[arg(long, value_delimiter = ',', value_name = "KIND")]
    kind: Vec<String>,
    /// Emit newline-delimited JSON (one finding per line), ideal for jq/grep.
    #[arg(short, long)]
    json: bool,
}

// ─── Project ──────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
struct ProjectArgs {
    #[command(subcommand)]
    action: ProjectAction,
}

#[derive(Debug, Subcommand)]
enum ProjectAction {
    /// Run a CTF project file (YAML/JSON).
    Run {
        /// Project file path
        file: PathBuf,
        #[arg(short, long)]
        trace: bool,
    },
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
        Command::Operations(a) => cmd_operations(a),
        Command::Operation(a) => cmd_operation(a),
        Command::List(a) => cmd_list(a),
        Command::Info(a) => cmd_info(a),
        Command::Run(a) => cmd_run(a),
        Command::Pipe(a) => cmd_pipe(a),
        Command::Recipe(a) => cmd_recipe(a),
        Command::Bake(a) => cmd_bake(a),
        Command::Pipeline(a) => cmd_pipeline(a),
        Command::Var(a) => cmd_var(a),
        Command::History(a) => cmd_history(a),
        Command::Magic(a) => cmd_magic(a),
        Command::Scan(a) => cmd_scan(a),
        Command::Project(a) => cmd_project(a),
        Command::Serve(a) => cmd_serve(a),
    }
}

fn cmd_operations(a: OperationsArgs) -> Result<(), String> {
    let operations = rxchef::integration::operations()?;
    if a.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&operations).map_err(|error| error.to_string())?
        );
    } else {
        for operation in &operations {
            println!(
                "{:<28} {:<18} {}",
                operation.name, operation.module, operation.description
            );
        }
        eprintln!("\n{} operation(s)", operations.len());
    }
    Ok(())
}

fn cmd_operation(a: OperationArgs) -> Result<(), String> {
    match a.action {
        OperationAction::Describe { operation, json } => {
            let descriptor = rxchef::integration::describe(&operation)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&descriptor).map_err(|error| error.to_string())?
                );
            } else {
                cmd_info(InfoArgs {
                    operation,
                    json: false,
                })?;
            }
        }
    }
    Ok(())
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
    let steps = a
        .steps
        .iter()
        .map(|s| parse_step_str(s))
        .collect::<Result<Vec<_>, _>>()?;
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

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum BakeRecipe {
    Steps(Vec<rxchef::integration::RecipeStep>),
    Document {
        #[serde(alias = "pipeline")]
        steps: Vec<rxchef::integration::RecipeStep>,
    },
}

impl BakeRecipe {
    fn into_steps(self) -> Vec<rxchef::integration::RecipeStep> {
        match self {
            Self::Steps(steps) | Self::Document { steps } => steps,
        }
    }
}

fn cmd_bake(a: BakeArgs) -> Result<(), String> {
    let (content, is_yaml) = match (a.recipe, a.recipe_json) {
        (Some(path), None) => {
            let content = fs::read_to_string(&path)
                .map_err(|error| format!("cannot read '{}': {error}", path.display()))?;
            let is_yaml = matches!(
                path.extension().and_then(|extension| extension.to_str()),
                Some("yaml" | "yml")
            );
            (content, is_yaml)
        }
        (None, Some(content)) => (content, false),
        _ => return Err("provide exactly one of --recipe or --recipe-json".into()),
    };
    let recipe: BakeRecipe = if is_yaml {
        serde_yaml::from_str(&content).map_err(|error| format!("invalid recipe YAML: {error}"))?
    } else {
        serde_json::from_str(&content).map_err(|error| format!("invalid recipe JSON: {error}"))?
    };
    let input = load_input_from(a.input, a.input_file, &[])?.bytes;
    let result = rxchef::integration::bake(input, &recipe.into_steps())?;
    if a.json {
        println!(
            "{}",
            serde_json::to_string(&result).map_err(|error| error.to_string())?
        );
    } else {
        write_output(&result.into_bytes()?, a.hex)?;
    }
    Ok(())
}

fn cmd_serve(a: ServeArgs) -> Result<(), String> {
    if !a.stdio {
        return Err("only --stdio transport is currently supported".into());
    }
    let stdin = io::stdin();
    let stdout = io::stdout();
    rxchef::integration::serve_jsonl(BufReader::new(stdin.lock()), stdout.lock())
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
            let parsed = parse_step_str(&step)?;
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
    use rxchef::magic::{magic, MagicOptions};

    let input = load_input_from(a.input, a.input_file, &[])?.bytes;
    let crib = match &a.crib {
        Some(c) => {
            Some(regex::Regex::new(c).map_err(|e| format!("invalid crib regex '{c}': {e}"))?)
        }
        None => None,
    };
    let opts = MagicOptions {
        depth: a.depth,
        crib,
        intensive: a.intensive,
        max_results: 20,
    };
    let results = magic(&input, &opts);

    // --decode: emit only the winning plaintext, raw. Nothing else on stdout.
    if a.decode {
        return match results.first() {
            Some(m) => write_output(&m.data, a.hex),
            None => Err("magic: no decoding found".into()),
        };
    }

    if a.json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
        return Ok(());
    }

    if results.is_empty() {
        println!("No candidate decodings found.");
        if !input.is_empty() {
            eprintln!(
                "hint: try --intensive for aggressive decoders, or --depth for deeper chains"
            );
        }
        return Ok(());
    }

    println!("{} candidate decoding(s), best first:\n", results.len());
    for (i, m) in results.iter().enumerate() {
        let recipe = format_recipe(&m.recipe);
        let crib_tag = if m.matched_crib { "  ✓crib" } else { "" };
        println!(
            "{:>2}. {}  [entropy {:.2}]{}",
            i + 1,
            recipe,
            m.entropy,
            crib_tag
        );
        println!("    {}", m.preview);
    }
    eprintln!("\nRe-run one recipe with:  rxchef pipe \"...\" --input <data>");
    Ok(())
}

fn format_recipe(steps: &[rxchef::magic::RecipeStep]) -> String {
    steps
        .iter()
        .map(|s| {
            if s.args.is_empty() {
                s.op.clone()
            } else {
                format!("{}({})", s.op, s.args.join(","))
            }
        })
        .collect::<Vec<_>>()
        .join(" → ")
}

// ─── Scan ─────────────────────────────────────────────────────────────────────

fn cmd_scan(a: ScanArgs) -> Result<(), String> {
    use rxchef::scan::{ScanOptions, Scanner};

    let crib = match &a.crib {
        Some(c) => {
            Some(regex::Regex::new(c).map_err(|e| format!("invalid crib regex '{c}': {e}"))?)
        }
        None => None,
    };
    let opts = ScanOptions {
        min_len: a.min_len,
        max_len: 1 << 20,
        decode: a.decode,
        depth: a.depth,
        crib,
        min_entropy: a.entropy.unwrap_or(0.0),
        only_kinds: a.kind.clone(),
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut total = 0usize;

    // Collect the source list: explicit paths (walked) or stdin.
    let mut sources: Vec<Option<PathBuf>> = Vec::new();
    if a.paths.is_empty() {
        sources.push(None); // stdin
    } else {
        for p in &a.paths {
            collect_paths(p, a.recursive, &mut sources)?;
        }
    }

    for src in sources {
        let label = src
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "<stdin>".to_string());

        let reader: Box<dyn Read> = match &src {
            Some(p) => Box::new(
                fs::File::open(p).map_err(|e| format!("cannot open '{}': {e}", p.display()))?,
            ),
            None => Box::new(io::stdin().lock()),
        };
        let mut reader = io::BufReader::with_capacity(64 * 1024, reader);

        let mut scanner = Scanner::new(opts.clone());
        let mut buf = vec![0u8; 64 * 1024];
        let mut emit = |f: rxchef::scan::Finding| {
            total += 1;
            print_finding(&mut out, &label, &f, a.json);
        };
        loop {
            let n = reader
                .read(&mut buf)
                .map_err(|e| format!("read error on '{label}': {e}"))?;
            if n == 0 {
                break;
            }
            scanner.push(&buf[..n], &mut emit);
        }
        scanner.finish(&mut emit);
    }

    eprintln!("\n{} finding(s)", total);
    Ok(())
}

/// Expand a path into a list of files to scan, walking directories.
fn collect_paths(
    path: &std::path::Path,
    recursive: bool,
    out: &mut Vec<Option<PathBuf>>,
) -> Result<(), String> {
    let meta = fs::metadata(path).map_err(|e| format!("cannot stat '{}': {e}", path.display()))?;
    if meta.is_dir() {
        let entries =
            fs::read_dir(path).map_err(|e| format!("cannot read dir '{}': {e}", path.display()))?;
        let mut children: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
        children.sort();
        for child in children {
            if child.is_dir() {
                if recursive {
                    collect_paths(&child, recursive, out)?;
                }
            } else {
                out.push(Some(child));
            }
        }
    } else {
        out.push(Some(path.to_path_buf()));
    }
    Ok(())
}

fn print_finding<W: Write>(w: &mut W, file: &str, f: &rxchef::scan::Finding, json: bool) {
    if json {
        let mut v = serde_json::to_value(f).unwrap_or(serde_json::json!({}));
        v["file"] = serde_json::json!(file);
        let _ = writeln!(w, "{}", serde_json::to_string(&v).unwrap());
    } else {
        let kinds = if f.kinds.is_empty() {
            "high-entropy".to_string()
        } else {
            f.kinds.join(",")
        };
        let _ = writeln!(
            w,
            "{}:{}  [{}]  entropy {:.2}  len {}",
            file, f.offset, kinds, f.entropy, f.len
        );
        let _ = writeln!(w, "    token:  {}", f.token);
        if let Some(dec) = &f.decoded {
            let recipe = f
                .recipe
                .as_ref()
                .map(|r| format_recipe(r))
                .unwrap_or_default();
            let _ = writeln!(w, "    decode: {}  [{}]", dec, recipe);
        }
    }
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
    let is_flow_step = |name: &str| {
        matches!(
            name.chars()
                .filter(|character| character.is_ascii_alphanumeric())
                .flat_map(char::to_lowercase)
                .collect::<String>()
                .as_str(),
            "fork" | "merge" | "subsection" | "register" | "label" | "jump" | "conditionaljump"
        )
    };
    if steps.iter().any(|step| is_flow_step(&step.op)) {
        let recipe = steps
            .iter()
            .map(|step| rxchef::integration::RecipeStep {
                op: step.op.clone(),
                args: step
                    .args
                    .iter()
                    .map(|argument| store::expand_vars(argument, var_overrides))
                    .collect(),
            })
            .collect::<Vec<_>>();
        let final_output = rxchef::integration::bake(input, &recipe)?.into_bytes()?;
        if trace {
            eprintln!("── flow-controlled recipe: {} steps ──", steps.len());
            let _ = write_output_raw(&final_output, hex, &mut io::stderr().lock());
            eprintln!();
        }
        let last = recipe.len().saturating_sub(1);
        let history_steps = recipe
            .into_iter()
            .enumerate()
            .map(|(index, step)| store::HistoryStep {
                op: step.op,
                args: step.args,
                output_preview: if index == last {
                    store::bytes_preview(&final_output, 300)
                } else {
                    String::new()
                },
                output_bytes: if index == last { final_output.len() } else { 0 },
                error: None,
            })
            .collect();
        return Ok(RunResult {
            final_output,
            steps: history_steps,
        });
    }

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

fn parse_step_str(s: &str) -> Result<Step, String> {
    let fields = split_step_fields(s)?;
    let op = fields.first().cloned().unwrap_or_default();
    if op.is_empty() {
        return Err(format!("invalid empty operation in step '{s}'"));
    }
    Ok(Step {
        op,
        args: fields.into_iter().skip(1).collect(),
    })
}

/// Split the compact CLI step format while allowing commas in arguments.
///
/// Both single and double quotes group fields. A backslash escapes comma,
/// quote, or backslash; before any other character it is kept literally so
/// regular expressions such as `\d+` survive parsing.
fn split_step_fields(s: &str) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quote: Option<char> = None;
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => match chars.peek().copied() {
                Some(next) if next == ',' || next == '\\' || Some(next) == quote => {
                    field.push(chars.next().expect("peeked character"));
                }
                _ => field.push('\\'),
            },
            '\'' | '"' => match quote {
                Some(active) if active == ch => quote = None,
                None => quote = Some(ch),
                Some(_) => field.push(ch),
            },
            ',' if quote.is_none() => {
                fields.push(field.trim().to_string());
                field.clear();
            }
            _ => field.push(ch),
        }
    }

    if let Some(unclosed) = quote {
        return Err(format!("invalid step '{s}': unclosed {unclosed} quote"));
    }
    fields.push(field.trim().to_string());
    Ok(fields)
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

// ─── Project ──────────────────────────────────────────────────────────────────

fn cmd_project(a: ProjectArgs) -> Result<(), String> {
    match a.action {
        ProjectAction::Run { file, trace } => {
            let project =
                store::load_project(&file).map_err(|e| format!("Failed to load project: {}", e))?;

            let input_bytes = match project.data {
                Some(store::ProjectData::Inline { inline }) => inline.into_bytes(),
                Some(store::ProjectData::File { file: path }) => {
                    let base_dir = file.parent().unwrap_or(std::path::Path::new(""));
                    std::fs::read(base_dir.join(path)).map_err(|e| e.to_string())?
                }
                None => Vec::new(),
            };

            let steps: Vec<_> = project
                .pipeline
                .iter()
                .map(|s| Step {
                    op: s.op.clone(),
                    args: s.args.clone(),
                })
                .collect();

            let mut overrides = std::collections::HashMap::new();
            for (k, v) in project.variables.iter() {
                overrides.insert(k.clone(), v.clone());
            }

            let result = run_steps(&steps, input_bytes.clone(), &overrides, trace, false)?;
            write_output(&result.final_output, false)?;

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_set_vars, parse_step_str, split_step_fields};

    #[test]
    fn parses_plain_pipeline_step() {
        let step = parse_step_str("to_hex, Space, num:0").unwrap();
        assert_eq!(step.op, "to_hex");
        assert_eq!(step.args, ["Space", "num:0"]);
    }

    #[test]
    fn parses_quoted_and_escaped_commas() {
        assert_eq!(
            split_step_fields(r#"find_replace,"a,b",x\,y"#).unwrap(),
            ["find_replace", "a,b", "x,y"]
        );
    }

    #[test]
    fn preserves_regular_expression_backslashes() {
        let step = parse_step_str(r"regular_expression,User,\\d+").unwrap();
        assert_eq!(step.args[1], r"\d+");
    }

    #[test]
    fn rejects_unclosed_quotes_and_empty_operations() {
        assert!(parse_step_str(r#"op,"unterminated"#).is_err());
        assert!(parse_step_str(" ,arg").is_err());
    }

    #[test]
    fn variable_values_may_contain_equals() {
        let vars = parse_set_vars(&["TOKEN=a=b=c".into()]).unwrap();
        assert_eq!(vars["TOKEN"], "a=b=c");
    }
}
