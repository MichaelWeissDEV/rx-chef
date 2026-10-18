//! End-to-end tests against a real listening server and real worker processes.
use std::{
    net::SocketAddr,
    thread,
    time::{Duration, Instant},
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use rxchef_client::{
    export_bytes, EngineClient, EngineTask, ErrorKind, ExecuteRequest, JobStatus,
    LocalEngineClient, MagicRequest, RemoteEngineClient, ScanRequest,
};
use rxchef_server::{Cli, ServerConfig};
use serde_json::{json, Value};

const TOKEN: &str = "test-token-0123456789abcdef";

#[test]
fn projects_validate_through_real_workers_with_auth_and_no_file_access() {
    let server = TestServer::with(|config| {
        config.read_token = Some("read-only-project-token-0123456789".into())
    });
    let mut project = rxchef_client::WorkbenchProject::new("Session");
    project.graphs.push(rxchef::graph::Graph::from_recipe(
        "graph".into(),
        Vec::new(),
    ));
    let mut value = serde_json::to_value(&project).unwrap();
    value["input_files"] = json!([{"graph_id":"graph","input_name":"input","path":"/this-file-does-not-exist","external":true}]);
    let endpoint = server.url("/api/v1/projects/validate");
    assert_eq!(
        server
            .http()
            .post(&endpoint)
            .json(&value)
            .send()
            .unwrap()
            .status(),
        401
    );
    assert_eq!(
        server
            .http()
            .post(&endpoint)
            .bearer_auth("read-only-project-token-0123456789")
            .json(&value)
            .send()
            .unwrap()
            .status(),
        403
    );
    let response = server
        .http()
        .post(&endpoint)
        .bearer_auth(TOKEN)
        .json(&value)
        .send()
        .unwrap();
    assert!(response.status().is_success());
    let normalized: Value = response.json().unwrap();
    assert_eq!(normalized["input_files"][0]["external"], true);
    assert!(!normalized.to_string().contains("PRIVATE-DATA"));
    let mut extra = value.clone();
    extra["graphs"][0]["nodes"][0]["unknown_payload"] = json!("PRIVATE-DATA");
    let response = server
        .http()
        .post(&endpoint)
        .bearer_auth(TOKEN)
        .json(&extra)
        .send()
        .unwrap();
    assert_eq!(response.status(), 400);
    assert!(!response.text().unwrap().contains("PRIVATE-DATA"));
    let typed: rxchef_client::WorkbenchProject = serde_json::from_value(normalized).unwrap();
    assert_eq!(
        server
            .client()
            .validate_project(typed)
            .unwrap()
            .graphs
            .len(),
        1
    );
    let forbidden = json!({"version":1,"name":"secret","token":"PRIVATE-TOKEN"});
    let response = server
        .http()
        .post(&endpoint)
        .bearer_auth(TOKEN)
        .json(&forbidden)
        .send()
        .unwrap();
    assert_eq!(response.status(), 400);
    assert!(!response.text().unwrap().contains("PRIVATE-TOKEN"));
}

struct TestServer {
    addr: SocketAddr,
    stop: Option<tokio::sync::oneshot::Sender<()>>,
    thread: Option<thread::JoinHandle<()>>,
}

impl TestServer {
    fn start() -> Self {
        Self::with(|_| {})
    }

    fn with(tweak: impl FnOnce(&mut ServerConfig)) -> Self {
        let (ready_tx, ready_rx) = std::sync::mpsc::channel();
        let (stop, stop_rx) = tokio::sync::oneshot::channel::<()>();
        let mut config = ServerConfig::loopback(
            "127.0.0.1:0".parse().unwrap(),
            TOKEN,
            env!("CARGO_BIN_EXE_rxchef-server").into(),
        );
        tweak(&mut config);
        let thread = thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
                let addr = listener.local_addr().unwrap();
                let mut config = config;
                // Host allow-list must name the real ephemeral port.
                config.allowed_hosts.extend([
                    format!("127.0.0.1:{}", addr.port()),
                    format!("localhost:{}", addr.port()),
                ]);
                config
                    .allowed_origins
                    .extend([format!("http://127.0.0.1:{}", addr.port())]);
                ready_tx.send(addr).unwrap();
                rxchef_server::serve(listener, config, async {
                    let _ = stop_rx.await;
                })
                .await
                .unwrap();
            });
        });
        let addr = ready_rx.recv().unwrap();
        Self {
            addr,
            stop: Some(stop),
            thread: Some(thread),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    fn client(&self) -> RemoteEngineClient {
        RemoteEngineClient::new(
            &format!("http://{}", self.addr),
            Some(TOKEN),
            Duration::from_secs(60),
        )
        .unwrap()
    }

    fn http(&self) -> reqwest::blocking::Client {
        reqwest::blocking::Client::builder()
            .no_proxy()
            .build()
            .unwrap()
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn step(op: &str, args: &[&str]) -> rxchef::execution::RecipeStep {
    rxchef::execution::RecipeStep {
        op: op.into(),
        args: args.iter().map(|s| s.to_string()).collect(),
    }
}

fn request(input: &[u8], recipe: Vec<rxchef::execution::RecipeStep>) -> ExecuteRequest {
    ExecuteRequest {
        input_base64: Some(STANDARD.encode(input)),
        recipe,
        variables: Default::default(),
        trace: None,
    }
}

#[test]
fn remote_and_local_are_byte_identical_for_all_byte_values() {
    let server = TestServer::start();
    let remote = server.client();
    let local = LocalEngineClient::default();
    let all: Vec<u8> = (0..=255).collect();
    let mut nul_and_invalid_utf8 = vec![0u8, 0xff, 0xfe, b'a', 0, 0xc3, 0x28];
    nul_and_invalid_utf8.extend(&all);
    for input in [vec![], all, nul_and_invalid_utf8, vec![0xAB; 3 << 20]] {
        for recipe in [
            vec![],
            vec![step("To Base64", &[]), step("From Base64", &[])],
            vec![step("To Hex", &["None"])],
            vec![
                step("To Base64", &[]),
                step("From Base64", &[]),
                step("Reverse", &[]),
            ],
            vec![
                step("Fork", &["\\n", "\\n"]),
                step("To Upper case", &[]),
                step("Merge", &[]),
            ],
        ] {
            let a = local.execute(request(&input, recipe.clone()));
            let b = remote.execute(request(&input, recipe));
            // Text operations may expand invalid UTF-8 past the output budget.
            // Verify identical bounded rejection rather than increasing production limits.
            if let Err(error) = a.as_ref() {
                assert_eq!(error.kind, ErrorKind::LimitExceeded);
                assert_eq!(b.unwrap_err().kind, error.kind);
                continue;
            }
            let a = a.unwrap();
            let b = b.unwrap();
            assert_eq!(
                export_bytes(&a.result).unwrap(),
                export_bytes(&b.result).unwrap()
            );
        }
    }
}

#[test]
fn discovery_and_capabilities_over_http() {
    let server = TestServer::start();
    let remote = server.client();
    let capabilities = remote.capabilities().unwrap();
    assert!(capabilities.jobs);
    let operations = remote.operations().unwrap();
    assert!(operations.len() > 400);
    assert_eq!(remote.describe("To Base64").unwrap().name, "To Base64");
    assert_eq!(
        remote.describe("No Such Operation").unwrap_err().kind,
        ErrorKind::UnknownOperation
    );
    assert!(!remote.search("base64").unwrap().is_empty());
    let health: Value = server
        .http()
        .get(server.url("/api/v1/health"))
        .send()
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(health["status"], "ok");
}

#[test]
fn recipe_errors_and_trace_are_structured_and_truncated() {
    let server = TestServer::start();
    let remote = server.client();
    let error = remote
        .execute(request(
            b"x",
            vec![step("From Base64", &[]), step("Nonexistent Op", &[])],
        ))
        .unwrap_err();
    assert!(matches!(
        error.kind,
        ErrorKind::UnknownOperation | ErrorKind::InvalidRecipe
    ));
    assert!(remote
        .validate(&request(b"x", vec![step("To Hex", &[])]))
        .is_ok());
    let mut traced = request(
        &vec![7u8; 100],
        vec![step("To Hex", &[]), step("To Base64", &[])],
    );
    traced.trace = Some(rxchef_client::TraceLimits {
        max_entries: 10,
        max_bytes_per_entry: 16,
        max_total_bytes: 1024,
    });
    let response = remote.execute(traced).unwrap();
    assert_eq!(response.trace.len(), 2);
    assert!(response
        .trace
        .iter()
        .all(|t| t.truncated && STANDARD.decode(&t.output_base64).unwrap().len() == 16));
}

#[test]
fn magic_and_scan_work_remotely() {
    let server = TestServer::start();
    let remote = server.client();
    let encoded = STANDARD.encode(STANDARD.encode(b"The quick brown fox jumps over the lazy dog"));
    let candidates = remote
        .magic(MagicRequest {
            input_base64: STANDARD.encode(encoded),
            depth: 3,
            intensive: false,
            max_results: 10,
            crib: Some("quick brown".into()),
        })
        .unwrap();
    assert!(candidates.iter().any(|c| c.matched_crib));
    let findings = remote
        .scan(ScanRequest {
            input_base64: STANDARD.encode(b"embedded SGVsbG8gV29ybGQ= payload"),
            min_len: 8,
            max_findings: 50,
            decode: false,
            depth: 2,
            crib: None,
        })
        .unwrap();
    assert!(!findings.is_empty());
}

#[test]
fn unauthenticated_and_wrongly_scoped_requests_are_refused() {
    let server = TestServer::with(|c| c.read_token = Some("read-only-token-0123456789".into()));
    let http = server.http();
    let none = http.get(server.url("/api/v1/capabilities")).send().unwrap();
    assert_eq!(none.status(), 401);
    let wrong = http
        .get(server.url("/api/v1/capabilities"))
        .bearer_auth("wrong-token-wrong-token")
        .send()
        .unwrap();
    assert_eq!(wrong.status(), 401);
    let read = RemoteEngineClient::new(
        &format!("http://{}", server.addr),
        Some("read-only-token-0123456789"),
        Duration::from_secs(10),
    )
    .unwrap();
    assert!(read.operations().is_ok());
    assert_eq!(
        read.execute(request(b"a", vec![step("To Hex", &[])]))
            .unwrap_err()
            .kind,
        ErrorKind::PolicyDenied
    );
    assert_eq!(
        read.submit_job(EngineTask::Validate(request(b"a", vec![])))
            .unwrap_err()
            .kind,
        ErrorKind::PolicyDenied
    );
}

#[test]
fn host_and_origin_are_checked() {
    let server = TestServer::start();
    let http = server.http();
    let rebinding = http
        .get(server.url("/api/v1/health"))
        .header("Host", "evil.example:80")
        .send()
        .unwrap();
    assert_eq!(rebinding.status(), 403);
    let foreign = http
        .get(server.url("/api/v1/health"))
        .header("Origin", "https://evil.example")
        .send()
        .unwrap();
    assert_eq!(foreign.status(), 403);
    assert!(foreign
        .headers()
        .get("access-control-allow-origin")
        .is_none());
    let allowed_origin = format!("http://{}", server.addr);
    let ok = http
        .get(server.url("/api/v1/health"))
        .header("Origin", &allowed_origin)
        .send()
        .unwrap();
    assert_eq!(ok.status(), 200);
    assert_eq!(
        ok.headers()["access-control-allow-origin"],
        allowed_origin.as_str()
    );
    let preflight = http
        .request(reqwest::Method::OPTIONS, server.url("/api/v1/execute"))
        .header("Origin", "https://evil.example")
        .send()
        .unwrap();
    assert_eq!(preflight.status(), 403);
}

#[test]
fn policy_blocks_host_effect_operations_everywhere() {
    let server = TestServer::start();
    let remote = server.client();
    // Direct, inside a Fork, and via Magic/jobs: all go through the same check.
    let direct = remote
        .execute(request(
            b"https://example.invalid",
            vec![step("HTTP request", &[])],
        ))
        .unwrap_err();
    assert_eq!(direct.kind, ErrorKind::PolicyDenied);
    let nested = remote
        .execute(request(
            b"a\nb",
            vec![
                step("Fork", &["\\n", "\\n"]),
                step("HTTP request", &[]),
                step("Merge", &[]),
            ],
        ))
        .unwrap_err();
    assert_eq!(nested.kind, ErrorKind::PolicyDenied);
    assert_eq!(nested.step, Some(2));
    let job = remote
        .submit_job(EngineTask::Execute(request(
            b"x",
            vec![step("http request", &[])],
        )))
        .unwrap();
    // Rejected either at submission or as a failed job, never executed.
    let done = wait_terminal(&remote, &job.id);
    assert_eq!(done.status, JobStatus::Failed);
    assert_eq!(done.error.unwrap().kind, ErrorKind::PolicyDenied);
}

fn wait_terminal(remote: &RemoteEngineClient, id: &str) -> rxchef_client::JobSnapshot {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let snapshot = remote.job(id).unwrap();
        if snapshot.status.is_terminal() {
            return snapshot;
        }
        assert!(Instant::now() < deadline, "job did not finish");
        thread::sleep(Duration::from_millis(25));
    }
}

fn wait_running(remote: &RemoteEngineClient, id: &str) {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        let snapshot = remote.job(id).unwrap();
        if snapshot.status == JobStatus::Running {
            return;
        }
        assert_eq!(
            snapshot.status,
            JobStatus::Queued,
            "job ended before running"
        );
        assert!(Instant::now() < deadline, "job did not start");
        thread::sleep(Duration::from_millis(10));
    }
}

