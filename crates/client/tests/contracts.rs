use base64::{engine::general_purpose::STANDARD, Engine as _};
use rxchef::execution::RecipeStep;
use rxchef_client::*;

fn request(input: &[u8], operations: &[&str]) -> ExecuteRequest {
    ExecuteRequest {
        input_base64: Some(STANDARD.encode(input)),
        recipe: operations
            .iter()
            .map(|op| RecipeStep {
                op: (*op).into(),
                args: vec![],
            })
            .collect(),
        variables: Default::default(),
        trace: Some(TraceLimits::default()),
    }
}

#[test]
fn exact_all_byte_values_and_invalid_utf8_survive_execution_and_json() {
    let input: Vec<u8> = (0..=255).collect();
    let outcome = LocalEngineClient::default()
        .execute(request(&input, &["To Base64", "From Base64"]))
        .unwrap();
    let wire = serde_json::to_vec(&outcome).unwrap();
    let decoded: ExecuteResponse = serde_json::from_slice(&wire).unwrap();
    assert_eq!(export_bytes(&decoded.result).unwrap(), input);
    assert!(!decoded.result.output_is_utf8);
    assert_eq!(
        STANDARD.decode(&decoded.trace[0].output_base64).unwrap(),
        STANDARD.encode(&input).as_bytes()
    );
    assert_eq!(
        STANDARD.decode(&decoded.trace[1].output_base64).unwrap(),
        input
    );
}

#[test]
fn fork_trace_retains_real_branch_outputs_and_recipe_indices() {
    let mut input = request(b"one\ntwo", &["Fork", "To Upper case", "Merge"]);
    input.recipe[0].args = vec!["\\n".into(), "|".into(), "false".into()];
    let outcome = LocalEngineClient::default().execute(input).unwrap();
    assert_eq!(export_bytes(&outcome.result).unwrap(), b"ONE|TWO");
    assert_eq!(
        outcome
            .trace
            .iter()
            .map(|entry| entry.step_index)
            .collect::<Vec<_>>(),
        vec![1, 1, 0]
    );
    let outputs: Vec<_> = outcome
        .trace
        .iter()
        .map(|entry| STANDARD.decode(&entry.output_base64).unwrap())
        .collect();
    assert_eq!(
        outputs,
        vec![b"ONE".to_vec(), b"TWO".to_vec(), b"ONE|TWO".to_vec()]
    );
}

#[test]
fn trace_limits_bound_payloads_without_truncating_result() {
    let mut input = request(b"abcdef", &["To Base64", "From Base64", "To Base64"]);
    input.trace = Some(TraceLimits {
        max_entries: 2,
        max_bytes_per_entry: 3,
        max_total_bytes: 4,
    });
    let outcome = LocalEngineClient::default().execute(input).unwrap();
    assert_eq!(outcome.trace.len(), 2);
    assert_eq!(
        outcome
            .trace
            .iter()
            .map(|entry| STANDARD.decode(&entry.output_base64).unwrap().len())
            .sum::<usize>(),
        4
    );
    assert!(outcome.trace_truncated);
    assert!(outcome.trace.iter().all(|entry| entry.truncated));
    assert_eq!(export_bytes(&outcome.result).unwrap(), b"YWJjZGVm");
}

#[test]
fn trace_payloads_are_opt_in() {
    let mut input = request(b"secret", &["To Base64"]);
    input.trace = None;
    let outcome = LocalEngineClient::default().execute(input).unwrap();
    assert!(outcome.trace.is_empty());
    assert!(!outcome.trace_truncated);
}

#[test]
fn validation_does_not_execute_and_preserves_error_types() {
    let client = LocalEngineClient::default();
    assert_eq!(
        client
            .validate(&request(b"", &["not-an-operation"]))
            .unwrap_err()
            .kind,
        ErrorKind::UnknownOperation
    );
    assert_eq!(
        client.validate(&request(b"", &["Merge"])).unwrap_err().kind,
        ErrorKind::InvalidRecipe
    );
    assert_eq!(
        client
            .validate(&request(b"", &["AES Encrypt"]))
            .unwrap_err()
            .kind,
        ErrorKind::InvalidArgument
    );
    let mut invalid = request(b"", &[]);
    invalid.input_base64 = Some("%%%".into());
    assert_eq!(
        client.execute(invalid).unwrap_err().kind,
        ErrorKind::InvalidInput
    );
}

