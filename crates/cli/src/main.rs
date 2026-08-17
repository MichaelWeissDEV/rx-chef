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

use std::io;

use clap::{CommandFactory, Parser};

mod cli;
mod commands;
mod error;
mod input;
mod output;
mod shell;
mod steps;

use cli::{Cli, Command};
use commands::{
    bake, history, magic, operations, pipe, pipeline, project, recipe, run as run_cmd, scan, serve,
    var,
};
use error::CliError;

fn main() {
    if let Err(e) = run() {
        eprintln!("rxchef: {e}");
        std::process::exit(e.exit_code());
    }
}

fn run() -> Result<(), CliError> {
    let cli = Cli::parse();
    if cli.interactive && cli.command.is_some() {
        return Err(CliError::InvalidInput(
            "--interactive cannot be combined with a subcommand".into(),
        ));
    }
    if cli.interactive {
        return shell::cmd_interactive().map_err(CliError::Execution);
    }
    let Some(command) = cli.command else {
        Cli::command()
            .print_help()
            .map_err(|e| CliError::StoreIo(e.to_string()))?;
        println!();
        return Ok(());
    };
    match command {
        Command::Operations(a) => operations::cmd_operations(a).map_err(CliError::Execution),
        Command::Operation(a) => operations::cmd_operation(a).map_err(CliError::InvalidInput),
        Command::List(a) => operations::cmd_list(a).map_err(CliError::Execution),
        Command::Info(a) => operations::cmd_info(a).map_err(CliError::InvalidInput),
        Command::Run(a) => run_cmd::cmd_run(a),
        Command::Pipe(a) => pipe::cmd_pipe(a).map_err(CliError::Execution),
        Command::Recipe(a) => recipe::cmd_recipe(a).map_err(CliError::Execution),
        Command::Bake(a) => bake::cmd_bake(a).map_err(CliError::Execution),
        Command::Pipeline(a) => pipeline::cmd_pipeline(a).map_err(CliError::StoreIo),
        Command::Var(a) => var::cmd_var(a).map_err(CliError::StoreIo),
        Command::History(a) => history::cmd_history(a).map_err(CliError::StoreIo),
        Command::Magic(a) => magic::cmd_magic(a).map_err(CliError::Execution),
        Command::Scan(a) => scan::cmd_scan(a).map_err(CliError::StoreIo),
        Command::Project(a) => project::cmd_project(a).map_err(CliError::StoreIo),
        Command::Serve(a) => serve::cmd_serve(a).map_err(CliError::Execution),
        Command::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "rxchef", &mut io::stdout());
            Ok(())
        }
        Command::Manpage { output } => {
            let mut page = Vec::new();
            clap_mangen::Man::new(Cli::command())
                .render(&mut page)
                .map_err(|error| CliError::StoreIo(error.to_string()))?;
            if let Some(path) = output {
                output::atomic_write(&path, &page).map_err(CliError::StoreIo)
            } else {
                output::write_bytes(&mut io::stdout().lock(), &page).map_err(CliError::StoreIo)
            }
        }
    }
}

// ─── Interactive shell ──────────────────────────────────────────────────────
