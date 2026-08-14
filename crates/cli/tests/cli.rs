use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Output, Stdio};

use base64::{engine::general_purpose, Engine as _};

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

fn rxchef_in(
    args: &[&str],
    stdin: Option<&[u8]>,
    cwd: &std::path::Path,
    home: &std::path::Path,
) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rxchef"));
    command
        .args(args)
        .current_dir(cwd)
        .env("RXCHEF_HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn isolated rxchef");
    if let Some(input) = stdin {
        child.stdin.take().unwrap().write_all(input).unwrap();
    }
    child.wait_with_output().expect("wait for isolated rxchef")
}

fn isolated_store() -> (std::path::PathBuf, std::path::PathBuf) {
    let root = std::env::temp_dir().join(format!(
        "rxchef-cli-store-{}-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("test"),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();
    (root, home)
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
fn stable_exit_codes_distinguish_usage_input_execution_and_io() {
    assert_eq!(rxchef(&["run"], None).status.code(), Some(2));
    assert_eq!(
        rxchef(&["info", "definitely-not-an-operation"], None)
            .status
            .code(),
        Some(3)
    );
    assert_eq!(
        rxchef(&["run", "definitely-not-an-operation"], Some(b"input"))
            .status
            .code(),
        Some(4)
    );
    assert_eq!(
        rxchef(&["scan", "/definitely/not/a/real/rxchef-file"], None)
            .status
            .code(),
        Some(5)
    );
    let disassembly = rxchef(&["run", "Disassemble x86"], Some(b"90"));
    if rxchef::integration::describe("Disassemble x86")
        .unwrap()
        .availability
        == rxchef::operation::Availability::FeatureDisabled
    {
        assert_eq!(disassembly.status.code(), Some(6));
    } else {
        assert_success(&disassembly);
    }
}

#[test]
fn cli_resource_limits_bound_magic_and_scan() {
    let magic = rxchef(
        &["magic", "--decode", "--max-candidates", "0"],
        Some(b"SGVsbG8="),
    );
    assert_eq!(magic.status.code(), Some(4));

    let scan = rxchef(
        &["scan", "--json", "--min-len", "8", "--max-findings", "1"],
        Some(b"SGVsbG8gV29ybGQ= U0dWc2JHOD0="),
    );
    assert_success(&scan);
    assert_eq!(scan.stdout.split(|byte| *byte == b'\n').count(), 2);
}

#[test]
fn top_level_and_subcommand_help_are_useful() {
    let top = rxchef(&["--help"], None);
    assert_success(&top);
    let help = String::from_utf8_lossy(&top.stdout);
    let commands = [
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
        "completions",
        "manpage",
    ];
    for command in commands {
        assert!(
            help.contains(command),
            "missing {command} in top-level help"
        );

        // `Command` is dispatched by an exhaustive match in the binary. Driving
        // every Clap variant through the real executable complements that
        // compile-time guarantee by detecting command-tree/dispatch drift.
        let subcommand = rxchef(&[command, "--help"], None);
        assert_success(&subcommand);
        assert!(
            !subcommand.stdout.is_empty(),
            "{command} --help produced no command-specific help"
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
fn completions_and_manpage_are_nonempty() {
    for shell in ["bash", "zsh", "fish", "powershell"] {
        let output = rxchef(&["completions", shell], None);
        assert_success(&output);
        assert!(output.stdout.len() > 100, "empty {shell} completion");
    }
    let manpage = rxchef(&["manpage"], None);
    assert_success(&manpage);
    assert!(String::from_utf8_lossy(&manpage.stdout).contains("rxchef"));
}

#[test]
fn project_discovery_scope_and_secrets_are_safe() {
    let (root, home) = isolated_store();
    let project = root.join("workspace");
    let nested = project.join("src/deep");
    let outside = root.join("outside");
    std::fs::create_dir_all(&nested).unwrap();
    std::fs::create_dir_all(&outside).unwrap();

    let init = rxchef_in(&["project", "init"], None, &project, &home);
    assert_success(&init);
    assert!(project.join(".rxchef").is_dir());

    let secret = rxchef_in(
        &["var", "set", "TOKEN", "--stdin", "--secret"],
        Some(b"never-print-this"),
        &nested,
        &home,
    );
    assert_success(&secret);
    assert!(!String::from_utf8_lossy(&secret.stdout).contains("never-print-this"));

    let listed = rxchef_in(&["var", "list", "--json"], None, &nested, &home);
    assert_success(&listed);
    assert!(!String::from_utf8_lossy(&listed.stdout).contains("never-print-this"));
    let listed: serde_json::Value = serde_json::from_slice(&listed.stdout).unwrap();
    assert_eq!(listed[0]["scope"], "project");
    assert_eq!(listed[0]["secret"], true);
    assert!(listed[0]["value"].is_null());

    let global = rxchef_in(
        &["var", "set", "OUTSIDE", "global-value"],
        None,
        &outside,
        &home,
    );
    assert_success(&global);
    assert!(home.join("vars.json").is_file());
    assert!(!outside.join(".rxchef").exists());

    let rejected = rxchef_in(
        &["var", "set", "NOPE", "value", "--project"],
        None,
        &outside,
        &home,
    );
    assert!(!rejected.status.success());
    assert!(!outside.join(".rxchef").exists());

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn pipeline_mutations_do_not_cross_scopes_and_recipe_load_is_read_only() {
    let (root, home) = isolated_store();
    let project = root.join("workspace");
    std::fs::create_dir_all(&project).unwrap();
    assert_success(&rxchef_in(&["project", "init"], None, &project, &home));
    assert_success(&rxchef_in(
        &["pipeline", "new", "same", "--global"],
        None,
        &project,
        &home,
    ));
    assert_success(&rxchef_in(
        &["pipeline", "new", "same", "--project"],
        None,
        &project,
        &home,
    ));
    assert_success(&rxchef_in(
        &["pipeline", "add", "same", "To Base64", "--global"],
        None,
        &project,
        &home,
    ));

    let global: serde_json::Value =
        serde_json::from_slice(&std::fs::read(home.join("recipes/same.json")).unwrap()).unwrap();
    let local: serde_json::Value =
        serde_json::from_slice(&std::fs::read(project.join(".rxchef/recipes/same.json")).unwrap())
            .unwrap();
    assert_eq!(global["steps"].as_array().unwrap().len(), 1);
    assert_eq!(local["steps"].as_array().unwrap().len(), 0);

    let recipe_path = project.join("one-shot.json");
    std::fs::write(&recipe_path, r#"[{"op":"To Base64","args":[]}]"#).unwrap();
    let run = rxchef_in(
        &["recipe", recipe_path.to_str().unwrap(), "--input", "hi"],
        None,
        &project,
        &home,
    );
    assert_success(&run);
    assert_eq!(run.stdout, b"aGk=");
    assert!(!project.join(".rxchef/recipes/one-shot.json").exists());

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn stdin_is_raw_and_stdout_is_pipe_clean() {
    let output = rxchef(&["run", "to_base64"], Some(b"hello\n"));
    assert_success(&output);
    assert_eq!(output.stdout, b"aGVsbG8K");
    assert!(output.stderr.is_empty());
}

#[test]
fn redirected_binary_output_is_exact_raw_bytes() {
    let encoded = b"AAEC//79gAA=";
    let output = rxchef(&["run", "From Base64"], Some(encoded));
    assert_success(&output);
    assert_eq!(
        output.stdout,
        [0x00, 0x01, 0x02, 0xff, 0xfe, 0xfd, 0x80, 0x00]
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn output_formats_and_output_file_obey_binary_contract() {
    let encoded = b"AAEC//79gAA=";

    let hex = rxchef(&["run", "From Base64", "--format", "hex"], Some(encoded));
    assert_success(&hex);
    assert_eq!(hex.stdout, b"00 01 02 ff fe fd 80 00\n");

    let base64 = rxchef(&["run", "From Base64", "--format", "base64"], Some(encoded));
    assert_success(&base64);
    assert_eq!(base64.stdout, encoded);

    let text = rxchef(&["run", "From Base64", "--format", "text"], Some(encoded));
    assert!(!text.status.success());
    assert!(String::from_utf8_lossy(&text.stderr).contains("not valid UTF-8"));

    let path = std::env::temp_dir().join(format!(
        "rxchef-cli-output-{}-{}.bin",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    std::fs::write(&path, b"old content").unwrap();
    let output = rxchef(
        &[
            "run",
            "From Base64",
            "--format",
            "hex",
            "--output-file",
            path.to_str().unwrap(),
        ],
        Some(encoded),
    );
    assert_success(&output);
    assert!(output.stdout.is_empty());
    assert_eq!(
        std::fs::read(&path).unwrap(),
        [0x00, 0x01, 0x02, 0xff, 0xfe, 0xfd, 0x80, 0x00]
    );
    std::fs::remove_file(path).unwrap();

    let empty = rxchef(&["run", "To Base64", "--input", ""], None);
    assert_success(&empty);
    assert!(empty.stdout.is_empty());

    let large_input = vec![0x5a; 1024 * 1024];
    let large = rxchef(&["run", "To Base64", "--format", "raw"], Some(&large_input));
    assert_success(&large);
    assert_eq!(large.stdout.len(), 1_398_104);

    let missing_parent = std::env::temp_dir()
        .join(format!("rxchef-missing-parent-{}", std::process::id()))
        .join("output.bin");
    let failed = rxchef(
        &[
            "run",
            "From Base64",
            "--output-file",
            missing_parent.to_str().unwrap(),
        ],
        Some(encoded),
    );
    assert_eq!(failed.status.code(), Some(5));
    assert!(failed.stdout.is_empty());
    assert!(!missing_parent.exists());
}

#[test]
fn operations_exits_cleanly_when_downstream_closes_the_pipe() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rxchef"))
        .arg("operations")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn rxchef operations");
    let mut first_line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut first_line)
        .unwrap();
    assert!(!first_line.is_empty());
    let output = child.wait_with_output().unwrap();
    assert_success(&output);
    assert!(!String::from_utf8_lossy(&output.stderr).contains("Broken pipe"));
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
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["success"], true);
    assert_eq!(value["output"], "aGk=");
    assert_eq!(value["output_len"], 4);
    assert_eq!(value["output_is_utf8"], true);
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
    assert_eq!(base64["id"], "from_base64");
    assert_eq!(base64["implementation_status"], "partial");
    assert_eq!(base64["availability"], "available");
    assert_eq!(base64["input_requirement"], "required");
    assert!(base64["deterministic"].as_bool().unwrap());
    assert_eq!(base64["args"][1]["kind"], "boolean");

    let filtered = rxchef(
        &[
            "operations",
            "--json",
            "--search",
            "base64",
            "--module",
            "Default",
            "--status",
            "partial",
        ],
        None,
    );
    assert_success(&filtered);
    let filtered: serde_json::Value = serde_json::from_slice(&filtered.stdout).unwrap();
    assert!(!filtered.as_array().unwrap().is_empty());
    assert!(filtered.as_array().unwrap().iter().all(|operation| {
        operation["module"] == "Default" && operation["implementation_status"] == "partial"
    }));

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

#[test]
fn library_cli_bake_pipe_and_server_produce_identical_flow_bytes() {
    let input = b"one\ntwo";
    let steps = vec![
        rxchef::execution::RecipeStep {
            op: "Fork".into(),
            args: vec!["\\n".into(), "|".into(), "false".into()],
        },
        rxchef::execution::RecipeStep {
            op: "To Upper case".into(),
            args: vec![],
        },
        rxchef::execution::RecipeStep {
            op: "Merge".into(),
            args: vec![],
        },
    ];
    let library = rxchef::execution::execute(rxchef::execution::ExecutionRequest {
        input: input.to_vec(),
        input_supplied: true,
        recipe: steps.clone().into(),
        variables: rxchef::execution::VariableContext::default(),
        options: rxchef::execution::ExecutionOptions::default(),
    })
    .unwrap()
    .output;

    let pipe = rxchef(
        &["pipe", r#"Fork,\n,|,false"#, "To Upper case", "Merge"],
        Some(input),
    );
    assert_success(&pipe);

    let recipe_json = serde_json::to_string(&steps).unwrap();
    let bake = rxchef(&["bake", "--recipe-json", &recipe_json], Some(input));
    assert_success(&bake);

    let request = serde_json::json!({
        "id": 1,
        "method": "bake",
        "params": {"input_base64": general_purpose::STANDARD.encode(input), "recipe": steps}
    });
    let shutdown = serde_json::json!({"id": 2, "method": "shutdown"});
    let server_input = format!("{request}\n{shutdown}\n");
    let server = rxchef(&["serve", "--stdio"], Some(server_input.as_bytes()));
    assert_success(&server);
    let response: serde_json::Value =
        serde_json::from_slice(server.stdout.split(|byte| *byte == b'\n').next().unwrap()).unwrap();
    let server_bytes = general_purpose::STANDARD
        .decode(response["result"]["output_base64"].as_str().unwrap())
        .unwrap();

    assert_eq!(library, b"ONE|TWO");
    assert_eq!(pipe.stdout, library);
    assert_eq!(bake.stdout, library);
    assert_eq!(server_bytes, library);
}
