use rxchef::{
    operation::{ArgKind, Availability, NumericBound},
    runtime::{self, data_type_name},
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeSet, HashSet},
    env, fs,
    path::PathBuf,
};

const REPRESENTATIVE_BENCHMARKS: &[&str] = &[
    "to_hex",
    "to_base64",
    "sha2",
    "aes_encrypt",
    "gzip_compress",
    "magic",
    "scan",
];

/// Create the initial explicit inventory. This command is deliberately
/// conservative: it maps each operation to its existing dedicated test module,
/// records only the representative benchmark catalog, and makes no KAT,
/// differential, property, or fuzz claim. Reviewers add those claims manually.
pub fn generate_manifest() -> Result<(), String> {
    let root = workspace_root()?;
    let mut operations = serde_json::Map::new();
    for name in runtime::operation_names(None) {
        let info = runtime::operation_info(&name)?;
        let source_id = runtime::operation_source(&name)?;
        let benchmark = REPRESENTATIVE_BENCHMARKS.contains(&info.id.as_str());
        operations.insert(
            info.id,
            json!({
                "correctness": [format!("tests/tests/operations/{source_id}.rs")],
                "known_answer": [],
                "differential": [],
                "property": [],
                "fuzz": [],
                "benchmark": if benchmark { vec!["representative release benchmark"] } else { Vec::<&str>::new() },
                "benchmark_skip_reason": if benchmark { Value::Null } else { json!("No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.") },
            }),
        );
    }
    let destination = root.join("verification/operations.json");
    fs::create_dir_all(destination.parent().unwrap()).map_err(|e| e.to_string())?;
    let document = json!({"schema_version": 1, "operations": operations});
    fs::write(
        &destination,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&document).map_err(|e| e.to_string())?
        ),
    )
    .map_err(|e| e.to_string())?;
    println!("wrote {}", destination.display());
    Ok(())
}

fn workspace_root() -> Result<PathBuf, String> {
    Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?).join("../.."))
}

