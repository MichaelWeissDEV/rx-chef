//! Conformance tests for `rxchef serve --stdio`, the persistent JSONL /
//! JSON-RPC server documented in `docs/cli/integration.md`.
//!
//! These tests drive the real `rxchef` binary as a subprocess, exactly the
//! way an editor plugin (Neovim etc.) would: a long-lived process, one JSON
//! request per line on stdin, one JSON response per line on stdout. Every
//! blocking read is bounded so a server that hangs fails the test instead of
//! wedging the suite.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde_json::{json, Value};

/// How long to wait for a response before declaring the server hung.
const RECV_TIMEOUT: Duration = Duration::from_secs(5);
/// How long to wait to *prove* no response arrives (notifications).
const SILENCE_TIMEOUT: Duration = Duration::from_millis(400);
/// How long to wait for the process to exit during teardown.
const WAIT_TIMEOUT: Duration = Duration::from_secs(5);

/// A persistent `rxchef serve --stdio` connection, mirroring the Neovim
/// process model described in the spec: one process, pipes held open,
/// requests written as they're issued, responses read by line.
struct ServeSession {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout_rx: Receiver<std::io::Result<String>>,
    stderr: Arc<Mutex<String>>,
}

impl ServeSession {
    fn start() -> Self {
        Self::start_with(&[])
    }

    fn start_with(extra_args: &[&str]) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_rxchef"))
            .args(["serve", "--stdio"])
            .args(extra_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn `rxchef serve --stdio`");

        let stdin = child.stdin.take().expect("piped stdin");
        let stdout = child.stdout.take().expect("piped stdout");
        let stderr_pipe = child.stderr.take().expect("piped stderr");

        // Drain stdout on a background thread so a large response (e.g. the
        // full `operations` list) can never fill the pipe buffer and block
        // the server, regardless of when the test gets around to reading it.
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if tx.send(line).is_err() {
                    break;
                }
            }
        });

        // Capture stderr so tests can assert nothing non-JSON leaks there
        // (mirrors the stdout-is-pipe-clean guarantee, checked on stderr
        // too since a plugin would treat any stray output as corruption).
        let stderr = Arc::new(Mutex::new(String::new()));
        let stderr_writer = Arc::clone(&stderr);
        thread::spawn(move || {
            let reader = BufReader::new(stderr_pipe);
            for line in reader.lines().map_while(Result::ok) {
                let mut guard = stderr_writer.lock().unwrap();
                guard.push_str(&line);
                guard.push('\n');
            }
        });

        Self {
            child,
            stdin: Some(stdin),
            stdout_rx: rx,
            stderr,
        }
    }

    /// Send one JSON-RPC/JSONL request.
    fn send(&mut self, value: Value) {
        self.send_raw_line(&value.to_string());
    }

    /// Send an arbitrary raw line (for malformed-JSON / framing tests).
    fn send_raw_line(&mut self, line: &str) {
        let stdin = self.stdin.as_mut().expect("stdin already closed");
        writeln!(stdin, "{line}").expect("write request line");
        stdin.flush().expect("flush request line");
    }

    /// Read the next response line, decoded as JSON. Panics rather than
    /// hanging if the server produces nothing within `RECV_TIMEOUT`.
    fn recv(&self) -> Value {
        match self.stdout_rx.recv_timeout(RECV_TIMEOUT) {
            Ok(Ok(line)) => serde_json::from_str(&line).unwrap_or_else(|error| {
                panic!("stdout line was not valid JSON: {error}\nline: {line:?}")
            }),
            Ok(Err(error)) => panic!("error reading server stdout: {error}"),
            Err(_) => panic!(
                "no response within {RECV_TIMEOUT:?} - server appears hung or silent \
                 (this would wedge a real editor session)"
            ),
        }
    }

    /// Assert no response arrives within a short window. Used to prove
    /// notifications (requests without `id`) produce no output.
    fn expect_silence(&self) {
        match self.stdout_rx.recv_timeout(SILENCE_TIMEOUT) {
            Ok(Ok(line)) => panic!("expected no response, but got: {line}"),
            Ok(Err(error)) => panic!("error reading server stdout: {error}"),
            Err(_) => {}
        }
    }

    /// Close stdin without waiting for exit (some tests want to keep reading
    /// after EOF-triggered shutdown).
    fn close_stdin(&mut self) {
        self.stdin.take();
    }

    /// Close stdin (if not already closed) and wait for the process to
    /// exit, bounded so a stuck server fails the test instead of hanging
    /// the suite. Returns the exit status and everything captured on
    /// stderr.
    fn finish(mut self) -> (ExitStatus, String) {
        self.close_stdin();
        let stderr = Arc::clone(&self.stderr);
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let status = self.child.wait();
            let _ = tx.send(status);
        });
        let status = match rx.recv_timeout(WAIT_TIMEOUT) {
            Ok(Ok(status)) => status,
            Ok(Err(error)) => panic!("error waiting for server process: {error}"),
            Err(_) => panic!("server did not exit within {WAIT_TIMEOUT:?} after stdin closed"),
        };
        // Give the stderr-draining thread a beat to flush its last line.
        thread::sleep(Duration::from_millis(50));
        let captured = stderr.lock().unwrap().clone();
        (status, captured)
    }
}

