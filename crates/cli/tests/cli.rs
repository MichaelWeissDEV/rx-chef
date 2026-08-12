use std::io::Write;
use std::process::{Command, Output, Stdio};

fn rxchef(args: &[&str], stdin: Option<&[u8]>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rxchef"));
    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn rxchef");
    if let Some(input) = stdin {
        child
            .stdin
            .take()
            .expect("piped stdin")
            .write_all(input)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for rxchef")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "status: {}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn top_level_and_subcommand_help_are_useful() {
    let top = rxchef(&["--help"], None);
    assert_success(&top);
    let help = String::from_utf8_lossy(&top.stdout);
    for command in [
        "list", "info", "run", "pipe", "recipe", "pipeline", "var", "history", "magic", "scan",
        "project",
    ] {
        assert!(
            help.contains(command),
            "missing {command} in top-level help"
        );
    }

    let pipe = rxchef(&["pipe", "--help"], None);
    assert_success(&pipe);
    let help = String::from_utf8_lossy(&pipe.stdout);
    assert!(help.contains("STEP SYNTAX"));
    assert!(help.contains("--input-file"));
    assert!(help.contains("--trace"));
}

#[test]
fn stdin_is_raw_and_stdout_is_pipe_clean() {
    let output = rxchef(&["run", "to_base64"], Some(b"hello\n"));
    assert_success(&output);
    assert_eq!(output.stdout, b"aGVsbG8K");
    assert!(output.stderr.is_empty());
}

#[test]
fn arbitrary_operations_chain_left_to_right() {
    let output = rxchef(
        &["pipe", "to_upper_case", "to_base64", "from_base64"],
        Some(b"Hello, pipeline!"),
    );
    assert_success(&output);
    assert_eq!(output.stdout, b"HELLO, PIPELINE!");
}

#[test]
fn compact_step_parser_accepts_commas_inside_arguments() {
    let output = rxchef(
        &[
            "pipe",
            r#"find_replace,"a,b",Simple string,x"#,
            "to_upper_case",
        ],
        Some(b"before a,b after"),
    );
    assert_success(&output);
    assert_eq!(output.stdout, b"BEFORE X AFTER");
}

#[test]
fn json_modes_are_machine_parseable() {
    let output = rxchef(&["info", "to_base64", "--json"], None);
    assert_success(&output);
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["name"], "To Base64");
    assert!(value["args"].is_array());

    let output = rxchef(&["run", "to_base64", "--input", "hi", "--json"], None);
    assert_success(&output);
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["output"], "aGk=");
    assert_eq!(value["output_len"], 4);
}

#[test]
fn invalid_pipeline_syntax_fails_with_context() {
    let output = rxchef(&["pipe", r#"to_hex,"broken"#], Some(b"hello"));
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unclosed"));
}