fn slow_request() -> ExecuteRequest {
    request(
        b"",
        vec![step(
            "Derive PBKDF2 key",
            &["password", "128", "4000000000", "SHA256", "salt"],
        )],
    )
}

#[test]
fn jobs_can_be_cancelled_and_the_worker_is_killed() {
    let server = TestServer::start();
    let remote = server.client();
    let job = remote
        .submit_job(EngineTask::Execute(slow_request()))
        .unwrap();
    assert!(matches!(job.status, JobStatus::Queued | JobStatus::Running));
    wait_running(&remote, &job.id);
    let cancelled = remote.cancel_job(&job.id).unwrap();
    assert_eq!(cancelled.status, JobStatus::Cancelled);
    assert_eq!(cancelled.progress, None);
    assert_eq!(remote.job(&job.id).unwrap().status, JobStatus::Cancelled);
    // The slot is free again immediately.
    assert!(remote.run("To Hex", b"ok", vec![]).is_ok());
}

#[test]
fn deadlines_kill_cpu_bound_work() {
    let server = TestServer::with(|c| c.timeout = Duration::from_secs(2));
    let remote = server.client();
    let started = Instant::now();
    let job = remote
        .submit_job(EngineTask::Execute(slow_request()))
        .unwrap();
    let done = wait_terminal(&remote, &job.id);
    assert_eq!(done.status, JobStatus::TimedOut);
    assert!(started.elapsed() < Duration::from_secs(15));
    // Synchronous endpoints are bounded too.
    assert_eq!(
        remote.execute(slow_request()).unwrap_err().kind,
        ErrorKind::Timeout
    );
}

