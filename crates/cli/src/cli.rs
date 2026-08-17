//! Command-line surface: the clap parser and every argument group.
//!
//! This module defines *what* the CLI accepts. Executing a command is the job
//! of the `commands` module, so argument parsing stays separate from the
//! behaviour it selects.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "rxchef",
    version,
    about = "CyberChef operations in the terminal",
    after_help = "Use 'rxchef <command> --help' for details on each command."
)]
pub(crate) struct Cli {
    /// Start the interactive rxchef shell.
    #[arg(short = 'i', long)]
    pub(crate) interactive: bool,
    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
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
    /// Generate a shell completion script on stdout.
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Generate the rxchef(1) manual page.
    Manpage {
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

// ─── Machine integration ─────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub(crate) struct OperationsArgs {
    /// Emit complete descriptors as JSON.
    #[arg(long)]
    pub(crate) json: bool,
    /// Search names, identifiers, and descriptions.
    #[arg(long)]
    pub(crate) search: Option<String>,
    /// Filter by module/category.
    #[arg(long)]
    pub(crate) module: Option<String>,
    /// Filter by status: complete, partial, unsupported, feature-gated, experimental.
    #[arg(long)]
    pub(crate) status: Option<String>,
    /// Include operations unavailable in this build.
    #[arg(long)]
    pub(crate) all: bool,
}

#[derive(Debug, Args)]
pub(crate) struct OperationArgs {
    #[command(subcommand)]
    pub(crate) action: OperationAction,
}

#[derive(Debug, Subcommand)]
pub(crate) enum OperationAction {
    /// Describe one operation and its complete argument schema.
    Describe {
        operation: String,
        /// Emit a machine-readable descriptor.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Args)]
pub(crate) struct BakeArgs {
    /// JSON/YAML recipe file (step array or object containing `steps`).
    #[arg(
        long,
        value_name = "PATH",
        conflicts_with = "recipe_json",
        required_unless_present = "recipe_json"
    )]
    pub(crate) recipe: Option<PathBuf>,
    /// Inline JSON recipe (step array or object containing `steps`).
    #[arg(
        long,
        value_name = "JSON",
        conflicts_with = "recipe",
        required_unless_present = "recipe"
    )]
    pub(crate) recipe_json: Option<String>,
    #[arg(short, long, conflicts_with = "input_file")]
    pub(crate) input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with = "input")]
    pub(crate) input_file: Option<PathBuf>,
    #[command(flatten)]
    pub(crate) output: OutputArgs,
}

#[derive(Debug, Args)]
pub(crate) struct ServeArgs {
    /// Read one JSON request per stdin line and write one response per stdout line.
    #[arg(long, required = true)]
    pub(crate) stdio: bool,
    /// Maximum bytes accepted in one JSON request line.
    #[arg(long, default_value_t = rxchef::integration::DEFAULT_MAX_REQUEST_BYTES)]
    pub(crate) max_request_bytes: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub(crate) enum OutputFormat {
    /// Exact bytes for pipes; safe text or a hex preview on a terminal.
    #[default]
    Auto,
    /// Always write exact bytes.
    Raw,
    /// Require valid UTF-8 text.
    Text,
    /// Render lowercase hexadecimal bytes.
    Hex,
    /// Render standard padded Base64.
    Base64,
    /// Emit a versioned binary-safe JSON envelope.
    Json,
}

#[derive(Debug, Args)]
pub(crate) struct OutputArgs {
    /// Select output representation.
    #[arg(long, value_enum, default_value = "auto")]
    pub(crate) format: OutputFormat,
    /// Write exact payload bytes to a file and leave stdout empty.
    #[arg(long, value_name = "PATH")]
    pub(crate) output_file: Option<PathBuf>,
    /// Compatibility alias for `--format hex`.
    #[arg(long)]
    pub(crate) hex: bool,
    /// Compatibility alias for `--format json`.
    #[arg(short, long)]
    pub(crate) json: bool,
}

impl OutputArgs {
    pub(crate) fn selected_format(&self) -> Result<OutputFormat, String> {
        if self.hex && self.json {
            return Err("--hex and --json cannot be used together".into());
        }
        if self.format != OutputFormat::Auto && (self.hex || self.json) {
            return Err("use either --format or the compatibility --hex/--json flag".into());
        }
        Ok(if self.hex {
            OutputFormat::Hex
        } else if self.json {
            OutputFormat::Json
        } else {
            self.format
        })
    }
}

// ─── List ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub(crate) struct ListArgs {
    /// Filter by name (case-insensitive).
    pub(crate) search: Option<String>,
    /// Show module/category.
    #[arg(short, long)]
    pub(crate) modules: bool,
    /// Output as JSON.
    #[arg(short, long)]
    pub(crate) json: bool,
}