#[test]
fn request_size_limit_is_enforced_and_connection_recovers() {
    let mut session = ServeSession::start_with(&["--max-request-bytes", "128"]);
    session.send_raw_line(&"x".repeat(129));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32004);

    session.send(json!({"id": 1, "method": "ping"}));
    assert_eq!(session.recv()["result"]["protocol_version"], 1);
    let (status, stderr) = session.finish();
    assert!(status.success());
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
}

fn request(id: i64, method: &str, params: Value) -> Value {
    json!({"id": id, "method": method, "params": params})
}

// ─── ping ─────────────────────────────────────────────────────────────────

#[test]
fn ping_reports_server_name_version_and_protocol() {
    let mut session = ServeSession::start();
    session.send(json!({"id": 1, "method": "ping"}));
    let response = session.recv();
    assert_eq!(response["result"]["name"], "rxchef");
    // The server version is derived from Cargo metadata, not a hand-maintained
    // protocol constant.
    assert_eq!(response["result"]["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(response["result"]["protocol_version"], 1);

    let (status, stderr) = session.finish();
    assert!(status.success());
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
}

// ─── operations ─────────────────────────────────────────────────────────────

#[test]
fn operations_returns_complete_ordered_descriptors() {
    let mut session = ServeSession::start();
    session.send(json!({"id": 1, "method": "operations"}));
    let response = session.recv();
    let operations = response["result"].as_array().expect("array result");
    assert!(
        operations.len() > 400,
        "expected hundreds of operations, got {}",
        operations.len()
    );

    let aes = operations
        .iter()
        .find(|op| op["name"] == "AES Decrypt")
        .expect("AES Decrypt descriptor present");
    for field in [
        "name",
        "module",
        "description",
        "input_type",
        "output_type",
        "broken",
        "implementation_status",
        "availability",
        "args",
    ] {
        assert!(
            aes.get(field).is_some(),
            "descriptor missing documented field '{field}'"
        );
    }
    assert_eq!(aes["module"], "Ciphers");
    assert_eq!(aes["broken"], false);

    // args schema is ordered (not alphabetized) - Key comes before IV, which
    // comes before Mode, matching the operation's declared argument order.
    let arg_names: Vec<&str> = aes["args"]
        .as_array()
        .expect("args array")
        .iter()
        .map(|arg| arg["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        arg_names,
        vec![
            "Key",
            "IV",
            "Mode",
            "Input",
            "Output",
            "GCM Tag",
            "Additional Authenticated Data",
        ]
    );

    session.send(json!({"id": 2, "method": "shutdown"}));
    let shutdown = session.recv();
    assert_eq!(shutdown["result"]["shutdown"], true);
}

// ─── describe ─────────────────────────────────────────────────────────────

#[test]
fn describe_normalizes_operation_names_like_the_cli() {
    let mut session = ServeSession::start();
    for (id, spelling) in ["From Base64", "from_base64", "frombase64"]
        .into_iter()
        .enumerate()
    {
        session.send(request(
            id as i64,
            "describe",
            json!({"operation": spelling}),
        ));
        let response = session.recv();
        assert_eq!(
            response["result"]["name"], "From Base64",
            "spelling '{spelling}' should normalize to 'From Base64'"
        );
        assert_eq!(response["result"]["input_type"], "String");
        assert_eq!(response["result"]["output_type"], "Bytes");
    }
}

#[test]
fn describe_unknown_operation_is_error_code_32001() {
    let mut session = ServeSession::start();
    session.send(request(
        1,
        "describe",
        json!({"operation": "NOT_A_REAL_OPERATION"}),
    ));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32001);
    assert!(response["result"].is_null());
}

// ─── run ─────────────────────────────────────────────────────────────────

#[test]
fn run_with_input_decodes_utf8_text_through_the_operation() {
    let mut session = ServeSession::start();
    // Matches the exact example from docs/cli/integration.md: `input` here
    // is the raw text handed to "From Base64", which then decodes it.
    session.send(request(
        3,
        "run",
        json!({"operation": "From Base64", "input": "SGVsbG8=", "args": []}),
    ));
    let response = session.recv();
    assert_eq!(response["result"]["output"], "Hello");
}

#[test]
fn run_with_input_base64_round_trips_exact_non_utf8_bytes() {
    let mut session = ServeSession::start();
    // 0x00 0x01 0x02 0xFF 0xFE 0xFD is not valid UTF-8. "Reverse" (By: Byte)
    // reverses the raw byte sequence, so the exact bytes must survive the
    // round trip through output_base64 even though `output` is lossy.
    session.send(request(
        1,
        "run",
        json!({
            "operation": "reverse",
            "input_base64": "AAEC//79",
            "args": ["Byte"]
        }),
    ));
    let response = session.recv();
    let result = &response["result"];
    assert!(result.get("output").is_some());
    assert!(result.get("output_base64").is_some());
    assert!(result.get("output_len").is_some());
    assert_eq!(result["output_base64"], "/f7/AgEA");
    assert_eq!(result["output_len"], 6);

    use base64::{engine::general_purpose, Engine as _};
    let decoded = general_purpose::STANDARD
        .decode(result["output_base64"].as_str().unwrap())
        .expect("valid base64");
    assert_eq!(decoded, vec![0xfd, 0xfe, 0xff, 0x02, 0x01, 0x00]);
}

#[test]
fn run_input_and_input_base64_together_is_error_code_32602() {
    let mut session = ServeSession::start();
    session.send(request(
        1,
        "run",
        json!({
            "operation": "to_upper_case",
            "input": "a",
            "input_base64": "YQ=="
        }),
    ));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32602);
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("input_base64"));
}