#[test]
fn queue_and_concurrency_are_bounded() {
    let server = TestServer::with(|c| {
        c.workers = 1;
        c.max_pending = 2;
        c.timeout = Duration::from_secs(30);
    });
    let remote = server.client();
    let first = remote
        .submit_job(EngineTask::Execute(slow_request()))
        .unwrap();
    // Submission spawns an async task: wait until the first job owns the only
    // worker permit before asserting that the next submission stays queued.
    wait_running(&remote, &first.id);
    let second = remote
        .submit_job(EngineTask::Execute(slow_request()))
        .unwrap();
    assert_eq!(
        remote
            .submit_job(EngineTask::Execute(slow_request()))
            .unwrap_err()
            .kind,
        ErrorKind::Busy
    );
    assert_eq!(remote.job(&first.id).unwrap().status, JobStatus::Running);
    assert_eq!(remote.job(&second.id).unwrap().status, JobStatus::Queued);
    // Cancelling a queued job never starts a worker.
    assert_eq!(
        remote.cancel_job(&second.id).unwrap().status,
        JobStatus::Cancelled
    );
    remote.cancel_job(&first.id).unwrap();
}

#[test]
fn parallel_jobs_complete_and_results_are_retained_boundedly() {
    let server = TestServer::with(|c| c.retain = 3);
    let remote = server.client();
    let ids: Vec<_> = (0..6)
        .map(|i| {
            remote
                .submit_job(EngineTask::Execute(request(
                    &[i],
                    vec![step("To Hex", &[])],
                )))
                .unwrap()
                .id
        })
        .collect();
    let mut found = 0;
    for id in &ids {
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            match remote.job(id) {
                Ok(s) if s.status.is_terminal() => {
                    assert_eq!(s.status, JobStatus::Succeeded);
                    found += 1;
                    break;
                }
                Ok(_) => thread::sleep(Duration::from_millis(20)),
                Err(e) => {
                    assert_eq!(e.kind, ErrorKind::NotFound);
                    break;
                }
            }
            assert!(Instant::now() < deadline);
        }
    }
    assert!(found >= 1);
    assert!(ids.iter().filter(|id| remote.job(id).is_ok()).count() <= 3);
    assert_eq!(
        remote
            .job("00000000-0000-4000-8000-000000000000")
            .unwrap_err()
            .kind,
        ErrorKind::NotFound
    );
}

