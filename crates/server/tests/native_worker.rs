//! Native client uses the same supervisor without opening an HTTP listener.
use rxchef_client::{
    jobs::WorkerConfig, EngineClient, EngineTask, ErrorKind, ExecuteRequest, GraphRequest,
    JobStatus, SupervisedEngineClient,
};
use std::{
    thread,
    time::{Duration, Instant},
};
fn host(timeout: Duration) -> (tokio::runtime::Runtime, SupervisedEngineClient) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut config = WorkerConfig::local(env!("CARGO_BIN_EXE_rxchef-server").into());
    config.timeout = timeout;
    config.workers = 1;
    let client = SupervisedEngineClient::new(config, runtime.handle().clone()).unwrap();
    (runtime, client)
}
fn slow() -> EngineTask {
    EngineTask::Execute(ExecuteRequest {
        input_base64: Some(String::new()),
        recipe: vec![rxchef::execution::RecipeStep {
            op: "Derive PBKDF2 key".into(),
            args: vec![
                "password".into(),
                "128".into(),
                "4000000000".into(),
                "SHA256".into(),
                "salt".into(),
            ],
        }],
        variables: Default::default(),
        trace: None,
    })
}
fn terminal(client: &SupervisedEngineClient, id: &str) -> rxchef_client::JobSnapshot {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let job = client.job(id).unwrap();
        if job.status.is_terminal() {
            return job;
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(10));
    }
}
#[test]
fn graph_events_report_real_running_nodes_before_completion_and_confirm_interruption() {
    let (runtime, client) = host(Duration::from_secs(10));
    let EngineTask::Execute(request) = slow() else {
        panic!()
    };
    let graph = rxchef::graph::Graph::from_recipe("events".into(), request.recipe);
    let job = client
        .submit_job(EngineTask::GraphExecute(GraphRequest {
            graph,
            inputs_base64: std::collections::BTreeMap::from([("input".into(), String::new())]),
            outputs: Vec::new(),
        }))
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let snapshot = client.job(&job.id).unwrap();
        assert!(snapshot.result.is_none());
        assert!(snapshot.progress.is_none());
        if snapshot.graph_nodes.iter().any(|node| {
            node.node_id == "recipe" && node.state == rxchef_client::GraphNodeState::Running
        }) {
            assert!(snapshot
                .graph_nodes
                .iter()
                .any(|node| node.node_id == "input"
                    && node.state == rxchef_client::GraphNodeState::Succeeded));
            break;
        }
        assert!(
            Instant::now() < deadline,
            "no running node event: {:?}",
            snapshot.error
        );
        thread::sleep(Duration::from_millis(10));
    }
    let ended = client.cancel_job(&job.id).unwrap();
    assert_eq!(ended.status, JobStatus::Cancelled);
    assert!(ended
        .graph_nodes
        .iter()
        .any(|node| node.node_id == "recipe"
            && node.state == rxchef_client::GraphNodeState::Cancelled));
    assert!(ended.result.is_none());
    runtime.block_on(client.shutdown());
}
#[cfg(target_os = "macos")]
#[test]
fn native_memory_guard_kills_and_reaps_a_worker_before_publishing_outputs() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut config = WorkerConfig::local(env!("CARGO_BIN_EXE_rxchef-server").into());
    config.memory_bytes = 1;
    let client = SupervisedEngineClient::new(config, runtime.handle().clone()).unwrap();
    let job = client.submit_job(slow()).unwrap();
    let ended = terminal(&client, &job.id);
    assert_eq!(ended.status, JobStatus::Failed);
    assert_eq!(ended.error.unwrap().kind, ErrorKind::LimitExceeded);
    assert!(ended.result.is_none());
    runtime.block_on(client.shutdown());
}
#[test]
fn native_worker_recipe_and_graph_are_byte_exact_without_http() {
    let (runtime, client) = host(Duration::from_secs(10));
    assert!(client.capabilities().unwrap().jobs);
    let result = client
        .execute(ExecuteRequest {
            input_base64: Some("AP/A".into()),
            recipe: vec![],
            variables: Default::default(),
            trace: None,
        })
        .unwrap();
    assert_eq!(
        rxchef_client::export_bytes(&result.result).unwrap(),
        [0, 255, 192]
    );
    let graph = rxchef::graph::Graph::from_recipe("native".into(), vec![]);
    let result = client
        .execute_graph(GraphRequest {
            graph,
            inputs_base64: std::collections::BTreeMap::from([("input".into(), "AP/A".into())]),
            outputs: vec![],
        })
        .unwrap();
    assert_eq!(
        rxchef_client::graph_output_bytes(&result.outputs[0]).unwrap(),
        [0, 255, 192]
    );
    runtime.block_on(client.shutdown());
}
#[test]
fn native_cancel_running_and_queued_jobs_releases_capacity() {
    let (runtime, client) = host(Duration::from_secs(30));
    for attempt in 0..3 {
        let task = if attempt == 1 {
            let EngineTask::Execute(request) = slow() else {
                unreachable!()
            };
            EngineTask::GraphExecute(GraphRequest {
                graph: rxchef::graph::Graph::from_recipe("slow".into(), request.recipe),
                inputs_base64: std::collections::BTreeMap::from([("input".into(), String::new())]),
                outputs: vec![],
            })
        } else {
            slow()
        };
        let running = client.submit_job(task).unwrap();
        thread::sleep(Duration::from_millis(100));
        let queued = client.submit_job(slow()).unwrap();
        assert_eq!(
            client.cancel_job(&queued.id).unwrap().status,
            JobStatus::Cancelled
        );
        assert_eq!(
            client.cancel_job(&running.id).unwrap().status,
            JobStatus::Cancelled
        );
        assert!(terminal(&client, &running.id).result.is_none());
        assert!(client.run("To Hex", b"ok", vec![]).is_ok());
    }
    runtime.block_on(client.shutdown());
}
#[test]
fn native_timeout_shutdown_and_missing_executable_are_structured() {
    let (runtime, client) = host(Duration::from_millis(100));
    let id = client.submit_job(slow()).unwrap().id;
    assert_eq!(terminal(&client, &id).status, JobStatus::TimedOut);
    let id = client.submit_job(slow()).unwrap().id;
    runtime.block_on(client.shutdown());
    assert_eq!(terminal(&client, &id).status, JobStatus::Cancelled);
    assert_eq!(client.submit_job(slow()).unwrap_err().kind, ErrorKind::Busy);
    let mut config = WorkerConfig::local(std::path::PathBuf::from("/nonexistent/rxchef-worker"));
    config.timeout = Duration::from_secs(1);
    let broken = SupervisedEngineClient::new(config, runtime.handle().clone()).unwrap();
    assert_eq!(
        terminal(&broken, &broken.submit_job(slow()).unwrap().id).status,
        JobStatus::Failed
    );
    runtime.block_on(broken.shutdown());
}