#[test]
fn run_unknown_operation_is_error_code_32002() {
    let mut session = ServeSession::start();
    session.send(request(
        1,
        "run",
        json!({"operation": "NOT_A_REAL_OPERATION", "input": "a"}),
    ));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32002);
}

#[test]
fn run_genuine_execution_failure_is_error_code_32002() {
    // Distinct from an unknown-operation lookup failure: this is a real
    // operation (AES Decrypt) rejecting its arguments at execution time.
    let mut session = ServeSession::start();
    session.send(request(
        1,
        "run",
        json!({
            "operation": "aes_decrypt",
            "input": "deadbeef",
            "args": ["bad_key_not_hex", "also_bad"]
        }),
    ));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32002);
    assert!(response["error"]["message"].as_str().unwrap().len() > 0);
}

// ─── bake ─────────────────────────────────────────────────────────────────

#[test]
fn bake_chains_a_multi_step_recipe() {
    let mut session = ServeSession::start();
    session.send(request(
        4,
        "bake",
        json!({
            "input": "Hello",
            "recipe": [
                {"op": "to_upper_case", "args": []},
                {"op": "to_base64", "args": []}
            ]
        }),
    ));
    let response = session.recv();
    assert_eq!(response["result"]["output"], "SEVMTE8=");
}