// ─── Info ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub(crate) struct InfoArgs {
    /// Operation name (normalized: to_hex = ToHex = "To Hex").
    pub(crate) operation: String,
    /// Output as JSON.
    #[arg(short, long)]
    pub(crate) json: bool,
}

// ─── Run ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
#[command(
    after_long_help = "INPUT\n  Use --input for literal UTF-8 text, --input-file for exact file bytes, or omit\n  both to read stdin. Operation arguments support num:, bool:, hex:, and bytes:\n  type prefixes. Use --arg NAME=VALUE to address an argument by schema name.\n\nEXAMPLES\n  printf 'hello' | rxchef run to_base64\n  rxchef run sha2 --input hello --arg Size=256\n  rxchef run xor --input-file data.bin hex:deadbeef Standard false"
)]
pub(crate) struct RunArgs {
    /// Operation name.
    pub(crate) operation: String,
    /// Literal input text.
    #[arg(short, long, conflicts_with_all = ["input_file"])]
    pub(crate) input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with_all = ["input"])]
    pub(crate) input_file: Option<PathBuf>,
    #[command(flatten)]
    pub(crate) output: OutputArgs,
    /// Named argument: --arg Key=hex:00... (can repeat).
    #[arg(long = "arg", value_name = "NAME=VALUE")]
    pub(crate) named_args: Vec<String>,
    /// Override variables: KEY=value.
    #[arg(long = "set", value_name = "KEY=VALUE")]
    pub(crate) set_vars: Vec<String>,
    /// Positional operation arguments.
    #[arg(
        value_name = "ARG",
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    pub(crate) args: Vec<String>,
}

// ─── Pipe ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
#[command(
    after_long_help = "STEP SYNTAX\n  Each STEP is a comma-separated operation and its arguments. Quote or escape a\n  comma inside an argument: 'find_replace,\"a,b\",Simple string,x'.\n  Steps execute left-to-right; stdout contains only the final bytes and --trace\n  writes intermediate results to stderr. Input comes from --input, --input-file,\n  or stdin.\n\nEXAMPLES\n  printf 'hello' | rxchef pipe to_upper_case to_base64\n  rxchef pipe 'find_replace,\"a,b\",Simple string,x' to_base64 --input 'a,b'"
)]
pub(crate) struct PipeArgs {
    /// Steps: "OpName" or "OpName,arg1,arg2". Supports to_hex / ToHex / "To Hex".
    #[arg(value_name = "STEP")]
    pub(crate) steps: Vec<String>,
    #[arg(short, long, conflicts_with_all = ["input_file"])]
    pub(crate) input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with_all = ["input"])]
    pub(crate) input_file: Option<PathBuf>,
    /// Show output after each step.
    #[arg(short, long)]
    pub(crate) trace: bool,
    #[command(flatten)]
    pub(crate) output: OutputArgs,
    /// Save run to history.
    #[arg(long)]
    pub(crate) save: bool,
    /// Override variables: KEY=value.
    #[arg(long = "set", value_name = "KEY=VALUE")]
    pub(crate) set_vars: Vec<String>,
}

// ─── Recipe ───────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub(crate) struct RecipeArgs {
    /// JSON/YAML recipe file path OR inline JSON string.
    pub(crate) recipe: String,
    #[arg(short, long, conflicts_with_all = ["input_file"])]
    pub(crate) input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with_all = ["input"])]
    pub(crate) input_file: Option<PathBuf>,
    #[arg(short, long)]
    pub(crate) trace: bool,
    #[command(flatten)]
    pub(crate) output: OutputArgs,
    #[arg(long)]
    pub(crate) save: bool,
    /// Override variables: KEY=value.
    #[arg(long = "set", value_name = "KEY=VALUE")]
    pub(crate) set_vars: Vec<String>,
}

// ─── Pipeline ─────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub(crate) struct PipelineArgs {
    #[command(subcommand)]
    pub(crate) action: PipelineAction,
}

#[derive(Debug, Subcommand)]
pub(crate) enum PipelineAction {
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
        #[arg(long, conflicts_with = "global")]
        project: bool,
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
        #[arg(long, conflicts_with = "global")]
        project: bool,
    },
    /// Remove a step from a pipeline by 1-based index.
    Remove {
        pipeline: String,
        /// Step index (1-based).
        index: usize,
        #[arg(short, long)]
        global: bool,
        #[arg(long, conflicts_with = "global")]
        project: bool,
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
        #[arg(long, conflicts_with = "global")]
        project: bool,
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
        #[arg(long, conflicts_with = "global")]
        project: bool,
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
        #[arg(long, conflicts_with = "global")]
        project: bool,
    },
    /// Rename a pipeline.
    Rename {
        old_name: String,
        new_name: String,
        #[arg(short, long)]
        global: bool,
        #[arg(long, conflicts_with = "global")]
        project: bool,
    },
}

// ─── Var ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub(crate) struct VarArgs {
    #[command(subcommand)]
    pub(crate) action: VarAction,
}