#[test]
fn host_effects_are_denied_before_execution_even_through_aliases() {
    let client = LocalEngineClient::default();
    let host_ops: Vec<_> = client
        .operations()
        .unwrap()
        .into_iter()
        .filter(|op| {
            op.side_effects.iter().any(|effect| {
                matches!(
                    effect,
                    rxchef::SideEffect::Network
                        | rxchef::SideEffect::FilesystemRead
                        | rxchef::SideEffect::FilesystemWrite
                        | rxchef::SideEffect::ExternalProcess
                )
            })
        })
        .collect();
    assert!(!host_ops.is_empty());
    for op in host_ops {
        let error = client.validate(&request(b"", &[&op.id])).unwrap_err();
        assert_eq!(error.kind, ErrorKind::PolicyDenied, "{}", op.name);
        assert_eq!(error.step, Some(1));
    }
}

#[test]
fn operation_allowlist_uses_canonical_ids() {
    let mut client = LocalEngineClient::default();
    let id = client.describe("To Base64").unwrap().id;
    client.policy.allowed_operations = Some([id].into());
    assert!(client.validate(&request(b"hi", &["to_base64"])).is_ok());
    assert_eq!(
        client
            .validate(&request(b"hi", &["From Base64"]))
            .unwrap_err()
            .kind,
        ErrorKind::PolicyDenied
    );
}

#[test]
fn input_output_and_step_limits_are_structured() {
    let mut client = LocalEngineClient::default();
    client.policy.max_input_bytes = 2;
    assert_eq!(
        client.execute(request(b"abc", &[])).unwrap_err().kind,
        ErrorKind::LimitExceeded
    );
    client.policy.max_input_bytes = 10;
    client.policy.max_output_bytes = 3;
    assert_eq!(
        client
            .execute(request(b"ab", &["To Base64"]))
            .unwrap_err()
            .kind,
        ErrorKind::LimitExceeded
    );
    client.policy.max_steps = 0;
    assert_eq!(
        client
            .execute(request(b"a", &["To Base64"]))
            .unwrap_err()
            .kind,
        ErrorKind::LimitExceeded
    );
}

#[test]
fn exports_ignore_lossy_preview_and_check_payload_length() {
    let mut result = rxchef::integration::ExecutionResult::from_bytes(vec![0, 255, 192]);
    result.output = "not the original bytes".into();
    assert_eq!(export_bytes(&result).unwrap(), vec![0, 255, 192]);
    result.output_len = 99;
    assert_eq!(export_bytes(&result).unwrap_err().kind, ErrorKind::Protocol);
}

#[test]
fn large_binary_identity_does_not_reconstruct_text() {
    let input: Vec<u8> = (0..2 * 1024 * 1024).map(|index| index as u8).collect();
    let outcome = LocalEngineClient::default()
        .execute(request(&input, &[]))
        .unwrap();
    assert_eq!(export_bytes(&outcome.result).unwrap(), input);
}

#[cfg(feature = "remote")]
#[test]
fn remote_rejects_insecure_origins_and_embedded_secrets() {
    use std::time::Duration;
    for origin in [
        "http://example.com",
        "http://localhost",
        "https://user:secret@example.com",
        "https://example.com/?token=secret",
        "https://example.com/path",
        "file:///tmp/socket",
    ] {
        assert!(
            RemoteEngineClient::new(origin, None, Duration::from_secs(1)).is_err(),
            "{origin}"
        );
    }
    assert!(RemoteEngineClient::new(
        "http://127.0.0.1:9876",
        Some("\r\nsecret"),
        Duration::from_secs(1)
    )
    .is_err());
    assert!(RemoteEngineClient::new("http://[::1]:9876", None, Duration::from_secs(1)).is_ok());
}