#[test]
fn bake_accepts_operation_as_alias_for_op() {
    let mut session = ServeSession::start();
    session.send(request(
        1,
        "bake",
        json!({
            "input": "Hello",
            "recipe": [{"operation": "to_upper_case"}]
        }),
    ));
    let response = session.recv();
    assert_eq!(response["result"]["output"], "HELLO");
}

#[test]
fn bake_empty_recipe_passes_input_through_unchanged() {
    let mut session = ServeSession::start();
    session.send(request(1, "bake", json!({"input": "Hello", "recipe": []})));
    let response = session.recv();
    assert_eq!(response["result"]["output"], "Hello");
    assert_eq!(response["result"]["output_base64"], "SGVsbG8=");
    assert_eq!(response["result"]["output_len"], 5);
}

#[test]
fn bake_failing_step_reports_one_based_index_and_operation_name() {
    let mut session = ServeSession::start();
    session.send(request(
        1,
        "bake",
        json!({
            "input": "a",
            "recipe": [
                {"op": "to_upper_case"},
                {"op": "NOT_A_REAL_OPERATION"}
            ]
        }),
    ));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32002);
    let message = response["error"]["message"].as_str().unwrap();
    // Step is one-based: the failing step is the *second* recipe entry.
    assert!(
        message.contains("step 2") && message.contains("NOT_A_REAL_OPERATION"),
        "message should identify the one-based step and operation, got: {message}"
    );
}

// ─── framing: JSON-RPC 2.0 vs compact JSONL ────────────────────────────────

#[test]
fn accepts_both_jsonrpc_framed_and_compact_jsonl_requests() {
    let mut session = ServeSession::start();
    session.send(json!({"jsonrpc": "2.0", "id": 1, "method": "ping"}));
    let framed = session.recv();
    assert_eq!(framed["jsonrpc"], "2.0");
    assert_eq!(framed["result"]["name"], "rxchef");

    session.send(json!({"id": 2, "method": "ping"}));
    let compact = session.recv();
    // Response is always framed as JSON-RPC 2.0 even for a compact request.
    assert_eq!(compact["jsonrpc"], "2.0");
    assert_eq!(compact["result"]["name"], "rxchef");
}

#[test]
fn every_response_line_carries_jsonrpc_2_0_including_errors() {
    let mut session = ServeSession::start();
    session.send(json!({"id": 1, "method": "ping"}));
    assert_eq!(session.recv()["jsonrpc"], "2.0");

    session.send(json!({"id": 2, "method": "does_not_exist"}));
    assert_eq!(session.recv()["jsonrpc"], "2.0");

    session.send_raw_line("{not json");
    assert_eq!(session.recv()["jsonrpc"], "2.0");
}

// ─── notifications (no id) ──────────────────────────────────────────────────

#[test]
fn requests_without_id_are_notifications_and_produce_no_response() {
    let mut session = ServeSession::start();
    session.send(json!({"method": "ping"}));
    session.expect_silence();

    // A notification that would otherwise error also stays silent.
    session.send(json!({"method": "does_not_exist"}));
    session.expect_silence();

    // Prove the connection is still alive and processing in order.
    session.send(json!({"id": 1, "method": "ping"}));
    let response = session.recv();
    assert_eq!(response["result"]["name"], "rxchef");
}

// ─── shutdown / lifecycle ───────────────────────────────────────────────────

#[test]
fn shutdown_returns_result_flushes_and_exits_zero() {
    let mut session = ServeSession::start();
    session.send(json!({"id": 1, "method": "shutdown"}));
    let response = session.recv();
    assert_eq!(response["result"]["shutdown"], true);

    let (status, stderr) = session.finish();
    assert!(status.success(), "expected exit 0, got {status:?}");
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
}

