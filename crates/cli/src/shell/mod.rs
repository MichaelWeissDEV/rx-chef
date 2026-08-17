//! Interactive shell.
//!
//! Deliberately thin: it parses a line into pipeline steps and hands them to
//! the same [`crate::steps::run_steps`] every other command uses. It carries
//! no argument semantics and no pipeline engine of its own.

mod completion;

use std::collections::HashMap;
use std::io::{self, BufRead, IsTerminal};

use rustyline::{error::ReadlineError, Editor};

use rxchef::runtime;

use crate::steps::{self, Step};
use completion::{repl_candidates, ReplHelper};

pub(crate) fn cmd_interactive() -> Result<(), String> {
    println!("rxchef interactive shell — type 'help' for commands, Tab completes names.");
    let mut data = Vec::new();

    if io::stdin().is_terminal() {
        let mut editor = Editor::<ReplHelper, rustyline::history::DefaultHistory>::new()
            .map_err(|error| error.to_string())?;
        editor.set_helper(Some(ReplHelper {
            candidates: repl_candidates(),
        }));
        loop {
            match editor.readline("rxchef> ") {
                Ok(line) => {
                    let _ = editor.add_history_entry(line.as_str());
                    match handle_repl_line(&line, &mut data) {
                        Ok(false) => break,
                        Ok(true) => {}
                        Err(error) => eprintln!("rxchef: {error}"),
                    }
                }
                Err(ReadlineError::Interrupted) => continue,
                Err(ReadlineError::Eof) => break,
                Err(error) => return Err(format!("interactive input failed: {error}")),
            }
        }
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match handle_repl_line(&line.map_err(|error| error.to_string())?, &mut data) {
                Ok(false) => break,
                Ok(true) => {}
                Err(error) => eprintln!("rxchef: {error}"),
            }
        }
    }
    Ok(())
}

pub(crate) fn handle_repl_line(line: &str, data: &mut Vec<u8>) -> Result<bool, String> {
    let line = line.trim();
    if line.is_empty() {
        return Ok(true);
    }
    match line.to_ascii_lowercase().as_str() {
        "exit" | "quit" => return Ok(false),
        "help" => {
            println!("Commands:\n  data TEXT       set the current input\n  show            show current data\n  clear           clear current data\n  list            list operation names\n  OP [ARGS]       execute an operation\n  OP [ARGS] | OP  execute a pipeline\n  exit            leave the shell");
            return Ok(true);
        }
        "show" => {
            print_repl_result(data);
            return Ok(true);
        }
        "clear" => {
            data.clear();
            println!("Current data cleared.");
            return Ok(true);
        }
        "list" => {
            for name in runtime::operation_names(None) {
                println!("{:<28} {}", runtime::canonical_identifier(&name), name);
            }
            return Ok(true);
        }
        _ => {}
    }
    if line
        .get(..4)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("data"))
        && line
            .get(4..)
            .is_some_and(|rest| rest.starts_with(char::is_whitespace))
    {
        *data = line[4..].trim_start().as_bytes().to_vec();
        println!("Current data set ({} byte(s)).", data.len());
        return Ok(true);
    }

    let steps = parse_repl_pipeline(line)?;
    let result = steps::run_steps(&steps, data.clone(), true, &HashMap::new(), false, false)?;
    *data = result.final_output;
    println!("Executed pipeline:");
    for step in &steps {
        let canonical =
            runtime::resolve_operation_name(&step.op).unwrap_or_else(|| step.op.clone());
        if step.args.is_empty() {
            println!("  {canonical}");
        } else {
            println!("  {canonical} ({})", step.args.join(", "));
        }
    }
    println!("Result:");
    print_repl_result(data);
    Ok(true)
}

pub(crate) fn parse_repl_pipeline(line: &str) -> Result<Vec<Step>, String> {
    line.split('|')
        .map(|part| {
            let words = shlex::split(part).ok_or_else(|| format!("invalid quoting in '{part}'"))?;
            if words.is_empty() {
                return Err("empty pipeline step".to_string());
            }
            let operation_word_count = (1..=words.len())
                .rev()
                .find(|count| runtime::resolve_operation_name(&words[..*count].join(" ")).is_some())
                .ok_or_else(|| format!("operation '{}' was not found", words[0]))?;
            Ok(Step {
                op: words[..operation_word_count].join(" "),
                args: words[operation_word_count..].to_vec(),
            })
        })
        .collect()
}

pub(crate) fn print_repl_result(data: &[u8]) {
    match std::str::from_utf8(data) {
        Ok(text) => println!("{text}"),
        Err(_) => println!("hex:{}", hex::encode(data)),
    }
}
