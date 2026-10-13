//! Generate authoritative wire schemas and representative runtime fixtures.
use rxchef::integration::{ArgumentDescriptor, ExecutionResult, OperationDescriptor};
use rxchef_client::*;
use schemars::JsonSchema;
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Serialize, JsonSchema)]
struct Contracts {
    operations: Vec<OperationDescriptor>,
    argument: ArgumentDescriptor,
    result: ExecutionResult,
    request: ExecuteRequest,
    response: ExecuteResponse,
    capabilities: Capabilities,
    magic_request: MagicRequest,
    magic: Vec<MagicCandidate>,
    scan_request: ScanRequest,
    scan: Vec<rxchef::scan::Finding>,
    error: ClientError,
    job: JobSnapshot,
    task: EngineTask,
}
fn main() {
    let client = LocalEngineClient::default();
    let mut operations = client.operations().unwrap();
    // Exercise alias arrays as a protocol shape, without claiming new registry aliases.
    let mut alias_fixture = client.describe("To Hex").unwrap();
    alias_fixture.aliases = vec!["to_hex".into(), "to-hex".into()];
    operations.push(alias_fixture);
    let request = ExecuteRequest {
        input_base64: Some("AP/A".into()),
        recipe: vec![],
        variables: Default::default(),
        trace: Some(TraceLimits::default()),
    };
    let response = client.execute(request.clone()).unwrap();
    let fixture = Contracts {
        task: EngineTask::Execute(request.clone()),
        operations,
        argument: client.describe("AES Encrypt").unwrap().args.remove(0),
        result: response.result.clone(),
        request,
        response,
        capabilities: client.capabilities().unwrap(),
        magic_request: serde_json::from_value(serde_json::json!({"input_base64":"U0dWc2JHOD0="}))
            .unwrap(),
        magic: client
            .magic(
                serde_json::from_value(serde_json::json!({"input_base64":"U0dWc2JHOD0="})).unwrap(),
            )
            .unwrap(),
        scan_request: serde_json::from_value(
            serde_json::json!({"input_base64":"U0dWc2JHOGdWMjl5YkdRPQ=="}),
        )
        .unwrap(),
        scan: client
            .scan(
                serde_json::from_value(
                    serde_json::json!({"input_base64":"U0dWc2JHOGdWMjl5YkdRPQ=="}),
                )
                .unwrap(),
            )
            .unwrap(),
        error: ClientError {
            kind: ErrorKind::InvalidArgument,
            message: "invalid argument".into(),
            step: Some(2),
        },
        job: JobSnapshot {
            id: "example".into(),
            status: JobStatus::Failed,
            progress: None,
            result: None,
            error: Some(ClientError::new(ErrorKind::Operation, "failed")),
        },
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../apps/web/src/generated");
    let check = std::env::args().any(|arg| arg == "--check");
    for (name, value) in [
        (
            "contracts.schema.json",
            serde_json::to_value(schemars::schema_for!(Contracts)).unwrap(),
        ),
        (
            "contracts.fixture.json",
            serde_json::to_value(fixture).unwrap(),
        ),
    ] {
        let text = format!("{}\n", serde_json::to_string_pretty(&value).unwrap());
        if check {
            assert_eq!(
                fs::read_to_string(root.join(name)).unwrap(),
                text,
                "stale contract: {name}"
            );
        } else {
            fs::create_dir_all(&root).unwrap();
            fs::write(root.join(name), text).unwrap();
        }
    }
}