#[test]
fn body_limit_and_limits_are_enforced() {
    let server = TestServer::with(|c| {
        c.max_body = 1024;
    });
    let big = server
        .http()
        .post(server.url("/api/v1/execute"))
        .bearer_auth(TOKEN)
        .body(vec![b' '; 4096])
        .send()
        .unwrap();
    assert_eq!(big.status(), 413);
    let malformed = server
        .http()
        .post(server.url("/api/v1/execute"))
        .bearer_auth(TOKEN)
        .body("{not json")
        .send()
        .unwrap();
    assert_eq!(malformed.status(), 400);
    let error: Value = malformed.json().unwrap();
    assert_eq!(error["kind"], "invalid_input");
    // Input policy limit (separate from the body limit).
    let server = TestServer::with(|c| c.policy.max_input_bytes = 10);
    let error = server
        .client()
        .run("To Hex", &[0; 100], vec![])
        .unwrap_err();
    assert_eq!(error.kind, ErrorKind::LimitExceeded);
}

#[test]
fn json_rpc_over_http() {
    let server = TestServer::start();
    let http = server.http();
    let call = |body: Value| -> Value {
        http.post(server.url("/api/v1/rpc"))
            .bearer_auth(TOKEN)
            .json(&body)
            .send()
            .unwrap()
            .json()
            .unwrap()
    };
    let reply = call(json!({"jsonrpc": "2.0", "id": 1, "method": "capabilities"}));
    assert_eq!(
        reply["result"]["protocol_version"],
        rxchef_client::PROTOCOL_VERSION
    );
    let exec = call(
        json!({"jsonrpc": "2.0", "id": "a", "method": "execute", "params": request(b"hi", vec![step("To Hex", &[])])}),
    );
    assert_eq!(exec["id"], "a");
    assert_eq!(
        STANDARD
            .decode(exec["result"]["result"]["output_base64"].as_str().unwrap())
            .unwrap(),
        b"68 69"
    );
    assert_eq!(
        call(json!({"jsonrpc": "2.0", "id": 2, "method": "nope"}))["error"]["code"],
        -32601
    );
    assert_eq!(
        call(json!({"jsonrpc": "2.0", "id": 3, "method": "execute", "params": {"bad": 1}}))
            ["error"]["code"],
        -32602
    );
    assert_eq!(
        call(json!({"id": 4, "method": "capabilities"}))["error"]["code"],
        -32600
    );
    let denied = call(
        json!({"jsonrpc": "2.0", "id": 5, "method": "execute", "params": request(b"x", vec![step("HTTP request", &[])])}),
    );
    assert_eq!(denied["error"]["data"]["kind"], "policy_denied");
    let unauth = http
        .post(server.url("/api/v1/rpc"))
        .json(&json!({"jsonrpc": "2.0", "id": 1, "method": "capabilities"}))
        .send()
        .unwrap();
    assert_eq!(unauth.status(), 401);
}