pub fn run() -> Result<(), String> {
    let root = workspace_root()?;
    let manifest_path = root.join("verification/operations.json");
    let manifest: Value = serde_json::from_str(
        &fs::read_to_string(&manifest_path)
            .map_err(|e| format!("cannot read {}: {e}", manifest_path.display()))?,
    )
    .map_err(|e| format!("invalid {}: {e}", manifest_path.display()))?;
    if manifest["schema_version"] != 1 {
        return Err("verification manifest schema_version must be 1".into());
    }
    let evidence = manifest["operations"]
        .as_object()
        .ok_or_else(|| "verification manifest operations must be an object".to_string())?;
    let names = runtime::operation_names(None);
    let mut errors = Vec::new();
    let mut seen_names = HashSet::new();
    let mut seen_ids = HashSet::new();
    let mut rows = Vec::with_capacity(names.len());

    for name in names {
        let info = runtime::operation_info(&name)?;
        if !seen_names.insert(info.name.to_ascii_lowercase()) {
            errors.push(format!("duplicate operation name: {}", info.name));
        }
        if !seen_ids.insert(info.id.clone()) {
            errors.push(format!("duplicate normalized operation id: {}", info.id));
        }
        if info.name.trim().is_empty() || info.module.trim().is_empty() {
            errors.push(format!("operation without metadata: {}", info.id));
        }
        if info.description.trim().is_empty() {
            errors.push(format!("empty description: {}", info.name));
        }
        if info.is_broken != (info.availability == Availability::FeatureDisabled) {
            errors.push(format!("is_broken/availability mismatch: {}", info.name));
        }
        if info.availability == Availability::FeatureDisabled
            && info.feature_requirements.is_empty()
        {
            errors.push(format!(
                "feature-gated operation without feature metadata: {}",
                info.name
            ));
        }

        for argument in info.args {
            if argument.name.trim().is_empty() || argument.description.trim().is_empty() {
                errors.push(format!(
                    "operation has undocumented arguments: {}",
                    info.name
                ));
            }
            if argument.kind == ArgKind::Enum && argument.choices.is_empty() {
                errors.push(format!(
                    "enum argument without choices: {} / {}",
                    info.name, argument.name
                ));
            }
            if argument.kind != ArgKind::Enum && !argument.choices.is_empty() {
                errors.push(format!(
                    "non-enum argument has choices: {} / {}",
                    info.name, argument.name
                ));
            }
            if !argument.choices.is_empty()
                && !argument.default_value.is_empty()
                && !argument
                    .choices
                    .iter()
                    .any(|choice| choice.eq_ignore_ascii_case(argument.default_value))
            {
                errors.push(format!(
                    "default is not an allowed choice: {} / {}",
                    info.name, argument.name
                ));
            }
            if argument.required && !argument.default_value.is_empty() {
                errors.push(format!(
                    "required argument also declares a default: {} / {}",
                    info.name, argument.name
                ));
            }
            if let (Some(minimum), Some(maximum)) = (argument.minimum, argument.maximum) {
                if bound_value(minimum) > bound_value(maximum) {
                    errors.push(format!(
                        "invalid numeric bounds: {} / {}",
                        info.name, argument.name
                    ));
                }
            }
            if (argument.minimum.is_some() || argument.maximum.is_some())
                && !matches!(
                    argument.kind,
                    ArgKind::Integer | ArgKind::UnsignedInteger | ArgKind::Float
                )
            {
                errors.push(format!(
                    "non-numeric argument has numeric bounds: {} / {}",
                    info.name, argument.name
                ));
            }
        }

        let source_id = runtime::operation_source(&name)?;
        let source = root.join(format!("src/operations/{source_id}.rs"));
        if !source.is_file() {
            errors.push(format!(
                "missing module for {}: {}",
                info.name,
                source.display()
            ));
            continue;
        }
        let source_text = fs::read_to_string(&source).map_err(|e| e.to_string())?;
        for marker in ["todo!", "unimplemented!", "stub implementation"] {
            if source_text.to_ascii_lowercase().contains(marker) {
                errors.push(format!(
                    "placeholder marker '{marker}' in {}",
                    source.display()
                ));
            }
        }

        let Some(verification) = evidence.get(&info.id) else {
            errors.push(format!(
                "operation without verification evidence: {}",
                info.name
            ));
            continue;
        };
        let correctness = string_array(verification, "correctness", &info.name, &mut errors);
        let known_answer = string_array(verification, "known_answer", &info.name, &mut errors);
        let differential = string_array(verification, "differential", &info.name, &mut errors);
        let property = string_array(verification, "property", &info.name, &mut errors);
        let fuzz = string_array(verification, "fuzz", &info.name, &mut errors);
        let benchmark_entries = string_array(verification, "benchmark", &info.name, &mut errors);
        if correctness.is_empty() {
            errors.push(format!(
                "operation without correctness evidence: {}",
                info.name
            ));
        }
        for mapping in &correctness {
            if !root.join(mapping).is_file() {
                errors.push(format!(
                    "missing correctness evidence for {}: {mapping}",
                    info.name
                ));
            }
        }
        let docs = root.join(format!(
            "docs/operations/{}.md",
            info.name.replace('/', "_")
        ));
        if !docs.is_file() {
            errors.push(format!("operation without docs: {}", info.name));
        }
        let benchmark_skip_reason = verification["benchmark_skip_reason"].as_str();
        if benchmark_entries.is_empty()
            && benchmark_skip_reason.is_none_or(|reason| reason.trim().is_empty())
        {
            errors.push(format!(
                "operation without benchmark case or skip reason: {}",
                info.name
            ));
        }

        rows.push(json!({
            "name": info.name,
            "slug": info.id.replace('_', "-"),
            "id": info.id,
            "module": info.module,
            "status": info.implementation_status,
            "availability": info.availability,
            "feature": info.feature_requirements,
            "input": data_type_name(info.input_type),
            "output": data_type_name(info.output_type),
            "args_documented": info.args.iter().all(|arg| !arg.name.trim().is_empty() && !arg.description.trim().is_empty()),
            "correctness_test": !correctness.is_empty(),
            "known_answer_test": !known_answer.is_empty(),
            "differential_test": !differential.is_empty(),
            "property_test": !property.is_empty(),
            "test_mapping": correctness,
            "known_answer": known_answer,
            "differential": differential,
            "property": property,
            "fuzz": fuzz,
            "fuzz_target": !fuzz.is_empty(),
            "benchmark": !benchmark_entries.is_empty(),
            "benchmark_evidence": benchmark_entries,
            "benchmark_skip_reason": benchmark_skip_reason,
            "docs": docs.is_file(),
            "parity": info.parity,
        }));
    }

    for id in evidence.keys() {
        if !seen_ids.contains(id) {
            errors.push(format!(
                "verification evidence references unknown operation id: {id}"
            ));
        }
    }

    if !errors.is_empty() {
        errors.sort();
        return Err(format!(
            "operation audit failed with {} issue(s):\n{}",
            errors.len(),
            errors.join("\n")
        ));
    }

    rows.sort_by(|left, right| text_field(left, "name").cmp(text_field(right, "name")));
    let generated = root.join("docs/_generated/operation-quality.json");
    let reference = root.join("docs/reference/operation-matrix.md");
    fs::create_dir_all(generated.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::create_dir_all(reference.parent().unwrap()).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&json!({
        "schema_version": 1,
        "operation_count": rows.len(),
        "operations": rows,
    }))
    .map_err(|e| e.to_string())?;
    fs::write(&generated, format!("{json}\n")).map_err(|e| e.to_string())?;
    fs::write(&reference, render_markdown(&rows)).map_err(|e| e.to_string())?;
    println!("operation audit passed ({} operations)", rows.len());
    Ok(())
}

