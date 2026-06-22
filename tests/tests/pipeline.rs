// Pipeline integration tests.
//
// These tests verify that operations chain correctly through the Pipeline API,
// including automatic type coercions at operation boundaries.
//
// Run:  cargo test -p cyberchef-rust-tests --test pipeline
use rxchef::operation::{ArgValue, OperationData};
use rxchef::operations::get_operation;
use rxchef::pipeline::Pipeline;

fn op(name: &str) -> Box<dyn rxchef::operation::Operation> {
    get_operation(name).unwrap_or_else(|| panic!("operation not found: {name}"))
}

// ─── Roundtrip tests ─────────────────────────────────────────────────────────

#[test]
fn hex_roundtrip() {
    let result = Pipeline::new()
        .then(
            op("To Hex"),
            vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
        )
        .then(op("From Hex"), vec![ArgValue::Str("Auto".into())])
        .run_text("Hello, World!")
        .unwrap();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn base64_roundtrip() {
    let result = Pipeline::new()
        .then(op("To Base64"), vec![ArgValue::Str("A-Za-z0-9+/=".into())])
        .then(
            op("From Base64"),
            vec![ArgValue::Str("A-Za-z0-9+/=".into()), ArgValue::Bool(false)],
        )
        .run_text("the quick brown fox")
        .unwrap();
    assert_eq!(result, "the quick brown fox");
}

#[test]
fn three_step_encode_decode_hash() {
    // Hex-encode, then re-decode, then SHA1 — verifies 3-step pipeline
    let pipeline = Pipeline::new()
        .then(
            op("To Hex"),
            vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
        )
        .then(op("From Hex"), vec![ArgValue::Str("Auto".into())])
        .then(op("SHA1"), vec![]);
    let result = pipeline.run_text("Hello").unwrap();
    // SHA1("Hello")
    assert_eq!(result, "f7ff9e8b7bb2e09b70935a5d785e0cc5d9d0abf0");
}

// ─── Type coercion tests ──────────────────────────────────────────────────────

#[test]
fn bytes_coerced_to_text_between_ops() {
    // To Hex outputs DataType::String (text); feeding it again to To Hex
    // requires coercion of the output string back to bytes.
    let result = Pipeline::new()
        .then(
            op("To Hex"),
            vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
        )
        .then(
            op("To Hex"),
            vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
        )
        .run_text("A")
        .unwrap();
    // "A" → hex "41" → hex of "41" → "3431"
    assert_eq!(result, "3431");
}

#[test]
fn operationdata_text_coerces_to_bytes() {
    let data = OperationData::Text("41".to_owned());
    let bytes = data.coerce_to(rxchef::operation::DataType::Bytes).unwrap();
    match bytes {
        OperationData::Bytes(b) => assert_eq!(b, b"41"),
        _ => panic!("expected Bytes"),
    }
}

#[test]
fn operationdata_bytes_coerces_to_text() {
    let data = OperationData::Bytes(b"hello".to_vec());
    let text = data.coerce_to(rxchef::operation::DataType::String).unwrap();
    match text {
        OperationData::Text(s) => assert_eq!(s, "hello"),
        _ => panic!("expected Text"),
    }
}

#[test]
fn operationdata_text_coerces_to_number() {
    let data = OperationData::Text("  42.5  ".to_owned());
    let num = data.coerce_to(rxchef::operation::DataType::Number).unwrap();
    match num {
        OperationData::Number(n) => assert!((n - 42.5).abs() < f64::EPSILON),
        _ => panic!("expected Number"),
    }
}

#[test]
fn operationdata_json_coerces_to_text() {
    let data = OperationData::Json(serde_json::json!({"key": "value"}));
    let text = data.coerce_to(rxchef::operation::DataType::String).unwrap();
    match text {
        OperationData::Text(s) => assert!(s.contains("key")),
        _ => panic!("expected Text"),
    }
}

// ─── Empty pipeline ───────────────────────────────────────────────────────────

#[test]
fn empty_pipeline_is_passthrough() {
    let result = Pipeline::new().run_text("unchanged").unwrap();
    assert_eq!(result, "unchanged");
}

#[test]
fn empty_pipeline_bytes_passthrough() {
    let input = vec![0xDE_u8, 0xAD, 0xBE, 0xEF];
    let result = Pipeline::new().run_bytes(input.clone()).unwrap();
    assert_eq!(result, input);
}

// ─── Error propagation ───────────────────────────────────────────────────────

#[test]
fn pipeline_error_identifies_failing_step() {
    let err = Pipeline::new()
        .then(
            op("From Base64"),
            vec![ArgValue::Str("A-Za-z0-9+/=".into()), ArgValue::Bool(false)],
        )
        .run_text("this is not base64!!!")
        .unwrap_err();
    assert_eq!(err.step_index, 0);
    assert_eq!(err.step_name, "From Base64");
}

#[test]
fn rc4_roundtrip() {
    let rc4_enc = op("RC4");
    let rc4_dec = get_operation("RC4").unwrap();
    let args = vec![
        ArgValue::Str("mysecretkey".into()),
        ArgValue::Str("Raw".into()),
        ArgValue::Str("Raw".into()),
    ];
    let encrypted = rc4_enc
        .run(b"Secret data for RC4 roundtrip".to_vec(), &args)
        .unwrap();
    let decrypted = rc4_dec.run(encrypted, &args).unwrap();
    assert_eq!(decrypted, b"Secret data for RC4 roundtrip");
}