#[derive(Debug, Subcommand)]
pub(crate) enum VarAction {
    /// Set a variable (project inside a project, otherwise global).
    Set {
        name: String,
        #[arg(required_unless_present = "stdin", conflicts_with = "stdin")]
        value: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        global: bool,
        #[arg(long, conflicts_with = "global")]
        project: bool,
        #[arg(long)]
        secret: bool,
        #[arg(long)]
        stdin: bool,
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
        /// Include non-secret values in output.
        #[arg(long)]
        show_values: bool,
        /// Include secret values in output; implies --show-values.
        #[arg(long)]
        show_secrets: bool,
    },
    /// Remove a variable.
    Unset {
        name: String,
        #[arg(short, long)]
        global: bool,
        #[arg(long, conflicts_with = "global")]
        project: bool,
    },
}

// ─── History ──────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub(crate) struct HistoryArgs {
    #[command(subcommand)]
    pub(crate) action: HistoryAction,
}

#[derive(Debug, Subcommand)]
pub(crate) enum HistoryAction {
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
        #[arg(short, long, conflicts_with = "input_file")]
        input: Option<String>,
        #[arg(short = 'f', long, conflicts_with = "input")]
        input_file: Option<PathBuf>,
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
pub(crate) struct MagicArgs {
    #[arg(short, long, conflicts_with = "input_file")]
    pub(crate) input: Option<String>,
    #[arg(short = 'f', long, value_name = "PATH", conflicts_with = "input")]
    pub(crate) input_file: Option<PathBuf>,
    /// Maximum recursion depth (chained decodes).
    #[arg(short, long, default_value = "3")]
    pub(crate) depth: usize,
    /// Known-plaintext filter (substring/regex); matching candidates rank first.
    #[arg(long, value_name = "REGEX")]
    pub(crate) crib: Option<String>,
    /// Try aggressive decoders too (ROT13, Base58/85).
    #[arg(long)]
    pub(crate) intensive: bool,
    /// Maximum decoder attempts across the search tree.
    #[arg(long, default_value = "256")]
    pub(crate) max_candidates: usize,
    /// Maximum bytes accepted for one candidate input or output.
    #[arg(long, default_value = "8388608")]
    pub(crate) max_candidate_bytes: usize,
    /// Maximum cumulative decoded bytes accepted during the search.
    #[arg(long, default_value = "33554432")]
    pub(crate) max_total_decoded_bytes: usize,
    /// Print only the best decoded output (raw, pipe-friendly).
    #[arg(long)]
    pub(crate) decode: bool,
    /// Output as hex when using --decode.
    #[arg(long)]
    pub(crate) hex: bool,
    /// Output as JSON.
    #[arg(short, long)]
    pub(crate) json: bool,
}

// ─── Scan ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub(crate) struct ScanArgs {
    /// Files or directories to scan. Omit to read from stdin.
    #[arg(value_name = "PATH")]
    pub(crate) paths: Vec<PathBuf>,
    /// Recurse into directories.
    #[arg(short, long)]
    pub(crate) recursive: bool,
    /// Minimum token length to consider.
    #[arg(long, default_value = "16")]
    pub(crate) min_len: usize,
    /// Maximum bytes retained for a single token.
    #[arg(long, default_value = "1048576")]
    pub(crate) max_token_size: usize,
    /// Stop after this many findings across all inputs.
    #[arg(long, default_value = "10000")]
    pub(crate) max_findings: usize,
    /// Attempt to decode each finding via the magic engine.
    #[arg(short, long)]
    pub(crate) decode: bool,
    /// Recursion depth when decoding.
    #[arg(long, default_value = "3")]
    pub(crate) depth: usize,
    /// Only report findings whose decode matches this crib (regex; implies --decode).
    #[arg(long, value_name = "REGEX")]
    pub(crate) crib: Option<String>,
    /// Also report tokens with entropy >= this (bits/byte), even if no decoder fired.
    #[arg(long, value_name = "BITS")]
    pub(crate) entropy: Option<f64>,
    /// Restrict to these encodings, comma-separated (e.g. base64,hex).
    #[arg(long, value_delimiter = ',', value_name = "KIND")]
    pub(crate) kind: Vec<String>,
    /// Emit newline-delimited JSON (one finding per line), ideal for jq/grep.
    #[arg(short, long)]
    pub(crate) json: bool,
}

// ─── Project ──────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub(crate) struct ProjectArgs {
    #[command(subcommand)]
    pub(crate) action: ProjectAction,
}

#[derive(Debug, Subcommand)]
pub(crate) enum ProjectAction {
    /// Create an explicit `.rxchef` store in the current directory.
    Init,
    /// Run a CTF project file (YAML/JSON).
    Run {
        /// Project file path
        file: PathBuf,
        #[arg(short, long)]
        trace: bool,
    },
}

// ─── Entry point ─────────────────────────────────────────────────────────────