fn bound_value(bound: NumericBound) -> f64 {
    match bound {
        NumericBound::Integer(value) => value as f64,
        NumericBound::Unsigned(value) => value as f64,
        NumericBound::Float(value) => value,
    }
}

fn string_array(
    value: &Value,
    field: &str,
    operation: &str,
    errors: &mut Vec<String>,
) -> Vec<String> {
    let Some(array) = value[field].as_array() else {
        errors.push(format!(
            "verification field '{field}' is not an array: {operation}"
        ));
        return Vec::new();
    };
    let mut output = Vec::with_capacity(array.len());
    for entry in array {
        if let Some(entry) = entry.as_str().filter(|entry| !entry.trim().is_empty()) {
            output.push(entry.to_string());
        } else {
            errors.push(format!(
                "verification field '{field}' has an invalid entry: {operation}"
            ));
        }
    }
    output
}

fn text_field<'a>(value: &'a Value, field: &str) -> &'a str {
    value[field].as_str().unwrap_or("")
}

fn render_markdown(rows: &[Value]) -> String {
    let mut modules = BTreeSet::new();
    for row in rows {
        modules.insert(text_field(row, "module"));
    }
    let mut output = format!(
        "# Operation quality matrix\n\n<!-- Generated by `cargo xtask audit-operations`; do not edit. -->\n\nRegistered operations: **{}**. `partial` and `unknown` are deliberate audit results, not release-completeness claims.\n\n",
        rows.len()
    );
    let module_count = modules.len();
    for (module_index, module) in modules.into_iter().enumerate() {
        output.push_str(&format!("## {module}\n\n"));
        output.push_str("| Operation | Status | Parity | Args | Test | KAT | Diff | Property | Fuzz | Bench | Docs |\n|---|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|\n");
        for row in rows
            .iter()
            .filter(|row| text_field(row, "module") == module)
        {
            output.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                text_field(row, "name").replace('|', "\\|"),
                text_field(row, "status"),
                text_field(row, "parity"),
                mark(row["args_documented"].as_bool().unwrap_or(false)),
                mark(row["correctness_test"].as_bool().unwrap_or(false)),
                mark(row["known_answer_test"].as_bool().unwrap_or(false)),
                mark(row["differential_test"].as_bool().unwrap_or(false)),
                mark(row["property_test"].as_bool().unwrap_or(false)),
                mark(row["fuzz_target"].as_bool().unwrap_or(false)),
                mark(row["benchmark"].as_bool().unwrap_or(false)),
                mark(row["docs"].as_bool().unwrap_or(false)),
            ));
        }
        if module_index + 1 < module_count {
            output.push('\n');
        }
    }
    output
}

fn mark(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "—"
    }
}
