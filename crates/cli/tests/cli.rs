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
        "operations",
        "operation",
        "list",
        "info",
        "run",
        "pipe",
        "recipe",
        "bake",
        "pipeline",
        "var",
        "history",
        "magic",
        "scan",
        "project",
        "serve",
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

#[test]
fn integration_commands_expose_complete_descriptors() {
    let output = rxchef(&["operations", "--json"], None);
    assert_success(&output);
    let operations: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let operations = operations.as_array().unwrap();
    assert!(operations.len() > 400);
    let base64 = operations
        .iter()
        .find(|operation| operation["name"] == "From Base64")
        .expect("From Base64 descriptor");
    assert!(base64["args"].is_array());
    assert!(base64["description"].as_str().unwrap().len() > 10);

    let output = rxchef(&["operation", "describe", "from_base64", "--json"], None);
    assert_success(&output);
    let descriptor: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(descriptor["name"], "From Base64");
    assert_eq!(descriptor["input_type"], "String");
}

#[test]
fn bake_accepts_inline_recipe_and_stdin() {
    let recipe = r#"[{"op":"to_upper_case"},{"op":"to_base64"},{"op":"from_base64"}]"#;
    let output = rxchef(&["bake", "--recipe-json", recipe], Some(b"Hello"));
    assert_success(&output);
    assert_eq!(output.stdout, b"HELLO");
}

#[test]
fn pipe_and_bake_share_flow_control_semantics() {
    let output = rxchef(
        &["pipe", r#"Fork,\n,|,false"#, "To Upper case", "Merge"],
        Some(b"one\ntwo"),
    );
    assert_success(&output);
    assert_eq!(output.stdout, b"ONE|TWO");

    let recipe = r#"[{"op":"Subsection","args":["[0-9]+","true","true"]},{"op":"To Base64"},{"op":"Merge"}]"#;
    let output = rxchef(&["bake", "--recipe-json", recipe], Some(b"a12b345c"));
    assert_success(&output);
    assert_eq!(output.stdout, b"aMTI=bMzQ1c");
}

#[test]
fn stdio_server_handles_multiple_requests_in_one_process() {
    let requests = concat!(
        "{\"id\":1,\"method\":\"operations\"}\n",
        "{\"id\":2,\"method\":\"describe\",\"params\":{\"operation\":\"XOR\"}}\n",
        "{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"bake\",\"params\":{\"input\":\"Hello\",\"recipe\":[{\"op\":\"to_upper_case\"},{\"op\":\"to_base64\"}]}}\n",
        "{\"id\":4,\"method\":\"shutdown\"}\n"
    );
    let output = rxchef(&["serve", "--stdio"], Some(requests.as_bytes()));
    assert_success(&output);
    assert!(output.stderr.is_empty());
    let responses: Vec<serde_json::Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(responses.len(), 4);
    assert!(responses[0]["result"].as_array().unwrap().len() > 400);
    assert_eq!(responses[1]["result"]["name"], "XOR");
    assert_eq!(responses[2]["result"]["output"], "SEVMTE8=");
    assert_eq!(responses[3]["result"]["shutdown"], true);
}