#[test]
fn stdin_eof_without_shutdown_is_also_a_clean_exit() {
    let mut session = ServeSession::start();
    session.send(json!({"id": 1, "method": "ping"}));
    let response = session.recv();
    assert_eq!(response["result"]["name"], "rxchef");

    // Close stdin (EOF) instead of sending `shutdown`.
    let (status, stderr) = session.finish();
    assert!(status.success(), "expected exit 0 on EOF, got {status:?}");
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
}

// ─── error codes ───────────────────────────────────────────────────────────

#[test]
fn invalid_json_is_error_32700_and_does_not_kill_the_connection() {
    let mut session = ServeSession::start();
    session.send_raw_line("{this is not valid json");
    let error_response = session.recv();
    assert_eq!(error_response["error"]["code"], -32700);
    assert_eq!(error_response["jsonrpc"], "2.0");

    // The server must still be alive and answer subsequent requests on the
    // same connection - this is the whole point of a persistent server.
    session.send(json!({"id": 1, "method": "ping"}));
    let response = session.recv();
    assert_eq!(response["result"]["name"], "rxchef");

    session.send(json!({"id": 2, "method": "shutdown"}));
    let response = session.recv();
    assert_eq!(response["result"]["shutdown"], true);
}

#[test]
fn invalid_request_missing_method_is_error_32600() {
    let mut session = ServeSession::start();
    session.send(json!({"id": 1}));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32600);
}

#[test]
fn invalid_request_wrong_jsonrpc_version_is_error_32600() {
    let mut session = ServeSession::start();
    session.send(json!({"jsonrpc": "1.0", "id": 1, "method": "ping"}));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32600);
}

#[test]
fn unknown_method_is_error_32601() {
    let mut session = ServeSession::start();
    session.send(json!({"id": 1, "method": "frobnicate"}));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32601);
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("frobnicate"));
}

#[test]
fn invalid_params_missing_field_is_error_32602() {
    let mut session = ServeSession::start();
    session.send(json!({"id": 1, "method": "describe", "params": {}}));
    let response = session.recv();
    assert_eq!(response["error"]["code"], -32602);
}

// ─── ordering across a pipelined batch ─────────────────────────────────────

#[test]
fn responses_preserve_request_order_across_a_pipelined_batch() {
    let mut session = ServeSession::start();
    // Write several requests back-to-back before reading any response, and
    // give them IDs that run opposite to send order, so a response ordered
    // by ID instead of arrival would be caught.
    session.send(request(
        10,
        "run",
        json!({"operation": "to_upper_case", "input": "a"}),
    ));
    session.send(request(
        9,
        "run",
        json!({"operation": "to_upper_case", "input": "b"}),
    ));
    session.send(request(
        8,
        "run",
        json!({"operation": "to_upper_case", "input": "c"}),
    ));

    let first = session.recv();
    let second = session.recv();
    let third = session.recv();

    assert_eq!(first["id"], 10);
    assert_eq!(first["result"]["output"], "A");
    assert_eq!(second["id"], 9);
    assert_eq!(second["result"]["output"], "B");
    assert_eq!(third["id"], 8);
    assert_eq!(third["result"]["output"], "C");

    session.send(json!({"id": 1, "method": "shutdown"}));
    assert_eq!(session.recv()["result"]["shutdown"], true);
}

// ─── stdout hygiene ─────────────────────────────────────────────────────────

#[test]
fn stdout_never_contains_non_json_output_across_a_mixed_session() {
    let mut session = ServeSession::start();
    // A mix of success, every documented error path, and a large response,
    // all in one connection: every single stdout line must parse as JSON.
    session.send_raw_line("{not json at all");
    session.send(json!({"id": 1, "method": "ping"}));
    session.send(json!({"id": 2, "method": "operations"}));
    session.send(json!({"id": 3, "method": "does_not_exist"}));
    session.send(json!({
        "id": 4,
        "method": "describe",
        "params": {"operation": "NOT_A_REAL_OPERATION"}
    }));
    session.send(json!({"id": 5, "method": "shutdown"}));

    for _ in 0..6 {
        // recv() already asserts each line parses as JSON; failing to parse
        // panics with the offending line.
        let _ = session.recv();
    }

    let (status, stderr) = session.finish();
    assert!(status.success());
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
}