mod websocket {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::{
        connect_async,
        tungstenite::{client::IntoClientRequest, Message},
    };

    async fn recv(
        ws: &mut (impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin),
    ) -> Option<Value> {
        match tokio::time::timeout(Duration::from_secs(30), ws.next())
            .await
            .ok()??
        {
            Ok(Message::Text(text)) => serde_json::from_str(text.as_str()).ok(),
            _ => None,
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn rpc_auth_and_job_events_over_websocket() {
        let server = tokio::task::spawn_blocking(TestServer::start)
            .await
            .unwrap();
        let url = format!("ws://{}/api/v1/ws", server.addr);
        let (mut ws, _) = connect_async(&url).await.unwrap();
        ws.send(Message::Text(
            json!({"type": "auth", "token": TOKEN}).to_string().into(),
        ))
        .await
        .unwrap();
        assert_eq!(recv(&mut ws).await.unwrap()["type"], "ready");
        ws.send(Message::Text(json!({"jsonrpc": "2.0", "id": 1, "method": "execute", "params": request(&[0, 255, 1], vec![step("To Hex", &[])])}).to_string().into())).await.unwrap();
        let reply = recv(&mut ws).await.unwrap();
        assert_eq!(
            STANDARD
                .decode(reply["result"]["result"]["output_base64"].as_str().unwrap())
                .unwrap(),
            b"00 ff 01"
        );

        // Job lifecycle with real status events and a cancellation from a second request.
        ws.send(Message::Text(json!({"jsonrpc": "2.0", "id": 2, "method": "job.submit", "params": {"method": "execute", "params": slow_request()}}).to_string().into())).await.unwrap();
        let job_id = recv(&mut ws).await.unwrap()["result"]["id"]
            .as_str()
            .unwrap()
            .to_string();
        ws.send(Message::Text(
            json!({"jsonrpc": "2.0", "id": 3, "method": "job.subscribe", "params": {"id": job_id}})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
        ws.send(Message::Text(
            json!({"jsonrpc": "2.0", "id": 4, "method": "job.cancel", "params": {"id": job_id}})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
        let mut last_status = String::new();
        while last_status != "cancelled" {
            let frame = recv(&mut ws).await.expect("frame");
            if frame["type"] == "job" {
                assert!(frame["progress"].is_null(), "no fabricated progress");
                last_status = frame["status"].as_str().unwrap().to_string();
            }
        }
        drop(ws);
        tokio::task::spawn_blocking(move || drop(server))
            .await
            .unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn websocket_connection_and_inflight_limits() {
        let server = tokio::task::spawn_blocking(|| {
            TestServer::with(|c| {
                c.max_ws_connections = 2;
                c.max_ws_inflight_per_conn = 1;
                c.max_ws_subscriptions_per_conn = 2;
            })
        })
        .await
        .unwrap();
        let url = format!("ws://{}/api/v1/ws", server.addr);

        // Connect 1st client
        let (mut ws1, _) = connect_async(&url).await.unwrap();
        ws1.send(Message::Text(
            json!({"type": "auth", "token": TOKEN}).to_string().into(),
        ))
        .await
        .unwrap();
        assert_eq!(recv(&mut ws1).await.unwrap()["type"], "ready");

        // Connect 2nd client
        let (mut ws2, _) = connect_async(&url).await.unwrap();
        ws2.send(Message::Text(
            json!({"type": "auth", "token": TOKEN}).to_string().into(),
        ))
        .await
        .unwrap();
        assert_eq!(recv(&mut ws2).await.unwrap()["type"], "ready");

        // Connect 3rd client (should be rejected with 503 Busy as max_ws_connections = 2)
        assert!(connect_async(&url).await.is_err());

        // Test in-flight limit on ws1: submit a slow job in flight then an execute request
        ws1.send(Message::Text(
            json!({"jsonrpc": "2.0", "id": "slow", "method": "execute", "params": slow_request()})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
        // Sending another RPC immediately should exceed inflight limit (max 1)
        ws1.send(Message::Text(
            json!({"jsonrpc": "2.0", "id": "fast", "method": "capabilities"})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
        let reply = recv(&mut ws1).await.unwrap();
        assert_eq!(reply["id"], "fast");
        assert_eq!(reply["error"]["data"]["kind"], "busy");

        // Test subscription limit on ws2
        ws2.send(Message::Text(json!({"jsonrpc":"2.0","id":"submit","method":"job.submit","params":{"method":"execute","params":slow_request()}}).to_string().into())).await.unwrap();
        let job_id = recv(&mut ws2).await.unwrap()["result"]["id"]
            .as_str()
            .unwrap()
            .to_string();
        for id in 0..100 {
            ws2.send(Message::Text(
                json!({"jsonrpc":"2.0","id":id,"method":"job.subscribe","params":{"id":job_id}})
                    .to_string()
                    .into(),
            ))
            .await
            .unwrap();
        }
        let mut accepted = 0;
        let mut rejected = 0;
        while accepted + rejected < 100 {
            let reply = recv(&mut ws2).await.expect("subscription response");
            if reply["id"].is_number() {
                if reply.get("error").is_some() {
                    assert_eq!(reply["error"]["data"]["kind"], "limit_exceeded");
                    rejected += 1;
                } else {
                    accepted += 1;
                }
            }
        }
        assert_eq!(accepted, 2);
        assert_eq!(rejected, 98);

        drop(ws1);
        drop(ws2);
        tokio::task::spawn_blocking(move || drop(server))
            .await
            .unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn websocket_flood_disconnect_releases_workers_and_connections() {
        for _ in 0..3 {
            let server = tokio::task::spawn_blocking(|| {
                TestServer::with(|c| {
                    c.max_ws_connections = 1;
                    c.max_ws_inflight_per_conn = 1;
                    c.workers = 1;
                    c.ws_idle_timeout = Duration::from_secs(5);
                })
            })
            .await
            .unwrap();
            let url = format!("ws://{}/api/v1/ws", server.addr);
            let (mut ws, _) = connect_async(&url).await.unwrap();
            ws.send(Message::Text(
                json!({"type":"auth","token":TOKEN}).to_string().into(),
            ))
            .await
            .unwrap();
            assert_eq!(recv(&mut ws).await.unwrap()["type"], "ready");
            ws.send(Message::Text("{bad".into())).await.unwrap();
            assert_eq!(recv(&mut ws).await.unwrap()["error"]["code"], -32700);
            ws.send(Message::Text(
                json!({"jsonrpc":"2.0","id":"slow","method":"execute","params":slow_request()})
                    .to_string()
                    .into(),
            ))
            .await
            .unwrap();
            for id in 0..100 {
                // Deliberately do not read: admission and the outgoing queue stay bounded.
                if ws
                    .send(Message::Text(
                        json!({"jsonrpc":"2.0","id":id,"method":"capabilities"})
                            .to_string()
                            .into(),
                    ))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            drop(ws);
            let deadline = Instant::now() + Duration::from_secs(8);
            let mut next = loop {
                if let Ok((ws, _)) = connect_async(&url).await {
                    break ws;
                }
                assert!(Instant::now() < deadline, "connection permit was leaked");
                tokio::time::sleep(Duration::from_millis(25)).await;
            };
            next.send(Message::Text(
                json!({"type":"auth","token":TOKEN}).to_string().into(),
            ))
            .await
            .unwrap();
            assert_eq!(recv(&mut next).await.unwrap()["type"], "ready");
            next.send(Message::Text(json!({"jsonrpc":"2.0","id":"after","method":"execute","params":request(b"ok",vec![])}).to_string().into())).await.unwrap();
            assert_eq!(
                recv(&mut next).await.unwrap()["result"]["result"]["output"],
                "ok"
            );
            // Incoming inactivity closes even a client that never sends Close.
            assert!(recv(&mut next).await.is_none());
            drop(next);
            tokio::task::spawn_blocking(move || drop(server))
                .await
                .unwrap();
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn websocket_refuses_missing_bad_auth_and_foreign_origin() {
        let server = tokio::task::spawn_blocking(TestServer::start)
            .await
            .unwrap();
        let url = format!("ws://{}/api/v1/ws", server.addr);
        let (mut ws, _) = connect_async(&url).await.unwrap();
        ws.send(Message::Text(
            json!({"type": "auth", "token": "nope-nope-nope-nope"})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
        assert_eq!(recv(&mut ws).await.unwrap()["type"], "error");
        let (mut ws, _) = connect_async(&url).await.unwrap();
        ws.send(Message::Text(
            json!({"jsonrpc": "2.0", "id": 1, "method": "capabilities"})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
        assert_eq!(recv(&mut ws).await.unwrap()["type"], "error");
        let mut request = url.as_str().into_client_request().unwrap();
        request
            .headers_mut()
            .insert("Origin", "https://evil.example".parse().unwrap());
        assert!(connect_async(request).await.is_err());
        let mut request = url.as_str().into_client_request().unwrap();
        request
            .headers_mut()
            .insert("Host", "evil.example".parse().unwrap());
        assert!(connect_async(request).await.is_err());
        tokio::task::spawn_blocking(move || drop(server))
            .await
            .unwrap();
    }
}

#[test]
fn shutdown_cancels_running_jobs_and_stops_listening() {
    let server = TestServer::start();
    let remote = server.client();
    let job = remote
        .submit_job(EngineTask::Execute(slow_request()))
        .unwrap();
    wait_running(&remote, &job.id);
    let started = Instant::now();
    // The join proves serve() returned and released its owned listener. A new
    // connect to the ephemeral address can hit another parallel test server
    // after the OS reuses the port, so it cannot identify this server.
    drop(server);
    assert!(started.elapsed() < Duration::from_secs(10));
    let _ = job;
}

#[test]
fn remote_configuration_requires_explicit_opt_in() {
    use clap::Parser;
    let parse = |args: &[&str]| {
        ServerConfig::from_cli(
            Cli::parse_from(std::iter::once("rxchef-server").chain(args.iter().copied())),
            "x".into(),
        )
    };
    assert!(parse(&["--listen", "0.0.0.0:8787"]).is_err());
    assert!(parse(&["--listen", "0.0.0.0:8787", "--allow-remote"]).is_err());
    assert!(parse(&[
        "--listen",
        "0.0.0.0:8787",
        "--allow-remote",
        "--trust-tls-terminator"
    ])
    .is_err());
    assert!(parse(&[
        "--listen",
        "0.0.0.0:8787",
        "--allow-remote",
        "--trust-tls-terminator",
        "--allowed-host",
        "chef.example"
    ])
    .is_ok());
    assert!(parse(&["--token", "short"]).is_err());
    assert!(parse(&[]).unwrap().generated_token);
    let debug = format!("{:?}", parse(&["--token", "0123456789abcdef0123"]).unwrap());
    assert!(!debug.contains("0123456789abcdef0123"));

    // Overflow protection on MiB limits
    assert!(parse(&["--max-body-mib", &format!("{}", usize::MAX)]).is_err());
    assert!(parse(&["--max-input-mib", "1000"]).is_err()); // Exceeds worker IPC capacity (32 MiB)
    assert!(parse(&["--max-output-mib", "1000"]).is_err()); // Exceeds worker IPC capacity (48 MiB)
    assert!(parse(&["--max-ws-connections", "0"]).is_err());
    assert!(parse(&["--max-ws-inflight-per-conn", "0"]).is_err());
    assert!(parse(&["--ws-idle-timeout-secs", "0"]).is_err());
}

#[cfg(feature = "web-ui")]
#[test]
fn public_embedded_assets_do_not_unlock_api_or_bypass_host_checks() {
    let server = TestServer::start();
    let http = server.http();
    for path in ["/", "/index.html"] {
        let response = http.get(server.url(path)).send().unwrap();
        assert_eq!(response.status(), 200);
        assert!(response.headers()["content-type"]
            .to_str()
            .unwrap()
            .contains("text/html"));
        let html = response.text().unwrap();
        for asset in html.split('"').filter(|s| s.starts_with("/assets/")) {
            let response = http.get(server.url(asset)).send().unwrap();
            assert_eq!(response.status(), 200);
            assert!(!response.bytes().unwrap().is_empty());
        }
    }
    for path in ["/api/v1/operations", "/api/v1/jobs", "/api/unknown"] {
        assert_eq!(http.get(server.url(path)).send().unwrap().status(), 401);
    }
    let unknown = http
        .get(server.url("/api/unknown"))
        .bearer_auth(TOKEN)
        .send()
        .unwrap();
    assert_eq!(unknown.status(), 404);
    assert!(!unknown.text().unwrap().contains("<!doctype"));
    assert_eq!(
        http.get(server.url("/"))
            .header("Host", "evil.example")
            .send()
            .unwrap()
            .status(),
        403
    );
    assert_eq!(
        http.get(server.url("/"))
            .header("Origin", "https://evil.example")
            .send()
            .unwrap()
            .status(),
        403
    );
    assert_eq!(
        http.head(server.url("/"))
            .send()
            .unwrap()
            .bytes()
            .unwrap()
            .len(),
        0
    );
}

#[test]
fn shutdown_refuses_job_from_request_with_late_body() {
    use std::io::{Read, Write};
    let mut server = TestServer::start();
    let body = serde_json::to_vec(&slow_request()).unwrap();
    let mut socket = std::net::TcpStream::connect(server.addr).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    write!(socket, "POST /api/v1/execute HTTP/1.1\r\nHost: {}\r\nAuthorization: Bearer {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", server.addr, TOKEN, body.len()).unwrap();
    socket.write_all(&body[..1]).unwrap();
    thread::sleep(Duration::from_millis(100));
    server.stop.take().unwrap().send(()).unwrap();
    thread::sleep(Duration::from_millis(100));
    socket.write_all(&body[1..]).unwrap();
    let mut response = String::new();
    socket.read_to_string(&mut response).unwrap();
    assert!(response.starts_with("HTTP/1.1 503"), "{response}");
    assert!(response.contains("server is shutting down"));
    drop(server);
}

fn multi_input_graph_request() -> rxchef_client::GraphRequest {
    rxchef_client::GraphRequest {
        graph: rxchef::graph::Graph::from_json(
            include_bytes!("fixtures/three-inputs.rxgraph.json"),
            &rxchef::graph::GraphPolicy::default(),
        )
        .unwrap(),
        inputs_base64: std::collections::BTreeMap::from([
            (
                "payload".into(),
                STANDARD.encode((0u8..=255).collect::<Vec<_>>()),
            ),
            ("second".into(), STANDARD.encode([0, 255, 192])),
            ("third".into(), STANDARD.encode(b"text")),
        ]),
        outputs: vec![],
    }
}
#[test]
fn graph_rest_rpc_jobs_and_local_preserve_three_inputs_and_named_outputs() {
    let server = TestServer::start();
    let remote = server.client();
    assert!(remote.capabilities().unwrap().graphs);
    let request = multi_input_graph_request();
    remote.validate_graph(&request).unwrap();
    let local = LocalEngineClient::default()
        .execute_graph(request.clone())
        .unwrap();
    let rest = remote.execute_graph(request.clone()).unwrap();
    let rpc = server
        .http()
        .post(server.url("/api/v1/rpc"))
        .bearer_auth(TOKEN)
        .json(&json!({"jsonrpc":"2.0","id":1,"method":"graph.execute","params":request}))
        .send()
        .unwrap();
    assert!(rpc.status().is_success());
    let rpc: Value = rpc.json().unwrap();
    let rpc: rxchef_client::GraphResult = serde_json::from_value(rpc["result"].clone()).unwrap();
    let job = remote
        .submit_job(EngineTask::GraphExecute(request))
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    let job = loop {
        let state = remote.job(&job.id).unwrap();
        if state.status == JobStatus::Succeeded {
            break state;
        }
        assert_ne!(state.status, JobStatus::Failed, "{:?}", state.error);
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(10));
    };
    let Some(rxchef_client::TaskResult::Graph(job)) = job.result else {
        panic!("missing graph result")
    };
    for actual in [rest, rpc, job] {
        for expected in &local.outputs {
            let actual = actual
                .outputs
                .iter()
                .find(|o| o.name == expected.name)
                .unwrap();
            assert_eq!(
                rxchef_client::graph_output_bytes(actual).unwrap(),
                rxchef_client::graph_output_bytes(expected).unwrap()
            );
        }
        assert_eq!(
            actual
                .nodes
                .iter()
                .filter(|n| n.node_id == "encode")
                .count(),
            1
        );
    }
}
#[test]
fn graph_methods_enforce_auth_scopes_and_resource_limits() {
    let server = TestServer::with(|config| {
        config.read_token = Some("read-only-graph-token-0123456789".into());
        config.policy.max_input_bytes = 1;
    });
    let request = multi_input_graph_request();
    for endpoint in ["/api/v1/graphs/validate", "/api/v1/graphs/execute"] {
        assert_eq!(
            server
                .http()
                .post(server.url(endpoint))
                .json(&request)
                .send()
                .unwrap()
                .status(),
            401
        );
        assert_eq!(
            server
                .http()
                .post(server.url(endpoint))
                .bearer_auth("read-only-graph-token-0123456789")
                .json(&request)
                .send()
                .unwrap()
                .status(),
            403
        );
    }
    assert_eq!(
        server.client().execute_graph(request).unwrap_err().kind,
        ErrorKind::LimitExceeded
    );
}

fn slow_graph_request() -> rxchef_client::GraphRequest {
    rxchef_client::GraphRequest {
        graph: rxchef::graph::Graph::from_recipe("slow-graph".into(), slow_request().recipe),
        inputs_base64: std::collections::BTreeMap::from([("input".into(), String::new())]),
        outputs: vec![],
    }
}
#[test]
fn graph_cancellation_kills_the_worker_and_releases_capacity() {
    let server = TestServer::start();
    let remote = server.client();
    let job = remote
        .submit_job(EngineTask::GraphExecute(slow_graph_request()))
        .unwrap();
    wait_running(&remote, &job.id);
    let cancelled = remote.cancel_job(&job.id).unwrap();
    assert_eq!(cancelled.status, JobStatus::Cancelled);
    assert!(cancelled.result.is_none());
    assert!(remote.execute_graph(multi_input_graph_request()).is_ok());
}
#[test]
fn graph_deadlines_kill_cpu_bound_embedded_recipe_work() {
    let server = TestServer::with(|c| c.timeout = Duration::from_secs(1));
    let remote = server.client();
    let job = remote
        .submit_job(EngineTask::GraphExecute(slow_graph_request()))
        .unwrap();
    assert_eq!(wait_terminal(&remote, &job.id).status, JobStatus::TimedOut);
    assert_eq!(
        remote.execute_graph(slow_graph_request()).unwrap_err().kind,
        ErrorKind::Timeout
    );
}
